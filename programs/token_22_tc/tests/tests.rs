use {litesvm::LiteSVM, solana_keypair::Keypair, solana_signer::Signer};

#[test]
fn test_initialize() {
    let program_id = token_22_tc::id();
    let payer = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!(concat!(
        env!("CARGO_TARGET_TMPDIR"),
        "/../deploy/token_22_tc.so"
    ));
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();
}
