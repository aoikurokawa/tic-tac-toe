use solana_program_test::{ProgramTest, ProgramTestContext};
use solana_sdk::pubkey;

use super::tic_tac_toe_client::TicTacToeProgramClient;

pub struct TestBuilder {
    context: ProgramTestContext,
}

impl TestBuilder {
    pub async fn new() -> Self {
        // $ cargo-build-sbf && SBF_OUT_DIR=$(pwd)/target/sbf-solana-solana/release cargo nextest run
        let mut program_test = ProgramTest::default();
        program_test.prefer_bpf(true);
        program_test.add_program("tic_tac_toe", tic_tac_toe::id(), None);

        let context = program_test.start_with_context().await;
        Self { context }
    }

    pub fn tic_tac_toe_client(&self) -> TicTacToeProgramClient {
        TicTacToeProgramClient::new(
            self.context.banks_client.clone(),
            self.context.payer.insecure_clone(),
        )
    }
}
