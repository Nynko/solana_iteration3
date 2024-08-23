use anchor_lang::prelude::*;

use crate::{error::IdendityError, IdAccount, PseudoAccount};

#[derive(Accounts)]
#[instruction(_pseudo: String)]
pub struct AddPseudo<'info> {
    #[account(mut, seeds = [b"identity", owner.key().as_ref()], bump = idendity.bump)]
    pub idendity: Account<'info, IdAccount>,
    #[account(init, seeds = [b"pseudo", _pseudo.as_bytes()], bump, payer=payer, space = PseudoAccount::LEN)]
    pub pseudo_account: Account<'info, PseudoAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_pseudo: String)]
pub struct UpdatePseudo<'info> {
    #[account(mut, seeds = [b"identity", owner.key().as_ref()], bump = idendity.bump)]
    pub idendity: Account<'info, IdAccount>,
    #[account(init, seeds = [b"pseudo", _pseudo.as_bytes()], bump, payer=owner, space = PseudoAccount::LEN)]
    pub new_pseudo_account: Account<'info, PseudoAccount>,
    #[account(mut, close= owner, constraint = old_pseudo_account.key() == idendity.associated_pseudo.ok_or(IdendityError::PseudoDontExist)?.key())]
    pub old_pseudo_account: Account<'info, PseudoAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
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