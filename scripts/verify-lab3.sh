#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LAB="$ROOT/lab3-process-registry"
STARTER="$LAB/src/lib.rs"
SOLUTION="$ROOT/.solution/lab3/src/lib.rs"
BACKUP="$(mktemp)"

cleanup() {
  if [[ -f "$BACKUP" ]]; then
    mv "$BACKUP" "$STARTER"
  fi
}
trap cleanup EXIT

cp "$STARTER" "$BACKUP"

echo "=== Starter (should FAIL to compile) ==="
if (cd "$LAB" && cargo test 2>&1); then
  echo "ERROR: starter unexpectedly passed"
  exit 1
else
  echo "OK: starter fails as expected"
fi

echo "=== Reference solution (should PASS all tests) ==="
cp "$SOLUTION" "$STARTER"
cd "$LAB"
cargo test -- --test-threads=1
cargo test m1_
cargo test m2_
cargo test m3_
cargo test m4_
cargo test m5_
echo "All milestone filters passed"
