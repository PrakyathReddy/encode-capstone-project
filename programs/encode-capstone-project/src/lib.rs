declare_id!("8p4SQHSagBPXu5S35TELTMEgstfgfYNxmETKjbEYXuvo");

#[program]
mod appartment_dao {
    use super::*;

    pub fn verify_nft(ctx: Context<VerifyNft>) -> Result<()> {
        let nft_token_account = &ctx.accounts.nft_token_account;
        let user = &ctx.accounts.user;
        let nft_mint_account = &ctx.accounts.nft_mint;

        assert_eq!(nft_token_account.owner, user.key());
        assert_eq!(nft_token_account.mint, nft_mint_account.key());
        assert_eq!(nft_token_account.amount, 1);

        let master_edition_seed = &[
            PREFIX.as_bytes(),
            ctx.accounts.nft_metadata_account.key.as_ref(),
            nft_token_account.mint.as_ref(),
            EDITION.as_bytes(),
        ];

        // let (master_edition_key, master_edition_seed) =
        //     Pubkey::find_program_address(
        //         master_edition_seed,
        //         ctx.accounts.nft_metadata_account.key
        //     );

        // assert_eq!(master_edition_key, ctx.accounts.creature_edition.key());

        // if ctx.accounts.creature_edition.data_is_empty() {
        //     return Err(ErrorCode::NotInitialized.into());
        // };

        let nft_metadata_account = &ctx.accounts.nft_metadata_account;
        let nft_mint_account_pubkey = ctx.accounts.nft_mint.key();

        //seeds for PDA
        let metadata_seed = &[
            "metadata".as_bytes(),
            nft_metadata_account.key.as_ref(),
            nft_mint_account_pubkey.as_ref(),
        ];

        //The derived key
        let (metadata_derived_key, _bumb_seed) =
            Pubkey::find_program_address(metadata_seed, nft_metadata_account.key);

        //check that the derived key is the current metadata account key
        assert_eq!(metadata_derived_key, nft_metadata_account.key());

        //check if initialized
        if nft_metadata_account.data_is_empty() {
            return err!(NftError::DataTooLarge);
        }

        //Get the metadata account struct so we can access its values
        let metadata_full_account =
            &mut Metadata::from_account_info(&ctx.accounts.nft_metadata_account)?;

        let full_metadata_clone = metadata_full_account.clone();

        use solana_program::{pubkey, pubkey::Pubkey};
        let expected_creator =
            Pubkey::from_str("FNfZnXe6VpaEwyZez1kwtHfMgNrtPtzumtxsUysYxLEP").unwrap();

        //make sure expected creator is present in metadata
        assert_eq!(
            full_metadata_clone.data.creators.as_ref().unwrap()[0].address,
            expected_creator
        );

        if !full_metadata_clone.data.creator.unwrap()[0].verified {
            // returns some error as the expected creator is not verified
            return Err(ErrorCode::AlreadyVerified.into());
        };

        Ok(())
    }
}

#[derive(Accounts)]
pub struct VerifyNft<'info> {
    // The owner of NFT
    pub user: Signer<'info>,

    //The mint account of NFT
    pub nft_mint: Account<'info, Mint>,

    //The assosiated token account that hold the NFT for the user
    pub nft_token_account: Account<'info, TokenAccount>,

    // The metadata account of the NFT
    #[account(address = metadata_program_id)]
    pub nft_metadata_account: AccountInfo<'info>,
}

#[error_code]
pub enum NftError {
    #[msg("MyAccount may only hold data below 100")]
    DataTooLarge,
}
