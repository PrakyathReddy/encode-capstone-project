use anchor_lang::prelude::*;

declare_id!("2fEUszgesnJpufDjX3VzGzN8h1CMeAPyj1J6staE5hoZ");

#[program]
pub mod encode_capstone_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
