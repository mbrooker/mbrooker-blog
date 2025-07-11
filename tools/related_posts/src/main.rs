use std::collections::{HashMap, HashSet};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// Number of related posts to include
const NUM_RELATED_POSTS: usize = 3;

#[derive(Debug, Clone)]
struct BlogPost {
    path: PathBuf,
    content: String,
    frontmatter: String,
    body: String,
    title: String,
    url: String,
    keywords: HashSet<String>,
}

fn extract_frontmatter_and_content(content: &str) -> (String, String) {
    if content.starts_with("---") {
        if let Some(end_index) = content[3..].find("---") {
            let frontmatter = &content[0..end_index + 6];
            let body = &content[end_index + 6..];
            return (frontmatter.to_string(), body.trim().to_string());
        }
    }
    
    ("".to_string(), content.to_string())
}

fn extract_title_and_url(path: &Path) -> (String, String) {
    let file_name = path.file_name().unwrap().to_str().unwrap();
    
    // Extract date and slug from filename (YYYY-MM-DD-slug.md)
    let parts: Vec<&str> = file_name.split('-').collect();
    if parts.len() >= 4 {
        let year = parts[0];
        let month = parts[1];
        let day = parts[2];
        
        // Join the remaining parts to form the slug
        let slug_with_ext: String = parts[3..].join("-");
        let slug = slug_with_ext.trim_end_matches(".md");
        
        // Construct URL in Jekyll format
        let url = format!("/{}/{}/{}/{}", year, month, day, slug);
        
        // Try to extract title from frontmatter or use slug as fallback
        let title = slug.replace('-', " ");
        
        return (title, url);
    }
    
    (file_name.to_string(), format!("/{}", file_name))
}

fn extract_keywords(text: &str) -> HashSet<String> {
    let stop_words: HashSet<&str> = [
        "a", "an", "the", "and", "or", "but", "if", "because", "as", "what",
        "when", "where", "how", "all", "any", "both", "each", "few", "more",
        "most", "some", "such", "no", "nor", "not", "only", "own", "same", "so",
        "than", "too", "very", "s", "t", "can", "will", "just", "don", "should",
        "now", "d", "ll", "m", "o", "re", "ve", "y", "ain", "aren", "couldn",
        "didn", "doesn", "hadn", "hasn", "haven", "isn", "ma", "mightn", "mustn",
        "needn", "shan", "shouldn", "wasn", "weren", "won", "wouldn", "i", "me",
        "my", "myself", "we", "our", "ours", "ourselves", "you", "your", "yours",
        "yourself", "yourselves", "he", "him", "his", "himself", "she", "her",
        "hers", "herself", "it", "its", "itself", "they", "them", "their", "theirs",
        "themselves", "this", "that", "these", "those", "am", "is", "are", "was",
        "were", "be", "been", "being", "have", "has", "had", "having", "do", "does",
        "did", "doing", "to", "from", "in", "out", "on", "off", "over", "under",
        "again", "further", "then", "once", "here", "there", "why", "how", "with",
        "about", "against", "between", "into", "through", "during", "before", "after",
        "above", "below", "up", "down", "for", "of", "at", "by", "for", "with",
        "about", "against", "between", "into", "through", "during", "before", "after"
    ].iter().cloned().collect();
    
    let mut keywords = HashSet::new();
    
    // Split text into words, convert to lowercase, and filter out stop words and short words
    for word in text.split(|c: char| !c.is_alphanumeric())
        .map(|s| s.to_lowercase())
        .filter(|s| s.len() > 3 && !stop_words.contains(s.as_str())) {
        keywords.insert(word);
    }
    
    keywords
}

fn calculate_similarity(post1: &BlogPost, post2: &BlogPost) -> f32 {
    let common_keywords = post1.keywords.intersection(&post2.keywords).count();
    let total_keywords = post1.keywords.union(&post2.keywords).count();
    
    if total_keywords == 0 {
        return 0.0;
    }
    
    common_keywords as f32 / total_keywords as f32
}

fn update_frontmatter(post: &BlogPost, related_posts: &[&BlogPost]) -> String {
    let mut new_frontmatter = post.frontmatter.clone();
    
    // Remove existing related_posts if present
    if new_frontmatter.contains("related_posts:") {
        let start_idx = new_frontmatter.find("related_posts:").unwrap();
        let mut end_idx = new_frontmatter[start_idx..].find("\n---").unwrap_or_else(|| new_frontmatter[start_idx..].len());
        end_idx += start_idx;
        
        let before = &new_frontmatter[..start_idx];
        let after = if end_idx + 1 < new_frontmatter.len() {
            &new_frontmatter[end_idx..]
        } else {
            ""
        };
        
        new_frontmatter = format!("{}{}", before, after);
    }
    
    // Add new related_posts
    if !related_posts.is_empty() {
        let related_urls: Vec<String> = related_posts.iter()
            .map(|p| p.url.clone())
            .collect();
        
        // Find where to insert the related_posts
        if let Some(idx) = new_frontmatter.rfind("---") {
            let (before, after) = new_frontmatter.split_at(idx);
            new_frontmatter = format!("{}related_posts:\n{}\n{}", 
                before,
                related_urls.iter()
                    .map(|url| format!("  - \"{}\"", url))
                    .collect::<Vec<_>>()
                    .join("\n"),
                after
            );
        }
    }
    
    new_frontmatter
}

fn find_blog_root() -> PathBuf {
    // First, try to use the current directory
    let current_dir = env::current_dir().expect("Failed to get current directory");
    
    // Check if we're in the blog root (has _posts directory)
    if current_dir.join("_posts").exists() {
        return current_dir;
    }
    
    // Check if we're in the tools/related_posts directory
    let parent_dir = current_dir.parent().expect("Failed to get parent directory");
    if parent_dir.file_name().map_or(false, |name| name == "tools") {
        let blog_root = parent_dir.parent().expect("Failed to get blog root directory");
        if blog_root.join("_posts").exists() {
            return blog_root.to_path_buf();
        }
    }
    
    // If we're already in the tools directory
    if current_dir.file_name().map_or(false, |name| name == "tools") {
        let blog_root = current_dir.parent().expect("Failed to get blog root directory");
        if blog_root.join("_posts").exists() {
            return blog_root.to_path_buf();
        }
    }
    
    // If we're in the related_posts directory
    if current_dir.file_name().map_or(false, |name| name == "related_posts") {
        let tools_dir = current_dir.parent().expect("Failed to get tools directory");
        if tools_dir.file_name().map_or(false, |name| name == "tools") {
            let blog_root = tools_dir.parent().expect("Failed to get blog root directory");
            if blog_root.join("_posts").exists() {
                return blog_root.to_path_buf();
            }
        }
    }
    
    panic!("Could not find blog root directory with _posts folder");
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting related posts generator...");
    
    // Check for dry-run flag
    let args: Vec<String> = env::args().collect();
    let dry_run = args.iter().any(|arg| arg == "--dry-run");
    
    if dry_run {
        println!("Running in dry-run mode (no files will be modified)");
    }
    
    // Find the blog root directory
    let blog_root = find_blog_root();
    println!("Found blog root at: {}", blog_root.display());
    
    // Set the posts directory relative to the blog root
    let posts_dir = blog_root.join("_posts");
    println!("Reading blog posts from: {}", posts_dir.display());
    
    let mut posts = Vec::new();
    
    // Read all markdown files in the _posts directory
    for entry in WalkDir::new(&posts_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "md") {
            let mut file = File::open(path)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            
            let (frontmatter, body) = extract_frontmatter_and_content(&content);
            let (title, url) = extract_title_and_url(path);
            
            // Extract keywords from title and body
            let text_for_keywords = format!("{} {}", title, body);
            let keywords = extract_keywords(&text_for_keywords);
            
            posts.push(BlogPost {
                path: path.to_path_buf(),
                content,
                frontmatter,
                body,
                title,
                url,
                keywords,
            });
        }
    }
    
    println!("Found {} posts", posts.len());
    
    // Calculate similarities and find related posts
    println!("Finding related posts...");
    let mut related_posts_map: HashMap<PathBuf, Vec<&BlogPost>> = HashMap::new();
    
    for i in 0..posts.len() {
        let mut similarities: Vec<(usize, f32)> = Vec::new();
        
        for j in 0..posts.len() {
            if i != j {
                let similarity = calculate_similarity(&posts[i], &posts[j]);
                similarities.push((j, similarity));
            }
        }
        
        // Sort by similarity (highest first)
        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // Take top N related posts
        let related: Vec<&BlogPost> = similarities.iter()
            .take(NUM_RELATED_POSTS)
            .map(|(idx, _)| &posts[*idx])
            .collect();
        
        related_posts_map.insert(posts[i].path.clone(), related);
    }
    
    // Update frontmatter in each post
    println!("Updating post frontmatter...");
    for post in &posts {
        if let Some(related) = related_posts_map.get(&post.path) {
            let updated_content = format!("{}{}", 
                update_frontmatter(post, related),
                post.body
            );
            
            // Write updated content back to file (unless in dry-run mode)
            if !dry_run {
                let mut file = File::create(&post.path)?;
                file.write_all(updated_content.as_bytes())?;
            }
            
            println!("Updated: {}", post.path.display());
        }
    }
    
    if dry_run {
        println!("Dry run completed. No files were modified.");
    } else {
        println!("Done! Updated {} posts with related posts information.", posts.len());
    }
    
    Ok(())
}
