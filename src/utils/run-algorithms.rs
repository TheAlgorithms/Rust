//! run-algorithms — execute `cargo test` for algorithms listed in a CSV file.
//!
//! Usage:
//!   cargo run --bin run-algorithms -- [--startnum N] [--num2run N]
//!
//! Arguments:
//!   --startnum N   1-based row number to start from (default: 1)
//!   --num2run  N   how many algorithms to run     (default: 1)
//!
//! The CSV file (`algorithms-001.csv`) must be in the project root.
//! Required columns (resolved by header name, order does not matter):
//!   seq, status, category1, category2, name, filename, wikipedia
//!
//! After each test the `status` column is updated to PASS or FAIL and the
//! file is written back to disk, so progress survives an interrupted run.

use std::env;
use std::fs;
use std::process::{self, Command};

// ── defaults ──────────────────────────────────────────────────────────────────
const CSV_FILE: &str = "algorithms-001.csv";
const DEFAULT_STARTNUM: usize = 1;
const DEFAULT_NUM2RUN: usize = usize::MAX; // run all rows from startnum when not specified

// ── CSV data ──────────────────────────────────────────────────────────────────
/// Holds the entire CSV in memory as raw string rows so every field
/// (including `status`) can be updated and written back to disk.
struct CsvData {
    headers:    Vec<String>,
    rows:       Vec<Vec<String>>,
    idx_cat1:   usize,
    idx_cat2:   usize,
    idx_name:   usize,
    idx_file:   usize,
    idx_status: usize,
}

impl CsvData {
    fn total(&self) -> usize { self.rows.len() }

    /// Extract an `AlgorithmEntry` view from one raw row.
    fn entry(&self, i: usize) -> AlgorithmEntry {
        let r = &self.rows[i];
        AlgorithmEntry {
            category1: r[self.idx_cat1].clone(),
            category2: r[self.idx_cat2].clone(),
            name:      r[self.idx_name].clone(),
            filename:  r[self.idx_file].clone(),
        }
    }

    /// Write "PASS" or "FAIL" into the status cell for row `i` and
    /// immediately persist the whole file to disk.
    fn set_status(&mut self, i: usize, status: &str, path: &str) {
        self.rows[i][self.idx_status] = status.to_string();
        self.write(path);
    }

    /// Serialise every row back to the CSV file.
    fn write(&self, path: &str) {
        let mut out = self.headers.join(",");
        out.push('\n');
        for row in &self.rows {
            out.push_str(&csv_join(row));
            out.push('\n');
        }
        fs::write(path, out).unwrap_or_else(|e| {
            eprintln!("Error: cannot write '{path}': {e}");
            process::exit(1);
        });
    }
}

/// Join fields with commas, quoting any field that contains a comma or quote.
fn csv_join(fields: &[String]) -> String {
    fields
        .iter()
        .map(|f| {
            if f.contains(',') || f.contains('"') || f.contains('\n') {
                format!("\"{}\"", f.replace('"', "\"\""))
            } else {
                f.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(",")
}

// ── algorithm entry ───────────────────────────────────────────────────────────
struct AlgorithmEntry {
    category1: String,
    category2: String,
    name:      String,
    filename:  String,
}

impl AlgorithmEntry {
    /// Build the `cargo test` filter string.
    ///
    /// `category2` is metadata-only: included in the filter only when the file
    /// physically lives at `src/category1/category2/filename`, avoiding false
    /// filters where `category2` names a sibling category rather than a real
    /// subdirectory.
    fn test_filter(&self) -> String {
        let module = self.filename.trim_end_matches(".rs");
        let sub_path = format!("src/{}/{}/{}", self.category1, self.category2, self.filename);
        if !self.category2.is_empty() && std::path::Path::new(&sub_path).exists() {
            format!("{}::{}::{module}", self.category1, self.category2)
        } else {
            format!("{}::{module}", self.category1)
        }
    }

    fn category_label(&self) -> String {
        if self.category2.is_empty() {
            self.category1.clone()
        } else {
            format!("{} / {}", self.category1, self.category2)
        }
    }
}

// ── CSV loading ───────────────────────────────────────────────────────────────
fn col_index(headers: &[String], name: &str) -> usize {
    headers.iter().position(|h| h.trim() == name).unwrap_or_else(|| {
        eprintln!("Error: required column '{name}' not found in CSV header.");
        process::exit(1);
    })
}

fn load_csv(path: &str) -> CsvData {
    let content = fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Error: cannot open '{path}': {e}");
        process::exit(1);
    });

    let mut lines = content.lines();

    // ── header ────────────────────────────────────────────────────────────
    let header_line = lines.next().unwrap_or_else(|| {
        eprintln!("Error: CSV file '{path}' is empty.");
        process::exit(1);
    });
    let headers: Vec<String> = header_line.split(',').map(|s| s.trim().to_string()).collect();
    let num_cols = headers.len();

    let idx_cat1   = col_index(&headers, "category1");
    let idx_cat2   = col_index(&headers, "category2");
    let idx_name   = col_index(&headers, "name");
    let idx_file   = col_index(&headers, "filename");
    let idx_status = col_index(&headers, "status");

    // ── data rows ─────────────────────────────────────────────────────────
    let mut rows: Vec<Vec<String>> = Vec::new();
    for (lineno, line) in lines.enumerate() {
        if line.trim().is_empty() { continue; }
        let mut parts: Vec<String> = line
            .splitn(num_cols, ',')
            .map(|s| s.trim().to_string())
            .collect();
        if parts.len() < num_cols {
            eprintln!("Warning: padding short row at line {lineno}.");
            parts.resize(num_cols, String::new());
        }
        rows.push(parts);
    }

    CsvData { headers, rows, idx_cat1, idx_cat2, idx_name, idx_file, idx_status }
}

// ── argument parsing ──────────────────────────────────────────────────────────
/// Return the numeric value of `--flag N` or `--flag=N` from `args`,
/// or `None` if the flag is absent or its value cannot be parsed.
fn parse_usize_arg(args: &[String], flag: &str) -> Option<usize> {
    let prefix = format!("{flag}=");
    for (i, arg) in args.iter().enumerate() {
        if let Some(val) = arg.strip_prefix(&prefix) {
            return val.parse().ok();
        }
        if arg == flag {
            return args.get(i + 1)?.parse().ok();
        }
    }
    None
}

// ── main ──────────────────────────────────────────────────────────────────────
fn main() {
    let args: Vec<String> = env::args().collect();

    let startnum = parse_usize_arg(&args, "--startnum").unwrap_or(DEFAULT_STARTNUM);
    let num2run  = parse_usize_arg(&args, "--num2run").unwrap_or(DEFAULT_NUM2RUN);

    let mut csv = load_csv(CSV_FILE);
    let total   = csv.total();

    if startnum < 1 || startnum > total {
        eprintln!("Error: --startnum {startnum} out of range. Valid range: 1–{total}.");
        process::exit(1);
    }

    let start_idx = startnum - 1;
    let end_idx   = (start_idx + num2run).min(total);
    let run_count = end_idx - start_idx;

    println!("CSV file  : {CSV_FILE}  ({total} entries)");
    println!("Running   : rows {startnum}–{}  ({run_count} algorithm(s))", end_idx);
    println!("{}", "─".repeat(64));

    let mut passed = 0usize;
    let mut failed = 0usize;

    for i in start_idx..end_idx {
        let entry   = csv.entry(i);
        let filter  = entry.test_filter();
        let row_num = i + 1;

        println!("\n[{row_num}/{total}] {} — {}", entry.category_label(), entry.name);
        println!("  file   : src/{}/{}", entry.category1, entry.filename);
        println!("  cmd    : cargo test {filter}");

        let exit_status = Command::new("cargo")
            .args(["test", &filter])
            .status()
            .unwrap_or_else(|e| {
                eprintln!("Error: failed to spawn cargo: {e}");
                process::exit(1);
            });

        let (label, status_str) = if exit_status.success() {
            ("PASSED", "PASS")
        } else {
            ("FAILED", "FAIL")
        };

        println!("  status : {label}");

        // Persist PASS/FAIL to the CSV immediately after each test.
        csv.set_status(i, status_str, CSV_FILE);

        if exit_status.success() { passed += 1; } else { failed += 1; }
    }

    println!("\n{}", "─".repeat(64));
    println!("Summary: {passed} passed, {failed} failed  ({run_count} run)");

    if failed > 0 {
        process::exit(1);
    }
}
