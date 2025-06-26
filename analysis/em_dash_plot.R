#!/usr/bin/env Rscript
#
# Em Dash Analysis Visualization
# 
# This script creates a scatter plot of em dashes per word by year
# with a linear trend line using ggplot2.
#

# Load required libraries
library(ggplot2)
library(dplyr)
library(scales)

# Set working directory to script location
script_dir <- dirname(rstudioapi::getSourceEditorContext()$path)
if (length(script_dir) == 0 || script_dir == "") {
  # Fallback if not running in RStudio
  script_dir <- getwd()
}
setwd(script_dir)

# Read the yearly summary data
data <- read.csv("em_dash_analysis_by_year.csv", stringsAsFactors = FALSE)

# Convert year to numeric for proper plotting
data$year <- as.numeric(data$year)

# Create the scatter plot with linear trend
p <- ggplot(data, aes(x = year, y = em_dashes_per_word)) +
  # Add scatter points
  geom_point(aes(size = total_posts), 
             color = "steelblue", 
             alpha = 0.7) +
  
  # Add linear trend line with confidence interval
  geom_smooth(method = "lm", 
              se = TRUE, 
              color = "red", 
              linetype = "dashed",
              alpha = 0.3) +
  
  # Customize the plot
  labs(
    title = "Em Dash Usage in Blog Posts Over Time",
    subtitle = "Em dashes per word by year with linear trend",
    x = "Year",
    y = "Em Dashes per Word",
    size = "Number of Posts",
    caption = "Point size represents number of posts published that year"
  ) +
  
  # Set axis scales
  scale_x_continuous(breaks = seq(2012, 2025, 2)) +
  scale_y_continuous(labels = scales::number_format(accuracy = 0.001)) +
  
  # Apply a clean theme
  theme_minimal() +
  theme(
    plot.title = element_text(size = 16, face = "bold", hjust = 0.5),
    plot.subtitle = element_text(size = 12, hjust = 0.5, color = "gray60"),
    plot.caption = element_text(size = 10, color = "gray60"),
    axis.title = element_text(size = 12),
    axis.text = element_text(size = 10),
    legend.position = "bottom",
    panel.grid.minor = element_blank()
  )

# Display the plot
print(p)

# Save the plot
ggsave("em_dash_trend_plot.png", 
       plot = p, 
       width = 10, 
       height = 6, 
       dpi = 300,
       bg = "white")

# Calculate and display trend statistics
model <- lm(em_dashes_per_word ~ year, data = data)
trend_summary <- summary(model)

cat("\n=== LINEAR TREND ANALYSIS ===\n")
cat(sprintf("Slope: %.6f em dashes per word per year\n", coef(model)[2]))
cat(sprintf("R-squared: %.4f\n", trend_summary$r.squared))
cat(sprintf("P-value: %.4f\n", trend_summary$coefficients[2, 4]))

if (trend_summary$coefficients[2, 4] < 0.05) {
  cat("The trend is statistically significant (p < 0.05)\n")
} else {
  cat("The trend is not statistically significant (p >= 0.05)\n")
}

# Additional statistics
cat("\n=== SUMMARY STATISTICS ===\n")
cat(sprintf("Years analyzed: %d to %d\n", min(data$year), max(data$year)))
cat(sprintf("Total posts: %d\n", sum(data$total_posts)))
cat(sprintf("Average em dashes per word: %.6f\n", mean(data$em_dashes_per_word)))
cat(sprintf("Standard deviation: %.6f\n", sd(data$em_dashes_per_word)))
cat(sprintf("Minimum rate: %.6f (year %d)\n", 
            min(data$em_dashes_per_word), 
            data$year[which.min(data$em_dashes_per_word)]))
cat(sprintf("Maximum rate: %.6f (year %d)\n", 
            max(data$em_dashes_per_word), 
            data$year[which.max(data$em_dashes_per_word)]))

# Create a secondary plot showing post volume over time
p2 <- ggplot(data, aes(x = year, y = total_posts)) +
  geom_col(fill = "steelblue", alpha = 0.7, width = 0.8) +
  geom_smooth(method = "loess", se = FALSE, color = "red", linetype = "dashed") +
  labs(
    title = "Blog Post Volume Over Time",
    x = "Year",
    y = "Number of Posts",
    caption = "Annual blog post publication count"
  ) +
  scale_x_continuous(breaks = seq(2012, 2025, 2)) +
  theme_minimal() +
  theme(
    plot.title = element_text(size = 14, face = "bold", hjust = 0.5),
    plot.caption = element_text(size = 10, color = "gray60"),
    axis.title = element_text(size = 12),
    axis.text = element_text(size = 10),
    panel.grid.minor = element_blank()
  )

# Save the secondary plot
ggsave("blog_post_volume.png", 
       plot = p2, 
       width = 10, 
       height = 6, 
       dpi = 300,
       bg = "white")

cat("\nPlots saved as 'em_dash_trend_plot.png' and 'blog_post_volume.png'\n")
