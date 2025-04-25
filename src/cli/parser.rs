use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "zeno")]
#[command(about = "A command-line Zettelkasten system", long_about = None)]
pub struct ZenoArgs {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long, default_value = "data/inbox")]
    pub inbox_path: String,

    #[arg(long, default_value = "data/archive")]
    pub archive_path: String,

    #[arg(long, default_value = "data/export")]
    pub export_path: String,
}

#[derive(Subcommand, Debug)]
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
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum IdType {
    Timestamp,
    Uuid,
    Luhmann,
}
