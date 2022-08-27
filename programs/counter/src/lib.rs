
use anchor_lang::prelude::*;

declare_id!("A96kZX8MaQoMrHuuRWiE4bhV7SDwQYigDaAvaYjikzyc");

#[program]
pub mod counter {
    use anchor_lang::solana_program::entrypoint_deprecated::ProgramResult;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let count_account = &mut ctx.accounts.count;
        count_account.value = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>)-> ProgramResult{
        let count_account = &mut ctx.accounts.count;
        count_account.value +=1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {

    #[account(
        init,
        payer = person_that_pays,
        space = 8 + 32 + 16
    )]
    pub count: Account<'info, Count>,
    #[account(mut)]
    pub person_that_pays: Signer<'info>,
    pub system_program:Program<'info, System>,


}


#[derive(Accounts)]
pub struct Increment<'info>{
    #[account(mut)]
    pub count: Account<'info, Count>
}



#[account]
pub struct Count{
    pub value: u16,
}