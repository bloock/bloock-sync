use std::env;

use clap::{Args, Parser, Subcommand};
use http::{bloock_http::BloockHttpClient, Client};
use proof::ProofService;
use sync::{GetListAnchorsResponse, SyncService};

mod http;
mod tree;
mod sync;
mod proof;

/// Program to synchronize with BLOOCK SMT blocks via IPFS storage
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
    /// Synchronize all block states from the initial state (Anchor 0) to the maximum defined anchor
    Sync(SyncArgs),
    /// Retrieve a proof for a given block hash
    Proof(ProofArgs),
    /// Delete the current tree from the database and restart synchronization
    Restart,
}

#[derive(Args, Debug)]
/// Arguments for synchronization command
struct SyncArgs {
    /// Maximum anchor number to synchronize
    #[arg(short, long)]
    max: Option<u64>,
}

#[derive(Args, Debug)]
/// Arguments for proof command
struct ProofArgs {
    /// Root of the tree, obtained from the SYNC command
    root: String,
    /// Hash for which proof is requested
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

    let tree_id = "/state".to_string();
    
    match &cli.command {
        Commands::Sync(sync_args) => {
            println!("iniciate Sync process");

            let base_url = http_client.get_api_host();
            let result: GetListAnchorsResponse = match http_client.get_json(format!("{base_url}/anchors/v1/anchors?page=1&per_page=10"), None).await {
                Ok(res) => res,
                Err(e) => return println!("error proof service: {:?}", e.to_string())
            };
            let max_anchor = match sync_args.max {
                Some(m) => m.min(result.meta.total),
                None => result.meta.total,
            };
            
            println!("max anchor selected: {:?}", max_anchor);
            let sync_service = SyncService::new(http_client, max_anchor, tree_id);

            println!("building SMT...");
            let response = sync_service.build_tree().await;
           
            println!("SMT was build successfully with ROOT: {}", response);
        },
        Commands::Proof(proof_args) => {
            println!("iniciate Proof process");
            let proof_service = ProofService::new(proof_args.root.clone(), tree_id);

            match proof_service.get_proof(vec![proof_args.hash.clone()]) {
                Ok(p) => println!("{:?}", serde_json::to_string_pretty(&p)),
                Err(e) => println!("error proof service: {:?}", e),
            };
        },
        Commands::Restart => {
            println!("iniciate Restart service");

            let sync_service = SyncService::new(http_client, 0, tree_id);

            sync_service.delete_tree();
        },
    }


}
