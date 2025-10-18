import random
import string
import argparse


def generate_relation(file_path: str, n: int = 1000, density: float = 0.02):
    """
    Generate a large relation test file using only non-space symbols.

    Parameters:
        file_path: output file name (e.g., "files/relation_big.txt")
        n: number of elements in the base set
        density: fraction of possible pairs to include (0.0–1.0)
    """
    symbol_pool = string.ascii_letters + string.digits + string.punctuation

    if n > len(symbol_pool):
        extra_needed = n - len(symbol_pool)
        extra_symbols = [
            chr(c) for c in range(0x2500, 0x2500 + extra_needed) if not chr(c).isspace()
        ]
        base = list(symbol_pool) + extra_symbols
    else:
        base = random.sample(symbol_pool, n)

    with open(file_path, "w", encoding="utf-8") as f:
        f.write(" ".join(base) + "\n")

        for a in base:
            for b in base:
                if random.random() < density:
                    f.write(f"{a} {b}\n")

    print(f"✅ Generated {file_path} with n={n}, density={density}, symbols only")


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

    args = parser.parse_args()
    generate_relation(args.output, args.num, args.density)


if __name__ == "__main__":
    main()
