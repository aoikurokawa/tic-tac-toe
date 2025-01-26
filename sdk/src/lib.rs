use borsh::BorshSerialize;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

pub fn setup_game(
    program_id: Pubkey,
    game: Pubkey,
    player_one: Pubkey,
    player_two: Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(game, false),
        AccountMeta::new(player_one, true),
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    Instruction {
        program_id,
        accounts,
        data: tic_tac_toe::instruction::SetupGame { player_two }
            .try_to_vec()
            .unwrap(),
    }
}
