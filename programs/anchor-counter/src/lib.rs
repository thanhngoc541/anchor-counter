use anchor_lang::prelude::*;

declare_id!("GhLApBGfMg7yYNa9WY2iK4CnkwFck1eTujbezy4JeB5Q");

#[program]
pub mod anchor_counter {
    use super::*;
    pub const E: f64 = 2.71828182845904523536028747135266250_f64;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0.0;
        msg!("Counter Account Created");
        msg!("Current Count: { }", counter.count);
        Ok(())
    }
    pub fn formulation1(ctx: Context<Update>, n: f64, m: f64, x: f64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        let mut y: f64 = n * x as f64 / m + 1.0;
        y = y.ln() * (1.0 / n);
        counter.count = y;
        Ok(())
    }

    pub fn formulation2(ctx: Context<Update>, n: f64, m: f64, x: f64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        let y: f64 = (m / n) * (E.powf(n * x) - 1.0);
        counter.count = y;
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: f64,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}

const DISCRIMINATOR: usize = 8;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,
        payer = user,
        space = DISCRIMINATOR + Counter::INIT_SPACE
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
