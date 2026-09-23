use anchor_spl::token_2022::spl_token_2022::extension::ExtensionType;

pub const SUPPORTED_EXTENSIONS: &[ExtensionType] = &[
    ExtensionType::MintCloseAuthority,
    ExtensionType::MetadataPointer,
    ExtensionType::DefaultAccountState,
    ExtensionType::TransferFeeConfig,
    ExtensionType::PermanentDelegate,
    ExtensionType::ConfidentialTransferMint,
];

pub const AE_CIPHERTEXT_LEN: usize = 36;
pub const ELGAMAL_CIPHERTEXT_LEN: usize = 64;
