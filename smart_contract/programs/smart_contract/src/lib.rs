use anchor_lang::prelude::*;

declare_id!("EsQM7kVd85DLBrsZSVfgjdbFM6w5VUkqbu2w5Kt2ezto");

#[program]
pub mod smart_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
