#!/bin/bash

set -e

echo "===================================="
echo "🚀 Building project..."
echo "===================================="
cargo build --release

FILES_DIR="files/relations"

FILES=(
  "base.txt"
  "reflexive.txt"
  "symmetric.txt"
  "antisymmetric.txt"
  "transitive.txt"
  "equivalence.txt"
  "partial_order.txt"
)

echo
echo "===================================="
echo "🧪 Running standard relation tests..."
echo "===================================="

for FILE in "${FILES[@]}"; do
    echo
    echo "=============================="
    echo "Testing file: $FILE"
    echo "=============================="
    ./target/release/rgu-labs-term3-discrete-math -t2 "$FILES_DIR/$FILE"
done


echo
echo "===================================="
echo "⚙️ Running stress test..."
echo "===================================="

BIG_FILE="files/relations/big.txt"

python3 generators/relations.py -o "$BIG_FILE" -n 1000 -d 1.0

echo
echo "🏃 Running program on large relation (timed + profiled)..."

START_TIME=$(date +%s.%N)

if command -v perf &>/dev/null; then
    echo
    echo "🔬 Using perf for live performance metrics..."
    echo "------------------------------------"
    perf stat -d -d -d \
      ./target/release/rgu-labs-term3-discrete-math -t2s "$BIG_FILE" \
      2>&1
    echo "------------------------------------"
else
    echo
    echo "ℹ️ 'perf' not found. Falling back to '/usr/bin/time'..."
    echo "------------------------------------"
    /usr/bin/time -v ./target/release/rgu-labs-term3-discrete-math -t2s "$BIG_FILE"
    echo "------------------------------------"
fi

END_TIME=$(date +%s.%N)
RUNTIME=$(echo "$END_TIME - $START_TIME" | bc)

printf "\n🕒 Exact execution time: %.3f seconds\n" "$RUNTIME"

rm -f "$BIG_FILE"
echo "🧹 Deleted $BIG_FILE"


echo
echo "✅ All tests completed successfully."
