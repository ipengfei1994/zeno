use crate::error::ZenoError;
use crate::models::zettel::Zettel;
use crate::models::reference::Reference;

pub trait ZettelStorage {
    fn save_zettel(&self, zettel: &Zettel) -> Result<(), ZenoError>;
    fn load_zettel(&self, id: &str) -> Result<Zettel, ZenoError>;
}

pub trait ReferenceStorage {
    fn save_reference(&self, reference: &Reference) -> Result<(), ZenoError>;
    fn load_reference(&self, url: &str) -> Result<Reference, ZenoError>;
}