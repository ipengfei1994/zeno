use clap::Parser;
use zeno::cli::parser::ZenoArgs;
use zeno::error::ZenoError;

fn main() -> Result<(), ZenoError> {
    let args = ZenoArgs::parse();
    zeno::cli::commands::run(args)?;
    Ok(())
}
