use anchor_lang::prelude::*;

declare_id!("CaCJAg3ifFiGyVKYxZr4QwH2R9RvrDiVEgPntzXhXVP3");

pub mod constants {
    pub const GOVERNANCE_SEED: &[u8] = b"governance";
    pub const USER_SEED: &[u8] = b"user";
    pub const PROPOSAL_SEED: &[u8] = b"proposal";
}

#[program]
pub mod voting {
    use anchor_lang::{
        context::Context,
        solana_program::{clock::Clock, pubkey::Pubkey, sysvar::Sysvar},
    };

    use crate::{CreateGovernance, CreateProposal, Join, StartVote, Vote, VotingErrorCode};

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
        title: String,
    ) -> anchor_lang::Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.governance = governance_key;
        proposal.title = title;
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

    pub fn vote(ctx: Context<Vote>, vote: bool) -> anchor_lang::Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let user = &mut ctx.accounts.user;

        if proposal.start == 0 {
            return Err(VotingErrorCode::NotStarted.into());
        }

        let clock = Clock::get()?;
        if proposal.end < clock.unix_timestamp {
            return Err(VotingErrorCode::AlreadyFinished.into());
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
#[instruction(title: String, governance_key: Pubkey)]
pub struct CreateProposal<'info> {
    #[account(
        init,
        seeds = [crate::constants::PROPOSAL_SEED, governance_key.as_ref(), title.as_str().as_ref()],
        bump,
        payer = user,
        space = 8 + std::mem::size_of::<Proposal>()
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
pub struct Vote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,

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
    votes_for: u32,
    votes_against: u32,
    start: i64,
    end: i64,
}

#[account]
pub struct User {
    points: u32,
}

#[error_code]
pub enum VotingErrorCode {
    #[msg("Voting has not started yet")]
    NotStarted,

    #[msg("Voting has already finished")]
    AlreadyFinished,
}
