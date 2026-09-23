use anchor_lang::{
    prelude::*,
    system_program::{create_account, CreateAccount},
};
use anchor_spl::{
    token_2022::{
        initialize_mint2,
        spl_token_2022::{extension::ExtensionType, state::Mint as MintState},
        InitializeMint2,
    },
    token_interface::{
        mint_close_authority_initialize, transfer_fee_initialize, Mint,
        MintCloseAuthorityInitialize, TokenInterface, TransferFeeInitialize,
    },
};

#[derive(Accounts)]
#[instruction(decimals: u8)]
pub struct CreateMintDeclarative<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        mint::decimals = decimals,
        mint::authority = payer,
        mint::token_program = token_program,
        extensions::close_authority::authority = payer,
        extensions::metadata_pointer::authority = payer,
        extensions::metadata_pointer::metadata_address = payer,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateMintWithFee<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: created and initialized in the handler, and required to sign
    /// because the account is made at its own address.
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateMintWithFee<'info> {
    pub fn create_mint_with_fee(
        &mut self,
        decimals: u8,
        basis_points: u16,
        maximum_fee: u64,
    ) -> Result<()> {
        let extensions = [
            ExtensionType::MintCloseAuthority,
            ExtensionType::TransferFeeConfig,
        ];

        let space = ExtensionType::try_calculate_account_len::<MintState>(&extensions)?;
        let lamports = Rent::get()?.minimum_balance(space);

        let new_acct = CreateAccount {
            from: self.payer.to_account_info(),
            to: self.mint.to_account_info(),
        };
        let new_acct_ctx = CpiContext::new(self.token_program.key(), new_acct);
        create_account(
            new_acct_ctx,
            lamports,
            space as u64,
            &self.token_program.key(),
        )?;

        let mint_close_auth_acct = MintCloseAuthorityInitialize {
            token_program_id: self.token_program.to_account_info(),
            mint: self.mint.to_account_info(),
        };
        let mint_close_auth_ctx = CpiContext::new(self.token_program.key(), mint_close_auth_acct);
        mint_close_authority_initialize(mint_close_auth_ctx, Some(&self.payer.key()))?;

        let transfer_fee_acct = TransferFeeInitialize {
            token_program_id: self.token_program.to_account_info(),
            mint: self.mint.to_account_info(),
        };
        let transfer_fee_ctx = CpiContext::new(self.token_program.key(), transfer_fee_acct);
        transfer_fee_initialize(
            transfer_fee_ctx,
            Some(&self.payer.key()),
            Some(&self.payer.key()),
            basis_points,
            maximum_fee,
        )?;

        let sealed_mint_acct = InitializeMint2 {
            mint: self.mint.to_account_info(),
        };
        let sealed_mint_ctx = CpiContext::new(self.token_program.key(), sealed_mint_acct);
        initialize_mint2(sealed_mint_ctx, decimals, &self.payer.key(), None)
    }
}
