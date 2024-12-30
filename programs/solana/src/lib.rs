use anchor_lang::prelude::*;

declare_id!("2jThcPqG33rxY4MkrBvBWCxdbePRig38bcGKnUiYXAHv");

#[program]
pub mod solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.bump = ctx.bumps.counter;
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Counter incremented! Current count :{}", counter.count);
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter incremenred! Current count: {}", counter.count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        seeds = [b"solana"],
        bump,
        payer = user,
        space = 8 + Counter::INIT_SPACE
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut,
        seeds = [b"solana"],
        bump = counter.bump
    )]
    pub counter: Account<'info, Counter>,
}

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: u64,
    pub bump: u8,
}
