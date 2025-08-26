use anchor_lang::prelude::*;

//声明程序的唯一标识符
declare_id!("TfGPfnacq4bDuAiYvbZEYbqLHFwu9BvCa1q7Jzr5cpH");

//Anchor宏，定义智能合约模块
#[program]
pub mod my_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

//定义账户验证结构
#[derive(Accounts)]
pub struct Initialize {}
