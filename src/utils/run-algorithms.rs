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
//! A `runsecs` column (wall-clock seconds of the test run, as a float) is
//! added automatically immediately after `status` when it is not present.
//!
//! After each test the `status` (PASS/FAIL) and `runsecs` columns are updated
//! and the file is written back to disk, so progress survives an interrupted run.
//!
//! A row whose `status` is `SKIP` (case-insensitive) is skipped: its test is
//! not run and its `status`/`runsecs` cells are left unchanged.
//!
//! A fresh log of runner output and `cargo test` output is written to a
//! `target/run-algorithms-YYYYMMDDTHHMMSS.NNNNNNNNNZ.log` file; a matching
//! `.txt` file stores the run summary with the log location and final byte size.

use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

// ── defaults ──────────────────────────────────────────────────────────────────
const CSV_FILE: &str = "algorithms-001.csv";
const LOG_DIR: &str = "target";
const DEFAULT_STARTNUM: usize = 1;
const DEFAULT_NUM2RUN: usize = usize::MAX; // run all rows from startnum when not specified

// ── logging ───────────────────────────────────────────────────────────────────
struct RunLogger {
    path: PathBuf,
    summary_path: PathBuf,
    file: Arc<Mutex<File>>,
}

impl RunLogger {
    fn create(log_dir: &str) -> Self {
        let (path, summary_path) = timestamped_output_paths(log_dir);
        let path = absolute_path(&path);
        let summary_path = absolute_path(&summary_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap_or_else(|e| {
                eprintln!(
                    "Error: cannot create log directory '{}': {e}",
                    parent.display()
                );
                std::process::exit(1);
            });
        }

        let file = File::create(&path).unwrap_or_else(|e| {
            eprintln!("Error: cannot create log file '{}': {e}", path.display());
            std::process::exit(1);
        });

        Self {
            path,
            summary_path,
            file: Arc::new(Mutex::new(file)),
        }
    }

    fn println(&self, message: impl AsRef<str>) {
        let line = format!("{}\n", message.as_ref());
        self.print_and_log(&line);
    }

    fn eprintln(&self, message: impl AsRef<str>) {
        let line = format!("{}\n", message.as_ref());
        let mut stderr = io::stderr();
        stderr.write_all(line.as_bytes()).unwrap_or_else(|e| {
            eprintln!("Error: cannot write stderr: {e}");
            std::process::exit(1);
        });
        stderr.flush().unwrap_or_else(|e| {
            eprintln!("Error: cannot flush stderr: {e}");
            std::process::exit(1);
        });
        self.write_log_bytes(line.as_bytes());
    }

    fn print_and_log(&self, message: &str) {
        self.print(message);
        self.write_log_bytes(message.as_bytes());
    }

    fn print(&self, message: &str) {
        let mut stdout = io::stdout();
        stdout.write_all(message.as_bytes()).unwrap_or_else(|e| {
            eprintln!("Error: cannot write stdout: {e}");
            std::process::exit(1);
        });
        stdout.flush().unwrap_or_else(|e| {
            eprintln!("Error: cannot flush stdout: {e}");
            std::process::exit(1);
        });
    }

    fn write_log_bytes(&self, bytes: &[u8]) {
        write_log_bytes(&self.file, bytes).unwrap_or_else(|e| {
            eprintln!(
                "Error: cannot write log file '{}': {e}",
                self.path.display()
            );
            std::process::exit(1);
        });
    }

    fn byte_len(&self) -> u64 {
        self.flush_log();
        fs::metadata(&self.path).map_or_else(
            |e| {
                eprintln!("Error: cannot stat log file '{}': {e}", self.path.display());
                std::process::exit(1);
            },
            |metadata| metadata.len(),
        )
    }

    fn display_path(&self) -> String {
        self.path.display().to_string()
    }
    fn display_summary_path(&self) -> String {
        self.summary_path.display().to_string()
    }

    fn write_summary(&self, summary: &str) {
        fs::write(&self.summary_path, summary).unwrap_or_else(|e| {
            eprintln!(
                "Error: cannot write summary file '{}': {e}",
                self.summary_path.display()
            );
            std::process::exit(1);
        });
    }

    fn flush_log(&self) {
        let mut file = self.file.lock().unwrap_or_else(|_| {
            eprintln!("Error: cannot lock log file '{}'.", self.path.display());
            std::process::exit(1);
        });
        file.flush().unwrap_or_else(|e| {
            eprintln!(
                "Error: cannot flush log file '{}': {e}",
                self.path.display()
            );
            std::process::exit(1);
        });
    }
}

fn timestamped_output_paths(log_dir: &str) -> (PathBuf, PathBuf) {
    // Build the extensions onto the full stem rather than using
    // `Path::with_extension`, whose "last dot" rule would otherwise treat the
    // `.NNNNNNNNNZ` nanosecond fraction as an extension and strip it.
    let stem = format!("run-algorithms-{}", utc_timestamp_nanos());
    let dir = Path::new(log_dir);
    (
        dir.join(format!("{stem}.log")),
        dir.join(format!("{stem}.txt")),
    )
}

fn utc_timestamp_nanos() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|e| {
            eprintln!("Error: cannot calculate UTC timestamp: {e}");
            std::process::exit(1);
        });

    let total_secs = timestamp.as_secs();
    let days = (total_secs / 86_400) as i64;
    let secs_of_day = total_secs % 86_400;
    let hour = secs_of_day / 3_600;
    let minute = (secs_of_day % 3_600) / 60;
    let second = secs_of_day % 60;
    let (year, month, day) = civil_from_days(days);

    format!(
        "{year:04}{month:02}{day:02}T{hour:02}{minute:02}{second:02}.{:09}Z",
        timestamp.subsec_nanos()
    )
}

fn civil_from_days(days_since_unix_epoch: i64) -> (i32, u32, u32) {
    let z = days_since_unix_epoch + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let day_of_era = z - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_part = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_part + 2) / 5 + 1;
    let month = month_part + if month_part < 10 { 3 } else { -9 };
    let year = year + i64::from(month <= 2);

    (year as i32, month as u32, day as u32)
}
fn absolute_path(path: &Path) -> PathBuf {
    if path.is_absolute() {
        return path.to_path_buf();
    }

    env::current_dir().map_or_else(
        |e| {
            eprintln!("Error: cannot resolve current directory for log file: {e}");
            std::process::exit(1);
        },
        |dir| dir.join(path),
    )
}

fn write_log_bytes(file: &Arc<Mutex<File>>, bytes: &[u8]) -> io::Result<()> {
    let mut file = file
        .lock()
        .map_err(|_| io::Error::other("log file lock poisoned"))?;
    file.write_all(bytes)?;
    file.flush()
}

fn copy_output_to_log<R, W>(
    mut reader: R,
    mut writer: W,
    log_file: Arc<Mutex<File>>,
) -> thread::JoinHandle<io::Result<()>>
where
    R: Read + Send + 'static,
    W: Write + Send + 'static,
{
    thread::spawn(move || {
        let mut buffer = [0u8; 8192];
        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                return Ok(());
            }
            let chunk = &buffer[..bytes_read];
            writer.write_all(chunk)?;
            writer.flush()?;
            write_log_bytes(&log_file, chunk)?;
        }
    })
}

fn join_output_thread(handle: thread::JoinHandle<io::Result<()>>, label: &str, logger: &RunLogger) {
    match handle.join() {
        Ok(Ok(())) => {}
        Ok(Err(e)) => {
            logger.eprintln(format!("Error: cannot copy cargo {label}: {e}"));
            std::process::exit(1);
        }
        Err(_) => {
            logger.eprintln(format!("Error: cargo {label} copy thread panicked."));
            std::process::exit(1);
        }
    }
}

fn run_logged_cargo_test(filter: &str, logger: &RunLogger) -> ExitStatus {
    let mut child = Command::new("cargo")
        .args(["test", filter])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| {
            logger.eprintln(format!("Error: failed to spawn cargo: {e}"));
            std::process::exit(1);
        });

    let stdout = child.stdout.take().unwrap_or_else(|| {
        logger.eprintln("Error: failed to capture cargo stdout.");
        std::process::exit(1);
    });
    let stderr = child.stderr.take().unwrap_or_else(|| {
        logger.eprintln("Error: failed to capture cargo stderr.");
        std::process::exit(1);
    });

    let stdout_thread = copy_output_to_log(stdout, io::stdout(), Arc::clone(&logger.file));
    let stderr_thread = copy_output_to_log(stderr, io::stderr(), Arc::clone(&logger.file));

    let exit_status = child.wait().unwrap_or_else(|e| {
        logger.eprintln(format!("Error: failed to wait for cargo: {e}"));
        std::process::exit(1);
    });

    join_output_thread(stdout_thread, "stdout", logger);
    join_output_thread(stderr_thread, "stderr", logger);
    exit_status
}

// ── CSV data ──────────────────────────────────────────────────────────────────
/// Holds the entire CSV in memory as raw string rows so every field
/// (including `status`) can be updated and written back to disk.
struct CsvData {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    idx_cat1: usize,
    idx_cat2: usize,
    idx_name: usize,
    idx_file: usize,
    idx_status: usize,
    idx_runsecs: usize,
}

impl CsvData {
    fn total(&self) -> usize {
        self.rows.len()
    }

    /// Extract an `AlgorithmEntry` view from one raw row.
    fn entry(&self, i: usize) -> AlgorithmEntry {
        let r = &self.rows[i];
        AlgorithmEntry {
            category1: r[self.idx_cat1].clone(),
            category2: r[self.idx_cat2].clone(),
            name: r[self.idx_name].clone(),
            filename: r[self.idx_file].clone(),
        }
    }

    /// Current (trimmed) value of the `status` cell for row `i`.
    fn status(&self, i: usize) -> &str {
        self.rows[i][self.idx_status].trim()
    }

    /// Write the `status` (PASS/FAIL) and `runsecs` (elapsed wall-clock
    /// seconds) cells for row `i`, then immediately persist the whole file to
    /// disk so progress survives an interrupted run.
    fn set_result(&mut self, i: usize, status: &str, runsecs: f64, path: &str) {
        self.rows[i][self.idx_status] = status.to_string();
        self.rows[i][self.idx_runsecs] = format!("{runsecs:.3}");
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
            std::process::exit(1);
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
    name: String,
    filename: String,
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
        let sub_path = format!(
            "src/{}/{}/{}",
            self.category1, self.category2, self.filename
        );
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
    headers
        .iter()
        .position(|h| h.trim() == name)
        .unwrap_or_else(|| {
            eprintln!("Error: required column '{name}' not found in CSV header.");
            std::process::exit(1);
        })
}

fn load_csv(path: &str) -> CsvData {
    let content = fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Error: cannot open '{path}': {e}");
        std::process::exit(1);
    });

    let mut lines = content.lines();

    // ── header ────────────────────────────────────────────────────────────
    let header_line = lines.next().unwrap_or_else(|| {
        eprintln!("Error: CSV file '{path}' is empty.");
        std::process::exit(1);
    });
    let mut headers: Vec<String> = header_line
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    let num_cols = headers.len();

    // Resolve `status` up front: the optional `runsecs` column is inserted
    // immediately after it so `wikipedia` stays the final column and keeps
    // absorbing any stray commas (see the `splitn` parsing below).
    let idx_status = col_index(&headers, "status");

    // ── data rows ─────────────────────────────────────────────────────────
    let mut rows: Vec<Vec<String>> = Vec::new();
    for (lineno, line) in lines.enumerate() {
        if line.trim().is_empty() {
            continue;
        }
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

    // Ensure a `runsecs` column exists, inserting it — plus a matching empty
    // cell in every row — directly after `status` when it is absent.
    let idx_runsecs = match headers.iter().position(|h| h.trim() == "runsecs") {
        Some(i) => i,
        None => {
            let at = idx_status + 1;
            headers.insert(at, "runsecs".to_string());
            for row in &mut rows {
                row.insert(at, String::new());
            }
            at
        }
    };

    // Resolve the remaining columns *after* the possible insertion so their
    // indices reflect the shifted positions.
    let idx_cat1 = col_index(&headers, "category1");
    let idx_cat2 = col_index(&headers, "category2");
    let idx_name = col_index(&headers, "name");
    let idx_file = col_index(&headers, "filename");

    CsvData {
        headers,
        rows,
        idx_cat1,
        idx_cat2,
        idx_name,
        idx_file,
        idx_status,
        idx_runsecs,
    }
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

// ── elapsed-time formatting ────────────────────────────────────────────────────
/// Human-readable elapsed time: raw seconds (3 dp), plus an `h/m/s` breakdown
/// once the duration reaches a minute.
fn format_elapsed(secs: f64) -> String {
    if secs < 60.0 {
        return format!("{secs:.3}s");
    }
    let total = secs as u64;
    let (h, m, s) = (total / 3600, (total % 3600) / 60, total % 60);
    if h > 0 {
        format!("{secs:.3}s ({h}h {m:02}m {s:02}s)")
    } else {
        format!("{secs:.3}s ({m}m {s:02}s)")
    }
}

// ── main ──────────────────────────────────────────────────────────────────────
fn main() {
    let run_started = Instant::now();
    let args: Vec<String> = env::args().collect();
    let logger = RunLogger::create(LOG_DIR);

    let startnum = parse_usize_arg(&args, "--startnum").unwrap_or(DEFAULT_STARTNUM);
    let num2run = parse_usize_arg(&args, "--num2run").unwrap_or(DEFAULT_NUM2RUN);

    let mut csv = load_csv(CSV_FILE);
    let total = csv.total();

    if startnum < 1 || startnum > total {
        logger.eprintln(format!(
            "Error: --startnum {startnum} out of range. Valid range: 1–{total}."
        ));
        std::process::exit(1);
    }

    let start_idx = startnum - 1;
    let end_idx = (start_idx + num2run).min(total);
    let run_count = end_idx - start_idx;

    logger.println(format!("CSV file  : {CSV_FILE}  ({total} entries)"));
    logger.println(format!(
        "Running   : rows {startnum}–{end_idx}  ({run_count} algorithm(s))"
    ));
    logger.println("─".repeat(64));

    let mut passed = 0usize;
    let mut failed = 0usize;
    let mut skipped = 0usize;

    for i in start_idx..end_idx {
        let entry = csv.entry(i);
        let row_num = i + 1;

        logger.println(format!(
            "\n[{row_num}/{total}] {} — {}",
            entry.category_label(),
            entry.name
        ));

        // Honor an explicit SKIP sentinel in the status column: leave the row
        // untouched (status + runsecs preserved) and do not run its test.
        if csv.status(i).eq_ignore_ascii_case("SKIP") {
            logger.println("  status : SKIPPED");
            skipped += 1;
            continue;
        }

        let filter = entry.test_filter();
        logger.println(format!(
            "  file   : src/{}/{}",
            entry.category1, entry.filename
        ));
        logger.println(format!("  cmd    : cargo test {filter}"));

        let started = Instant::now();
        let exit_status = run_logged_cargo_test(&filter, &logger);
        let runsecs = started.elapsed().as_secs_f64();

        let (label, status_str) = if exit_status.success() {
            ("PASSED", "PASS")
        } else {
            ("FAILED", "FAIL")
        };

        logger.println(format!("  status : {label}"));
        logger.println(format!("  runsecs: {runsecs:.3}"));

        // Persist PASS/FAIL and elapsed seconds to the CSV after each test.
        csv.set_result(i, status_str, runsecs, CSV_FILE);

        if exit_status.success() {
            passed += 1;
        } else {
            failed += 1;
        }
    }

    let log_bytes = logger.byte_len();
    let summary = format!(
        "\n{}\nSummary: {passed} passed, {failed} failed, {skipped} skipped  ({run_count} selected)\nElapsed: {}\nLog file: {}\nLog bytes: {log_bytes} bytes\nSummary file: {}\n",
        "─".repeat(64),
        format_elapsed(run_started.elapsed().as_secs_f64()),
        logger.display_path(),
        logger.display_summary_path()
    );
    logger.print(&summary);
    logger.write_summary(&summary);

    if failed > 0 {
        std::process::exit(1);
    }
}
