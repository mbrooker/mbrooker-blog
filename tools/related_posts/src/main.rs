use std::collections::{HashMap, HashSet};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use aws_sdk_bedrockruntime::Client as BedrockClient;
use aws_sdk_bedrockruntime::primitives::Blob;
use serde::{Deserialize, Serialize};
use ndarray::{Array1, ArrayView1};
use futures::future::join_all;

mod keyword_extraction;

// Number of related posts to include
const NUM_RELATED_POSTS: usize = 3;
// Titan Text Embeddings model ID
const TITAN_EMBEDDINGS_MODEL_ID: &str = "amazon.titan-embed-text-v1";

#[derive(Debug, Clone)]
struct BlogPost {
    path: PathBuf,
    content: String,
    frontmatter: String,
    body: String,
    title: String,
    url: String,
    embedding: Option<Vec<f32>>,
    keywords: HashSet<String>,
}

#[derive(Serialize, Deserialize)]
struct TitanEmbeddingRequest {
    inputText: String,
}

#[derive(Serialize, Deserialize)]
struct TitanEmbeddingResponse {
    embedding: Vec<f32>,
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
        
        // Construct URL in Jekyll format (with trailing slash)
        let url = format!("/{}/{}/{}/{}.html", year, month, day, slug);
        
        // Try to extract title from frontmatter or use slug as fallback
        let title = slug.replace('-', " ");
        
        return (title, url);
    }
    
    (file_name.to_string(), format!("/{}", file_name))
}

async fn get_embedding(client: &BedrockClient, text: &str) -> Result<Vec<f32>, Box<dyn Error>> {
    // Truncate text if it's too long (Titan has a limit of around 8K tokens)
    let truncated_text = if text.len() > 8000 {
        eprintln!("Warning: Text truncated from {} to 8000 characters", text.len());
        &text[0..8000]
    } else {
        text
    };
    
    let request = TitanEmbeddingRequest {
        inputText: truncated_text.to_string(),
    };
    
    let request_json = match serde_json::to_string(&request) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error serializing embedding request to JSON: {}", e);
            eprintln!("Request data: input_text length = {}", truncated_text.len());
            return Err(Box::new(e));
        }
    };
    
    eprintln!("Making embedding request to model: {}", TITAN_EMBEDDINGS_MODEL_ID);
    eprintln!("Request payload size: {} bytes", request_json.len());
    
    let response = match client
        .invoke_model()
        .model_id(TITAN_EMBEDDINGS_MODEL_ID)
        .content_type("application/json")
        .accept("application/json")
        .body(Blob::new(request_json.clone()))
        .send()
        .await {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("Error invoking Bedrock model:");
            eprintln!("  Model ID: {}", TITAN_EMBEDDINGS_MODEL_ID);
            eprintln!("  Error: {}", e);
            eprintln!("  Request payload: {}", request_json);
            
            // Try to extract more specific error information
            if let Some(service_err) = e.as_service_error() {
                eprintln!("  Service error details: {:?}", service_err);
            }
            
            return Err(Box::new(e));
        }
    };
    
    let response_body = response.body.as_ref();
    eprintln!("Received response body size: {} bytes", response_body.len());
    
    let response_str = match std::str::from_utf8(response_body) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error converting response body to UTF-8 string: {}", e);
            eprintln!("Response body (first 500 bytes): {:?}", 
                &response_body[..std::cmp::min(500, response_body.len())]);
            return Err(Box::new(e));
        }
    };
    
    eprintln!("Response body: {}", response_str);
    
    let response_json: TitanEmbeddingResponse = match serde_json::from_str(response_str) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error parsing response JSON: {}", e);
            eprintln!("Raw response: {}", response_str);
            eprintln!("Expected format: {{\"embedding\": [f32, ...]}}");
            return Err(Box::new(e));
        }
    };
    
    eprintln!("Successfully parsed embedding with {} dimensions", response_json.embedding.len());
    Ok(response_json.embedding)
}

fn cosine_similarity(v1: &[f32], v2: &[f32]) -> f32 {
    let v1_array = Array1::from_vec(v1.to_vec());
    let v2_array = Array1::from_vec(v2.to_vec());
    
    let v1_view = v1_array.view();
    let v2_view = v2_array.view();
    
    let dot_product = dot(&v1_view, &v2_view);
    let norm1 = norm(&v1_view);
    let norm2 = norm(&v2_view);
    
    if norm1 == 0.0 || norm2 == 0.0 {
        return 0.0;
    }
    
    dot_product / (norm1 * norm2)
}

fn dot(v1: &ArrayView1<f32>, v2: &ArrayView1<f32>) -> f32 {
    v1.dot(v2)
}

fn norm(v: &ArrayView1<f32>) -> f32 {
    v.dot(v).sqrt()
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting related posts generator...");
    
    // Check for dry-run flag and force-keywords flag
    let args: Vec<String> = env::args().collect();
    let dry_run = args.iter().any(|arg| arg == "--dry-run");
    let force_keywords = args.iter().any(|arg| arg == "--force-keywords");
    
    if dry_run {
        println!("Running in dry-run mode (no files will be modified)");
    }
    
    if force_keywords {
        println!("Forcing keyword-based similarity instead of embeddings");
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
            
            // Extract keywords for fallback approach
            let text_for_keywords = format!("{} {}", title, body);
            let keywords = keyword_extraction::extract_keywords(&text_for_keywords);
            
            posts.push(BlogPost {
                path: path.to_path_buf(),
                content,
                frontmatter,
                body,
                title,
                url,
                embedding: None,
                keywords,
            });
        }
    }
    
    println!("Found {} posts", posts.len());
    
    let mut use_embeddings = !force_keywords;
    let mut posts_with_embeddings = Vec::new();
    
    if use_embeddings {
        // Try to initialize AWS SDK
        println!("Initializing AWS SDK...");
        match aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await {
            config => {
                let bedrock_client = BedrockClient::new(&config);
                
                // Generate embeddings for all posts
                println!("Generating embeddings using Amazon Bedrock Titan Text Embeddings...");
                
                // Process posts in batches to avoid overwhelming the API
                let batch_size = 10;
                let mut embedding_failures = 0;
                
                for chunk in posts.chunks(batch_size) {
                    let mut futures = Vec::new();
                    
                    for post in chunk {
                        let text_for_embedding = format!("{} {}", post.title, post.body);
                        let client = &bedrock_client;
                        
                        let future = async move {
                            let mut post_clone = post.clone();
                            match get_embedding(client, &text_for_embedding).await {
                                Ok(embedding) => {
                                    post_clone.embedding = Some(embedding);
                                    println!("Generated embedding for: {}", post_clone.path.display());
                                    post_clone
                                },
                                Err(e) => {
                                    eprintln!("Error generating embedding for {}: {}", post_clone.path.display(), e);
                                    post_clone
                                }
                            }
                        };
                        
                        futures.push(future);
                    }
                    
                    let results = join_all(futures).await;
                    
                    // Count failures
                    for post in &results {
                        if post.embedding.is_none() {
                            embedding_failures += 1;
                        }
                    }
                    
                    posts_with_embeddings.extend(results);
                }
                
                // If more than 50% of embeddings failed, fall back to keyword-based approach
                if embedding_failures > posts.len() / 2 {
                    println!("Too many embedding failures ({}). Falling back to keyword-based approach.", embedding_failures);
                    use_embeddings = false;
                    posts_with_embeddings = posts.clone();
                }
            }
        }
    } else {
        // Use keyword-based approach
        posts_with_embeddings = posts.clone();
    }
    
    // Calculate similarities and find related posts
    println!("Finding related posts...");
    let mut related_posts_map: HashMap<PathBuf, Vec<&BlogPost>> = HashMap::new();
    
    if use_embeddings {
        println!("Using embedding-based similarity (cosine similarity)");
        for i in 0..posts_with_embeddings.len() {
            let mut similarities: Vec<(usize, f32)> = Vec::new();
            
            if let Some(ref embedding_i) = posts_with_embeddings[i].embedding {
                for j in 0..posts_with_embeddings.len() {
                    if i != j {
                        if let Some(ref embedding_j) = posts_with_embeddings[j].embedding {
                            let similarity = cosine_similarity(embedding_i, embedding_j);
                            similarities.push((j, similarity));
                        }
                    }
                }
                
                // Sort by similarity (highest first)
                similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                
                // Take top N related posts
                let related: Vec<&BlogPost> = similarities.iter()
                    .take(NUM_RELATED_POSTS)
                    .map(|(idx, _)| &posts_with_embeddings[*idx])
                    .collect();
                
                related_posts_map.insert(posts_with_embeddings[i].path.clone(), related);
            }
        }
    } else {
        println!("Using keyword-based similarity");
        for i in 0..posts_with_embeddings.len() {
            let mut similarities: Vec<(usize, f32)> = Vec::new();
            
            for j in 0..posts_with_embeddings.len() {
                if i != j {
                    let similarity = keyword_extraction::calculate_keyword_similarity(&posts_with_embeddings[i], &posts_with_embeddings[j]);
                    similarities.push((j, similarity));
                }
            }
            
            // Sort by similarity (highest first)
            similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            
            // Take top N related posts
            let related: Vec<&BlogPost> = similarities.iter()
                .take(NUM_RELATED_POSTS)
                .map(|(idx, _)| &posts_with_embeddings[*idx])
                .collect();
            
            related_posts_map.insert(posts_with_embeddings[i].path.clone(), related);
        }
    }
    
    // Update frontmatter in each post
    println!("Updating post frontmatter...");
    for post in &posts_with_embeddings {
        if let Some(related) = related_posts_map.get(&post.path) {
            let updated_content = format!("{}\n{}", 
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
        println!("Done! Updated {} posts with related posts information.", posts_with_embeddings.len());
    }
    
    Ok(())
}
