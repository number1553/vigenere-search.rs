use crate::collection::Collection;
use crate::document::Document;
use crate::field::FieldValue;
use crate::inverted_index::InvertedIndex;
use crate::token::Token;

pub fn compute_freq(inverted_index: &InvertedIndex, document: &mut Document, term: &str) -> f32 {
    let entry = inverted_index.get(&Token::new(term.to_string()));

    if let Some(entry) = entry {
        if entry.iter().find(|x| **x == document).is_some() {
            let field_value = document.get(Box::new(inverted_index.field_name).to_string().as_str()).unwrap();
            let count = field_value.value_tokens.as_ref().unwrap()
                .iter().filter(|x| x.value == term).count() as f32;
        }
    }

    todo!()
}

pub fn compute_idf(inverted_index: &InvertedIndex, term: &str) -> f32 {
    let documents_count = inverted_index.collection.documents.len() as f32;
    let documents_with_word = inverted_index.index.get(&Token {
        value: term.to_string()
    });

    if documents_with_word.is_none() {
        return 0 as f32;
    }

    let documents_with_word_count = documents_with_word.unwrap().len() as f32;

    (
        (documents_count - documents_with_word_count + 0.5f32)
            /
            (documents_with_word_count + 0.5f32)
            +
            1f32
    ).ln()
}

/*pub fn test_freq() {

}

pub fn test_idf() {
    let mut collection = Collection::new();
    let corpus = vec![
        "this is a a sample",
        "this is another another example example example"
    ];

    for text in corpus {
        let mut document = Document::new();
        let mut field_value = FieldValue::new();
        field_value.value_string = Some(text.to_string());
        document.push("a", field_value);
        collection.push(document);
    }

    let mut inverted_index = InvertedIndex::new("a", Box::new(collection));
    InvertedIndex::commit(&mut inverted_index);

    assert_eq!(compute_idf(&inverted_index, "exampl"), 0.6931472);
}*/