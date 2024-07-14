use std::str::FromStr;

use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, signature::read_keypair_file, signer::Signer,
    },
    Client, Cluster,
};
use anchor_lang::system_program;
use clap::{Parser, Subcommand};
use solana_program::pubkey::Pubkey;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a governance
    CreateGovernance {
        /// Superteam
        name: String,
    },

    /// Join the governance
    Join {
        /// Superteam
        name: String,
    },

    /// Create a proposal
    CreateProposal {
        /// Superteam
        name: String,

        /// 2024-06-13 13:03:00
        title: String,
    },

    /// Start the voting
    StartVote {
        /// proposal key
        proposal_key: String,

        /// 2024-06-13 13:03:00
        end: String,
    },

    /// Vote
    Vote {
        /// proposal key
        proposal_key: String,

        /// 1 => Yes, 0 => No
        vote: u8,
    },
}

fn main() {
    let program_id = "2BqXsVFG5Woo6VVg6pK4RM7g6W7YZwCSM9wYou8kzu6F";

    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Devnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();

    let cli = Cli::parse();
    match &cli.command {
        Commands::CreateGovernance { name } => {
            let (governance_pda, _bump) =
                Pubkey::find_program_address(&[b"governance", name.as_bytes()], &program_id);

            let sig = program
                .request()
                .accounts(voting::accounts::CreateGovernance {
                    governance: governance_pda,
                    authority: payer.pubkey(),
                    system_program: system_program::ID,
                })
                .args(voting::instruction::CreateGovernance {
                    name: name.to_string(),
                })
                .send()
                .expect("Failed to send create governance transaction");

            println!("Successfully initialized: https://solscan.io/tx/{sig}?cluster=devnet");
        }
        Commands::Join { name } => {
            let (governance_pda, _bump) =
                Pubkey::find_program_address(&[b"governance", name.as_bytes()], &program_id);
            let (user_pda, _bump) = Pubkey::find_program_address(
                &[b"user", governance_pda.as_ref(), payer.pubkey().as_ref()],
                &program_id,
            );

            let sig = program
                .request()
                .accounts(voting::accounts::Join {
                    user: user_pda,
                    authority: payer.pubkey(),
                    system_program: system_program::ID,
                })
                .args(voting::instruction::Join {
                    governance_key: governance_pda,
                })
                .send()
                .expect("Failed to send create governance transaction");

            println!("Successfully initialized: https://solscan.io/tx/{sig}?cluster=devnet");
        }
        Commands::CreateProposal { name, title } => {
            let (governance_pda, _bump) =
                Pubkey::find_program_address(&[b"governance", name.as_bytes()], &program_id);
            let (user_pda, _bump) = Pubkey::find_program_address(
                &[b"user", governance_pda.as_ref(), payer.pubkey().as_ref()],
                &program_id,
            );
            let (proposal_pda, _bump) = Pubkey::find_program_address(
                &[b"proposal", governance_pda.as_ref(), title.as_bytes()],
                &program_id,
            );

            let sig = program
                .request()
                .accounts(voting::accounts::CreateProposal {
                    proposal: proposal_pda,
                    user: user_pda,
                    system_program: system_program::ID,
                })
                .args(voting::instruction::CreateProposal {
                    governance_key: governance_pda,
                    title: title.to_string(),
                })
                .send()
                .expect("Failed to send create governance transaction");

            println!("Successfully initialized: https://solscan.io/tx/{sig}?cluster=devnet");
        }
        Commands::Vote { proposal_key, vote } => {
            let (governance_pda, _bump) =
                Pubkey::find_program_address(&[b"governance", name.as_bytes()], &program_id);
            let (user_pda, _bump) = Pubkey::find_program_address(
                &[b"user", governance_pda.as_ref(), payer.pubkey().as_ref()],
                &program_id,
            );
            let (proposal_pda, _bump) = Pubkey::find_program_address(
                &[b"proposal", governance_pda.as_ref(), title.as_bytes()],
                &program_id,
            );
        }
    }
}
