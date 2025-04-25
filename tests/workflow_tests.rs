use zeno::workflow::inbox::Inbox;
use zeno::workflow::archive::Archive;
use zeno::workflow::references::References;
use zeno::workflow::export;
use zeno::error::ZenoError;
use zeno::id::ZenoId;
use tempfile::tempdir;
use std::fs;

#[test]
fn test_archive_zettel() -> Result<(), ZenoError> {
    let temp_dir = tempdir()?;
    let inbox_dir = temp_dir.path().join("inbox");
    let archive_dir = temp_dir.path().join("archive");
    let inbox_path = inbox_dir
        .to_str()
        .ok_or_else(|| ZenoError::Io(std::io::Error::new(std::io::ErrorKind::Other, "Invalid path")))?;
    let archive_path = archive_dir
        .to_str()
        .ok_or_else(|| ZenoError::Io(std::io::Error::new(std::io::ErrorKind::Other, "Invalid path")))?;

    let inbox = Inbox::new(inbox_path);
    let archive = Archive::new(archive_path);

    // Create a zettel in inbox
    let id = "20250425120000".to_string();
    let zettel = inbox.create_zettel(id.clone(), "Test Note".to_string(), "Test content.".to_string())?;

    // Archive the zettel
    archive.archive_zettel(zettel.clone())?;

    // Verify the zettel is in the archive
    let archived_zettel = archive.load_zettel(&id)?;
    assert_eq!(archived_zettel.id, zettel.id);
    assert_eq!(archived_zettel.title, "Test Note");
    assert_eq!(archived_zettel.content, "Test content.");
    Ok(())
}

#[test]
fn test_export_zettel() -> Result<(), ZenoError> {
    let temp_dir = tempdir()?;
    let inbox_dir = temp_dir.path().join("inbox");
    let export_dir = temp_dir.path().join("export");
    let archive_dir = temp_dir.path().join("archive"); // 绑定临时值
    let inbox_path = inbox_dir
        .to_str()
        .ok_or_else(|| ZenoError::Io(std::io::Error::new(std::io::ErrorKind::Other, "Invalid path")))?;
    let export_path = export_dir
        .to_str()
        .ok_or_else(|| ZenoError::Io(std::io::Error::new(std::io::ErrorKind::Other, "Invalid path")))?;
    let archive_path = archive_dir
        .to_str()
        .ok_or_else(|| ZenoError::Io(std::io::Error::new(std::io::ErrorKind::Other, "Invalid path")))?;

    let inbox = Inbox::new(inbox_path);

    // Create a zettel in inbox
    let id = "20250425120000".to_string();
    let zettel = inbox.create_zettel(id.clone(), "Test Note".to_string(), "Test content.".to_string())?;

    // Verify the zettel before export
    assert_eq!(zettel.id, ZenoId::Timestamp(id.clone()));
    assert_eq!(zettel.title, "Test Note");
    assert_eq!(zettel.content, "Test content.");

    // Export the zettel
    let export_file_path = export_dir.join(format!("{}.md", id));
    let result = export::export_zettel(&id, inbox_path, archive_path, export_path)?;
    assert!(result.contains(&format!("{}", export_file_path.display())));

    // Verify the exported file
    let exported_content = fs::read_to_string(&export_file_path)?;
    assert!(exported_content.contains("# Test Note"));
    assert!(exported_content.contains("**ID:** 20250425120000"));
    assert!(exported_content.contains("Test content."));
    Ok(())
}

#[test]
fn test_add_reference() -> Result<(), ZenoError> {
    let temp_dir = tempdir()?;
    let refs_dir = temp_dir.path().join("references");
    let refs_path = refs_dir
        .to_str()
        .ok_or_else(|| ZenoError::Io(std::io::Error::new(std::io::ErrorKind::Other, "Invalid path")))?;

    let references = References::new(refs_path);

    // Add a reference
    let url = "https://example.com".to_string();
    let title = Some("Example".to_string());
    let reference = references.add_reference(url.clone(), title.clone())?;

    // Verify the reference
    assert_eq!(reference.url, url);
    assert_eq!(reference.title, title);

    // Load and verify the reference
    let loaded_reference = references.load_reference(&url)?;
    assert_eq!(loaded_reference.url, url);
    assert_eq!(loaded_reference.title, title);
    Ok(())
}

#[test]
fn test_workflow_placeholder() {
    assert!(true);
}