# Related Posts Generator for Jekyll using Amazon Bedrock

This tool uses Amazon Bedrock's Titan Text Embeddings model to find semantically related posts in your Jekyll blog and updates each post's frontmatter with links to related content.

## How It Works

1. Reads all Markdown posts from your `_posts` directory
2. Generates text embeddings for each post using Amazon Bedrock's Titan Text Embeddings model
3. Calculates cosine similarity between post embeddings to find semantically related content
4. For each post, finds the most similar posts
5. Updates the frontmatter of each post with related posts URLs

## Requirements

- Rust (latest stable version)
- AWS account with access to Amazon Bedrock
- AWS credentials configured locally

## AWS Setup

1. Make sure you have AWS credentials configured in your environment:
   ```
   aws configure
   ```

2. Ensure your AWS account has access to Amazon Bedrock and the Titan Text Embeddings model (`amazon.titan-embed-text-v1`).

3. Grant the necessary permissions to your AWS user or role:
   ```json
   {
       "Version": "2012-10-17",
       "Statement": [
           {
               "Effect": "Allow",
               "Action": [
                   "bedrock:InvokeModel"
               ],
               "Resource": [
                   "arn:aws:bedrock:*:*:model/amazon.titan-embed-text-v1"
               ]
           }
       ]
   }
   ```

## Usage

1. Navigate to the blog root directory or the tools/related_posts directory:
   ```
   cd /path/to/blog
   # or
   cd /path/to/blog/tools/related_posts
   ```

2. Build and run the program:
   ```
   cargo build --release
   cargo run --release
   ```

3. To run in dry-run mode (no files will be modified):
   ```
   cargo run --release -- --dry-run
   ```

4. The program will automatically:
   - Find the blog root directory
   - Locate all posts
   - Generate embeddings using Amazon Bedrock
   - Calculate similarities between posts
   - Update posts with a `related_posts` section in the frontmatter

5. Make sure your Jekyll post layout uses these related posts:
   ```html
   <div id="related">
     <h2>Related Posts</h2>
     <ul class="posts">
       {% if page.related_posts %}
         {% for related_url in page.related_posts %}
           {% for post in site.posts %}
             {% if post.url == related_url %}
               <li><span>{{ post.date | date_to_string }}</span> &raquo; <a href="/blog{{ post.url }}">{{ post.title }}</a></li>
               {% break %}
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
- Modify the embedding generation logic to include different parts of the post content
- Adjust the batch size for API calls to balance speed and API rate limits

## Benefits of Using Embeddings

Using Amazon Bedrock's Titan Text Embeddings model provides several advantages over simple keyword matching:

1. **Semantic Understanding**: The model understands the meaning and context of words, not just their presence
2. **Better Relevance**: Posts are matched based on conceptual similarity, not just shared keywords
3. **Language Nuance**: The model captures synonyms, related concepts, and contextual relationships
4. **Improved User Experience**: Readers are presented with truly related content, enhancing site engagement

## Notes

- This is a one-time operation that updates your post files. Run it whenever you add new content.
- The tool uses the post title and body content to generate embeddings for similarity matching.
- Make sure to back up your posts before running this tool for the first time.
- Use the `--dry-run` option to see what changes would be made without actually modifying any files.
- API calls to Amazon Bedrock may incur costs in your AWS account.
