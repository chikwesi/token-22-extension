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
                confidential_transfer::instruction as confidential_instruction, ExtensionType,
            },
            state::{AccountState, Mint as MintState},
        },
        InitializeMint2,
    },
    token_2022_extensions::{
        default_account_state::{default_account_state_initialize, DefaultAccountStateInitialize},
        metadata_pointer::{metadata_pointer_initialize, MetadataPointerInitialize},
        permanent_delegate::{permanent_delegate_initialize, PermanentDelegateInitialize},
    },
    token_interface::{
        mint_close_authority_initialize, transfer_fee_initialize, MintCloseAuthorityInitialize,
        TokenInterface, TransferFeeInitialize,
    },
};

#[derive(Accounts)]
pub struct CreateSeizableMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: created and initialized in this handler.
    #[account(mut, signer)]
    pub mint: UncheckedAccount<'info>,
    /// CHECK: stored by Token-2022 as the permanent delegate.
    pub permanent_delegate: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateSeizableMint<'info> {
    pub fn create_seizable_mint(
        &mut self,
        decimals: u8,
        basis_points: u16,
        maximum_fee: u64,
    ) -> Result<()> {
        let extensions = [
            ExtensionType::MintCloseAuthority,
            ExtensionType::MetadataPointer,
            ExtensionType::DefaultAccountState,
            ExtensionType::TransferFeeConfig,
            ExtensionType::PermanentDelegate,
            ExtensionType::ConfidentialTransferMint,
        ];

        let space = ExtensionType::try_calculate_account_len::<MintState>(&extensions)?;
        let lamports = Rent::get()?.minimum_balance(space);

        let create_mint_account = CreateAccount {
            from: self.payer.to_account_info(),
            to: self.mint.to_account_info(),
        };
        let create_mint_ctx = CpiContext::new(self.system_program.key(), create_mint_account);
        create_account(
            create_mint_ctx,
            lamports,
            space as u64,
            &self.token_program.key(),
        )?;

        let mint_close_auth_acct = MintCloseAuthorityInitialize {
            token_program_id: self.token_program.to_account_info(),
            mint: self.mint.to_account_info(),
        };
        let mint_close_auth_ctx = CpiContext::new(self.token_program.key(), mint_close_auth_acct);
        mint_close_authority_initialize(mint_close_auth_ctx, Some(&self.payer.key()))?;

        let metadata_pointer_acct = MetadataPointerInitialize {
            token_program_id: self.token_program.to_account_info(),
            mint: self.mint.to_account_info(),
        };
        let metadata_pointer_ctx = CpiContext::new(self.token_program.key(), metadata_pointer_acct);
        metadata_pointer_initialize(
            metadata_pointer_ctx,
            Some(self.payer.key()),
            Some(self.mint.key()),
        )?;

        let default_account_state_acct = DefaultAccountStateInitialize {
            token_program_id: self.token_program.to_account_info(),
            mint: self.mint.to_account_info(),
        };
        let default_account_state_ctx =
            CpiContext::new(self.token_program.key(), default_account_state_acct);
        default_account_state_initialize(default_account_state_ctx, &AccountState::Frozen)?;

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

        let permanent_delegate_acct = PermanentDelegateInitialize {
            token_program_id: self.token_program.to_account_info(),
            mint: self.mint.to_account_info(),
        };
        let permanent_delegate_ctx =
            CpiContext::new(self.token_program.key(), permanent_delegate_acct);
        permanent_delegate_initialize(permanent_delegate_ctx, &self.permanent_delegate.key())?;

        let mint_info = self.mint.to_account_info();
        let token_program_info = self.token_program.to_account_info();
        invoke(
            &confidential_instruction::initialize_mint(
                &self.token_program.key(),
                &self.mint.key(),
                Some(self.payer.key()),
                false,
                None,
            )?,
            &[mint_info.clone(), token_program_info],
        )?;

        let init_mint_ctx = CpiContext::new(
            self.token_program.key(),
            InitializeMint2 { mint: mint_info },
        );
        initialize_mint2(
            init_mint_ctx,
            decimals,
            &self.payer.key(),
            Some(&self.payer.key()),
        )
    }
}
