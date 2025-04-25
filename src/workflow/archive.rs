use crate::error::ZenoError;
use crate::models::zettel::Zettel;
use crate::storage::json::archive::JsonArchive;
use crate::storage::traits::ZettelStorage;
use std::fs;

pub struct Archive {
    storage: JsonArchive,
}

impl Archive {
    pub fn new(path: &str) -> Self {
        if !std::path::Path::new(path).exists() {
            fs::create_dir_all(path).expect("Failed to create archive directory");
        }
        Archive {
            storage: JsonArchive::new(path),
        }
    }

    pub fn archive_zettel(&self, zettel: Zettel) -> Result<(), ZenoError> {
        self.storage.save_zettel(&zettel)?;
        Ok(())
    }

    pub fn load_zettel(&self, id: &str) -> Result<Zettel, ZenoError> {
        self.storage.load_zettel(id)
    }
}