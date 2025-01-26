use solana_program_test::BanksClient;
use solana_sdk::{
    commitment_config::CommitmentLevel, native_token::sol_to_lamports, pubkey::Pubkey,
    signature::Keypair, signer::Signer, system_instruction::transfer, transaction::Transaction,
};
use tic_tac_toe::state::game::Game;

use super::TestResult;

pub struct TicTacToeProgramClient {
    banks_client: BanksClient,
    payer: Keypair,
}

impl TicTacToeProgramClient {
    pub const fn new(banks_client: BanksClient, payer: Keypair) -> Self {
        Self {
            banks_client,
            payer,
        }
    }

    pub async fn do_setup_game(&mut self) -> TestResult<Keypair> {
        let game_pubkey = Game::find_program_address(&tic_tac_toe::id()).0;
        let signer1 = Keypair::new();

        self.airdrop(&signer1.pubkey(), 1.0).await?;

        Ok(signer1)
    }

    pub async fn setup_game(&mut self, game: &Pubkey, config_admin: &Keypair) -> TestResult<()> {
        let blockhash = self.banks_client.get_latest_blockhash().await?;
        self.process_transaction(&Transaction::new_signed_with_payer(
            &[initialize_config(
                &jito_restaking_program::id(),
                config,
                &config_admin.pubkey(),
                &jito_vault_program::id(),
            )],
            Some(&config_admin.pubkey()),
            &[config_admin],
            blockhash,
        ))
        .await
    }

    pub async fn airdrop(&mut self, to: &Pubkey, sol: f64) -> TestResult<()> {
        let blockhash = self.banks_client.get_latest_blockhash().await?;
        self.banks_client
            .process_transaction_with_preflight_and_commitment(
                Transaction::new_signed_with_payer(
                    &[transfer(&self.payer.pubkey(), to, sol_to_lamports(sol))],
                    Some(&self.payer.pubkey()),
                    &[&self.payer],
                    blockhash,
                ),
                CommitmentLevel::Processed,
            )
            .await?;
        Ok(())
    }
}
