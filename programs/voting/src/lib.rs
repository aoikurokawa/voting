use anchor_lang::prelude::*;

declare_id!("CaCJAg3ifFiGyVKYxZr4QwH2R9RvrDiVEgPntzXhXVP3");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
