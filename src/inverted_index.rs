use std::collections::HashMap;
use crate::collection::Collection;
use crate::document::Document;
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct InvertedIndex<'a> {
    pub field_name: &'a str,
    pub collection: Box<Collection>,
    pub index: HashMap<Token, Vec<&'a Document>>,
}

impl InvertedIndex<'_> {
    pub fn new(field_name: &str, collection: Box<Collection>) -> InvertedIndex {
        InvertedIndex {
            field_name,
            collection,
            index: HashMap::new(),
        }
    }

    pub fn commit<'a>(this: &'a mut InvertedIndex<'a>) {
        for document in this.collection.documents.iter_mut() {
            let field_value = document.process_field(this.field_name);
            let tokens = field_value.unwrap().value_tokens.as_ref().unwrap().clone();

            for token in tokens {
                InvertedIndex::push(this, token, document);
            }
        }
    }

    pub fn push<'a>(this: &'a mut InvertedIndex<'a>, token: Token, document: &'a mut Document) {
        let entry = this.index.get_mut(&token);

        if let Some(entry) = entry {
            if entry.iter().find(|x| **x == document).is_none() {
                entry.push(document);
            }
        } else {
            this.index.insert(token, vec![document]);
        }
    }

    pub fn get(&self, token: &Token) -> Option<&Vec<&Document>> {
        self.index.get(token)
    }
}