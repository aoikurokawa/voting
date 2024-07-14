use std::{str::FromStr, sync::Arc};

use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        signature::{read_keypair_file, Keypair, Signature},
        signer::Signer,
    },
    Client, ClientError, Cluster, Program,
};
use anchor_lang::system_program;
use solana_program::pubkey::Pubkey;

#[allow(unused_imports)]
mod test;

pub const PROGRAM_ID: &str = "CaCJAg3ifFiGyVKYxZr4QwH2R9RvrDiVEgPntzXhXVP3";

pub struct TestSetup {
    pub payer: Arc<Keypair>,
    pub client: Client<Arc<Keypair>>,
    pub program_id: Pubkey,
    pub program: Program<Arc<Keypair>>,
}

impl TestSetup {
    pub fn new() -> Self {
        let program_id = PROGRAM_ID;
        let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
        let payer = read_keypair_file(&anchor_wallet).unwrap();
        let payer = Arc::new(payer);

        let client = Client::new_with_options(
            Cluster::Localnet,
            payer.clone(),
            CommitmentConfig::confirmed(),
        );
        let program_id = Pubkey::from_str(program_id).unwrap();
        let program = client.program(program_id).unwrap();

        Self {
            payer,
            client,
            program_id,
            program,
        }
    }

    pub fn get_governance_pda(&self, name: &str) -> Pubkey {
        let (governance_pda, _bump) =
            Pubkey::find_program_address(&[b"governance", name.as_bytes()], &self.program_id);

        governance_pda
    }

    pub fn get_user_pda(&self, name: &str) -> Pubkey {
        let governance_pda = self.get_governance_pda(name);
        let (user_pda, _bump) = Pubkey::find_program_address(
            &[
                b"user",
                governance_pda.as_ref(),
                self.payer.pubkey().as_ref(),
            ],
            &self.program_id,
        );

        user_pda
    }

    pub fn get_proposal_pda(&self, name: &str, title: &str) -> Pubkey {
        let governance_pda = self.get_governance_pda(name);
        let (proposal_pda, _bump) = Pubkey::find_program_address(
            &[b"proposal", governance_pda.as_ref(), title.as_bytes()],
            &self.program_id,
        );

        proposal_pda
    }

    pub fn get_vote_pda(&self, name: &str, title: &str) -> Pubkey {
        let governance_pda = self.get_governance_pda(name);
        let proposal_pda = self.get_proposal_pda(name, title);

        let (vote_pda, _bump) = Pubkey::find_program_address(
            &[
                b"commit_vote",
                governance_pda.as_ref(),
                proposal_pda.as_ref(),
                self.payer.pubkey().as_ref(),
            ],
            &self.program_id,
        );

        vote_pda
    }

    pub fn create_governance(&self, name: &str) -> Result<Signature, ClientError> {
        let governance_pda = self.get_governance_pda(name);

        self.program
            .request()
            .accounts(voting::accounts::CreateGovernance {
                governance: governance_pda,
                authority: self.payer.pubkey(),
                system_program: system_program::ID,
            })
            .args(voting::instruction::CreateGovernance {
                name: name.to_string(),
            })
            .send()
    }

    pub fn join(&self, name: &str) -> Result<Signature, ClientError> {
        let governance_pda = self.get_governance_pda(name);
        let user_pda = self.get_user_pda(name);

        self.program
            .request()
            .accounts(voting::accounts::Join {
                user: user_pda,
                authority: self.payer.pubkey(),
                system_program: system_program::ID,
            })
            .args(voting::instruction::Join {
                governance_key: governance_pda,
            })
            .send()
    }

    pub fn create_proposal(&self, name: &str, title: &str) -> Result<Signature, ClientError> {
        let governance_pda = self.get_governance_pda(name);
        let proposal_pda = self.get_proposal_pda(name, title);

        self.program
            .request()
            .accounts(voting::accounts::CreateProposal {
                proposal: proposal_pda,
                user: self.payer.pubkey(),
                system_program: system_program::ID,
            })
            .args(voting::instruction::CreateProposal {
                governance_key: governance_pda,
                title: title.as_bytes().to_vec(),
            })
            .send()
    }

    pub fn start_vote(&self, name: &str, title: &str, end: i64) -> Result<Signature, ClientError> {
        let proposal_pda = self.get_proposal_pda(name, title);

        self.program
            .request()
            .accounts(voting::accounts::StartVote {
                proposal: proposal_pda,
            })
            .args(voting::instruction::StartVote { end })
            .send()
    }

    pub fn commit_vote(
        &self,
        name: &str,
        title: &str,
        vote: u8,
        salt: &str,
    ) -> Result<Signature, ClientError> {
        let governance_pda = self.get_governance_pda(name);
        let proposal_pda = self.get_proposal_pda(name, title);
        let vote_pda = self.get_vote_pda(name, title);

        let vote = vote == 1;
        let temp = format!("{}{}", vote, salt);
        let commitment = solana_program::hash::hash(temp.as_bytes());

        self.program
            .request()
            .accounts(voting::accounts::CommitVote {
                governance: governance_pda,
                proposal: proposal_pda,
                vote_commitment: vote_pda,
                user: self.payer.pubkey(),
                system_program: system_program::ID,
            })
            .args(voting::instruction::CommitVote {
                commitment: commitment.to_string(),
            })
            .send()
    }

    pub fn reveal_vote(
        &self,
        name: &str,
        title: &str,
        vote: u8,
        salt: &str,
    ) -> Result<Signature, ClientError> {
        let proposal_pda = self.get_proposal_pda(name, title);
        let vote_pda = self.get_vote_pda(name, title);
        let user_pda = self.get_user_pda(name);
        let vote = vote == 1;

        self.program
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
    }
}
