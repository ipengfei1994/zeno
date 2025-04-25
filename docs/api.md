Zeno API Documentation
Modules
workflow::inbox

Inbox::new(path: &str) -> Inbox: Creates a new inbox at the specified path.
Inbox::create_zettel(id: String, title: String, content: String) -> Result<Zettel, ZenoError>: Creates a new zettel.
Inbox::load_zettel(id: &str) -> Result<Zettel, ZenoError>: Loads a zettel by ID.

workflow::archive

Archive::new(path: &str) -> Archive: Creates a new archive at the specified path.
Archive::archive_zettel(zettel: Zettel) -> Result<(), ZenoError>: Archives a zettel.
Archive::load_zettel(id: &str) -> Result<Zettel, ZenoError>: Loads a zettel by ID.

workflow::references

References::new(path: &str) -> References: Creates a new references storage at the specified path.
References::add_reference(url: String, title: Option<String>) -> Result<Reference, ZenoError>: Adds a new reference.
References::load_reference(url: &str) -> Result<Reference, ZenoError>: Loads a reference by URL.

workflow::export

export_zettel(id: &str, export_dir: &str) -> Result<String, ZenoError>: Exports a zettel to Markdown in the specified directory.
