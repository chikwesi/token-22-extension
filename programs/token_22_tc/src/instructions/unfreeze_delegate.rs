use anchor_lang::prelude::*;
use anchor_spl::token_interface::{approve, Approve, TokenInterface};

#[derive(Accounts)]
pub struct UnfreezeDelegate<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    /// CHECK: validated by Token-2022 during Approve.
    #[account(mut, owner = token_program.key())]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: any address may receive delegation; Token-2022 stores it as is.
    pub delegate: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> UnfreezeDelegate<'info> {
    pub fn delegate_to_program(&mut self, amount: u64) -> Result<()> {
        let accounts = Approve {
            to: self.token_account.to_account_info(),
            delegate: self.delegate.to_account_info(),
            authority: self.owner.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.key(), accounts);

        approve(cpi_ctx, amount)
    }
}
