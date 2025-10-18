import random
import string


def generate_relation(file_path: str, n: int = 1000, density: float = 0.02):
    """
    Generate a large relation test file using only non-space symbols.

    Parameters:
        file_path: output file name (e.g., "relation_big.txt")
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


if __name__ == "__main__":
    generate_relation("files/relations_big.txt", n=10000, density=0.52)
