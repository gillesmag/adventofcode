from config import INPUTS_DIR, EXAMPLES_DIR
from pathlib import Path
import importlib


def main():
    days = sorted([p.stem for p in Path(__file__).parent.glob("day*.py")])

    for didx, day in enumerate(days):
        module = importlib.import_module(day)
        parts = [p for p in dir(module) if p.startswith("part_")]
        examples = [p for p in dir(module) if p.startswith("example_")]
        message = f"Running {day}"
        print(message)
        print("=" * len(message))
        for idx, example in enumerate(examples):
            result = getattr(module, example)(
                open(EXAMPLES_DIR / f"{day}-{idx+1}.txt").read()
            )
            print(f"  Example {idx+1}:", result)

        for idx, part in enumerate(parts):
            result = getattr(module, part)(open(INPUTS_DIR / f"{day}.txt").read())
            print(f"  Part {idx+1}   :", result)

        if didx >= 0:
            print("\n")


if __name__ == "__main__":
    main()
