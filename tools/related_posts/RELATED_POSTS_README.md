# Related Posts Generator for Jekyll

This tool uses keyword similarity to find semantically related posts in your Jekyll blog and updates each post's frontmatter with links to related content.

## How It Works

1. Reads all Markdown posts from your `_posts` directory
2. Extracts content and metadata from each post
3. Analyzes keywords from each post
4. Calculates similarity between posts using keyword overlap
5. For each post, finds the most similar posts
6. Updates the frontmatter of each post with related posts URLs

## Requirements

- Rust (latest stable version)

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

4. The program will automatically find the blog root directory, locate all posts, and update them with a `related_posts` section in the frontmatter.

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
- Modify the keyword extraction logic to improve similarity matching
- Adjust the stop words list to better fit your content

## Notes

- This is a one-time operation that updates your post files. Run it whenever you add new content.
- The tool uses the post title and body content to generate keywords for similarity matching.
- Make sure to back up your posts before running this tool for the first time.
- Use the `--dry-run` option to see what changes would be made without actually modifying any files.
