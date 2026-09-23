use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::spl_token_2022::{
        extension::{
            transfer_fee::TransferFeeConfig, BaseStateWithExtensions, StateWithExtensions,
        },
        state::Mint as MintState,
    },
    token_interface::{transfer_checked_with_fee, Mint, TokenInterface, TransferCheckedWithFee},
};

use crate::error::MintError;

#[derive(Accounts)]
pub struct PermanentDelegateSeize<'info> {
    #[account(mut)]
    pub permanent_delegate: Signer<'info>,
    #[account(owner = token_program.key())]
    pub mint: InterfaceAccount<'info, Mint>,
    /// CHECK: Validated by Token-2022 CPI during transfer.
    #[account(mut, owner = token_program.key())]
    pub source: UncheckedAccount<'info>,
    /// CHECK: Validated by Token-2022 CPI during transfer.
    #[account(mut, owner = token_program.key())]
    pub destination: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> PermanentDelegateSeize<'info> {
    pub fn seize_tokens(&mut self, amount: u64, decimals: u8) -> Result<()> {
        let mint_info = self.mint.to_account_info();
        let mint_data = mint_info.try_borrow_data()?;
        let mint_state = StateWithExtensions::<MintState>::unpack(&mint_data)?;
        let fee = mint_state
            .get_extension::<TransferFeeConfig>()?
            .calculate_epoch_fee(Clock::get()?.epoch, amount)
            .ok_or(MintError::MathsOverflow)?;

        let accounts = TransferCheckedWithFee {
            token_program_id: self.token_program.to_account_info(),
            source: self.source.to_account_info(),
            destination: self.destination.to_account_info(),
            mint: self.mint.to_account_info(),
            authority: self.permanent_delegate.to_account_info(),
        };

        let ctx = CpiContext::new(self.token_program.key(), accounts);

        transfer_checked_with_fee(ctx, amount, decimals, fee)
    }
}
