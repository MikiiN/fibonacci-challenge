import matplotlib.pyplot as plt
import numpy as np
import os

data_dir = "data"

if not os.path.exists(data_dir):
    raise FileNotFoundError(f"Data directory not found: {data_dir}")

files = [f for f in os.listdir(data_dir) if os.path.isfile(os.path.join(data_dir, f))]
if not files:
    raise ValueError(f"No data files found in directory: {data_dir}")
plt.figure(figsize=(10, 6))

for filename in files:
    filepath = os.path.join(data_dir, filename)
    try:
        data = np.loadtxt(filepath, delimiter=None)

        if data.shape[1] != 3:
            print(f"Skipping {filename}: expected 3 columns, found {data.shape[1]}")
            continue

        index, number, time_ns = data[:, 0], data[:, 1], data[:, 2]

        time_ms = time_ns / 1e6

        plt.plot(index, time_ms, label=filename[:-4], linewidth=2)
    except Exception as e:
        print(f"Error reading {filename}: {e}")

plt.title("Algorithms Evaluations")
plt.xlabel("Fibonacci number index")
plt.ylabel("Time [ms]")
plt.grid(True)
plt.legend()
plt.tight_layout()
plt.show()