#!/bin/bash
set -e

echo "=== ogit integration test ==="

# Setup
TEST_DIR=$(mktemp -d)
cd "$TEST_DIR"
echo "Working in: $TEST_DIR"

# Build
OGIT="cargo run --manifest-path $OLDPWD/Cargo.toml --quiet --"

# Test init
echo "Testing: init"
$OGIT init
[ -d ".ogit/objects" ] || { echo "FAIL: .ogit/objects not created"; exit 1; }

# Test store
echo "Testing: store"
echo "Hello, ogit!" > test.txt
BLOB_HASH=$($OGIT store test.txt)
[ -n "$BLOB_HASH" ] || { echo "FAIL: store returned empty hash"; exit 1; }

# Test cat
echo "Testing: cat"
CONTENT=$($OGIT cat "$BLOB_HASH")
[ "$CONTENT" = "Hello, ogit!" ] || { echo "FAIL: cat content mismatch"; exit 1; }

# Test write-tree
echo "Testing: write-tree"
mkdir -p src
echo "fn main() {}" > src/main.rs
TREE_HASH=$($OGIT write-tree .)
[ -n "$TREE_HASH" ] || { echo "FAIL: write-tree returned empty hash"; exit 1; }

# Test commit
echo "Testing: commit"
COMMIT_HASH=$($OGIT commit -m "Test commit")
[ -n "$COMMIT_HASH" ] || { echo "FAIL: commit returned empty hash"; exit 1; }

# Test show
echo "Testing: show"
$OGIT show "$COMMIT_HASH" | grep -q "tree:" || { echo "FAIL: show missing tree"; exit 1; }

# Test log
echo "Testing: log"
$OGIT log | grep -q "Test commit" || { echo "FAIL: log missing message"; exit 1; }

# Test ls-objects
echo "Testing: ls-objects"
OBJECT_COUNT=$($OGIT ls-objects | wc -l)
[ "$OBJECT_COUNT" -ge 3 ] || { echo "FAIL: expected at least 3 objects"; exit 1; }

# Test second commit (parent chain)
echo "Testing: commit with parent"
echo "new file" > new.txt
COMMIT2_HASH=$($OGIT commit -m "Second commit")
$OGIT show "$COMMIT2_HASH" | grep -q "parent:" || { echo "FAIL: second commit missing parent"; exit 1; }

# Cleanup
rm -rf "$TEST_DIR"

echo "=== ALL TESTS PASSED ==="