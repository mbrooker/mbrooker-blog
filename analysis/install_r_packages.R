#!/usr/bin/env Rscript
#
# Install Required R Packages
#
# This script installs the R packages needed for the em dash analysis visualization
#

# List of required packages
required_packages <- c("ggplot2", "dplyr", "scales")

# Function to install packages if they're not already installed
install_if_missing <- function(packages) {
  for (pkg in packages) {
    if (!require(pkg, character.only = TRUE, quietly = TRUE)) {
      cat(sprintf("Installing package: %s\n", pkg))
      install.packages(pkg, dependencies = TRUE, repos = "https://cran.rstudio.com/")
      
      # Try to load the package after installation
      if (!require(pkg, character.only = TRUE, quietly = TRUE)) {
        cat(sprintf("ERROR: Failed to install or load package: %s\n", pkg))
      } else {
        cat(sprintf("Successfully installed and loaded: %s\n", pkg))
      }
    } else {
      cat(sprintf("Package already available: %s\n", pkg))
    }
  }
}

cat("Checking and installing required R packages...\n")
cat("=" * 50, "\n")

install_if_missing(required_packages)

cat("\nPackage installation complete!\n")
cat("You can now run: Rscript em_dash_plot_simple.R\n")
