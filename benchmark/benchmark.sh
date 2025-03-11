#!/bin/bash

# Run the benchmark.py script 
python generator\\benchmark.py

# Move the benchmark.csv file to the ../plots directory
mv benchmark.csv plots/

# Run the graph.r script to generate the benchmark.png file
Rscript plots\\graph.r

mv benchmark.png result/

echo "Benchmark completed successfully!"
open result/benchmark.png