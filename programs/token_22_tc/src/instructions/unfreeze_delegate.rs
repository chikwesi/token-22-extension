use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::{thaw_account, ThawAccount},
    token_interface::TokenInterface,
};

#[derive(Accounts)]
pub struct ThawKycAccount<'info> {
    pub freeze_authority: Signer<'info>,
    /// CHECK: Token-2022 validates this is the frozen account for the mint.
    #[account(mut, owner = token_program.key())]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: Token-2022 validates this is the account mint.
    #[account(owner = token_program.key())]
    pub mint: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> ThawKycAccount<'info> {
    pub fn thaw_after_kyc(&mut self) -> Result<()> {
        let accounts = ThawAccount {
            account: self.token_account.to_account_info(),
            mint: self.mint.to_account_info(),
            authority: self.freeze_authority.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.key(), accounts);

        thaw_account(cpi_ctx)
    }
}
