import random
import string


def generate_set_commands(
    file_path: str, n_sets: int = 3, elements_per_set: int = 5, universe_size: int = 10
):
    """
    Generate a file with random set manipulation commands.

    Example of output format:
        new A
        new B
        add A a
        add A b
        add B b
        see
        A + B
        A & B
        A - B
        pow A
        rem A b
        del B
        see

    Parameters:
        file_path: where to save the command file
        n_sets: how many sets to create
        elements_per_set: approximate number of elements to add to each
        universe_size: number of unique elements to pick from (e.g. 10 → {'a'..'j'})
    """
    letters = string.ascii_lowercase[:universe_size]
    set_names = [chr(ord("A") + i) for i in range(n_sets)]

    lines = []

    for name in set_names:
        lines.append(f"new {name}")

    for name in set_names:
        elems = random.sample(letters, k=min(elements_per_set, universe_size))
        for e in elems:
            lines.append(f"add {name} {e}")

    lines.append("see")
    for name in set_names:
        lines.append(f"see {name}")

    for a in set_names:
        for b in set_names:
            if a != b:
                lines.append(f"{a} + {b}")
                lines.append(f"{a} & {b}")
                lines.append(f"{a} - {b}")
                lines.append(f"{a} < {b}")
                lines.append(f"{b} < {a}")
                lines.append(f"{a} = {b}")

    for name in set_names:
        lines.append(f"pow {name}")
        to_remove = random.choice(letters)
        lines.append(f"rem {name} {to_remove}")

    for name in random.sample(set_names, k=len(set_names) // 2 or 1):
        lines.append(f"del {name}")

    lines.append("see")

    with open(file_path, "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")

    print(
        f"✅ Generated {file_path} with {len(set_names)} sets and {len(lines)} commands."
    )


if __name__ == "__main__":
    generate_set_commands(
        "files/sets_big.txt", n_sets=10, elements_per_set=20, universe_size=10
    )
