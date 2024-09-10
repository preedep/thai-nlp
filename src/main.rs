fn main() {
    let r = thai_wbk::segments::trie::load_dictionary_from_file("datas/lexitron.txt");
    match r {
        Ok(trie) => {
            let text = "กรม";
            let result = thai_wbk::segments::trie::segment_thai_text(text, &trie);
            println!("{:?}", result);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
