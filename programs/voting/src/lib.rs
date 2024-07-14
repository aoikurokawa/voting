use anchor_lang::prelude::*;

declare_id!("CaCJAg3ifFiGyVKYxZr4QwH2R9RvrDiVEgPntzXhXVP3");

pub mod constants {
    pub const GOVERNANCE_SEED: &[u8] = b"governance";
    pub const USER_SEED: &[u8] = b"user";
    pub const PROPOSAL_SEED: &[u8] = b"proposal";
    pub const COMMIT_VOTE_SEED: &[u8] = b"commit_vote";
}

#[program]
pub mod voting {
    use anchor_lang::{
        context::Context,
        solana_program::{self, clock::Clock, pubkey::Pubkey, sysvar::Sysvar},
    };

    use crate::{
        CommitVote, CreateGovernance, CreateProposal, Join, RevealVote, StartVote, VotingErrorCode,
    };

    pub fn create_governance(
        ctx: Context<CreateGovernance>,
        name: String,
    ) -> anchor_lang::Result<()> {
        let governance = &mut ctx.accounts.governance;
        governance.name = name;

        Ok(())
    }

    pub fn join(ctx: Context<Join>, governance_key: Pubkey) -> anchor_lang::Result<()> {
        let user = &mut ctx.accounts.user;
        user.points = 0;

        let _governance_key = governance_key;

        Ok(())
    }

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        governance_key: Pubkey,
        title: Vec<u8>,
    ) -> anchor_lang::Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.governance = governance_key;
        proposal.title = String::from_utf8(title).unwrap();
        proposal.votes_for = 0;
        proposal.votes_against = 0;
        proposal.start = 0;
        proposal.end = 0;

        Ok(())
    }

    pub fn start_vote(ctx: Context<StartVote>, end: i64) -> anchor_lang::Result<()> {
        let proposal = &mut ctx.accounts.proposal;

        let clock = Clock::get()?;
        proposal.start = clock.unix_timestamp;
        proposal.end = end;

        Ok(())
    }

    pub fn commit_vote(ctx: Context<CommitVote>, commitment: String) -> anchor_lang::Result<()> {
        let proposal = &ctx.accounts.proposal;

        let current_time = Clock::get()?.unix_timestamp;
        if proposal.start > current_time {
            return Err(VotingErrorCode::NotStarted.into());
        }

        if proposal.end < current_time {
            return Err(VotingErrorCode::VotingEnded.into());
        }

        let vote_commitment = &mut ctx.accounts.vote_commitment;

        vote_commitment.commitment = commitment;

        Ok(())
    }

    pub fn reveal_vote(
        ctx: Context<RevealVote>,
        vote: bool,
        salt: String,
    ) -> anchor_lang::Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let user = &mut ctx.accounts.user;

        let clock = Clock::get()?;
        if proposal.end > clock.unix_timestamp {
            return Err(VotingErrorCode::VotingNotEnded.into());
        }

        let vote_commitment = &ctx.accounts.vote_commitment;
        let temp = format!("{}{}", vote, salt);
        let hash = solana_program::hash::hash(temp.as_bytes());
        if vote_commitment.commitment != hash.to_string() {
            return Err(VotingErrorCode::InvalidCommitment.into());
        }

        if vote {
            proposal.votes_for += 1;
        } else {
            proposal.votes_against += 1;
        }

        user.points += 1;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateGovernance<'info> {
    #[account(
        init,
        seeds = [crate::constants::GOVERNANCE_SEED, name.as_str().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<Governance>()
    )]
    pub governance: Account<'info, Governance>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(governance_key: Pubkey)]
pub struct Join<'info> {
    #[account(
        init,
        seeds = [crate::constants::USER_SEED, governance_key.as_ref(), authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<User>()
    )]
    pub user: Account<'info, User>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(governance_key: Pubkey, title: String)]
pub struct CreateProposal<'info> {
    #[account(
        init,
        seeds = [crate::constants::PROPOSAL_SEED, governance_key.as_ref(), title.as_str().as_ref()],
        bump,
        payer = user,
        space = 8 + 32 + 4 + title.len() + 4 + 4 + 8 + 8
    )]
    pub proposal: Account<'info, Proposal>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct StartVote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
}

#[derive(Accounts)]
pub struct CommitVote<'info> {
    #[account(mut)]
    pub governance: Account<'info, Governance>,

    #[account(mut)]
    pub proposal: Account<'info, Proposal>,

    #[account(
        init,
        seeds = [crate::constants::COMMIT_VOTE_SEED, governance.key().as_ref(), proposal.key().as_ref(), user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + std::mem::size_of::<Proposal>()
    )]
    pub vote_commitment: Account<'info, VoteCommitment>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RevealVote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,

    #[account(mut)]
    pub vote_commitment: Account<'info, VoteCommitment>,

    #[account(mut)]
    pub user: Account<'info, User>,
}

#[account]
pub struct Governance {
    name: String,
}

#[account]
pub struct Proposal {
    governance: Pubkey,
    title: String,
    pub votes_for: u32,
    pub votes_against: u32,
    start: i64,
    end: i64,
}

#[account]
pub struct VoteCommitment {
    commitment: String,
}

#[account]
pub struct User {
    points: u32,
}

#[error_code]
pub enum VotingErrorCode {
    #[msg("Voting has not started yet")]
    NotStarted,

    #[msg("Voting ended")]
    VotingEnded,

    #[msg("Voting not ended")]
    VotingNotEnded,

    #[msg("Invalid Commitment")]
    InvalidCommitment,
}
