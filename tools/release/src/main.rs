mod candidates;
mod model;
mod prepare;
mod process;
mod publish;
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
#[derive(Parser)]
#[command(
    about = "Collect, independently verify, and locally publish binding candidates",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    Prepare {
        #[arg(long)]
        repo: String,
    },
    Inventory {
        #[arg(long)]
        dir: PathBuf,
        #[arg(long)]
        recipe: String,
        #[arg(long)]
        group: String,
        #[arg(long)]
        candidate: Option<PathBuf>,
        #[arg(long)]
        output: PathBuf,
    },
    Candidate {
        #[arg(long)]
        dir: PathBuf,
        #[arg(long)]
        recipe: String,
        #[arg(long)]
        run_id: String,
        #[arg(long)]
        attempt: String,
        #[arg(long)]
        output: PathBuf,
    },
    Verify {
        #[arg(long)]
        candidate: String,
        #[arg(long)]
        group: String,
        #[arg(long)]
        output: PathBuf,
    },
    /// Authenticate a public candidate and rebuild the groups supported by this host.
    VerifyCandidate {
        /// Public candidate release tag in the selected repository.
        id: String,
        /// Expected SHA-256 of the exact candidate.json bytes.
        sha256: String,
        /// Repository trusted for both candidate assets and recipe branch history.
        #[arg(long, default_value = "lightningdevkit/ldk-node-bindings")]
        repo: String,
        /// Rebuild just this supported group instead of all host groups.
        #[arg(long)]
        group: Option<String>,
        /// Empty directory for downloads, pinned recipe, receipts, and diagnostics.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    Collect {
        #[arg(long)]
        candidate: String,
        /// Optional directory containing trusted independent rebuild receipts.
        #[arg(long)]
        receipts: Option<PathBuf>,
        /// Skip prompts for comparisons performed without a receipt.
        #[arg(long)]
        non_interactive: bool,
        /// Require every group to have a receipt or out-of-band confirmation.
        #[arg(long)]
        require_verification: bool,
        #[arg(long)]
        output: PathBuf,
    },
    Publish {
        /// Collection ledger; --verified remains accepted for older commands.
        #[arg(long, alias = "verified")]
        collected: PathBuf,
        /// Explicitly select the default requirement for complete verification.
        #[arg(long, conflicts_with = "allow_unverified")]
        require_verification: bool,
        /// Allow publication with groups lacking receipts or out-of-band confirmations.
        #[arg(long)]
        allow_unverified: bool,
        #[arg(long)]
        candidate: String,
        #[arg(long, value_enum)]
        destination: publish::Destination,
        #[arg(long)]
        confirm: String,
        #[command(flatten)]
        swift: publish::SwiftOptions,
    },
}
fn main() {
    if let Err(error) = execute() {
        eprintln!("error: {error:#}");
        std::process::exit(1);
    }
}
fn execute() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Prepare { repo } => prepare::prepare(&process::root()?, &repo),
        Commands::Inventory {
            dir,
            recipe,
            group,
            candidate,
            output,
        } => candidates::inventory(
            &process::root()?,
            &dir,
            &recipe,
            &group,
            candidate.as_deref(),
            &output,
        ),
        Commands::Candidate {
            dir,
            recipe,
            run_id,
            attempt,
            output,
        } => candidates::candidate(&process::root()?, &dir, &recipe, &run_id, &attempt, &output),
        Commands::Verify {
            candidate,
            group,
            output,
        } => candidates::verify(&process::root()?, &candidate, &group, &output),
        Commands::VerifyCandidate {
            id,
            sha256,
            repo,
            group,
            output,
        } => candidates::verify_candidate(&id, &sha256, &repo, group.as_deref(), output.as_deref()),
        Commands::Collect {
            candidate,
            receipts,
            non_interactive,
            require_verification,
            output,
        } => candidates::collect(
            &process::root()?,
            &candidate,
            receipts.as_deref(),
            &output,
            non_interactive,
            require_verification,
        ),
        Commands::Publish {
            collected,
            require_verification: _,
            allow_unverified,
            candidate,
            destination,
            confirm,
            swift,
        } => publish::publish(
            &process::root()?,
            &collected,
            &candidate,
            destination,
            &confirm,
            allow_unverified,
            &swift,
        ),
    }
}
