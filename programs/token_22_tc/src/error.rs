use anchor_lang::prelude::*;

#[error_code]
pub enum MintError {
    #[msg("mint carries an extension this program has not been written to handle")]
    UnsupportedExtension,
    #[msg("Maths overfow")]
    MathsOverflow,
}
