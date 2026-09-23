use anchor_lang::prelude::*;
use anchor_spl::token_interface::{transfer_checked, Mint, TokenInterface, TransferChecked};

#[derive(Accounts)]
#[instruction(decimals: u8)]
pub struct CreateSeizableMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        mint::authority = payer,
        mint::decimals = decimals,
        mint::token_program = token_program,
        extensions::permanent_delegate::delegate = payer
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PermanentDelegateSeize<'info> {
    #[account(mut)]
    pub permanent_delegate: Signer<'info>,
    #[account(
        owner = token_program.key()
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    /// CHECK: Validated by Token-2022 CPI during transfer
    #[account(mut, owner = token_program.key())]
    pub source: UncheckedAccount<'info>,
    /// CHECK: Validated by Token-2022 CPI during transfer
    #[account(mut, owner = token_program.key())]
    pub destination: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> PermanentDelegateSeize<'info> {
    // pub fn create_seizable_mint(&self, decimals: u8) -> Result<()> {
    //     msg!(
    //         "seizable mint {} with {} deccimals, permanent delegate",
    //         self.mint.key(),
    //         decimals,
    //         // ctx.accounts.payer.key()
    //     );
    //     Ok(())
    // }

    pub fn seize_tokens(&mut self, amount: u64, decimals: u8) -> Result<()> {
        let accounts = TransferChecked {
            from: self.source.to_account_info(),
            to: self.destination.to_account_info(),
            mint: self.mint.to_account_info(),
            authority: self.permanent_delegate.to_account_info(),
        };

        let ctx = CpiContext::new(self.token_program.key(), accounts);

        transfer_checked(ctx, amount, decimals)
    }
}
