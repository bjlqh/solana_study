use anchor_lang::prelude::*;

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

pub const COUNTER_ACCOUT_SPACE: usize = ANCHOR_DISCRIMINATOR_SIZE + 8;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("4zhzFF4mhTFvbhJvFxFPFBkEHEMLiZmte5hAWWswfjME");

#[program]
mod simple_counter {
    use super::*;

    //初始化计数器
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter;
        counter_account.count = 0;
        msg!("Counter initialized to 0 at {}", counter_account.key());
        Ok(())
    }

    // 自增计数器
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter;
        counter_account.count += 1;
        msg!("Counter incremented to {}", counter_account.count);
        Ok(())
    }
}

// PDA 账户数据结构
#[account]
pub struct Counter {
    pub count: u64,
}

// initialize指令所需账户
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_if_needed,
        payer = user,
        space = COUNTER_ACCOUT_SPACE,
        seeds = [b"counter", user.key().as_ref()],
        bump
    )]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// increment指令所需账户
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(
        mut,
        seeds = [b"counter", user.key().as_ref()],
        bump
    )]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub user: Signer<'info>,
}
