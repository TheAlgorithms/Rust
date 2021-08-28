import os

from typing import Iterator

URL_BASE = "https://github.com/TheAlgorithms/Rust/blob/master"

g_output = []


def good_filepaths(top_dir: str = ".") -> Iterator[str]:
    fs_exts = tuple(".rs".split())
    for dirpath, dirnames, filenames in os.walk(top_dir):
        dirnames[:] = [d for d in dirnames if d[0] not in "._"]
        for filename in filenames:
            if filename != "mod.rs" and os.path.splitext(filename)[1].lower() in fs_exts:
                yield os.path.join(dirpath, filename).lstrip("./")


def md_prefix(i):
    return f"{i * '  '}*" if i else "\n##"


def print_path(old_path: str, new_path: str) -> str:
    global g_output
    old_parts = old_path.split(os.sep)
    for i, new_part in enumerate(new_path.split(os.sep)):
        if i + 1 > len(old_parts) or old_parts[i] != new_part:
            if new_part:
                print(f"{md_prefix(i)} {new_part.replace('_', ' ').title()}")
                g_output.append(f"{md_prefix(i)} {new_part.replace('_', ' ').title()}")
    return new_path


def build_directory_md(top_dir: str = ".") -> str:
    global g_output
    old_path = ""
    for filepath in sorted(good_filepaths(), key=str.lower):
        filepath, filename = os.path.split(filepath)
        if filepath != old_path:
            old_path = print_path(old_path, filepath)
        indent = (filepath.count(os.sep) + 1) if filepath else 0
        url = "/".join((URL_BASE, filepath, filename)).replace(" ", "%20")
        filename = os.path.splitext(filename.replace("_", " ").title())[0]
        print((f"{md_prefix(indent)} [{filename}]({url})"))
        g_output.append(f"{md_prefix(indent)} [{filename}]({url})")

    return "# List of all files\n" + "\n".join(g_output)


with open("DIRECTORY.md", "w") as out_file:
    out_file.write(build_directory_md(".") + "\n")
