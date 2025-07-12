use std::collections::HashSet;
use crate::BlogPost;

pub fn extract_keywords(text: &str) -> HashSet<String> {
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

pub fn calculate_keyword_similarity(post1: &BlogPost, post2: &BlogPost) -> f32 {
    let common_keywords = post1.keywords.intersection(&post2.keywords).count();
    let total_keywords = post1.keywords.union(&post2.keywords).count();
    
    if total_keywords == 0 {
        return 0.0;
    }
    
    common_keywords as f32 / total_keywords as f32
}
