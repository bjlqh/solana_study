use anchor_lang::prelude::*;

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

// 修正拼写错误：ACCOUT -> ACCOUNT
pub const COUNTER_ACCOUNT_SPACE: usize = ANCHOR_DISCRIMINATOR_SIZE + 8;

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
        space = COUNTER_ACCOUNT_SPACE, // 修正拼写错误
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


//test
#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_counter_size() {
        // 测试Counter结构体的大小
        assert_eq!(mem::size_of::<Counter>(), 8);
    }

    #[test]
    fn test_counter_account_space() {
        // 测试账户空间计算
        assert_eq!(COUNTER_ACCOUNT_SPACE, ANCHOR_DISCRIMINATOR_SIZE + 8);
    }

    #[test]
    fn test_counter_initialization() {
        // 测试Counter结构体的初始化
        let counter = Counter { count: 42 };
        assert_eq!(counter.count, 42);
    }
}