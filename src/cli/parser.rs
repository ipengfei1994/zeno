use clap::{Parser, Subcommand};

   #[derive(Parser)]
   #[command(author, version, about, long_about = None)]
   pub struct ZenoArgs {
       #[command(subcommand)]
       pub command: Commands,

       #[arg(long, default_value = "data/inbox")]
       pub inbox_path: String,

       #[arg(long, default_value = "data/archive")]
       pub archive_path: String,

       #[arg(long, default_value = "data/export")]
       pub export_path: String,

       #[arg(long, default_value = "data/references")]
       pub refs_path: String,
   }

   #[derive(Subcommand)]
   pub enum Commands {
       Create {
           #[arg(value_enum)]
           id_type: IdType,
       },
       Archive {
           id: String,
       },
       Export {
           id: String,
       },
       List {
           #[arg(long, default_value = "inbox")]
           source: String, // "inbox" 或 "archive"
       },
   }

   #[derive(clap::ValueEnum, Clone)]
   pub enum IdType {
       Timestamp,
       Uuid,
       Luhmann,
   }