pub mod constants;
pub mod error;
pub mod instructions;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;

declare_id!("5RNPVEf64CuQR2y752tC2chqAdCtNSjnzuxCSPbU3htT");

#[program]
pub mod token_22_tc {

    use super::*;

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

    pub fn thaw_kyc_account(ctx: Context<ThawKycAccount>) -> Result<()> {
        ctx.accounts.thaw_after_kyc()
    }

    pub fn create_permanent_seize(
        ctx: Context<CreateSeizableMint>,
        decimals: u8,
        basis_points: u16,
        maximum_fee: u64,
    ) -> Result<()> {
        ctx.accounts
            .create_seizable_mint(decimals, basis_points, maximum_fee)
    }

    pub fn permanently_seize_mint(
        ctx: Context<PermanentDelegateSeize>,
        amount: u64,
        decimals: u8,
    ) -> Result<()> {
        ctx.accounts.seize_tokens(amount, decimals)
    }

    pub fn configure_confidential_account(
        ctx: Context<ConfigureConfidentialAccount>,
        decryptable_zero_balance: [u8; AE_CIPHERTEXT_LEN],
        maximum_pending_balance_credit_counter: u64,
    ) -> Result<()> {
        ctx.accounts.configure_account(
            decryptable_zero_balance,
            maximum_pending_balance_credit_counter,
        )
    }

    pub fn approve_confidential_account(ctx: Context<ApproveConfidentialAccount>) -> Result<()> {
        ctx.accounts.approve_account()
    }

    pub fn deposit_confidential_tokens(
        ctx: Context<ConfidentialLifecycle>,
        amount: u64,
        decimals: u8,
    ) -> Result<()> {
        ctx.accounts.deposit_confidential(amount, decimals)
    }

    pub fn apply_pending_confidential_balance(
        ctx: Context<ConfidentialLifecycle>,
        expected_pending_balance_credit_counter: u64,
        new_decryptable_available_balance: [u8; AE_CIPHERTEXT_LEN],
    ) -> Result<()> {
        ctx.accounts.apply_pending_balance(
            expected_pending_balance_credit_counter,
            new_decryptable_available_balance,
        )
    }

    pub fn transfer_confidential_tokens(
        ctx: Context<ConfidentialTransfer>,
        new_source_decryptable_available_balance: [u8; AE_CIPHERTEXT_LEN],
        transfer_amount_auditor_ciphertext_lo: [u8; ELGAMAL_CIPHERTEXT_LEN],
        transfer_amount_auditor_ciphertext_hi: [u8; ELGAMAL_CIPHERTEXT_LEN],
    ) -> Result<()> {
        ctx.accounts.transfer_confidential(
            new_source_decryptable_available_balance,
            transfer_amount_auditor_ciphertext_lo,
            transfer_amount_auditor_ciphertext_hi,
        )
    }

    pub fn withdraw_confidential_tokens(
        ctx: Context<ConfidentialWithdraw>,
        expected_pending_balance_credit_counter: u64,
        applied_decryptable_available_balance: [u8; AE_CIPHERTEXT_LEN],
        amount: u64,
        decimals: u8,
        post_withdraw_decryptable_available_balance: [u8; AE_CIPHERTEXT_LEN],
    ) -> Result<()> {
        ctx.accounts.apply_then_withdraw_confidential(
            expected_pending_balance_credit_counter,
            applied_decryptable_available_balance,
            amount,
            decimals,
            post_withdraw_decryptable_available_balance,
        )
    }
}
