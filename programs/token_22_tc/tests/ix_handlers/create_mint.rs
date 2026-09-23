use anchor_lang::{
    prelude::*,
    solana_program::{instruction::Instruction, system_program},
    InstructionData,
};
use anchor_spl;
use token_22_tc::{accounts, instruction, ID};

const TOKEN_2022_PROGRAM_ID: Pubkey = anchor_spl::token_interface::spl_token_2022::ID;

pub fn create_declarative_mint_ix(payer: Pubkey, mint: Pubkey, decimals: u8) -> Instruction {
    Instruction::new_with_bytes(
        ID,
        &instruction::CreateMintDeclarative { decimals }.data(),
        accounts::CreateMintDeclarative {
            payer,
            mint,
            token_program: TOKEN_2022_PROGRAM_ID,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    )
}

pub fn create_mint_with_fe_ix(
    payer: Pubkey,
    mint: Pubkey,
    decimals: u8,
    basis_points: u16,
    maximum_fee: u64,
) -> Instruction {
    Instruction::new_with_bytes(
        ID,
        &instruction::CreateMintWithFee {
            decimals,
            basis_points,
            maximum_fee,
        }
        .data(),
        accounts::CreateMintWithFee {
            payer,
            mint,
            token_program: TOKEN_2022_PROGRAM_ID,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    )
}
