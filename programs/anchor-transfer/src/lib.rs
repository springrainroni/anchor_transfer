use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
declare_id!("9t2PZvmfiuXgG1osrTETKvXA37ax9CxmkgBjvzPX2KzM");

#[program]
pub mod anchor_transfer {
    use super::*;

    pub fn transfer_native_sol(ctx: Context<SolSend>) -> Result<()> {
        let lamports: u64  = 100000000;

        let sol_transfer = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.from.key,
            &ctx.accounts.to.key,
            lamports,
        );
        invoke(
            &sol_transfer,
            &[
                ctx.accounts.from.to_account_info(),
                ctx.accounts.to.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
        
        Ok(())
    }
    
}

#[derive(Accounts)]
pub struct SolSend<'info> {
    #[account(mut, signer)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub from: AccountInfo<'info>,       
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub to: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub system_program: AccountInfo<'info>,
}

