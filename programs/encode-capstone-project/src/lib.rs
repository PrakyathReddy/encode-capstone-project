use anchor_lang::prelude::*;

declare_id!("BfL4d7RJNYD7gceqRt1SWyped4goq3wCdHpqG3JsAPhh");

#[constant]
pub const USER_SEED: &[u8] = b"user";

#[constant]
pub const ISSUE_SEED: &[u8] = b"issue";

#[constant]
pub const VOTE_SEED: &[u8] = b"vote";

#[program]
mod appartment_dao {
    use super::*;

    // Instructions ////////////////////////////////////////////////////////////////////////////////////

    pub fn initialize_new_user(ctx: Context<Initialize>, name: String) -> Result<()> {
        //here we have to verify that the user hold our NFT | this is WIP

        let new_account = &mut ctx.accounts.new_account;
        let signer = &mut ctx.accounts.signer;

        new_account.name = name;
        new_account.issue_count = 0;
        new_account.signer = signer.key();

        Ok(())
    }

    pub fn add_new_issue(ctx: Context<CreatePost>, title: String, content: String) -> Result<()> {
        let issue_account = &mut ctx.accounts.issue_account;
        let user_account = &mut ctx.accounts.user_account;
        let signer = &mut ctx.accounts.signer;

        issue_account.title = title;
        issue_account.content = content;
        issue_account.signer = signer.key();
        // issue_account.up_votes = 0;
        // issue_account.down_votes = 0;

        user_account.issue_count = user_account.issue_count.checked_add(1).unwrap();
        Ok(())
    }

    // pub fn vote_for_issue(ctx: Context<CastVote>, vote: u8) -> Result<()> {
    pub fn vote_for_issue(ctx: Context<CastVote>) -> Result<()> {
        let issue_account = &mut ctx.accounts.issue_account;
        let vote_account = &mut ctx.accounts.vote_account;
        let user_account = &mut ctx.accounts.user_account;

        vote_account.issue = issue_account.key();
        vote_account.voter = user_account.key();

        issue_account.votes = issue_account.votes.checked_add(1).unwrap();

        // if vote == 1 {
        //     vote_account.vote = vote;
        //     issue_account.up_votes = issue_account.up_votes.checked_add(1).unwrap();
        // }
        //
        // if vote == 0 {
        //     vote_account.vote = vote;
        //     issue_account.down_votes = issue_account.down_votes.checked_add(1).unwrap();
        // }

        Ok(())
    }
}

// CONTEXTS ////////////////////////////////////////////////////////////////////////////////////

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [USER_SEED, signer.key().as_ref()],
        bump,
        payer = signer,
        space = 8 + 250
    )]
    pub new_account: Account<'info, DAOUser>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(
        init,
        seeds = [ISSUE_SEED, signer.key().as_ref(),[user_account.issue_count].as_ref()],
        bump,
        payer = signer,
        space = 8 + 200 + 200 + 2 + 2
    )]
    pub issue_account: Account<'info, NewIssue>,

    #[account(
        mut,
        seeds = [USER_SEED, signer.key().as_ref()],
        bump,
        has_one = signer
    )]
    pub user_account: Account<'info, DAOUser>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(
        init,
        seeds = [VOTE_SEED ,issue_account.key().as_ref(), signer.key().as_ref()],
        bump,
        payer = signer,
        space = 8 + 32 + 32 + 1
    )]
    pub vote_account: Account<'info, Vote>,

    #[account(mut)]
    pub issue_account: Account<'info, NewIssue>,

    #[account(mut)]
    pub user_account: Account<'info, DAOUser>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// PDAs //////////////////////////////////////////////////////////////////////////////////////////////

#[account]
pub struct DAOUser {
    pub name: String,
    pub issue_count: u8,
    pub signer: Pubkey,
}

#[account]
pub struct NewIssue {
    pub title: String,
    pub content: String,
    pub signer: Pubkey,
    pub votes: u16,
    //pub up_votes: u16,
    //pub down_votes: u16,
}

#[account]
pub struct Vote {
    pub issue: Pubkey,
    pub voter: Pubkey,
    //pub vote: u8,
}
