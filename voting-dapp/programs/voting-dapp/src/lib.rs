use anchor_lang::prelude::*;

declare_id!("2SskyB9xzfCx7fBJoV5DW3KDhu9NYZfm5GcrhT6b9mWV");

#[program]
pub mod voting_dapp {
    use super::*;

    pub fn initialize_poll(
        ctx: Context<InitializePoll>, 
        poll_id: u64,
        
    ) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instuction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[accounts(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,  
        space = 8 + Poll::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub poll: Account<'info, Poll>,
    pub system_progran: Program<'info, System>,

}

#[account]
#[derive(InitSpace)]
pub struct Poll{
    pub poll_id: u64,
    #[max_len(300)]
    pub description: String,
    pub poll_start: u64,
    pub poll_end: u64,
    pub candiate_amount: u64,
}
