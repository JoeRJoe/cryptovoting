use std::time::SystemTime;

use anchor_lang::prelude::*;

declare_id!("EsQM7kVd85DLBrsZSVfgjdbFM6w5VUkqbu2w5Kt2ezto");

#[program]
pub mod smart_contract {
    use std::time::UNIX_EPOCH;

    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    pub fn create_votazione(context: Context<CreateVotazione>, nome: String, descrizione: String, data_scadenza: u64) -> ProgramResult {
        let votazione = &mut context.accounts.votazione;
        votazione.nome = nome;
        votazione.descrizione = descrizione;
        votazione.data_inizio = SystemTime::now().duration_since(UNIX_EPOCH).expect("Errore nel calcolo della data").as_secs() as u64;
        votazione.data_scadenza = data_scadenza;
        votazione.owner = *context.accounts.user.key;
        Ok(())
    }

    pub fn create_crowdfunding(context: Context<CreateCrowdfunding>, votazione: Votazione, data_scadenza: u64) -> ProgramResult {
        let crowdfunding = &mut context.accounts.crowdfunding;
        crowdfunding.votazione = votazione;
        crowdfunding.data_inizio = SystemTime::now().duration_since(UNIX_EPOCH).expect("Errore nel calcolo della data").as_secs() as u64;
        crowdfunding.data_scadenza = data_scadenza;
        crowdfunding.owner = *context.accounts.user.key;
        crowdfunding.totale_donato = 0;
        Ok(())
    }

    pub fn vota(context: Context<Vota>, votazione: Votazione) -> ProgramResult {
        let voto = &mut context.accounts.voto;
        voto.votazione = votazione;
        voto.votante = *context.accounts.user.key;
        voto.data = SystemTime::now().duration_since(UNIX_EPOCH).expect("Errore nel calcolo della data").as_secs() as u64;
        Ok(())
    }

}

#[account]
pub struct Votazione {
    pub nome: String,
    pub descrizione: String,
    pub owner: Pubkey,
    pub data_inizio: u64,
    pub data_scadenza: u64,
}

#[account]
pub struct Crowdfunding {
    pub owner: Pubkey,
    pub totale_donato: u64,
    pub votazione: Votazione,
    pub data_inizio: u64,
    pub data_scadenza: u64,
}

#[account]
pub struct Voto {
    pub votazione: Votazione,
    pub votante: Pubkey,
    pub data: u64,
}

#[derive(Accounts)]
pub struct CreateVotazione<'info> {
    #[account(init, payer = user, space = 250)]
    pub votazione: Account<'info, Votazione>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateCrowdfunding<'info> {
    #[account(init, payer = user, space = 300, seeds = [votazione.key().as_ref()], bump)]
    pub crowdfunding: Account<'info, Crowdfunding>,
    pub votazione: Account<'info, Votazione>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vota<'info> {
    #[account(init, payer = user, space = 150, seeds = [votazione.key().as_ref(), user.key.as_ref()], bump)]
    pub voto: Account<'info, Voto>,
    pub votazione: Account<'info, Votazione>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}