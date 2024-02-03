use std::time::SystemTime;

use anchor_lang::prelude::*;

declare_id!("EsQM7kVd85DLBrsZSVfgjdbFM6w5VUkqbu2w5Kt2ezto");

#[program]
pub mod smart_contract {
    use super::*;

}

#[account]
pub struct Votazione {
    pub nome: String,
    pub descrizione: String,
    pub crowdfunding: Option<Pubkey>,
    pub owner: Pubkey,
    pub attiva: bool,
    pub data_inizio: u64,
    pub data_scadenza: u64,
}

#[account]
pub struct Crowdfunding {
    pub owner: Pubkey,
    pub totale_donato: u64,
    pub votazione: Pubkey,
    pub attiva: bool,
    pub data_inizio: u64,
    pub data_scadenza: u64,
}

#[account]
pub struct Voto {
    pub votazione: Pubkey,
    pub votante: Pubkey,
    pub data: u64,
}