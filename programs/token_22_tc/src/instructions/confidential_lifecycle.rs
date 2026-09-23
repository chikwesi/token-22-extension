use anchor_lang::{prelude::*, solana_program::program::invoke};
use anchor_spl::{
    token_2022::spl_token_2022::extension::confidential_transfer::{
        instruction as confidential_instruction, DecryptableBalance,
    },
    token_interface::TokenInterface,
};

use crate::AE_CIPHERTEXT_LEN;

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

impl<'info> ConfidentialLifecycle<'info> {
    pub fn deposit_confidential(&mut self, amount: u64, decimals: u8) -> Result<()> {
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
        let balance: DecryptableBalance = new_decryptable_available_balance.into();
        let ix = confidential_instruction::apply_pending_balance(
            &self.token_program.key(),
            &self.token_account.key(),
            expected_pending_balance_credit_counter,
            &balance,
            &self.authority.key(),
            &[],
        )?;
        invoke(
            &ix,
            &[
                self.token_account.to_account_info(),
                self.authority.to_account_info(),
                self.token_program.to_account_info(),
            ],
        )?;

        Ok(())
    }
}
