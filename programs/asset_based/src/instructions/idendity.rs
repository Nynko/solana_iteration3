use anchor_lang::prelude::*;

use crate::{error::IdendityError, IdAccount, Issuer};

#[derive(Accounts)]
pub struct InitializeId<'info> {
    pub issuer: Signer<'info>,
    #[account(init, seeds = [b"identity", owner.key().as_ref()], bump, payer = payer, space = IdAccount::INIT_LEN)]
    pub idendity: Account<'info, IdAccount>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddIssuer<'info> {
    #[account(mut)]
    pub issuer: Signer<'info>,
    #[account(mut,  seeds = [b"identity", owner.key().as_ref()], bump, realloc = idendity.get_add_issuer_len(), realloc::payer = owner , realloc::zero = false)]
    pub idendity: Account<'info, IdAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}


// DELETE ? 


pub fn _initialize_id(ctx: Context<InitializeId>, id_validity_duration: i64) -> Result<()> {
    let clock = Clock::get()?;
    let idendity = &mut ctx.accounts.idendity;
    idendity.owner = ctx.accounts.owner.key().clone();

    let issuer = Issuer {
        key: ctx.accounts.issuer.key().clone(),
        last_modified: clock.unix_timestamp,
        expires_at: clock.unix_timestamp + id_validity_duration,
        active: true,
    };
    idendity.issuers = vec![issuer];
    idendity.bump = ctx.bumps.idendity;
    idendity.associated_pseudo = None;
    Ok(())
}

pub fn _add_issuer_to_id(ctx: Context<AddIssuer>, id_validity_duration: i64) -> Result<()> {
    // Check if the idendity has been recovered
    if ctx.accounts.idendity.recovered_address.is_some() {
        return Err(IdendityError::IdendityRecovered.into());
    }

    // Check if the issuer is in the list of authorized issuers
    let issuers = &mut ctx.accounts.idendity.issuers;
    if issuers.iter().any(|i| i.key == ctx.accounts.issuer.key()) {
        return Err(IdendityError::IdendityAlreadyExists.into());
    }
    let current_timestamp = Clock::get()?.unix_timestamp;
    let new_issuer = Issuer {
        key: ctx.accounts.issuer.key().clone(),
        last_modified: current_timestamp,
        expires_at: current_timestamp + id_validity_duration,
        active: true,
    };
    issuers.push(new_issuer);

    Ok(())
}




#[inline(always)]
pub fn check_idendity_not_recovered(idendity: &IdAccount) -> Result<()> {
    if idendity.recovered_address.is_some(){
        return Err(IdendityError::IdendityRecovered.into());
    }
    Ok(())
}
