use crate::tokenizer::tokenize;
use crate::document::Document;

#[derive(Debug)]
pub struct Collection {
    pub documents: Vec<Document>,
}

impl Collection {
    pub fn new() -> Self {
        Collection {
            documents: Vec::new()
        }
    }

    pub fn push(&mut self, document: Document) {
        self.documents.push(document);
    }

    pub fn search(query: String) {
        let tokens = tokenize(query.as_str());
    }
}

impl Clone for Collection {
    fn clone(&self) -> Self {
        Collection {
            documents: self.documents.clone()
        }
    }
}