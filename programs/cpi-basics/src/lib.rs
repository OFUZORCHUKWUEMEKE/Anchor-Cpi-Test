use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("HpKJxxMy5NAiSbJ1q6qBCYBtftJ4ZG76ZbC9DFmZMk2X");

#[program]
pub mod cpi_basics {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.accounts.bob_data_account.key());
        Ok(())
    }

    pub fn add_and_store(ctx:Context<BobAddOp>,a:u64,b:u64)->Result<()>{
        let result = a + b;
        ctx.accounts.bob_data_account.result = result;
        Ok(())
    }


}

#[account]
pub struct BobData{
    pub result:u64,
}

#[derive(Accounts)]
pub struct BobAddOp<'info>{
    #[account(mut)]
    pub bob_data_account:Account<'info,BobData>
}

#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(init,payer=signer, space=size_of::<BobData>() + 8)]
    pub bob_data_account:Account<'info,BobData>,

    #[account(mut)]
    pub signer:Signer<'info>,

    #[account(mut)]
    pub system_program: Program<'info, System>
}

