use clap::{Parser, Subcommand, Args};

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
    #[arg(default_value_t = 1740)]
    max: u64,
}

#[derive(Args, Debug)]
struct ProofArgs {
    /// hash string to get the proof
    hash: String,
}

pub fn init() {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Sync(sync_args) => {
            println!("'myapp sync' was used, name is: {:?}", sync_args.max);
        }
        Commands::Proof(proof_args) => {
            println!("'myapp proof' was used, name is: {:?}", proof_args.hash);
        },
    }
}
