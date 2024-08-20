use anchor_lang::{prelude::*, solana_program::program};
use anchor_spl::{token::spl_token, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{
    check_idendity_not_recovered, error::{IdendityError, TransferError, TwoAuthError}, two_auth, wrapper_account, IdAccount, Issuer, TwoAuth, TwoAuthParameters, WrappedTokenAccount, WrapperAccount
};

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut, token::authority = wrapper_account, seeds=[b"wrapped_token", wrapper_account.key().as_ref(), mint.key().as_ref(), source_owner.key().as_ref()], bump)]
    pub source_wrapped_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub source_owner: Signer<'info>,
    #[account(seeds = [b"identity", source_owner.key().as_ref()], bump= idendity_sender.bump)]
    pub idendity_sender: Box<Account<'info, IdAccount>>,
    #[account(mut, seeds=[b"two_auth", wrapper_account.key().as_ref(), source_owner.key().as_ref()], bump = two_auth.bump)]
    pub two_auth: Account<'info,TwoAuth>,
    #[account(init_if_needed, token::mint = mint, token::authority = wrapper_account, seeds=[b"wrapped_token", wrapper_account.key().as_ref(), mint.key().as_ref(), destination_owner.key().as_ref()], bump, payer = source_owner)]
    pub destination_wrapped_account: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: The owner of the destination account
    pub destination_owner: AccountInfo<'info>,
    #[account(seeds = [b"identity", destination_owner.key().as_ref()], bump)]
    pub idendity_receiver: Box<Account<'info, IdAccount>>,
    pub two_auth_signer: Option<Signer<'info>>,
    #[account(seeds=[b"wrapper", approver.key().as_ref()], bump = wrapper_account.bump)]
    pub wrapper_account: Account<'info, WrapperAccount>,
    /// CHECK: The approver of the wrapper
    pub approver: UncheckedAccount<'info>,
    #[account(mint::token_program = token_program)]
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program : Program<'info, System>
}

pub fn _transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
    let source = &mut ctx.accounts.source_wrapped_account;
    let destination = &mut ctx.accounts.destination_wrapped_account;

    let self_transfer = source.key() == destination.key();

    check_idendity_not_recovered(&ctx.accounts.idendity_sender)?;
    if !self_transfer{
        check_idendity_not_recovered(&ctx.accounts.idendity_receiver)?;
    }
    let _two_auth = &mut ctx.accounts.two_auth;
    let two_auth = &mut _two_auth.two_auth;
    let two_auth_signer = &ctx.accounts.two_auth_signer;

    let current_time = Clock::get()?.unix_timestamp;

    if !self_transfer{
        check_two_auth(two_auth, two_auth_signer, amount, current_time, ctx.accounts.idendity_receiver.key())?;
    }
    let sender_issuers = &ctx.accounts.idendity_sender.issuers;
    let receiver_issuers = &ctx.accounts.idendity_receiver.issuers;
    let allowed_issuers =  &ctx.accounts.wrapper_account.id_issuers;


    check_idendities(sender_issuers, allowed_issuers,current_time)?;
    if !self_transfer{
        check_idendities(receiver_issuers, allowed_issuers,current_time)?;
    }


    _two_auth.last_tx = current_time;

    if self_transfer{ // Otherwise the source and destination are treated as different entities which leads to different amount
        return Ok(());
    }


    let approver = ctx.accounts.approver.key();
    let bump = ctx.accounts.wrapper_account.bump;
    let seed: &[&[&[u8]]]  = &[&[b"wrapper", approver.as_ref(), &[bump]]];

    // CPI to transfer tokens from source_user to destination_user
    let ix = spl_token::instruction::transfer(
        ctx.accounts.token_program.key,
        &ctx.accounts.source_wrapped_account.key(),
        &ctx.accounts.destination_wrapped_account.key(),
        &ctx.accounts.wrapper_account.key(),
        &[&ctx.accounts.wrapper_account.key()],
        amount,
    )?;
    program::invoke_signed(
        &ix,
        &[
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.source_wrapped_account.to_account_info(),
            ctx.accounts.destination_wrapped_account.to_account_info(),
            ctx.accounts.wrapper_account.to_account_info(),
        ],
        seed
    )?;

    Ok(())
}


/*
Check that at least one of the idendity issuer is active and not expired and among the allowed issuers
*/
#[inline(always)]
pub fn check_idendities(user_issuers: &Vec<Issuer>, allowed_issuers: &Vec<Pubkey>, current_time: i64) -> Result<()> {
    for issuer in user_issuers{
        if allowed_issuers.contains(&issuer.key) && issuer.active && issuer.expires_at > current_time {
            return Ok(()); // At least one of the issuer is valid
        }
    }

    return Err(IdendityError::InvalidIdendity.into());
}


#[inline(always)]
pub fn check_two_auth(two_auth: &mut Option<TwoAuthParameters>, two_auth_signer: &Option<Signer>, amount: u64, current_time: i64, receiver: Pubkey) -> Result<()> {
    if two_auth.is_some() {
        let two_auth_parameters = two_auth.as_mut().unwrap();
        let functions  = &mut two_auth_parameters.functions;

        match two_auth_signer {
            Some(signer) => {
                if two_auth_parameters.two_auth_entity != signer.key() {
                    return Err(TwoAuthError::WrongApproval.into());
                } 
                // if we have a proper two auth signature, no need to check if the two auth is needed 
            }
            None => {if two_auth::apply_two_auth_functions(amount, functions, current_time, receiver) {
                        return Err(TwoAuthError::NeedTwoAuthApproval.into());
                        }
                        return Ok(()); // No need for two auth
                    }
        }
    }
    Ok(())
}