#[cfg(test)]
mod tests {
    use solana_sdk::pubkey::Pubkey;

    use crate::fixtures::fixture::TestBuilder;

    #[tokio::test]
    async fn test_setup_game_ok() {
        let fixture = TestBuilder::new().await;
        let mut tic_tac_toe_clinet = fixture.tic_tac_toe_client();

        let player_two = Pubkey::new_unique();

        tic_tac_toe_clinet.do_setup_game(player_two).await.unwrap();
    }
}
