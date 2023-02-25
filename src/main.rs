use std::fs;

mod collection;
mod document;
mod token;
mod tokenizer;
mod field;
mod inverted_index;
mod ranking;

use crate::collection::Collection;
use crate::document::Document;
use crate::field::FieldValue;
use crate::inverted_index::InvertedIndex;
use crate::ranking::*;

fn load_sample_corpus(file_path: &str) -> Collection {
    let file_content = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let array: Box<Vec<String>> = Box::new(
        serde_json::from_str(file_content.as_str())
            .expect("JSON was not well-formatted")
    );
    let mut collection = Collection::new();

    for x in array.iter() {
        let mut document = Document::new();
        let mut field_value = FieldValue::new();
        field_value.value_string = Some(x.clone());

        document.push("title", field_value);
        collection.push(document);
    }

    collection
}

fn main() {
    let mut collection = load_sample_corpus("corpus.json");
    let mut inverted_index = InvertedIndex::new("title", Box::new(collection));
    InvertedIndex::commit(&mut inverted_index);

    // test_freq();
    // test_idf();

    println!("TF: {}", compute_freq(&inverted_index, collection.documents.get_mut(0).unwrap(), "must"));
    println!("IDF: {}", compute_idf(&inverted_index, "must"))
}
