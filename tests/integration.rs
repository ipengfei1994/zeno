use tempfile::tempdir;
use zeno::error::ZenoError;
use zeno::workflow::inbox::Inbox;

#[test]
fn test_create_zettel() -> Result<(), ZenoError> {
    let temp_dir = tempdir()?;
    let inbox_dir = temp_dir.path().join("inbox");
    let inbox_path = inbox_dir.to_str().ok_or_else(|| {
        ZenoError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Invalid path",
        ))
    })?;
    let inbox = Inbox::new(inbox_path);
    let zettel = inbox.create_zettel(
        "20250425120000".to_string(),
        "Test Note".to_string(),
        "Test content.".to_string(),
    )?;
    assert_eq!(zettel.title, "Test Note");
    assert_eq!(zettel.content, "Test content.");
    Ok(())
}

#[test]
fn test_load_zettel() -> Result<(), ZenoError> {
    let temp_dir = tempdir()?;
    let inbox_dir = temp_dir.path().join("inbox");
    let inbox_path = inbox_dir.to_str().ok_or_else(|| {
        ZenoError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Invalid path",
        ))
    })?;
    let inbox = Inbox::new(inbox_path);

    // First, create a zettel
    let id = "20250425120000".to_string();
    let zettel = inbox.create_zettel(
        id.clone(),
        "Test Note".to_string(),
        "Test content.".to_string(),
    )?;

    // Then, load the zettel
    let loaded_zettel = inbox.load_zettel(&id)?;

    // Verify the loaded zettel matches the created one
    assert_eq!(loaded_zettel.id, zettel.id);
    assert_eq!(loaded_zettel.title, "Test Note");
    assert_eq!(loaded_zettel.content, "Test content.");
    Ok(())
}
