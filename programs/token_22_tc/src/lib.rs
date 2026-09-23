pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("5RNPVEf64CuQR2y752tC2chqAdCtNSjnzuxCSPbU3htT");

#[program]
pub mod token_22_tc {

    use super::*;

    pub fn create_mint_declarative(
        ctx: Context<CreateMintDeclarative>,
        decimals: u8,
    ) -> Result<()> {
        msg!(
            "mint {} created with {} decimals",
            ctx.accounts.mint.key(),
            decimals
        );

        Ok(())
    }

    pub fn create_mint_with_fee(
        ctx: Context<CreateMintWithFee>,
        decimals: u8,
        basis_points: u16,
        maximum_fee: u64,
    ) -> Result<()> {
        ctx.accounts
            .create_mint_with_fee(decimals, basis_points, maximum_fee)
    }

    pub fn assert_supported_mint(
        ctx: Context<AssertSupportedMint>,
        amount: u64,
        decimal: u8,
    ) -> Result<()> {
        ctx.accounts.assert_supported_mint(amount, decimal)
    }

    pub fn reassign_freeze_delegate_to_program(
        ctx: Context<UnfreezeDelegate>,
        amount: u64,
    ) -> Result<()> {
        ctx.accounts.delegate_to_program(amount)
    }

    pub fn create_permanent_seize(ctx: Context<CreateSeizableMint>, decimals: u8) -> Result<()> {
        msg!(
            "Creates seizable mint: {} with decimals: {}",
            ctx.accounts.mint.key(),
            decimals
        );
        Ok(())
    }

    pub fn permanently_seize_mint(
        ctx: Context<PermanentDelegateSeize>,
        amount: u64,
        decimals: u8,
    ) -> Result<()> {
        ctx.accounts.seize_tokens(amount, decimals)
    }

    pub fn create_confidential_fee_mint(
        ctx: Context<CreateConfidentialFeeMint>,
        decimals: u8,
        basis_points: u16,
        maximum_fee: u64,
        withdraw_withheld_authority_elgamal_pubkey: [u8; 32],
    ) -> Result<()> {
        ctx.accounts.create_confidential_fee_mint(
            decimals,
            basis_points,
            maximum_fee,
            withdraw_withheld_authority_elgamal_pubkey,
        )
    }

    pub fn handle_confidential_lifecycle(
        ctx: Context<ConfidentialLifecycle>,
        amount: u64,
        decimals: u8,
        expected_pending_balance_credit_counter: u64,
        new_decryptable_available_balance: [u8; AE_CIPHERTEXT_LEN],
    ) -> Result<()> {
        ctx.accounts.deposit_confidential(amount, decimals)?;

        ctx.accounts.apply_pending_balance(
            expected_pending_balance_credit_counter,
            new_decryptable_available_balance,
        )
    }
}
