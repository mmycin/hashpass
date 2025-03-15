from lib.utils import run_benchmark, save_to_csv
from typing import Final

PROMPT: Final[str] = "hashpass -g mycin -r "
CSV_FILE: Final[str] = "benchmark.csv"


def main():
    """Main function to execute the benchmarking process."""
    benchmark_data = run_benchmark(4, 17, PROMPT)
    save_to_csv(CSV_FILE, benchmark_data)
    print(f"Benchmark data saved to {CSV_FILE}")

if __name__ == "__main__":
    main()
