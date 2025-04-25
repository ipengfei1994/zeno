use zeno::error::ZenoError;
use zeno::workflow::inbox::Inbox;

fn main() -> Result<(), ZenoError> {
    let inbox = Inbox::new("data/inbox");
    let zettel = inbox.create_zettel(
        "20250424120000".to_string(),
        "Sample Note".to_string(),
        "This is a sample note content.".to_string(),
    )?;
    println!("Created zettel: {:?}", zettel);
    Ok(())
}
