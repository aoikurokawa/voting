use std::str::FromStr;

use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, signature::read_keypair_file, signer::Signer,
    },
    Client, Cluster,
};
use anchor_lang::system_program;
use chrono::DateTime;
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

        /// Proposal title
        title: String,
    },

    /// Start the voting
    StartVote {
        /// Superteam
        name: String,

        /// Proposal title
        title: String,

        /// 2024-06-13 13:03:00
        end: String,
    },

    /// Start the voting
    CommitVote {
        /// Superteam
        name: String,

        /// Proposal title
        title: String,

        /// 1 => Yes, 0 => No
        vote: u8,

        /// salt
        salt: String,
    },

    /// Reveal vote
    RevealVote {
        /// Superteam
        name: String,

        /// Proposal title
        title: String,

        /// 1 => Yes, 0 => No
        vote: u8,

        /// salt
        salt: String,
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
                .expect("Failed to send join transaction");

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
                .expect("Failed to send create proposal transaction");

            println!("Successfully initialized: https://solscan.io/tx/{sig}?cluster=devnet");
        }
        Commands::StartVote { name, title, end } => {
            let (governance_pda, _bump) =
                Pubkey::find_program_address(&[b"governance", name.as_bytes()], &program_id);

            let (proposal_pda, _bump) = Pubkey::find_program_address(
                &[b"proposal", governance_pda.as_ref(), title.as_bytes()],
                &program_id,
            );

            let end = format!("{end} +0000");
            let end = DateTime::parse_from_str(&end, "%Y-%m-%d %H:%M:%S %z").unwrap();

            let sig = program
                .request()
                .accounts(voting::accounts::StartVote {
                    proposal: proposal_pda,
                })
                .args(voting::instruction::StartVote {
                    end: end.timestamp(),
                })
                .send()
                .expect("Failed to send start vote transaction");

            println!("Successfully initialized: https://solscan.io/tx/{sig}?cluster=devnet");
        }
        Commands::CommitVote {
            name,
            title,
            vote,
            salt,
        } => {
            let (governance_pda, _bump) =
                Pubkey::find_program_address(&[b"governance", name.as_bytes()], &program_id);
            // let (user_pda, _bump) = Pubkey::find_program_address(
            //     &[b"user", governance_pda.as_ref(), payer.pubkey().as_ref()],
            //     &program_id,
            // );
            let (proposal_pda, _bump) = Pubkey::find_program_address(
                &[b"proposal", governance_pda.as_ref(), title.as_bytes()],
                &program_id,
            );
            let (vote_pda, _bump) = Pubkey::find_program_address(
                &[
                    b"commit_vote",
                    governance_pda.as_ref(),
                    proposal_pda.as_ref(),
                    payer.pubkey().as_ref(),
                ],
                &program_id,
            );

            let vote = *vote == 1;
            let temp = format!("{}{}", vote, salt);
            let commitment = solana_program::hash::hash(temp.as_bytes());

            let sig = program
                .request()
                .accounts(voting::accounts::CommitVote {
                    governance: governance_pda,
                    proposal: proposal_pda,
                    vote_commitment: vote_pda,
                    user: payer.pubkey(),
                    system_program: system_program::ID,
                })
                .args(voting::instruction::CommitVote {
                    commitment: commitment.to_string(),
                })
                .send()
                .expect("Failed to send commit vote transaction");

            println!("Successfully initialized: https://solscan.io/tx/{sig}?cluster=devnet");
        }
        Commands::RevealVote {
            name,
            title,
            vote,
            salt,
        } => {
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
            let (vote_pda, _bump) = Pubkey::find_program_address(
                &[
                    b"commit_vote",
                    governance_pda.as_ref(),
                    proposal_pda.as_ref(),
                    payer.pubkey().as_ref(),
                ],
                &program_id,
            );

            let vote = *vote == 1;
            let sig = program
                .request()
                .accounts(voting::accounts::RevealVote {
                    proposal: proposal_pda,
                    vote_commitment: vote_pda,
                    user: user_pda,
                })
                .args(voting::instruction::RevealVote {
                    vote,
                    salt: salt.to_string(),
                })
                .send()
                .expect("Failed to send reveal vote transaction");

            println!("Successfully initialized: https://solscan.io/tx/{sig}?cluster=devnet");
        }
    }
}
