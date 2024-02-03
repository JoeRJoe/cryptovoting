use std::time::SystemTime;
use anchor_lang::prelude::*;

declare_id!("EsQM7kVd85DLBrsZSVfgjdbFM6w5VUkqbu2w5Kt2ezto");

#[program]
pub mod smart_contract {

    use std::time::UNIX_EPOCH;
    use anchor_lang::solana_program::entrypoint::ProgramResult;
    use super::*;

    pub fn create_votazione(
        context: Context<CreateVotazione>,
        nome: String,
        descrizione: String,
        data_scadenza: u64
    ) -> ProgramResult {
        let votazione = &mut context.accounts.votazione;
        votazione.nome = nome;
        votazione.descrizione = descrizione;
        votazione.data_inizio = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Errore nel calcolo della data")
            .as_secs() as u64;
        votazione.data_scadenza = data_scadenza;
        votazione.owner = *context.accounts.user.key;
        votazione.is_attiva = true;
        Ok(())
    }

    pub fn create_crowdfunding(
        context: Context<CreateCrowdfunding>,
        votazione: Votazione,
        data_scadenza: u64
    ) -> ProgramResult {
        let crowdfunding = &mut context.accounts.crowdfunding;
        crowdfunding.votazione = votazione;
        crowdfunding.data_inizio = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Errore nel calcolo della data")
            .as_secs() as u64;
        crowdfunding.data_scadenza = data_scadenza;
        crowdfunding.owner = *context.accounts.user.key;
        crowdfunding.totale_donato = 0;
        crowdfunding.is_attivo = true;
        Ok(())
    }

    pub fn vota(context: Context<Vota>, votazione: Votazione) -> ProgramResult {
        let voto = &mut context.accounts.voto;
        if !votazione.is_attiva {
            return Err(ProgramError::InvalidArgument);
        }
        voto.votazione = votazione;
        voto.votante = *context.accounts.user.key;
        voto.data = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Errore nel calcolo della data")
            .as_secs() as u64;
        Ok(())
    }

    pub fn dona(context: Context<Dona>, somma: u64) -> ProgramResult {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Errore nel calcolo della data")
            .as_secs() as u64;
        let crowdfunding = &mut context.accounts.crowdfunding;
        if crowdfunding.data_scadenza < now || !crowdfunding.is_attivo{
            return Err(ProgramError::InvalidArgument);
        }
        let instruction = anchor_lang::solana_program::system_instruction::transfer(
            &context.accounts.user.key(),
            &context.accounts.crowdfunding.key(),
            somma
        );
        let _ = anchor_lang::solana_program::program::invoke(
            &instruction,
            &[
                context.accounts.user.to_account_info(),
                context.accounts.crowdfunding.to_account_info(),
            ]
        );
        (&mut context.accounts.crowdfunding).totale_donato += somma;
        Ok(())
    }

    pub fn preleva(context: Context<Preleva>) -> ProgramResult {
        let crowdfunding = &mut context.accounts.crowdfunding;
        let user = &mut context.accounts.user;
        if crowdfunding.owner != *user.key {
            return Err(ProgramError::IncorrectProgramId);
        }
        if !crowdfunding.is_attivo {
            return Err(ProgramError::InvalidArgument);
        }
        let amount = **crowdfunding.to_account_info().lamports.borrow() - Rent::get()?.minimum_balance(crowdfunding.to_account_info().data_len());
        **crowdfunding.to_account_info().try_borrow_mut_lamports()? -= amount;
        **user.to_account_info().try_borrow_mut_lamports()? += amount;
        Ok(())
    }

    pub fn disattiva_votazione(context: Context<DisattivaVotazione>) -> ProgramResult {
        let user = &mut context.accounts.user;
        let votazione = &mut context.accounts.votazione;
        let crowdfunding = &mut context.accounts.crowdfunding;
        if votazione.owner != *user.key {
            return Err(ProgramError::IncorrectProgramId);
        }
        votazione.is_attiva = false;
        match crowdfunding {
            Some(crowdfunding) => {
                if crowdfunding.votazione != **votazione {
                    return Err(ProgramError::InvalidArgument);
                }
                crowdfunding.is_attivo = false;
            },
            None => {}
        }
        Ok(())
    }

    pub fn disattiva_crowdfunding(context: Context<DisattivaCrowdfunding>) -> ProgramResult {
        let crowdfunding = &mut context.accounts.crowdfunding;
        let user = &mut context.accounts.user;
        if crowdfunding.owner!= *user.key {
            return Err(ProgramError::IncorrectProgramId);
        }
        crowdfunding.is_attivo = false;
        Ok(())
    }
}

#[account]
#[derive(PartialEq)]
pub struct Votazione {
    pub nome: String,
    pub descrizione: String,
    pub owner: Pubkey,
    pub data_inizio: u64,
    pub data_scadenza: u64,
    pub is_attiva: bool,
}

#[account]
pub struct Crowdfunding {
    pub owner: Pubkey,
    pub totale_donato: u64,
    pub votazione: Votazione,
    pub data_inizio: u64,
    pub data_scadenza: u64,
    pub is_attivo: bool,
}

#[account]
pub struct Voto {
    pub votazione: Votazione,
    pub votante: Pubkey,
    pub data: u64,
}

#[derive(Accounts)]
#[instruction(nome: String)]
pub struct CreateVotazione<'info> {
    #[account(init, payer = user, space = 250, seeds = [nome.as_ref(), user.key.as_ref()], bump)]
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
    #[account(
        init,
        payer = user,
        space = 150,
        seeds = [votazione.key().as_ref(), user.key.as_ref()],
        bump
    )]
    pub voto: Account<'info, Voto>,
    pub votazione: Account<'info, Votazione>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Dona<'info> {
    #[account(mut)]
    pub crowdfunding: Account<'info, Crowdfunding>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Preleva<'info> {
    #[account(mut)]
    pub crowdfunding: Account<'info, Crowdfunding>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct DisattivaVotazione<'info>{
    #[account(mut)]
    pub votazione: Account<'info, Votazione>,
    #[account(mut)]
    pub crowdfunding: Option<Account<'info, Crowdfunding>>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DisattivaCrowdfunding<'info> {
    #[account(mut)]
    pub crowdfunding: Account<'info, Crowdfunding>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}