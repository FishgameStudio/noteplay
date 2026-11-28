# A Python script that scans Rust source files, 
# finds `///` doc comments directly above function 
# or macro_rules definitions, and reports those that 
# contain no markdown code fence ``` (doctest).

import re
from pathlib import Path

DOC_LINE_RE = re.compile(r"^///\s?(.*)$")
CODE_FENCE_RE = re.compile(r"^```")
FN_DECL_RE = re.compile(r"^\s*(?:pub\s+|const\s+|async\s+|unsafe\s+)*fn\s+([a-zA-Z0-9_]+)\s*\(")
MACRO_RULES_RE = re.compile(r"^\s*macro_rules!\s+([a-zA-Z0-9_]+)")


def scan_file(path: Path) -> bool:
    failed = False
    lines = path.read_text(encoding="utf-8").splitlines()
    doc_buffer: list[str] = []

    for line_no, raw in enumerate[str](lines, start=1):
        doc_match: re.Match[str] | None = DOC_LINE_RE.match(raw)
        fn_match: re.Match[str] | None = FN_DECL_RE.match(raw)
        macro_match: re.Match[str] | None = MACRO_RULES_RE.match(raw)

        if doc_match is not None:
            doc_buffer.append(doc_match.group(1))
            continue

        stripped = raw.strip()
        if stripped == "" or stripped.startswith("#"):
            continue

        if fn_match is not None:
            if doc_buffer:
                name = fn_match.group(1)
                has_code = any(CODE_FENCE_RE.search(line) for line in doc_buffer)
                if not has_code:
                    print(f"\x1b[91m{path}:{line_no}:\x1b[0m function `{name}` has doc comment without code example")
                    failed = True
            doc_buffer.clear()
        elif macro_match is not None:
            if doc_buffer:
                name = macro_match.group(1)
                has_code = any(CODE_FENCE_RE.search(line) for line in doc_buffer)
                if not has_code:
                    print(f"\x1b[91m{path}:{line_no}:\x1b[0m macro `{name}` has doc comment without code example")
                    failed = True
            doc_buffer.clear()
        else:
            doc_buffer.clear()
    return failed


def main() -> int:
    failed = False
    root = Path.cwd().relative_to(Path.cwd())
    for rs in root.rglob("*.rs"):
        if "target" in rs.parts:
            continue
        failed |= scan_file(rs)
    return failed


if __name__ == "__main__":
    import sys
    sys.exit(main())
