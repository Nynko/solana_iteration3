use anchor_lang::{prelude::*, solana_program::instruction};

use crate::{error::IdendityError, idendity_account, IdAccount, Issuer, PseudoAccount, WrapperAccount};

#[derive(Accounts)]
pub struct InitializeId<'info> {
    #[account(seeds=[b"wrapper", approver.key().as_ref()], bump)]
    pub wrapper_account: Account<'info, WrapperAccount>,
    /// CHECK: The approver of the wrapper
    pub approver: UncheckedAccount<'info>,
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
    #[account(seeds=[b"wrapper", approver.key().as_ref()], bump)]
    pub wrapper_account: Account<'info, WrapperAccount>,
    /// CHECK: The approver of the wrapper
    pub approver: UncheckedAccount<'info>,
    #[account(mut,  seeds = [b"identity", owner.key().as_ref()], bump, realloc = idendity.get_add_issuer_len(), realloc::payer = owner , realloc::zero = false)]
    pub idendity: Account<'info, IdAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(_pseudo: String)]
pub struct AddPseudo<'info> {
    #[account(mut, seeds = [b"identity", owner.key().as_ref()], bump = idendity.bump)]
    pub idendity: Account<'info, IdAccount>,
    #[account(init, seeds = [b"pseudo", _pseudo.as_bytes()], bump, payer=owner, space = 8 + 2)]
    pub pseudo_account: Account<'info, PseudoAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_pseudo: String)]
pub struct UpdatePseudo<'info> {
    #[account(mut, seeds = [b"identity", owner.key().as_ref()], bump = idendity.bump)]
    pub idendity: Account<'info, IdAccount>,
    #[account(init, seeds = [b"pseudo", _pseudo.as_bytes()], bump, payer=owner, space = 2)]
    pub new_pseudo_account: Account<'info, PseudoAccount>,
    #[account(mut, close= owner, constraint = old_pseudo_account.key() == idendity.associated_pseudo.ok_or(IdendityError::PseudoDontExist)?.key())]
    pub old_pseudo_account: Account<'info, PseudoAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}


pub fn _initialize_id(ctx: Context<InitializeId>, id_validity_duration: i64) -> Result<()> {
    // Check if the issuer has been approved
    let issuer = &ctx.accounts.issuer;
    let wrapper_account = &ctx.accounts.wrapper_account;

    if !wrapper_account.id_issuers.contains(issuer.key) {
        return Err(IdendityError::IssuerNotApproved.into());
    }

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
    // Check if the issuer has been approved
    let issuer = &ctx.accounts.issuer;
    let wrapper_account = &ctx.accounts.wrapper_account;

    if !wrapper_account.id_issuers.contains(issuer.key) {
        return Err(IdendityError::IssuerNotApproved.into());
    }

    // Check if the idendity has been recovered
    if ctx.accounts.idendity.recovered_address.is_some() {
        return Err(IdendityError::IdendityRecovered.into());
    }

    // Check if the issuer is in the list of authorized issuers or if they have a signature or smth like that
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


pub fn _add_pseudo(ctx: Context<AddPseudo>, _pseudo: String) -> Result<()>{
    let idendity_account = &mut ctx.accounts.idendity;
    let pseudo_account = &mut ctx.accounts.pseudo_account;
    if idendity_account.associated_pseudo.is_some(){
        return Err(IdendityError::PseudoAlreadyExist.into());
    } else {
        idendity_account.associated_pseudo = Some(pseudo_account.key());
        pseudo_account.initialized = true;
        pseudo_account.bump = ctx.bumps.pseudo_account;
        Ok(())
    }
}

pub fn _update_pseudo(ctx: Context<UpdatePseudo>, _pseudo: String) -> Result<()>{
    let idendity_account = &mut ctx.accounts.idendity;
    idendity_account.associated_pseudo =  Some(ctx.accounts.new_pseudo_account.key());
    Ok(())
}