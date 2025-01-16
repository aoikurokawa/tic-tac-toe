use anchor_lang::prelude::*;

declare_id!("5trraE6UJC9m6TKDRQQkXoC3VaFYGYnzKeTwyfXXjho7");

#[program]
pub mod hello_world_ncn {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
