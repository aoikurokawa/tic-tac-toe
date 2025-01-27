use solana_program_test::BanksClient;
use solana_sdk::{
    commitment_config::CommitmentLevel, native_token::sol_to_lamports, pubkey::Pubkey,
    signature::Keypair, signer::Signer, system_instruction::transfer, transaction::Transaction,
};
use tic_tac_toe::state::game::Game;
use tic_tac_toe_sdk::setup_game;

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

    pub async fn do_setup_game(&mut self, player_two: Pubkey) -> TestResult<()> {
        let game_pubkey = Game::find_program_address(&tic_tac_toe::id()).0;

        self.airdrop(&self.payer.pubkey(), 1.0).await?;

        self.setup_game(game_pubkey, player_two).await?;

        Ok(())
    }

    pub async fn setup_game(&mut self, game: Pubkey, player_two: Pubkey) -> TestResult<()> {
        let blockhash = self.banks_client.get_latest_blockhash().await?;
        self.process_transaction(&Transaction::new_signed_with_payer(
            &[setup_game(
                tic_tac_toe::id(),
                game,
                self.payer.pubkey(),
                player_two,
            )],
            Some(&self.payer.pubkey()),
            &[&self.payer],
            blockhash,
        ))
        .await
    }

    pub async fn process_transaction(&mut self, tx: &Transaction) -> TestResult<()> {
        self.banks_client
            .process_transaction_with_preflight_and_commitment(
                tx.clone(),
                CommitmentLevel::Processed,
            )
            .await?;
        Ok(())
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
