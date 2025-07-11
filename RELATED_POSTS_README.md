# Related Posts Generator for Jekyll

This tool uses Amazon Bedrock's embedding models to find semantically related posts in your Jekyll blog and updates each post's frontmatter with links to related content.

## How It Works

1. Reads all Markdown posts from your `_posts` directory
2. Extracts content and metadata from each post
3. Generates embeddings using Amazon Bedrock's Titan Embedding model
4. Calculates similarity between posts using cosine similarity
5. For each post, finds the most similar posts
6. Updates the frontmatter of each post with related posts URLs

## Requirements

- Rust (latest stable version)
- AWS credentials configured with access to Amazon Bedrock
- Permission to use the Titan Embedding model in Amazon Bedrock

## Usage

1. Make sure you have AWS credentials configured:
   ```
   export AWS_ACCESS_KEY_ID=your_access_key
   export AWS_SECRET_ACCESS_KEY=your_secret_key
   export AWS_REGION=us-east-1  # or your preferred region
   ```

2. Build and run the program:
   ```
   cargo build --release
   cargo run --release
   ```

3. The program will update all your posts with a `related_posts` section in the frontmatter.

4. Update your Jekyll post layout to use these related posts:
   ```html
   <div id="related">
     <h2>Related Posts</h2>
     <ul class="posts">
       {% if page.related_posts %}
         {% for related_url in page.related_posts %}
           {% for post in site.posts %}
             {% if post.url == related_url %}
               <li><span>{{ post.date | date_to_string }}</span> &raquo; <a href="/blog{{ post.url }}">{{ post.title }}</a></li>
             {% endif %}
           {% endfor %}
         {% endfor %}
       {% else %}
         {% for post in site.related_posts limit:3 %}
           <li><span>{{ post.date | date_to_string }}</span> &raquo; <a href="/blog{{ post.url }}">{{ post.title }}</a></li>
         {% endfor %}
       {% endif %}
     </ul>
   </div>
   ```

## Customization

- Change `NUM_RELATED_POSTS` in the code to adjust how many related posts are included
- Modify the embedding logic to use different parts of the post content
- Try different embedding models by changing the `MODEL_ID` constant

## Notes

- This is a one-time operation that updates your post files. Run it whenever you add new content.
- The tool uses the post title and first 1000 characters of content to generate embeddings.
- Make sure to back up your posts before running this tool for the first time.
