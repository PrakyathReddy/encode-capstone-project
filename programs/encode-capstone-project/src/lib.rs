use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;
use solana_program::sys;

declare_id!("2fEUszgesnJpufDjX3VzGzN8h1CMeAPyj1J6staE5hoZ");

#[program]
pub mod apartment_dao {
    use super::*;

    #[state]
    pub struct ApartmentDAO {
        pub issues: Vec<Issue>,
        pub voting_rights: Vec<Pubkey>,
        pub corpus_fund: Pubkey,
    }

    impl ApartmentDAO {
        pub fn new(ctx: Context<CreateApartmentDAO>, corpus_fund: Pubkey) -> ProgramResult {
            let apartment_dao = &mut ctx.accounts.apartment_dao;
            apartment_dao.issues = Vec::new();
            apartment_dao.voting_rights = Vec::new();
            apartment_dao.corpus_fund = corpus_fund;
            Ok(())
        }

        pub fn add_issue(ctx: Context<AddIssue>, description: String) -> ProgramResult {
            let apartment_dao = &mut ctx.accounts.apartment_dao;
            let issue = Issue {
                description,
                upvotes: 0,
                status: IssueStatus::InQueue,
            };
            apartment_dao.issues.push(issue);
            Ok(())
        }

        pub fn upvote_issue(ctx: Context<UpvoteIssue>, issue_index: u32) -> ProgramResult {
            let apartment_dao = &mut ctx.accounts.apartment_dao;
            apartment_dao.issues[issue_index as usize].upvotes += 1;
            Ok(())
        }

        pub fn resolve_issue(ctx: Context<ResolveIssue>, issue_index: u32) -> ProgramResult {
            let apartment_dao = &mut ctx.accounts.apartment_dao;
            apartment_dao.issues[issue_index as usize].status = IssueStatus::Resolved;
            Ok(())
        }
    }

    #[derive(Accounts)]
    pub struct CreateApartmentDAO<'info> {
        #[account(init, payer = user, space = 8 + 8)]
        pub apartment_dao: Account<'info, ApartmentDAO>,
        pub user: Signer<'info>,
        #[account("sysvar.programAccounts")]
        pub system_program: Program<sys::System>,
    }

    #[derive(Accounts)]
    pub struct AddIssue<'info> {
        #[account(mut)]
        pub apartment_dao: Account<'info, ApartmentDAO>,
        #[account("sysvar.programAccounts")]
        pub system_program: Program<sys::System>,
    }

    #[derive(Accounts)]
    pub struct UpvoteIssue<'info> {
        #[account(mut)]
        pub apartment_dao: Account<'info, ApartmentDAO>,
        #[account("sysvar.programAccounts")]
        pub system_program: Program<sys::System>,
    }

    #[derive(Accounts)]
    pub struct ResolveIssue<'info> {
        #[account(mut)]
        pub apartment_dao: Account<'info, ApartmentDAO>,
        #[account("sysvar.programAccounts")]
        pub system_program: Program<sys::System>,
    }

    #[account]
    pub struct Issue {
        pub description: String,
        pub upvotes: u32,
        pub status: IssueStatus,
    }

    #[derive(Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize)]
    pub enum IssueStatus {
        InQueue,
        WIP,
        Resolved,
    }
}
