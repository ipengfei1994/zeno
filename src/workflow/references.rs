use crate::error::ZenoError;
use crate::models::reference::Reference;
use crate::storage::json::references::JsonReferences;
use crate::storage::traits::ReferenceStorage;
use chrono::Utc;
use std::fs;

pub struct References {
    storage: JsonReferences,
}

impl References {
    pub fn new(path: &str) -> Self {
        if !std::path::Path::new(path).exists() {
            fs::create_dir_all(path).expect("Failed to create references directory");
        }
        References {
            storage: JsonReferences::new(path),
        }
    }

    pub fn add_reference(&self, url: String, title: Option<String>) -> Result<Reference, ZenoError> {
        let reference = Reference {
            url,
            title,
            accessed_at: Utc::now(),
        };
        self.storage.save_reference(&reference)?;
        Ok(reference)
    }

    pub fn load_reference(&self, url: &str) -> Result<Reference, ZenoError> {
        self.storage.load_reference(url)
    }
}