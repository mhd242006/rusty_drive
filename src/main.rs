use clap::{ Parser, Subcommand};

mod file_manager;
mod models;


#[derive(Parser)]
struct Cli{
    #[command(subcommand)]
   command: Commands,
}

#[derive(Subcommand)]
enum Commands{
     Upload{path: String},
    Download{id: String},
    List,
    Delete{id: String},
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    match &args.command {
        Commands::Upload { path } => {
            file_manager::upload_file(&path).await?;
        }
        Commands::Download { id } => {
            file_manager::download_file(&id).await?;
        }
        Commands::List => {
            file_manager::list_files().await?;
        }
        Commands::Delete { id } => {
            file_manager::delete_file(&id).await?;
        }
    }
    Ok(())
}

