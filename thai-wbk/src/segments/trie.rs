use std::collections::HashMap;

// Struct representing a Trie node
struct TrieNode {
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
pub struct Trie {
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
    fn search_longest_prefix(&self, text: &str) -> Option<Vec<String>> {
        let mut node = &self.root;
        let mut all_matches = Vec::new();
        let mut current_match = String::new();

        // Traverse through the characters of the text
        for ch in text.chars() {
            if let Some(next_node) = node.children.get(&ch) {
                current_match.push(ch);

                if next_node.is_end_of_word {
                    // If a valid word is found, store it in the matches
                    all_matches.push(current_match.clone());
                }

                node = next_node;
            } else {
                break;
            }
        }

        // Return all possible matches
        if all_matches.is_empty() {
            None
        } else {
            Some(all_matches)
        }
    }
}
pub fn load_dictionary_from_file(file_path: &str) -> Result<Trie, std::io::Error> {
    let mut trie = Trie::new();

    // Read the file and insert each word into the Trie
    let lines = std::fs::read_to_string(file_path)?;
    for line in lines.lines() {
        trie.insert(line);
    }

    Ok(trie)
}

// Function to segment the Thai text using the Trie
pub fn segment_thai_text(text: &str, trie: &Trie) -> Vec<String> {
    let mut result = Vec::new();
    let mut index = 0;
    let chars: Vec<(usize, char)> = text.char_indices().collect(); // Collect char indices

    while index < chars.len() {
        let remaining_text: String = chars[index..].iter().map(|&(_, c)| c).collect(); // Convert remaining chars to string

        // Try to find all possible matches
        if let Some(matches) = trie.search_longest_prefix(&remaining_text) {
            // Select the shortest match (first one in the list)
            let matching_word = &matches[0];

            result.push(matching_word.clone());

            // Move the index forward by the length of the matched word in characters
            let word_len = matching_word.chars().count();
            index += word_len;
        } else {
            // If no match is found, treat the current character as a separate token
            let mut token = chars[index].1.to_string();  // Use current character

            // Check if the next character is a diacritical mark and should be included
            if index + 1 < chars.len() {
                let next_char = chars[index + 1].1;
                if next_char == '\u{e47}' || next_char == '\u{e48}' || next_char == '\u{e49}' || next_char == '\u{e4a}' || next_char == '\u{e4b}' {
                    token.push(next_char);  // Combine current character with the tonal mark
                    index += 1;  // Skip the tonal mark character in the next loop
                }
            }

            result.push(token);
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