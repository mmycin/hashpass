import time
import csv
import os

def run_benchmark(start: int, end: int, PROMPT: str) -> dict[int, float]:
    """Runs the benchmarking process for hashpass command."""
    times = {}
    for i in range(start, end + 1):
        times[i] = measure_execution_time(f"{PROMPT}{i}")
    return times

def measure_execution_time(command: str) -> float:
    """Measures the execution time of a given command."""
    start_time = time.time()
    os.system(command)
    return time.time() - start_time

def save_to_csv(filename: str, data: dict[int, float]) -> None:
    """Saves the benchmark data to a CSV file."""
    with open(filename, "w", newline="") as file:
        writer = csv.writer(file)
        writer.writerow(["Rounds"] + list(data.keys()))
        writer.writerow(["Time (s)"] + list(data.values()))