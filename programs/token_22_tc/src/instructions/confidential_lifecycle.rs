use anchor_lang::{prelude::*, solana_program::program::invoke};
use anchor_spl::{
    token_2022::spl_token_2022::{
        extension::{
            confidential_transfer::{instruction as confidential_instruction, DecryptableBalance},
            StateWithExtensions,
        },
        state::{Account as TokenAccountState, Mint as MintState},
    },
    token_interface::TokenInterface,
};
use proofext::instruction::ProofLocation;
use zksdk::encryption::pod::elgamal::PodElGamalCiphertext;

use crate::{AE_CIPHERTEXT_LEN, ELGAMAL_CIPHERTEXT_LEN};

#[derive(Accounts)]
pub struct ConfigureConfidentialAccount<'info> {
    /// CHECK: Token-2022 validates the account and writes the confidential extension.
    #[account(mut, owner = token_program.key())]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: Token-2022 validates this mint has ConfidentialTransferMint.
    #[account(owner = token_program.key())]
    pub mint: UncheckedAccount<'info>,
    /// CHECK: Pre-verified PubkeyValidity proof context account.
    pub pubkey_validity_proof_context: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct ApproveConfidentialAccount<'info> {
    /// CHECK: Token-2022 validates this configured account.
    #[account(mut, owner = token_program.key())]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: Token-2022 validates this mint has ConfidentialTransferMint.
    #[account(owner = token_program.key())]
    pub mint: UncheckedAccount<'info>,
    pub confidential_authority: Signer<'info>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct ConfidentialLifecycle<'info> {
    /// CHECK: validated by Token-2022, which rejects any account that is not
    /// a token account for this mint configured for confidential transfers.
    #[account(mut, owner = token_program.key())]
    pub token_account: UncheckedAccount<'info>,

    /// CHECK: validated by Token-2022 during the deposit.
    #[account(owner = token_program.key())]
    pub mint: UncheckedAccount<'info>,

    pub authority: Signer<'info>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct ConfidentialTransfer<'info> {
    /// CHECK: Token-2022 validates this source account.
    #[account(mut, owner = token_program.key())]
    pub source: UncheckedAccount<'info>,
    /// CHECK: Token-2022 validates this mint.
    #[account(owner = token_program.key())]
    pub mint: UncheckedAccount<'info>,
    /// CHECK: Token-2022 validates this destination account.
    #[account(mut, owner = token_program.key())]
    pub destination: UncheckedAccount<'info>,
    /// CHECK: Pre-verified CiphertextCommitmentEquality proof context.
    pub equality_proof_context: UncheckedAccount<'info>,
    /// CHECK: Pre-verified BatchedGroupedCiphertext3HandlesValidity proof context.
    pub ciphertext_validity_proof_context: UncheckedAccount<'info>,
    /// CHECK: Pre-verified BatchedRangeProofU128 proof context.
    pub range_proof_context: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct ConfidentialWithdraw<'info> {
    /// CHECK: Token-2022 validates this configured account.
    #[account(mut, owner = token_program.key())]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: Token-2022 validates this mint.
    #[account(owner = token_program.key())]
    pub mint: UncheckedAccount<'info>,
    /// CHECK: Pre-verified CiphertextCommitmentEquality proof context.
    pub equality_proof_context: UncheckedAccount<'info>,
    /// CHECK: Pre-verified BatchedRangeProofU64 proof context.
    pub range_proof_context: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> ConfigureConfidentialAccount<'info> {
    pub fn configure_account(
        &mut self,
        decryptable_zero_balance: [u8; AE_CIPHERTEXT_LEN],
        maximum_pending_balance_credit_counter: u64,
    ) -> Result<()> {
        read_token_account(&self.token_account.to_account_info())?;
        read_mint(&self.mint.to_account_info())?;

        let balance: DecryptableBalance = decryptable_zero_balance.into();
        let proof_context_key = self.pubkey_validity_proof_context.key();
        let ix = confidential_instruction::inner_configure_account(
            &self.token_program.key(),
            &self.token_account.key(),
            &self.mint.key(),
            &balance,
            maximum_pending_balance_credit_counter,
            &self.authority.key(),
            &[],
            ProofLocation::ContextStateAccount(&proof_context_key),
        )?;

        invoke(
            &ix,
            &[
                self.token_account.to_account_info(),
                self.mint.to_account_info(),
                self.pubkey_validity_proof_context.to_account_info(),
                self.authority.to_account_info(),
                self.token_program.to_account_info(),
            ],
        )?;

        Ok(())
    }
}

impl<'info> ApproveConfidentialAccount<'info> {
    pub fn approve_account(&mut self) -> Result<()> {
        read_token_account(&self.token_account.to_account_info())?;
        read_mint(&self.mint.to_account_info())?;

        let ix = confidential_instruction::approve_account(
            &self.token_program.key(),
            &self.token_account.key(),
            &self.mint.key(),
            &self.confidential_authority.key(),
            &[],
        )?;
        invoke(
            &ix,
            &[
                self.token_account.to_account_info(),
                self.mint.to_account_info(),
                self.confidential_authority.to_account_info(),
                self.token_program.to_account_info(),
            ],
        )?;

        Ok(())
    }
}

impl<'info> ConfidentialLifecycle<'info> {
    pub fn deposit_confidential(&mut self, amount: u64, decimals: u8) -> Result<()> {
        read_token_account(&self.token_account.to_account_info())?;
        read_mint(&self.mint.to_account_info())?;

        let ix = confidential_instruction::deposit(
            &self.token_program.key(),
            &self.token_account.key(),
            &self.mint.key(),
            amount,
            decimals,
            &self.authority.key(),
            &[],
        )?;
        invoke(
            &ix,
            &[
                self.token_account.to_account_info(),
                self.mint.to_account_info(),
                self.authority.to_account_info(),
                self.token_program.to_account_info(),
            ],
        )?;

        Ok(())
    }

    pub fn apply_pending_balance(
        &mut self,
        expected_pending_balance_credit_counter: u64,
        new_decryptable_available_balance: [u8; AE_CIPHERTEXT_LEN],
    ) -> Result<()> {
        apply_pending_balance(
            self.token_program.to_account_info(),
            self.token_account.to_account_info(),
            self.authority.to_account_info(),
            expected_pending_balance_credit_counter,
            new_decryptable_available_balance,
        )
    }
}

impl<'info> ConfidentialTransfer<'info> {
    pub fn transfer_confidential(
        &mut self,
        new_source_decryptable_available_balance: [u8; AE_CIPHERTEXT_LEN],
        transfer_amount_auditor_ciphertext_lo: [u8; ELGAMAL_CIPHERTEXT_LEN],
        transfer_amount_auditor_ciphertext_hi: [u8; ELGAMAL_CIPHERTEXT_LEN],
    ) -> Result<()> {
        read_token_account(&self.source.to_account_info())?;
        read_token_account(&self.destination.to_account_info())?;
        read_mint(&self.mint.to_account_info())?;

        let balance: DecryptableBalance = new_source_decryptable_available_balance.into();
        let auditor_ciphertext_lo =
            PodElGamalCiphertext::from(transfer_amount_auditor_ciphertext_lo);
        let auditor_ciphertext_hi =
            PodElGamalCiphertext::from(transfer_amount_auditor_ciphertext_hi);
        let equality_key = self.equality_proof_context.key();
        let ciphertext_validity_key = self.ciphertext_validity_proof_context.key();
        let range_key = self.range_proof_context.key();

        let ix = confidential_instruction::inner_transfer(
            &self.token_program.key(),
            &self.source.key(),
            &self.mint.key(),
            &self.destination.key(),
            &balance,
            &auditor_ciphertext_lo,
            &auditor_ciphertext_hi,
            &self.authority.key(),
            &[],
            ProofLocation::ContextStateAccount(&equality_key),
            ProofLocation::ContextStateAccount(&ciphertext_validity_key),
            ProofLocation::ContextStateAccount(&range_key),
        )?;

        invoke(
            &ix,
            &[
                self.source.to_account_info(),
                self.mint.to_account_info(),
                self.destination.to_account_info(),
                self.equality_proof_context.to_account_info(),
                self.ciphertext_validity_proof_context.to_account_info(),
                self.range_proof_context.to_account_info(),
                self.authority.to_account_info(),
                self.token_program.to_account_info(),
            ],
        )?;

        Ok(())
    }
}

impl<'info> ConfidentialWithdraw<'info> {
    pub fn apply_then_withdraw_confidential(
        &mut self,
        expected_pending_balance_credit_counter: u64,
        applied_decryptable_available_balance: [u8; AE_CIPHERTEXT_LEN],
        amount: u64,
        decimals: u8,
        post_withdraw_decryptable_available_balance: [u8; AE_CIPHERTEXT_LEN],
    ) -> Result<()> {
        read_token_account(&self.token_account.to_account_info())?;
        read_mint(&self.mint.to_account_info())?;

        apply_pending_balance(
            self.token_program.to_account_info(),
            self.token_account.to_account_info(),
            self.authority.to_account_info(),
            expected_pending_balance_credit_counter,
            applied_decryptable_available_balance,
        )?;

        let balance: DecryptableBalance = post_withdraw_decryptable_available_balance.into();
        let equality_key = self.equality_proof_context.key();
        let range_key = self.range_proof_context.key();

        let ix = confidential_instruction::inner_withdraw(
            &self.token_program.key(),
            &self.token_account.key(),
            &self.mint.key(),
            amount,
            decimals,
            &balance,
            &self.authority.key(),
            &[],
            ProofLocation::ContextStateAccount(&equality_key),
            ProofLocation::ContextStateAccount(&range_key),
        )?;

        invoke(
            &ix,
            &[
                self.token_account.to_account_info(),
                self.mint.to_account_info(),
                self.equality_proof_context.to_account_info(),
                self.range_proof_context.to_account_info(),
                self.authority.to_account_info(),
                self.token_program.to_account_info(),
            ],
        )?;

        Ok(())
    }
}

fn apply_pending_balance<'info>(
    token_program: AccountInfo<'info>,
    token_account: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    expected_pending_balance_credit_counter: u64,
    new_decryptable_available_balance: [u8; AE_CIPHERTEXT_LEN],
) -> Result<()> {
    read_token_account(&token_account)?;

    let balance: DecryptableBalance = new_decryptable_available_balance.into();
    let ix = confidential_instruction::apply_pending_balance(
        token_program.key,
        token_account.key,
        expected_pending_balance_credit_counter,
        &balance,
        authority.key,
        &[],
    )?;
    invoke(&ix, &[token_account, authority, token_program])?;

    Ok(())
}

fn read_mint(mint: &AccountInfo) -> Result<()> {
    let data = mint.try_borrow_data()?;
    StateWithExtensions::<MintState>::unpack(&data)?;
    Ok(())
}

fn read_token_account(token_account: &AccountInfo) -> Result<()> {
    let data = token_account.try_borrow_data()?;
    StateWithExtensions::<TokenAccountState>::unpack(&data)?;
    Ok(())
}
