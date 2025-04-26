use crate::error::ZenoError;
use crate::id::ZenoId;
use crate::models::zettel::Zettel;
use crate::storage::json::inbox::JsonInbox;
use crate::storage::traits::ZettelStorage;
use chrono::Utc;
use std::fs;

pub struct Inbox {
    storage: JsonInbox,
}

impl Inbox {
    pub fn new(path: &str) -> Self {
        if !std::path::Path::new(path).exists() {
            fs::create_dir_all(path).expect("Failed to create inbox directory");
        }
        Inbox {
            storage: JsonInbox::new(path),
        }
    }

    pub fn create_zettel(&self, id: String, title: String, content: String) -> Result<Zettel, ZenoError> {
        let zettel = Zettel {
            id: ZenoId::Timestamp(id),
            title,
            content,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            links: Vec::new(),
        };
        self.storage.save_zettel(&zettel)?;
        Ok(zettel)
    }

    pub fn load_zettel(&self, id: &str) -> Result<Zettel, ZenoError> {
        self.storage.load_zettel(id)
    }

    pub fn save_zettel(&self, zettel: &Zettel) -> Result<(), ZenoError> {
        self.storage.save_zettel(zettel)
    }
}

impl ZettelStorage for Inbox {
    fn save_zettel(&self, zettel: &Zettel) -> Result<(), ZenoError> {
        self.storage.save_zettel(zettel)
    }

    fn load_zettel(&self, id: &str) -> Result<Zettel, ZenoError> {
        self.storage.load_zettel(id)
    }
}