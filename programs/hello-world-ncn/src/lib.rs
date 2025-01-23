use anchor_lang::prelude::*;

use instructions::{play, setup_game};

pub mod core;
pub mod instructions;

declare_id!("5trraE6UJC9m6TKDRQQkXoC3VaFYGYnzKeTwyfXXjho7");

#[program]
pub mod hello_world_ncn {

    use super::*;

    pub fn setup_game(ctx: Context<setup_game::SetupGame>, player_two: Pubkey) -> Result<()> {
        msg!("Instruction: SetupGame");
        instructions::setup_game::setup_game(ctx, player_two)
    }

    pub fn play(ctx: Context<play::Play>, tile: Tile) -> Result<()> {
        msg!("Instruction: Play");
        instructions::play::play(ctx, player_two)
    }
}
