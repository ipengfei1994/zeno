use crate::error::ZenoError;
use crate::models::reference::Reference;
use crate::storage::traits::ReferenceStorage;
use std::fs::File;
use std::path::Path;
use sha2::{Sha256, Digest};

pub struct JsonReferences {
    path: String,
}

impl JsonReferences {
    pub fn new(path: &str) -> Self {
        JsonReferences {
            path: path.to_string(),
        }
    }

    pub fn url_to_filename(&self, url: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(url);
        let result = hasher.finalize();
        format!("{:x}.json", result)
    }
}

impl ReferenceStorage for JsonReferences {
    fn save_reference(&self, reference: &Reference) -> Result<(), ZenoError> {
        let filename = self.url_to_filename(&reference.url);
        let file_path = Path::new(&self.path).join(filename);
        let file = File::create(file_path)?;
        serde_json::to_writer_pretty(file, reference)?;
        Ok(())
    }

    fn load_reference(&self, url: &str) -> Result<Reference, ZenoError> {
        let filename = self.url_to_filename(url);
        let file_path = Path::new(&self.path).join(filename);
        let file = File::open(file_path)?;
        let reference = serde_json::from_reader(file)?;
        Ok(reference)
    }
}