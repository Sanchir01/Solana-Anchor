use anchor_lang::prelude::*;

declare_id!("2jThcPqG33rxY4MkrBvBWCxdbePRig38bcGKnUiYXAHv");

#[program]
pub mod solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    pub fn increment(ctx: Context<Icrement>) -> Result<()>{
        let counter = &mut  ctx.accounts.counter;
        msg!("Counter incremented! Current count :{}", counter.count)
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + 8
    )]
    pub counter: Accounts<'info,Counter>,
    pub system_proram:Program<'info,System>
}

#[derive(Accounts)]
struct Increment<'info> {
    #[account(mut)]
    pub counter: Accounts<'info,Counter>
}

#[account]
pub struct Counter{
    pub count: u64
}
