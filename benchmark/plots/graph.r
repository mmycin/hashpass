main <- function() {
  # Read the CSV file correctly
  data <- read.csv("plots/benchmark.csv", header = FALSE, stringsAsFactors = FALSE)

  # Extracting values correctly
  rounds <- as.numeric(data[1, -1])  # First row (Rounds), skipping first column
  time_s <- as.numeric(data[2, -1])  # Second row (Time), skipping first column

  # Check for any NA values due to conversion issues
  if (any(is.na(rounds)) || any(is.na(time_s))) {
    stop("Error: Data conversion failed. Check CSV formatting.")
  }

  # Open a PNG device
  png("benchmark.png", width = 1000, height = 500)

  # Create the plot
  plot(rounds, time_s, type = "o", col = "blue", pch = 16, lty = 1,
       xlab = "Rounds", ylab = "Time (s)", main = "Benchmark Performance")

  # Close the PNG device
  dev.off()
}

# Run the function
main()
