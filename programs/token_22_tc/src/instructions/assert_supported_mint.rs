use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::spl_token_2022::{
        extension::{
            transfer_fee::TransferFeeConfig, BaseStateWithExtensions, StateWithExtensions,
        },
        state::Mint as MintState,
    },
    token_interface::{transfer_checked_with_fee, TokenInterface, TransferCheckedWithFee},
};

use crate::{error::MintError, SUPPORTED_EXTENSIONS};

#[derive(Accounts)]
pub struct AssertSupportedMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: Validated by Token-2022 CPI during transfer
    pub sender: UncheckedAccount<'info>,
    /// CHECK: Validated by Token-2022 CPI during transfer
    pub recipient: UncheckedAccount<'info>,
    /// CHECK: ownership enforced below, contents allowlisted in the handler.
    #[account(
        owner = token_program.key()
    )]
    pub mint: UncheckedAccount<'info>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> AssertSupportedMint<'info> {
    pub fn assert_supported_mint(&mut self, amount: u64, decimal: u8) -> Result<()> {
        let account_info = self.mint.to_account_info();
        let data = account_info.try_borrow_data()?;
        let state = StateWithExtensions::<MintState>::unpack(&data)?;

        for extension in state.get_extension_types()? {
            require!(
                SUPPORTED_EXTENSIONS.contains(&extension),
                MintError::UnsupportedExtension
            );
        }

        let fee = match state.get_extension::<TransferFeeConfig>() {
            Ok(config) => config
                .calculate_epoch_fee(Clock::get()?.epoch, amount)
                .ok_or(MintError::MathsOverflow)?,
            Err(_) => 0,
        };

        let accounts = TransferCheckedWithFee {
            token_program_id: self.token_program.to_account_info(),
            source: self.sender.to_account_info(),
            destination: self.recipient.to_account_info(),
            mint: self.mint.to_account_info(),
            authority: self.payer.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.key(), accounts);

        transfer_checked_with_fee(cpi_ctx, amount, decimal, fee)
    }
}
