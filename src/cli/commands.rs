use crate::error::ZenoError;
   use crate::cli::parser::{ZenoArgs, Commands};
   use crate::workflow::inbox::Inbox;
   use crate::workflow::archive::Archive;
   use crate::workflow::export;
   use std::io;
   use std::fs;

   pub fn run(args: ZenoArgs) -> Result<(), ZenoError> {
       match args.command {
           Commands::Create { id_type } => {
               let inbox = Inbox::new(&args.inbox_path);
               let id = match id_type {
                   crate::cli::parser::IdType::Timestamp => crate::id::timestamp::generate_timestamp_id(),
                   crate::cli::parser::IdType::Uuid => crate::id::uuid::generate_uuid_id(),
                   crate::cli::parser::IdType::Luhmann => crate::id::luhmann::generate_luhmann_id(None),
               };
               let zettel = inbox.create_zettel(id, "New Note".to_string(), "Content of the note.".to_string())?;
               println!("Created note: {:?}", zettel);
               Ok(())
           }
           Commands::Archive { id } => {
               let inbox = Inbox::new(&args.inbox_path);
               let archive = Archive::new(&args.archive_path);
               let zettel = inbox.load_zettel(&id).map_err(|e| match e {
                   ZenoError::Io(err) if err.kind() == io::ErrorKind::NotFound => {
                       ZenoError::Io(io::Error::new(io::ErrorKind::NotFound, format!("Note with ID {} not found in inbox", id)))
                   }
                   _ => e,
               })?;
               archive.archive_zettel(zettel)?;
               println!("Archived note with ID: {}", id);
               Ok(())
           }
           Commands::Export { id } => {
               let exported = export::export_zettel(&id, &args.inbox_path, &args.archive_path, &args.export_path)?;
               println!("Exported note: {}", exported);
               Ok(())
           }
           Commands::List { source } => {
               let path = if source == "inbox" {
                   &args.inbox_path
               } else if source == "archive" {
                   &args.archive_path
               } else {
                   return Err(ZenoError::Io(io::Error::new(io::ErrorKind::InvalidInput, "Source must be 'inbox' or 'archive'")));
               };

               println!("Listing notes in {}:", source);
               let entries = fs::read_dir(path)?;
               for entry in entries {
                   let entry = entry?;
                   if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                       if let Some(file_name) = entry.file_name().to_str() {
                           let id = file_name.strip_suffix(".json").unwrap_or(file_name);
                           println!("- ID: {}", id);
                       }
                   }
               }
               Ok(())
           }
       }
   }