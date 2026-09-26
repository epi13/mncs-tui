#!/usr/bin/env python3
"""Thin snapshot adapter: canonical family state -> family-status corpus.

Reads ONLY canonical sources:
  - Atlas registry.json (canonical repo roster and roles);
  - git (HEAD + clean/dirty per repo working tree);
  - Commons pressures/records/*.json (open-pressure counts per repo).

Emits an experiment corpus for examples.family_status whose snapshot rows
are (PASS=0 / FAIL=1 / UNKNOWN=2) verdict codes.

DEMO POLICY (explicit, not canonical health): a dirty working tree maps
to FAIL, a clean tree with no non-obsolete pressure records touching the
repo maps to PASS, anything else maps to UNKNOWN. This policy exists so
the projection example has realistic input to render; it is NOT a family
health verdict. Production wiring would map Forge receipts and mncs-test
result contracts to codes; no canonical per-repo verification-state
query exists yet (see the campaign tooling pressure).

Usage:
  python3 scripts/snapshot_family.py [--workspace DIR] [--out FILE]
"""
import argparse
import json
import os
import subprocess
import sys

CODES = {"PASS": 0, "FAIL": 1, "UNKNOWN": 2}


def git_head(path):
    try:
        return subprocess.run(
            ["git", "-C", path, "rev-parse", "--short", "HEAD"],
            capture_output=True, text=True, timeout=30).stdout.strip()
    except Exception:
        return "unknown"


def git_clean(path):
    try:
        out = subprocess.run(
            ["git", "-C", path, "status", "--porcelain"],
            capture_output=True, text=True, timeout=30).stdout.strip()
        return out == ""
    except Exception:
        return False


def open_pressures(repo_id, records_dir):
    n = 0
    try:
        names = os.listdir(records_dir)
    except OSError:
        return 0
    for name in names:
        if not name.endswith(".json"):
            continue
        try:
            with open(os.path.join(records_dir, name)) as fh:
                data = json.load(fh)
        except (OSError, ValueError):
            continue
        if data.get("status") in ("resolved", "obsolete"):
            continue
        blob = json.dumps(data.get("affectedRepositories", []))
        if repo_id in blob or "mncs-math" in blob and repo_id == "mncs-math":
            n += 1
    return n


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--workspace", default="/home/epi13/Documents/Projects")
    ap.add_argument("--out", default="family-status.snapshot.json")
    ap.add_argument("--rows", type=int, default=4)
    args = ap.parse_args()

    with open(os.path.join(args.workspace, "mncs-atlas", "registry.json")) as fh:
        atlas = json.load(fh)
    records = os.path.join(args.workspace, "MNCS-Commons", "pressures",
                           "records")
    repos = [p.get("id") for p in atlas.get("projects", [])][:args.rows]
    snapshot, notes = [], []
    for repo in repos:
        path = os.path.join(args.workspace, repo)
        clean = git_clean(path) if os.path.isdir(path) else False
        pressures = open_pressures(repo, records)
        if not os.path.isdir(path):
            code = CODES["UNKNOWN"]
        elif not clean:
            code = CODES["FAIL"]
        elif pressures == 0:
            code = CODES["PASS"]
        else:
            code = CODES["UNKNOWN"]
        snapshot.append(code)
        notes.append({"repo": repo, "head": git_head(path),
                      "clean": clean, "open_pressures": pressures,
                      "code": code})

    def ival(v):
        return {"integer": {"value": v,
                            "type": {"bits": 64, "signed": True}}}

    def case(cid, fn, fargs, exp):
        return {"id": cid,
                "request": {"schema_version": "0.1",
                            "target": {"module": "examples.family_status",
                                       "function": fn},
                            "arguments": fargs, "step_budget": 20000},
                "expected_status": "returned", "expected": [ival(exp)]}

    snap = {"sequence": {"values": [ival(v) for v in snapshot]}}
    corpus = {"schema_version": "0.1", "name": "family-status-snapshot",
              "cases": [
                  case("snap-pass", "count_verdict",
                       [snap, ival(0)], snapshot.count(0)),
                  case("snap-fail", "count_verdict",
                       [snap, ival(1)], snapshot.count(1)),
                  case("snap-unknown", "count_verdict",
                       [snap, ival(2)], snapshot.count(2)),
              ],
              "_notes": ("Demo-policy snapshot; rows: %s" % notes)}
    with open(args.out, "w") as fh:
        json.dump(corpus, fh, indent=1)
    print("wrote %s rows=%s notes=%s" % (args.out, snapshot, notes))
    return 0


if __name__ == "__main__":
    sys.exit(main())
