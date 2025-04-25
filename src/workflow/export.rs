use crate::error::ZenoError;
use crate::workflow::inbox::Inbox;
use crate::workflow::archive::Archive;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub fn export_zettel(id: &str, inbox_path: &str, archive_path: &str, export_dir: &str) -> Result<String, ZenoError> {
    // Try to load the zettel from inbox first
    let inbox = Inbox::new(inbox_path);
    let zettel = match inbox.load_zettel(id) {
        Ok(zettel) => zettel,
        Err(_) => {
            // If not found in inbox, try archive
            let archive = Archive::new(archive_path);
            archive.load_zettel(id)?
        }
    };

    // Create export directory if it doesn't exist
    if !Path::new(export_dir).exists() {
        fs::create_dir_all(export_dir)?;
    }

    // Export to Markdown
    let export_path = Path::new(export_dir).join(format!("{}.md", id));
    let mut file = File::create(&export_path)?;
    writeln!(file, "# {}", zettel.title)?;
    writeln!(file, "\n**ID:** {}", zettel.id)?;
    writeln!(file, "**Created At:** {}", zettel.created_at)?;
    writeln!(file, "**Updated At:** {}", zettel.updated_at)?;
    writeln!(file, "\n## Content")?;
    writeln!(file, "{}", zettel.content)?;
    
    Ok(format!("Exported to {}", export_path.display()))
}