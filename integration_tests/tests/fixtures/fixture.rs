use solana_program_test::{processor, ProgramTest, ProgramTestContext};
use solana_sdk::pubkey;
use tic_tac_toe::tic_tac_toe;

pub struct TestBuilder {
    context: ProgramTestContext,
}

impl TestBuilder {
    pub async fn new() -> Self {
        // $ cargo-build-sbf && SBF_OUT_DIR=$(pwd)/target/sbf-solana-solana/release cargo nextest run
        let mut program_test = ProgramTest::default();
        program_test.prefer_bpf(true);
        program_test.add_program(
            "tic_tac_toe",
            pubkey!("5trraE6UJC9m6TKDRQQkXoC3VaFYGYnzKeTwyfXXjho7"),
            None,
        );

        let context = program_test.start_with_context().await;
        Self { context }
    }
}
