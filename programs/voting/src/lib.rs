use anchor_lang::prelude::*;

declare_id!("CaCJAg3ifFiGyVKYxZr4QwH2R9RvrDiVEgPntzXhXVP3");

pub mod constants {
    pub const USER_SEED: &[u8] = b"user";
    pub const PROPOSAL_SEED: &[u8] = b"proposal";
    pub const PREDICTION_SEED: &[u8] = b"prediction";
}

#[program]
pub mod voting {
    use anchor_lang::{
        context::Context,
        solana_program::{clock::Clock, sysvar::Sysvar},
    };

    use crate::{CreateProposal, Join, StartVote, Vote, VotingErrorCode};

    pub fn join(ctx: Context<Join>) -> anchor_lang::Result<()> {
        let user = &mut ctx.accounts.user;
        user.points = 0;

        Ok(())
    }

    pub fn create_proposal(ctx: Context<CreateProposal>, title: String) -> anchor_lang::Result<()> {
        let proposal = &mut ctx.accounts.proposal;
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
pub struct Join<'info> {
    #[account(
        init,
        seeds = [crate::constants::USER_SEED, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + 64 + 4 + 4
    )]
    pub user: Account<'info, User>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateProposal<'info> {
    #[account(
        init,
        seeds = [crate::constants::PROPOSAL_SEED, title.as_str().as_ref()],
        bump,
        payer = user,
        space = 8 + 64 + 4 + 4
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
pub struct Proposal {
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
