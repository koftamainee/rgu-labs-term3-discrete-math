import random
import string
import argparse


def generate_relation_fast(file_path: str, n: int = 1000, density: float = 0.02):
    """
    Generate a large relation test file using only non-space symbols.
    Optimized for speed with n=100,000.
    """
    symbol_pool = string.ascii_letters + string.digits + string.punctuation

    # Generate base set
    if n > len(symbol_pool):
        extra_needed = n - len(symbol_pool)
        extra_symbols = []
        code_point = 0x2600  # Start from safe range

        while len(extra_symbols) < extra_needed:
            # Skip the surrogate range (0xD800-0xDFFF)
            if 0xD800 <= code_point <= 0xDFFF:
                code_point = 0xE000
                continue

            try:
                char = chr(code_point)
                if not char.isspace():
                    extra_symbols.append(char)
            except (ValueError, UnicodeEncodeError):
                pass

            code_point += 1
            if code_point > 0x10FFFF:
                # If we run out of Unicode, reuse existing pool
                extra_symbols = extra_symbols * (
                    (extra_needed // len(extra_symbols)) + 1
                )
                break

        base = list(symbol_pool) + extra_symbols[:extra_needed]
    else:
        base = random.sample(symbol_pool, n)

    # OPTIMIZATION 1: Write in large chunks instead of line by line
    with open(file_path, "w", encoding="utf-8", buffering=8192 * 8) as f:
        # Write header line
        f.write(" ".join(base) + "\n")

        # OPTIMIZATION 2: Pre-calculate total pairs and use batch writing
        total_pairs_expected = int(n * n * density)
        print(f"Generating approximately {total_pairs_expected:,} pairs...")

        # OPTIMIZATION 3: Use list comprehension and join for batch writing
        batch_size = 10000
        batch = []

        # OPTIMIZATION 4: Use random choices for entire row at once
        # Generate all random values in advance
        random_values = random.random()

        # For very large n, we need a smarter approach
        if n * n * density > 1000000:
            # OPTIMIZATION 5: For sparse matrices, generate indices directly
            # This is MUCH faster for low densities

            # Calculate total pairs to generate
            total_pairs = int(n * n * density)

            # Generate random indices
            indices = random.sample(range(n * n), total_pairs)

            # Convert to coordinates and write
            for idx in indices:
                i = idx // n
                j = idx % n
                batch.append(f"{base[i]} {base[j]}")

                # Write in batches
                if len(batch) >= batch_size:
                    f.write("\n".join(batch) + "\n")
                    batch = []

            # Write remaining
            if batch:
                f.write("\n".join(batch) + "\n")

        else:
            # For moderate densities, use row-by-row with optimization
            random_threshold = density

            for i in range(n):
                # Generate a row of random values at once
                row_random = [random.random() for _ in range(n)]

                for j in range(n):
                    if row_random[j] < random_threshold:
                        batch.append(f"{base[i]} {base[j]}")

                        # Write in batches
                        if len(batch) >= batch_size:
                            f.write("\n".join(batch) + "\n")
                            batch = []

            # Write any remaining pairs
            if batch:
                f.write("\n".join(batch) + "\n")

    print(f"✅ Generated {file_path} with n={n}, density={density}")
    print(f"   Base set size: {len(base)} symbols")


def generate_relation_very_fast(file_path: str, n: int = 100000, density: float = 0.99):
    """
    Ultra-fast version for very large n and high density.
    For density >= 0.5, it's faster to generate all pairs and skip some.
    """
    symbol_pool = string.ascii_letters + string.digits + string.punctuation

    # Generate base set (same as before)
    if n > len(symbol_pool):
        extra_needed = n - len(symbol_pool)
        # Use a simpler approach for speed
        extra_symbols = [f"X{i:06d}" for i in range(extra_needed)]
        base = list(symbol_pool) + extra_symbols[:extra_needed]
    else:
        base = random.sample(symbol_pool, n)

    with open(file_path, "w", encoding="utf-8", buffering=8192 * 16) as f:
        # Write header
        f.write(" ".join(base) + "\n")

        # For very high density like 0.99, generate complement instead
        if density > 0.5:
            # Generate missing pairs instead of existing ones
            missing_density = 1.0 - density
            total_missing = int(n * n * missing_density)

            print(
                f"High density ({density:.2f}), generating {total_missing:,} missing pairs..."
            )

            if total_missing == 0:
                # Write all pairs (density = 1.0 or very close)
                batch = []
                for i in range(n):
                    for j in range(n):
                        batch.append(f"{base[i]} {base[j]}")
                        if len(batch) >= 10000:
                            f.write("\n".join(batch) + "\n")
                            batch = []
                if batch:
                    f.write("\n".join(batch) + "\n")
            else:
                # Generate missing indices
                missing_indices = set(random.sample(range(n * n), total_missing))

                # Write all pairs except missing ones
                batch = []
                for idx in range(n * n):
                    if idx not in missing_indices:
                        i = idx // n
                        j = idx % n
                        batch.append(f"{base[i]} {base[j]}")

                        if len(batch) >= 10000:
                            f.write("\n".join(batch) + "\n")
                            batch = []

                if batch:
                    f.write("\n".join(batch) + "\n")
        else:
            # For low density, use direct random generation
            total_pairs = int(n * n * density)
            print(f"Generating {total_pairs:,} random pairs...")

            indices = random.sample(range(n * n), total_pairs)
            batch = []

            for idx in indices:
                i = idx // n
                j = idx % n
                batch.append(f"{base[i]} {base[j]}")

                if len(batch) >= 10000:
                    f.write("\n".join(batch) + "\n")
                    batch = []

            if batch:
                f.write("\n".join(batch) + "\n")

    print(f"✅ Generated {file_path} with n={n}, density={density}")


def main():
    parser = argparse.ArgumentParser(
        description="Generate a random relation test file for discrete math experiments."
    )
    parser.add_argument(
        "-o",
        "--output",
        type=str,
        default="files/relation_big.txt",
        help="Output file path (default: files/relation_big.txt)",
    )
    parser.add_argument(
        "-n",
        "--num",
        type=int,
        default=1000,
        help="Number of elements in the set (default: 1000)",
    )
    parser.add_argument(
        "-d",
        "--density",
        type=float,
        default=0.02,
        help="Fraction of possible pairs to include (0.0–1.0, default: 0.02)",
    )
    parser.add_argument(
        "-f",
        "--fast",
        action="store_true",
        help="Use ultra-fast generation method (recommended for n > 10000)",
    )

    args = parser.parse_args()

    if args.fast or args.num > 10000:
        generate_relation_very_fast(args.output, args.num, args.density)
    else:
        generate_relation_fast(args.output, args.num, args.density)


if __name__ == "__main__":
    main()
