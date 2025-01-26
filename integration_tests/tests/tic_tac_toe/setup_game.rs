#[cfg(test)]
mod tests {
    use solana_program::instruction::InstructionError;
    use solana_sdk::signature::{Keypair, Signer};

    use crate::fixtures::{
        assert_ix_error, restaking_client::NcnRoot, test_builder::TestBuilder,
        tip_router_client::assert_tip_router_error, TestResult,
    };

    #[tokio::test]
    async fn test_setup_game_ok() -> TestResult<()> {
        let mut fixture = TestBuilder::new().await;
        let mut tip_router_client = fixture.tip_router_client();
        let ncn_root = fixture.setup_ncn().await?;
        tip_router_client
            .do_initialize_config(ncn_root.ncn_pubkey, &ncn_root.ncn_admin)
            .await?;
        Ok(())
    }
}
