use std::collections::HashMap;

// Struct representing a Trie node
pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

impl TrieNode {
    // Constructor for TrieNode
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end_of_word: false,
        }
    }
}

// Struct representing the Trie
struct Trie {
    root: TrieNode,
}

impl Trie {
    // Constructor for Trie
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    // Method to insert a word into the Trie
    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;

        // Traverse through the characters of the word
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert(TrieNode::new());
        }

        // Mark the end of the word
        node.is_end_of_word = true;
    }

    // Method to search for the longest matching word in the Trie
    fn search_longest_prefix(&self, text: &str) -> Option<String> {
        let mut node = &self.root;
        let mut longest_match: Option<String> = None;
        let mut current_match = String::new();

        // Traverse through the characters of the text
        for ch in text.chars() {
            if let Some(next_node) = node.children.get(&ch) {
                current_match.push(ch);

                if next_node.is_end_of_word {
                    longest_match = Some(current_match.clone());
                }

                node = next_node;
            } else {
                break;
            }
        }

        longest_match
    }
}


// Function to segment the Thai text using the Trie
pub fn segment_thai_text(text: &str, trie: &Trie) -> Vec<String> {
    let mut result = Vec::new();
    let mut index = 0;

    while index < text.len() {
        // Try to find the longest match in the Trie
        if let Some(matching_word) = trie.search_longest_prefix(&text[index..]) {
            result.push(matching_word.clone());
            index += matching_word.chars().count();  // Move the index forward by the length of the matched word
        } else {
            // If no match is found, treat the current character as a separate token
            result.push(text[index..index + 1].to_string());
            index += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a Trie with Thai words
    fn create_trie_with_thai_words() -> Trie {
        let mut trie = Trie::new();
        trie.insert("สวัสดี");
        trie.insert("ครับ");
        trie.insert("คุณ");
        trie.insert("ไป");
        trie.insert("ที่ไหน");
        trie.insert("สวัสดีครับ");
        trie
    }

    #[test]
    fn test_segmentation_basic() {
        let trie = create_trie_with_thai_words();
        let text = "สวัสดีครับคุณไปที่ไหน";
        let expected = vec![
            "สวัสดี".to_string(),
            "ครับ".to_string(),
            "คุณ".to_string(),
            "ไป".to_string(),
            "ที่ไหน".to_string(),
        ];
        let result = segment_thai_text(text, &trie);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_segmentation_partial_match() {
        let trie = create_trie_with_thai_words();
        let text = "สวัสดีครับคุณไปไหน"; // 'ไปไหน' not in dictionary
        let expected = vec![
            "สวัสดี".to_string(),
            "ครับ".to_string(),
            "คุณ".to_string(),
            "ไป".to_string(),
            "ไ".to_string(),
            "ห".to_string(),
            "น".to_string(),
            "้".to_string(),
        ];
        let result = segment_thai_text(text, &trie);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_segmentation_empty_string() {
        let trie = create_trie_with_thai_words();
        let text = "";
        let expected: Vec<String> = vec![];
        let result = segment_thai_text(text, &trie);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_segmentation_no_matching_words() {
        let trie = create_trie_with_thai_words();
        let text = "abcdefg"; // No matching words in the dictionary
        let expected: Vec<String> = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
            "f".to_string(),
            "g".to_string(),
        ];
        let result = segment_thai_text(text, &trie);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_segmentation_mixed_content() {
        let trie = create_trie_with_thai_words();
        let text = "สวัสดีครับ123คุณไปที่ไหน"; // Mixed Thai and numeric content
        let expected = vec![
            "สวัสดี".to_string(),
            "ครับ".to_string(),
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "คุณ".to_string(),
            "ไป".to_string(),
            "ที่ไหน".to_string(),
        ];
        let result = segment_thai_text(text, &trie);
        assert_eq!(result, expected);
    }
}