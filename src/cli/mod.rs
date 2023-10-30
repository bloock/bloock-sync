use std::env;

use clap::{Args, Error, Parser, Subcommand};
use http::{bloock_http::BloockHttpClient, Client};
use sync::{GetListAnchorsResponse, SyncService};

mod http;
mod sync;

/// Program to synchronize with BLOOCK SMT blocks by using IPFS storage
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Commands you can execute
    #[command(subcommand)]
    command: Commands,
}

// Available commands to execute
#[derive(Subcommand, Debug)]
enum Commands {
    /// synchronize all the blocks states from initial state (Anchor 0) to the maximum anchor defined
    Sync(SyncArgs),
    /// get a proof by a given block hash
    Proof(ProofArgs),
}

#[derive(Args, Debug)]
struct SyncArgs {
    /// maximum anchor number to synchronize
    #[arg(short, long)]
    max: Option<u64>,
}

#[derive(Args, Debug)]
struct ProofArgs {
    /// hash string to get the proof
    hash: String,
}

pub async fn init() {
    let cli = Cli::parse();

    dotenv::dotenv().ok();

    let api_host = env::var("API_HOST")
        .expect("expected API_HOST to be set in the environment");
    let api_key = env::var("API_KEY")
        .expect("expected API_KEY to be set in the environment");

    let http_client = BloockHttpClient::new(api_key, api_host);
    
    match &cli.command {
        Commands::Sync(sync_args) => {
            let base_url = http_client.get_api_host();
            let result: GetListAnchorsResponse = match http_client.get_json(format!("{base_url}/anchors/v1/anchors?page=1&per_page=10"), None).await {
                Ok(res) => res,
                Err(e) => return println!("Error")
            };
            let max_anchor = match sync_args.max {
                Some(m) => m.min(result.meta.total),
                None => result.meta.total,
            };
            
            let sync_service = SyncService::new(http_client, max_anchor);

            let response = sync_service.build_tree().await;
            println!("{}", response);

            println!("'myapp sync' was used, name is: {:?}", max_anchor);
        }
        Commands::Proof(proof_args) => {
            println!("'myapp proof' was used, name is: {:?}", proof_args.hash);
        },
    }
}
