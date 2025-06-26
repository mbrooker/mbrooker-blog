#!/usr/bin/env python3
"""
Blog Post Em Dash Analyzer

This script analyzes blog posts for em dash usage and generates a CSV report
showing em dashes per word by year.
"""

import os
import re
import csv
import html
from pathlib import Path
from collections import defaultdict
from typing import Dict, List, Tuple


def extract_year_from_filename(filename: str) -> str:
    """Extract year from blog post filename (format: YYYY-MM-DD-title.md)"""
    match = re.match(r'^(\d{4})-\d{2}-\d{2}-.+\.md$', filename)
    if match:
        return match.group(1)
    return "unknown"


def count_em_dashes(text: str) -> int:
    """
    Count various forms of em dashes in text:
    - Unicode em dash: —
    - HTML entity: &mdash;
    - Double hyphen: --
    - Triple hyphen: ---
    """
    # Decode HTML entities first
    text = html.unescape(text)
    
    em_dash_count = 0
    
    # Unicode em dash (—)
    em_dash_count += text.count('—')
    
    # HTML entity &mdash; (case insensitive)
    em_dash_count += len(re.findall(r'&mdash;', text, re.IGNORECASE))
    
    # Triple hyphen --- (often used as em dash in markdown)
    em_dash_count += text.count('---')
    
    return em_dash_count


def count_words(text: str) -> int:
    """
    Count words in text, excluding YAML front matter and HTML tags.
    """
    # Remove YAML front matter (between --- markers at the start)
    text = re.sub(r'^---.*?---', '', text, flags=re.DOTALL | re.MULTILINE)
    
    # Remove HTML tags
    text = re.sub(r'<[^>]+>', '', text)
    
    # Remove markdown links but keep the text
    text = re.sub(r'\[([^\]]+)\]\([^)]+\)', r'\1', text)
    
    # Remove markdown formatting
    text = re.sub(r'[*_`#]+', '', text)
    
    # Split on whitespace and count non-empty strings
    words = [word.strip() for word in text.split() if word.strip()]
    
    return len(words)


def analyze_blog_post(filepath: Path) -> Tuple[str, str, int, int, float]:
    """
    Analyze a single blog post file.
    Returns: (filename, year, em_dash_count, word_count, em_dashes_per_word)
    """
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        filename = filepath.name
        year = extract_year_from_filename(filename)
        em_dash_count = count_em_dashes(content)
        word_count = count_words(content)
        
        # Calculate em dashes per word (avoid division by zero)
        em_dashes_per_word = em_dash_count / word_count if word_count > 0 else 0
        
        return filename, year, em_dash_count, word_count, em_dashes_per_word
        
    except Exception as e:
        print(f"Error processing {filepath}: {e}")
        return filepath.name, "error", 0, 0, 0


def main():
    """Main function to analyze all blog posts and generate CSV report."""
    
    # Get the directory containing this script (should be the _posts directory)
    posts_dir = Path(__file__).parent
    
    # Find all markdown files
    md_files = list(posts_dir.glob('*.md'))
    
    if not md_files:
        print("No markdown files found in the current directory.")
        return
    
    print(f"Found {len(md_files)} markdown files to analyze...")
    
    # Analyze each file
    results = []
    yearly_stats = defaultdict(lambda: {'total_em_dashes': 0, 'total_words': 0, 'post_count': 0})
    
    for md_file in md_files:
        filename, year, em_dash_count, word_count, em_dashes_per_word = analyze_blog_post(md_file)
        
        results.append({
            'filename': filename,
            'year': year,
            'em_dash_count': em_dash_count,
            'word_count': word_count,
            'em_dashes_per_word': em_dashes_per_word
        })
        
        # Aggregate yearly statistics
        if year != "error" and year != "unknown":
            yearly_stats[year]['total_em_dashes'] += em_dash_count
            yearly_stats[year]['total_words'] += word_count
            yearly_stats[year]['post_count'] += 1
        
        print(f"Processed: {filename} ({year}) - {em_dash_count} em dashes, {word_count} words")
    
    # Sort results by year and filename
    results.sort(key=lambda x: (x['year'], x['filename']))
    
    # Write detailed CSV file
    detailed_csv_path = posts_dir / 'em_dash_analysis_detailed.csv'
    with open(detailed_csv_path, 'w', newline='', encoding='utf-8') as csvfile:
        fieldnames = ['filename', 'year', 'em_dash_count', 'word_count', 'em_dashes_per_word']
        writer = csv.DictWriter(csvfile, fieldnames=fieldnames)
        
        writer.writeheader()
        for result in results:
            writer.writerow(result)
    
    print(f"\nDetailed analysis saved to: {detailed_csv_path}")
    
    # Write yearly summary CSV file
    yearly_csv_path = posts_dir / 'em_dash_analysis_by_year.csv'
    with open(yearly_csv_path, 'w', newline='', encoding='utf-8') as csvfile:
        fieldnames = ['year', 'total_posts', 'total_em_dashes', 'total_words', 'em_dashes_per_word']
        writer = csv.DictWriter(csvfile, fieldnames=fieldnames)
        
        writer.writeheader()
        
        # Sort years
        for year in sorted(yearly_stats.keys()):
            stats = yearly_stats[year]
            em_dashes_per_word = stats['total_em_dashes'] / stats['total_words'] if stats['total_words'] > 0 else 0
            
            writer.writerow({
                'year': year,
                'total_posts': stats['post_count'],
                'total_em_dashes': stats['total_em_dashes'],
                'total_words': stats['total_words'],
                'em_dashes_per_word': round(em_dashes_per_word, 6)
            })
    
    print(f"Yearly summary saved to: {yearly_csv_path}")
    
    # Print summary statistics
    print("\n=== SUMMARY BY YEAR ===")
    print(f"{'Year':<6} {'Posts':<6} {'Em Dashes':<10} {'Words':<8} {'Per Word':<10}")
    print("-" * 50)
    
    for year in sorted(yearly_stats.keys()):
        stats = yearly_stats[year]
        em_dashes_per_word = stats['total_em_dashes'] / stats['total_words'] if stats['total_words'] > 0 else 0
        print(f"{year:<6} {stats['post_count']:<6} {stats['total_em_dashes']:<10} {stats['total_words']:<8} {em_dashes_per_word:.6f}")
    
    # Overall statistics
    total_posts = sum(stats['post_count'] for stats in yearly_stats.values())
    total_em_dashes = sum(stats['total_em_dashes'] for stats in yearly_stats.values())
    total_words = sum(stats['total_words'] for stats in yearly_stats.values())
    overall_per_word = total_em_dashes / total_words if total_words > 0 else 0
    
    print("-" * 50)
    print(f"{'TOTAL':<6} {total_posts:<6} {total_em_dashes:<10} {total_words:<8} {overall_per_word:.6f}")


if __name__ == "__main__":
    main()
