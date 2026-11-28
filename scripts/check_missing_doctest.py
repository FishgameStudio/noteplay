# A Python script that scans Rust source files, 
# finds `///` doc comments directly above function 
# or macro_rules definitions, and reports those that 
# contain no markdown code fence ``` (doctest).

import re
from pathlib import Path

DOC_LINE_RE = re.compile(r"^\s*///\s?(.*)$")
CODE_FENCE_RE = re.compile(r"^```")
FN_LINE_RE = re.compile(
    r"^\s*pub\s+(?:const\s+|async\s+|unsafe\s+)*fn\s+([a-zA-Z0-9_]+)\s*\("
)
MACRO_RULES_LINE_RE = re.compile(
    r"^\s*macro_rules!\s+([a-zA-Z0-9_]+)\s*\{"
)
MACRO_EXPORT_ATTR_RE = re.compile(r"^\s*#\[macro_export\]\s*$")
GENERAL_ATTR_RE = re.compile(r"^\s*#\[.*\]\s*$")


def is_doc_line(raw: str) -> re.Match[str] | None:
    return DOC_LINE_RE.match(raw)


def is_blank(raw: str) -> bool:
    return raw.strip() == ""


def is_attribute(raw: str) -> bool:
    return GENERAL_ATTR_RE.match(raw) is not None


def is_macro_export_attr(raw: str) -> bool:
    return MACRO_EXPORT_ATTR_RE.match(raw) is not None


def match_pub_fn(raw: str) -> re.Match[str] | None:
    return FN_LINE_RE.match(raw)


def match_macro_rules(raw: str) -> re.Match[str] | None:
    return MACRO_RULES_LINE_RE.match(raw)


def scan_file(path: Path) -> bool:
    failed = False
    lines = path.read_text(encoding="utf-8").splitlines()
    rel_path = path.relative_to(Path.cwd())

    doc_buffer: list[str] = []
    attr_buffer: list[str] = []
    saw_macro_export: bool = False

    for line_no, raw in enumerate(lines, start=1):
        doc_match = is_doc_line(raw)
        if doc_match is not None:
            doc_buffer.append(doc_match.group(1))
            attr_buffer.clear()
            continue

        if is_blank(raw):
            doc_buffer.clear()
            attr_buffer.clear()
            saw_macro_export = False
            continue

        if is_attribute(raw):
            attr_buffer.append(raw)
            if is_macro_export_attr(raw):
                saw_macro_export = True
            continue

        fn_match = match_pub_fn(raw)
        macro_match = match_macro_rules(raw)

        if fn_match is not None:
            name = fn_match.group(1)
            if doc_buffer:
                has_code = any(CODE_FENCE_RE.search(line) for line in doc_buffer)
                if not has_code:
                    print(f"\x1b[91m{rel_path}:{line_no}:\x1b[0m function `{name}` has doc comment without code example")
                    failed = True
            doc_buffer.clear()
            attr_buffer.clear()
            saw_macro_export = False

        elif macro_match is not None:
            name = macro_match.group(1)
            if saw_macro_export and doc_buffer:
                has_code = any(CODE_FENCE_RE.search(line) for line in doc_buffer)
                if not has_code:
                    print(f"\x1b[91m{rel_path}:{line_no}:\x1b[0m macro `{name}` (#[macro_export]) has doc comment without code example")
                    failed = True
            doc_buffer.clear()
            attr_buffer.clear()
            saw_macro_export = False

        else:
            doc_buffer.clear()
            attr_buffer.clear()
            saw_macro_export = False

    return failed


def main() -> int:
    failed = False
    cwd = Path.cwd()
    for rs in cwd.rglob("*.rs"):
        if "target" in rs.parts:
            continue
        failed |= scan_file(rs)
    return failed


if __name__ == "__main__":
    import sys
    sys.exit(main())
