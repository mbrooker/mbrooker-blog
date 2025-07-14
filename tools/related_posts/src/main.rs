use std::collections::HashMap;
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
use rusqlite::{Connection, Result as SqliteResult};
use sha2::{Sha256, Digest};


// Number of related posts to include
const NUM_RELATED_POSTS: usize = 3;
// Number of dissimilar posts to include
const NUM_DISSIMILAR_POSTS: usize = 1;
// Titan Text Embeddings model ID
const TITAN_EMBEDDINGS_MODEL_ID: &str = "amazon.titan-embed-text-v2:0";

#[derive(Debug, Clone)]
struct BlogPost {
    path: PathBuf,
    content: String,
    frontmatter: String,
    body: String,
    title: String,
    url: String,
    embedding: Option<Vec<f32>>,
    content_hash: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct TitanEmbeddingRequest {
    inputText: String,
}

#[derive(Serialize, Deserialize)]
struct TitanEmbeddingResponse {
    embedding: Vec<f32>,
}

fn extract_frontmatter_and_content(content: &str) -> (String, String) {
    if let Some(stripped) = content.strip_prefix("---") {
        if let Some(end_index) = stripped.find("---") {
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
        let url = format!("/{year}/{month}/{day}/{slug}.html");
        
        // Try to extract title from frontmatter or use slug as fallback
        let title = slug.replace('-', " ");
        
        return (title, url);
    }
    
    (file_name.to_string(), format!("/{file_name}"))
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
            eprintln!("Error serializing embedding request to JSON: {e}");
            eprintln!("Request data: input_text length = {}", truncated_text.len());
            return Err(Box::new(e));
        }
    };
    
    eprintln!("Making embedding request to model: {TITAN_EMBEDDINGS_MODEL_ID}");
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
            eprintln!("  Model ID: {TITAN_EMBEDDINGS_MODEL_ID}");
            eprintln!("  Error: {e}");
            eprintln!("  Request payload: {request_json}");
            
            // Try to extract more specific error information
            if let Some(service_err) = e.as_service_error() {
                eprintln!("  Service error details: {service_err:?}");
            }
            
            return Err(Box::new(e));
        }
    };
    
    let response_body = response.body.as_ref();
    eprintln!("Received response body size: {} bytes", response_body.len());
    
    let response_str = match std::str::from_utf8(response_body) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error converting response body to UTF-8 string: {e}");
            eprintln!("Response body (first 500 bytes): {:?}", 
                &response_body[..std::cmp::min(500, response_body.len())]);
            return Err(Box::new(e));
        }
    };
    
    eprintln!("Response body: {response_str}");
    
    let response_json: TitanEmbeddingResponse = match serde_json::from_str(response_str) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error parsing response JSON: {e}");
            eprintln!("Raw response: {response_str}");
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

fn update_frontmatter(post: &BlogPost, related_posts: &[&BlogPost], dissimilar_posts: &[&BlogPost]) -> String {
    let mut new_frontmatter = post.frontmatter.clone();
    
    // Remove existing related_posts and dissimilar_posts if present
    for field in &["related_posts:", "dissimilar_posts:"] {
        if new_frontmatter.contains(field) {
            let start_idx = new_frontmatter.find(field).unwrap();
            let mut end_idx = new_frontmatter[start_idx..].find("\n---").unwrap_or_else(|| new_frontmatter[start_idx..].len());
            end_idx += start_idx;
            
            let before = &new_frontmatter[..start_idx];
            let after = if end_idx + 1 < new_frontmatter.len() {
                &new_frontmatter[end_idx..]
            } else {
                ""
            };
            
            new_frontmatter = format!("{before}{after}");
        }
    }
    
    // Add new related_posts and dissimilar_posts
    if !related_posts.is_empty() || !dissimilar_posts.is_empty() {
        // Find where to insert the posts data
        if let Some(idx) = new_frontmatter.rfind("---") {
            let (before, after) = new_frontmatter.split_at(idx);
            let mut posts_section = String::new();
            
            if !related_posts.is_empty() {
                let related_urls: Vec<String> = related_posts.iter()
                    .map(|p| p.url.clone())
                    .collect();
                
                posts_section.push_str("related_posts:\n");
                posts_section.push_str(&related_urls.iter()
                    .map(|url| format!("  - \"{url}\""))
                    .collect::<Vec<_>>()
                    .join("\n"));
                posts_section.push('\n');
            }
            
            if !dissimilar_posts.is_empty() {
                let dissimilar_urls: Vec<String> = dissimilar_posts.iter()
                    .map(|p| p.url.clone())
                    .collect();
                
                posts_section.push_str("dissimilar_posts:\n");
                posts_section.push_str(&dissimilar_urls.iter()
                    .map(|url| format!("  - \"{url}\""))
                    .collect::<Vec<_>>()
                    .join("\n"));
                posts_section.push('\n');
            }
            
            new_frontmatter = format!("{before}{posts_section}{after}");
        }
    }
    
    new_frontmatter
}

fn calculate_content_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    hex::encode(hasher.finalize())
}

fn init_embeddings_db(db_path: &Path) -> SqliteResult<Connection> {
    let conn = Connection::open(db_path)?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS embeddings (
            content_hash TEXT PRIMARY KEY,
            embedding BLOB NOT NULL,
            model_id TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    
    // Clean up embeddings from different models to save space
    conn.execute(
        "DELETE FROM embeddings WHERE model_id != ?1",
        [TITAN_EMBEDDINGS_MODEL_ID],
    )?;
    
    Ok(conn)
}

fn get_cached_embedding(conn: &Connection, content_hash: &str, model_id: &str) -> SqliteResult<Option<Vec<f32>>> {
    let mut stmt = conn.prepare(
        "SELECT embedding FROM embeddings WHERE content_hash = ?1 AND model_id = ?2"
    )?;
    
    let mut rows = stmt.query_map([content_hash, model_id], |row| {
        let blob: Vec<u8> = row.get(0)?;
        // Deserialize the blob back to Vec<f32>
        let embedding: Vec<f32> = bincode::deserialize(&blob)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Blob, Box::new(e)))?;
        Ok(embedding)
    })?;
    
    if let Some(row) = rows.next() {
        Ok(Some(row?))
    } else {
        Ok(None)
    }
}

fn cache_embedding(conn: &Connection, content_hash: &str, embedding: &[f32], model_id: &str) -> SqliteResult<()> {
    // Serialize the embedding to bytes
    let embedding_blob = bincode::serialize(embedding)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    
    conn.execute(
        "INSERT OR REPLACE INTO embeddings (content_hash, embedding, model_id) VALUES (?1, ?2, ?3)",
        rusqlite::params![content_hash, embedding_blob, model_id],
    )?;
    
    Ok(())
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
    if parent_dir.file_name().is_some_and(|name| name == "tools") {
        let blog_root = parent_dir.parent().expect("Failed to get blog root directory");
        if blog_root.join("_posts").exists() {
            return blog_root.to_path_buf();
        }
    }
    
    // If we're already in the tools directory
    if current_dir.file_name().is_some_and(|name| name == "tools") {
        let blog_root = current_dir.parent().expect("Failed to get blog root directory");
        if blog_root.join("_posts").exists() {
            return blog_root.to_path_buf();
        }
    }
    
    // If we're in the related_posts directory
    if current_dir.file_name().is_some_and(|name| name == "related_posts") {
        let tools_dir = current_dir.parent().expect("Failed to get tools directory");
        if tools_dir.file_name().is_some_and(|name| name == "tools") {
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
        if path.extension().is_some_and(|ext| ext == "md") {
            let mut file = File::open(path)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            
            let (frontmatter, body) = extract_frontmatter_and_content(&content);
            let (title, url) = extract_title_and_url(path);
            
            // Calculate content hash for memoization
            let text_for_embedding = format!("{title} {body}");
            let content_hash = calculate_content_hash(&text_for_embedding);
            
            posts.push(BlogPost {
                path: path.to_path_buf(),
                content,
                frontmatter,
                body,
                title,
                url,
                embedding: None,
                content_hash,
            });
        }
    }
    
    println!("Found {} posts", posts.len());
    
    // Initialize SQLite database for embedding memoization
    let db_path = blog_root.join("embeddings_cache.db");
    println!("Initializing embeddings cache at: {}", db_path.display());
    let conn = init_embeddings_db(&db_path)?;
    
    // Initialize AWS SDK
    println!("Initializing AWS SDK...");
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let bedrock_client = BedrockClient::new(&config);
    
    // Generate embeddings for all posts (using cache when available)
    println!("Generating embeddings using Amazon Bedrock Titan Text Embeddings (with caching)...");
    
    let mut posts_with_embeddings = Vec::new();
    let mut cache_hits = 0;
    let mut cache_misses = 0;
    
    // Process posts in batches to avoid overwhelming the API
    let batch_size = 10;
    let mut posts_needing_embeddings = Vec::new();
    
    // First pass: check cache for all posts
    for post in &posts {
        let mut post_clone = post.clone();
        
        // Try to get cached embedding
        match get_cached_embedding(&conn, &post.content_hash, TITAN_EMBEDDINGS_MODEL_ID) {
            Ok(Some(cached_embedding)) => {
                post_clone.embedding = Some(cached_embedding);
                posts_with_embeddings.push(post_clone);
                cache_hits += 1;
                println!("Using cached embedding for: {}", post.path.display());
            },
            Ok(None) => {
                posts_needing_embeddings.push(post_clone);
                cache_misses += 1;
            },
            Err(e) => {
                eprintln!("Error checking cache for {}: {}", post.path.display(), e);
                posts_needing_embeddings.push(post_clone);
                cache_misses += 1;
            }
        }
    }
    
    println!("Cache stats: {cache_hits} hits, {cache_misses} misses");
    
    // Second pass: generate embeddings for posts not in cache
    if !posts_needing_embeddings.is_empty() {
        println!("Generating {} new embeddings...", posts_needing_embeddings.len());
        
        for chunk in posts_needing_embeddings.chunks(batch_size) {
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
                            Ok(post_clone)
                        },
                        Err(e) => {
                            eprintln!("Error generating embedding for {}: {}", post_clone.path.display(), e);
                            Err(e)
                        }
                    }
                };
                
                futures.push(future);
            }
            
            let results = join_all(futures).await;
            
            for result in results {
                match result {
                    Ok(post) => {
                        // Cache the new embedding
                        if let Some(ref embedding) = post.embedding {
                            if let Err(e) = cache_embedding(&conn, &post.content_hash, embedding, TITAN_EMBEDDINGS_MODEL_ID) {
                                eprintln!("Warning: Failed to cache embedding for {}: {}", post.path.display(), e);
                            }
                        }
                        posts_with_embeddings.push(post);
                    },
                    Err(e) => return Err(e),
                }
            }
        }
    }
    
    // Calculate similarities and find related posts using embeddings
    println!("Finding related posts using embedding-based similarity (cosine similarity)...");
    let mut related_posts_map: HashMap<PathBuf, Vec<&BlogPost>> = HashMap::new();
    let mut dissimilar_posts_map: HashMap<PathBuf, Vec<&BlogPost>> = HashMap::new();
    
    for i in 0..posts_with_embeddings.len() {
        let mut similarities: Vec<(usize, f32)> = Vec::new();
        
        if let Some(ref embedding_i) = posts_with_embeddings[i].embedding {
            for (j, post_j) in posts_with_embeddings.iter().enumerate() {
                if i != j {
                    if let Some(ref embedding_j) = post_j.embedding {
                        let similarity = cosine_similarity(embedding_i, embedding_j);
                        similarities.push((j, similarity));
                    }
                }
            }
            
            // Sort by similarity (highest first)
            similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            
            // Take top N related posts (most similar)
            let related: Vec<&BlogPost> = similarities.iter()
                .take(NUM_RELATED_POSTS)
                .map(|(idx, _)| &posts_with_embeddings[*idx])
                .collect();
            
            // Take bottom N dissimilar posts (least similar)
            let dissimilar: Vec<&BlogPost> = similarities.iter()
                .rev()
                .take(NUM_DISSIMILAR_POSTS)
                .map(|(idx, _)| &posts_with_embeddings[*idx])
                .collect();
            
            related_posts_map.insert(posts_with_embeddings[i].path.clone(), related);
            dissimilar_posts_map.insert(posts_with_embeddings[i].path.clone(), dissimilar);
        }
    }
    
    // Update frontmatter in each post
    println!("Updating post frontmatter...");
    for post in &posts_with_embeddings {
        let related = related_posts_map.get(&post.path).map(|v| v.as_slice()).unwrap_or(&[]);
        let dissimilar = dissimilar_posts_map.get(&post.path).map(|v| v.as_slice()).unwrap_or(&[]);
        
        let updated_content = format!("{}\n{}", 
            update_frontmatter(post, related, dissimilar),
            post.body
        );
        
        // Write updated content back to file (unless in dry-run mode)
        if !dry_run {
            let mut file = File::create(&post.path)?;
            file.write_all(updated_content.as_bytes())?;
        }
        
        println!("Updated: {}", post.path.display());
    }
    
    if dry_run {
        println!("Dry run completed. No files were modified.");
    } else {
        println!("Done! Updated {} posts with related posts information.", posts_with_embeddings.len());
    }
    
    Ok(())
}
