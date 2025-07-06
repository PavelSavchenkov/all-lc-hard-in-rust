#!/usr/bin/env python3
"""Generate or refresh the LeetCode *Hard* problems table in README.md.

• Verifies authentication and fails fast on invalid cookies.
• Runtime & memory: REST first, GraphQL fallback.
• Last column header: 🔥 Difficulty (rows show 🔴 **Hard**).
• Case-insensitive URL discovery (`leetcode.com/problems/<slug>`).
"""
from __future__ import annotations

import argparse, json, os, re, sys, textwrap
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Dict, List, Tuple
import requests

# ── Config ─────────────────────────────────────────────────────────────────────
MARKER_START = "<!-- BEGIN LC TABLE -->"
MARKER_END   = "<!-- END LC TABLE -->"
GQL_ENDPOINT = "https://leetcode.com/graphql/"
REST_SUB_API = "https://leetcode.com/api/submissions/{slug}/?offset=0&limit=40" # Increased limit
HEADERS      = {"User-Agent": "Mozilla/5.0 (lc-readme-bot)", "Content-Type": "application/json"}
TIMEOUT      = 15
SLUG_RE      = re.compile(r"leetcode\.com\/problems\/([\w-]+)\/?", re.I)
_DEBUG = False

# ── Custom Exception for Clear Errors ──────────────────────────────────────────
class AuthenticationError(Exception):
    """Raised when LeetCode authentication fails."""
    pass

# ── Helpers ────────────────────────────────────────────────────────────────────
def debug(msg: str) -> None:
    if _DEBUG:
        print(f"DEBUG: {msg}", file=sys.stderr)

def graphql(s: requests.Session, q: str, vars: Dict[str, Any]) -> Dict[str, Any]:
    r = s.post(GQL_ENDPOINT, json={"query": q, "variables": vars}, headers=HEADERS, timeout=TIMEOUT)
    r.raise_for_status()
    data = r.json()
    if data.get("errors"):
        # If the error is about the user not being authenticated, raise our custom error
        if any("User is not signed in" in e.get("message", "") for e in data["errors"]):
            raise AuthenticationError("GraphQL query failed: User is not signed in.")
        raise RuntimeError(json.dumps(data["errors"], indent=2))
    return data["data"]

def count_loc(p: Path) -> int:
    loc, in_block = 0, False
    try:
        lines = p.read_text(encoding="utf-8", errors="ignore").splitlines()
    except Exception as e:
        debug(f"Could not read {p}: {e}")
        return 0
    for line in lines:
        t = line.strip()
        if not t: continue
        if in_block:
            if "*/" in t: in_block = False
            continue
        if t.startswith("//"): continue
        if "/*" in t:
            if "*/" not in t: in_block = True
            continue
        loc += 1
    return loc

# ── Parsing helpers ────────────────────────────────────────────────────────────
def parse_rate(r: str) -> float: return float(r.rstrip('%') or 0)

def parse_acc(a: str) -> int:
    a = a.replace(',', '').upper()
    mult = 1
    if a.endswith('K'):
        mult, a = 1_000, a[:-1]
    elif a.endswith('M'):
        mult, a = 1_000_000, a[:-1]
    try:
        return int(float(a) * mult)
    except ValueError:
        return 0

# ── LeetCode wrappers ──────────────────────────────────────────────────────────

# NEW: Function to verify authentication upfront
def verify_authentication(s: requests.Session) -> None:
    """Checks if the session is authenticated. Raises AuthenticationError on failure."""
    print("Verifying LeetCode authentication...", file=sys.stderr)
    try:
        q = "query { userStatus { isSignedIn username } }"
        data = graphql(s, q, {})
        user_status = data.get("userStatus", {})
        if not user_status.get("isSignedIn"):
            raise AuthenticationError(
                "Authentication failed: User is not signed in. "
                "Please update LC_SESSION and LC_CSRF cookies."
            )
        username = user_status.get("username", "Unknown")
        print(f"Successfully authenticated as: {username}", file=sys.stderr)
    except (requests.RequestException, RuntimeError, json.JSONDecodeError) as e:
        raise AuthenticationError(f"Authentication check failed: {e}") from e

def fetch_meta(s: requests.Session, slug: str) -> Dict[str, Any]:
    q = textwrap.dedent("""query($slug: String!){
        question(titleSlug: $slug){
            questionId title difficulty stats
        }
    }""")
    qd = graphql(s, q, {"slug": slug})["question"]
    st = json.loads(qd["stats"])
    return {
        "id": int(qd["questionId"]),
        "title": qd["title"],
        "difficulty": qd["difficulty"], # Keep difficulty for filtering
        "ac": parse_rate(st.get("acRate", "0")),
        "acc": parse_acc(st.get("totalAccepted", "0"))
    }

# UPDATED: Hardened function to get submission stats
def fastest_submission(s: requests.Session, slug: str) -> Tuple[str, str]:
    """Fetches fastest accepted submission via REST, with GraphQL fallback."""
    acc_submissions: List[Dict[str, Any]] = []

    # 1. Attempt REST API (faster)
    try:
        r = s.get(REST_SUB_API.format(slug=slug), headers=HEADERS, timeout=TIMEOUT)
        r.raise_for_status() # Fail fast on 4xx/5xx errors
        submissions = r.json().get("submissions_dump", [])
        acc_submissions = [sub for sub in submissions if sub.get("status_display") == "Accepted"]
    except (requests.RequestException, json.JSONDecodeError) as e:
        debug(f"REST API failed for '{slug}', falling back to GraphQL. Reason: {e}")

    # 2. Fallback to GraphQL if REST fails or finds no accepted submissions
    if not acc_submissions:
        try:
            gq = textwrap.dedent("""query($slug: String!){
                submissionList(questionSlug: $slug, offset: 0, limit: 40) {
                    submissions { statusDisplay runtime memory }
                }
            }""")
            subs = graphql(s, gq, {"slug": slug})["submissionList"]["submissions"]
            acc_submissions = [sub for sub in subs if sub.get("statusDisplay") == "Accepted"]
        except (requests.RequestException, RuntimeError) as e:
            debug(f"GraphQL submission fallback failed for '{slug}': {e}")

    if not acc_submissions:
        debug(f"No accepted submissions found for '{slug}'.")
        return "—", "—"

    # LeetCode runtime is a string like "123 ms". We parse the integer part for comparison.
    best = min(acc_submissions, key=lambda x: int(str(x.get("runtime", "9999999")).split()[0]))
    return best.get("runtime", "—"), best.get("memory", "—")

# ── Markdown builders ─────────────────────────────────────────────────────────
def row_md(r: Dict[str, Any]) -> str:
    return (f"| {r['id']} | {r['prob']} | {r['src']} | `{r['rt']}` | `{r['mem']}` | "
            f"{r['loc']} | {r['ac']:.1f}% | {r['acc']:,} | 🔴 **Hard** |")

def table(rows: List[Dict[str, Any]]) -> str:
    head = ("| # | Problem | Solution | ⚡ Runtime | 🧠 Memory | 📏 Lines | 📊 Ac % | 🟢 Accepted | 🔥 Level |\n"
            "| :- | :-- | :-- | :-- | :-- | :-- | :-- | :-- | :-- |")
    return head + "\n" + "\n".join(row_md(r) for r in rows) + "\n"

# ── README helpers ────────────────────────────────────────────────────────────
def update_readme(md: str, readme: Path) -> bool:
    """Insert or replace exactly **one** LC‑table block delimited by the markers.

    The function is idempotent: repeated runs never create duplicate blocks.
    Returns **True** when README.md was modified.
    """
    block = f"{MARKER_START}\n\n{md}\n{MARKER_END}"

    if not readme.exists():
        readme.write_text(block + "\n", encoding="utf-8")
        return True

    text = readme.read_text(encoding="utf-8")

    # Ensure the markers exist exactly once. If missing, append them at the end.
    if MARKER_START not in text or MARKER_END not in text:
        new_text = text.rstrip() + "\n\n" + block + "\n"
        readme.write_text(new_text, encoding="utf-8")
        return True

    # Replace *only the first* BEGIN/END block, non‑greedy, DOTALL.
    pattern = re.compile(
        rf"{re.escape(MARKER_START)}.*?{re.escape(MARKER_END)}",
        flags=re.DOTALL,
    )
    new_text, n_subs = pattern.subn(block, text, count=1)

    if n_subs and new_text != text:
        readme.write_text(new_text, encoding="utf-8")
        return True
    return False


def dump_json(rows: List[Dict[str, Any]], root: Path) -> None:
    public_dir = root / "public"
    public_dir.mkdir(exist_ok=True)
    (public_dir / "problems.json").write_text(
        json.dumps({
            "generated_at_utc": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
            "count": len(rows),
            "problems": rows
        }, indent=2) + "\n", encoding="utf-8")

# ── Discovery ────────────────────────────────────────────────────────────────
def discover(root: Path) -> Dict[str, Path]:
    mp: Dict[str, Path] = {}
    for p in root.rglob("*.rs"):
        if any(part in p.parts for part in ["template", "target", ".cargo"]):
            continue
        try:
            content = p.read_text(encoding="utf-8", errors="ignore")
            m = SLUG_RE.search(content)
            if m:
                slug = m.group(1).lower()
                mp[slug] = p
        except Exception as e:
            debug(f"Could not process {p}: {e}")
    return mp

# ── Row assembly ─────────────────────────────────────────────────────────────
def build_rows(s: requests.Session, mp: Dict[str, Path], root: Path, repo_url: str, mx: int | None) -> List[Dict[str, Any]]:
    rows = []
    slugs_to_process = list(mp.items())
    if mx:
        slugs_to_process = slugs_to_process[:mx]

    try:
        from rich.progress import Progress, BarColumn, TextColumn
        bar = Progress(
            TextColumn("[progress.description]{task.description}"),
            BarColumn(),
            TextColumn("[progress.percentage]{task.percentage:>3.0f}%"),
            TextColumn("({task.completed} of {task.total})"),
        )
        task = bar.add_task("Fetching...", total=len(slugs_to_process))
        bar.start()
    except ImportError:
        bar = None

    for slug, src_path in slugs_to_process:
        if bar: bar.update(task, description=f"Fetching [yellow]{slug}[/yellow]")
        try:
            meta = fetch_meta(s, slug)
            if meta.get("difficulty") != "Hard":
                debug(f"Skipping '{slug}' (not Hard)")
                if bar: bar.advance(task)
                continue

            rt, mem = fastest_submission(s, slug)
            rows.append({
                "id": meta["id"],
                "prob": f"[{meta['title']}](https://leetcode.com/problems/{slug})",
                "src": f"[source]({repo_url}/blob/main/{src_path.relative_to(root).as_posix()})",
                "rt": rt, "mem": mem, "loc": count_loc(src_path),
                "ac": meta["ac"], "acc": meta["acc"]
            })
        except Exception as e:
            print(f"\nERROR: Could not process slug '{slug}': {e}", file=sys.stderr)
        if bar: bar.advance(task)

    if bar: bar.stop()
    rows.sort(key=lambda r: r["acc"]) # Sort by fewest accepted first
    return rows

# ── Main ─────────────────────────────────────────────────────────────────────
def make_session() -> requests.Session:
    s = requests.Session()
    s.headers.update(HEADERS)
    session_cookie = os.getenv("LC_SESSION")
    csrf_token = os.getenv("LC_CSRF")
    if session_cookie and csrf_token:
        s.cookies.set("LEETCODE_SESSION", session_cookie, domain=".leetcode.com")
        s.cookies.set("csrftoken", csrf_token, domain=".leetcode.com")
        s.headers["x-csrftoken"] = csrf_token
    else:
        print("WARNING: LC_SESSION or LC_CSRF not found in environment.", file=sys.stderr)
    return s

def main() -> None:
    global _DEBUG
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawTextHelpFormatter)
    ap.add_argument("--repo-path", type=Path, default=".", help="Path to the repository root.")
    ap.add_argument("--repo-url", required=True, help="Full base URL of the GitHub repository.")
    ap.add_argument("--max-rows", type=int, help="Limit processing to N problems for quick tests.")
    ap.add_argument("--json", action="store_true", help="Generate a public/problems.json file.")
    ap.add_argument("--debug", action="store_true", help="Enable verbose debug output.")
    a = ap.parse_args()
    _DEBUG = a.debug

    root = a.repo_path.resolve()
    mp = discover(root)
    if not mp:
        sys.exit("No LeetCode problem URLs found in any .rs files. Aborting.")

    try:
        with make_session() as sess:
            # UPDATED: Verify auth right away. Fails fast.
            verify_authentication(sess)
            rows = build_rows(sess, mp, root, a.repo_url.rstrip("/"), a.max_rows)

        if not rows:
            sys.exit("No 'Hard' problems were successfully processed. Halting.")

        changed = update_readme(table(rows), root / "README.md")
        if a.json:
            dump_json(rows, root)
            print(f"Generated public/problems.json", file=sys.stderr)

        status = 'updated' if changed else 'already up-to-date'
        print(f"✅ Success! README.md is {status} with {len(rows)} Hard problem(s).")

    except AuthenticationError as e:
        print(f"\n❌ Authentication Error: {e}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"\n❌ An unexpected error occurred: {e}", file=sys.stderr)
        if _DEBUG:
            import traceback
            traceback.print_exc()
        sys.exit(1)

if __name__ == "__main__":
    main()