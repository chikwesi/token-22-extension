use anchor_lang::{
    prelude::*,
    solana_program::program::invoke,
    system_program::{create_account, CreateAccount},
};
use anchor_spl::{
    token_2022::{
        initialize_mint2,
        spl_token_2022::{
            extension::{
                confidential_transfer::instruction as confidential_instruction,
                confidential_transfer_fee::instruction as confidential_fee_instruction,
                ExtensionType,
            },
            state::Mint as MintState,
        },
        InitializeMint2,
    },
    token_interface::{transfer_fee_initialize, TokenInterface, TransferFeeInitialize},
};

#[derive(Accounts)]
pub struct CreateConfidentialFeeMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: created and initialized in the handler.
    #[account(mut, signer)]
    pub mint: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateConfidentialFeeMint<'info> {
    pub fn create_confidential_fee_mint(
        &mut self,
        decimals: u8,
        basis_points: u16,
        maximum_fee: u64,
        withdraw_withheld_authority_elgamal_pubkey: [u8; 32],
    ) -> Result<()> {
        let extensions = [
            ExtensionType::TransferFeeConfig,
            ExtensionType::ConfidentialTransferFeeConfig,
            ExtensionType::ConfidentialTransferMint,
        ];

        let space = ExtensionType::try_calculate_account_len::<MintState>(&extensions)?;
        let lamports = Rent::get()?.minimum_balance(space);

        let new_fee_mint_account = CreateAccount {
            from: self.payer.to_account_info(),
            to: self.mint.to_account_info(),
        };
        let mint_account_ctx = CpiContext::new(self.system_program.key(), new_fee_mint_account);
        create_account(
            mint_account_ctx,
            lamports,
            space as u64,
            &self.token_program.key(),
        )?;

        let mint_info = self.mint.to_account_info();
        let program_info = self.token_program.to_account_info();
        let infos = [mint_info.clone(), program_info.clone()];

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

        invoke(
            &confidential_instruction::initialize_mint(
                &self.token_program.key(),
                &self.mint.key(),
                Some(self.payer.key()),
                true,
                None,
            )?,
            &infos,
        )?;

        invoke(
            &confidential_fee_instruction::initialize_confidential_transfer_fee_config(
                &self.token_program.key(),
                &self.mint.key(),
                Some(self.payer.key()),
                &withdraw_withheld_authority_elgamal_pubkey.into(),
            )?,
            &infos,
        )?;

        let init_mint_ctx = CpiContext::new(
            self.token_program.key(),
            InitializeMint2 { mint: mint_info },
        );

        initialize_mint2(init_mint_ctx, decimals, &self.payer.key(), None)?;

        Ok(())
    }
}
