use crate::error::ZenoError;
use crate::models::zettel::Zettel;
use crate::storage::traits::ZettelStorage;
use std::fs::File;
use std::path::Path;

pub struct JsonInbox {
    path: String,
}

impl JsonInbox {
    pub fn new(path: &str) -> Self {
        JsonInbox {
            path: path.to_string(),
        }
    }
}

impl ZettelStorage for JsonInbox {
    fn save_zettel(&self, zettel: &Zettel) -> Result<(), ZenoError> {
        let file_path = Path::new(&self.path).join(format!("{}.json", zettel.id));
        let file = File::create(file_path)?;
        serde_json::to_writer_pretty(file, zettel)?;
        Ok(())
    }

    fn load_zettel(&self, id: &str) -> Result<Zettel, ZenoError> {
        let file_path = Path::new(&self.path).join(format!("{}.json", id));
        let file = File::open(file_path)?;
        let zettel = serde_json::from_reader(file)?;
        Ok(zettel)
    }
}