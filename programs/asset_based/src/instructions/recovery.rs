use anchor_lang::{prelude::*, solana_program::program};
use anchor_spl::{token::{spl_token, Token}, token_interface::{Mint, TokenAccount}};

use crate::{error::RecoveryError, IdAccount, RecoveryAuthorities, RecoveryAuthority, TwoAuth, WrapperAccount};


#[derive(Accounts)]
#[instruction(recovery_delegates : Vec<RecoveryAuthority>)]
pub struct InitializeRecovery<'info> {
    #[account(init, seeds = [b"recovery", owner.key().as_ref()], bump, payer = payer, space = RecoveryAuthorities::get_init_len(&recovery_delegates))]
    pub recovery_authority: Account<'info, RecoveryAuthorities>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(seeds=[b"wrapper", approver.key().as_ref()], bump)]
    pub wrapper_account: Account<'info, WrapperAccount>,
    /// CHECK: The approver of the wrapper
    pub approver: UncheckedAccount<'info>,
    #[account(mut, token::mint = mint, token::authority = wrapper_account, seeds=[b"wrapped_token", wrapper_account.key().as_ref(), mint.key().as_ref(), owner.key().as_ref()], bump)]
    pub user_wrapped_token_account: InterfaceAccount<'info, TokenAccount>,
    pub owner: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
}

// #[derive(Accounts)]
// #[instruction(recovery_delegates : Vec<RecoveryAuthority>)]
// pub struct UpdateRecovery<'info> {
//     #[account(mut, seeds = [b"recovery", owner.key().as_ref()], bump, realloc = RecoveryAuthorities::get_init_len(&recovery_delegates), realloc::payer=payer, realloc::zero=true)]
//     pub recovery_authority: Account<'info, RecoveryAuthorities>,
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     #[account(seeds=[b"wrapper", approver.key().as_ref()], bump)]
//     pub wrapper_account: Account<'info, WrapperAccount>,
//     /// CHECK: The approver of the wrapper
//     pub approver: UncheckedAccount<'info>,
//     #[account(mut, token::mint = mint, token::authority = wrapper_account, seeds=[b"wrapped_token", wrapper_account.key().as_ref(), mint.key().as_ref(), owner.key().as_ref()], bump)]
//     pub user_wrapped_token_account: InterfaceAccount<'info, TokenAccount>,
//     pub owner: Signer<'info>,
//     pub two_auth_entity: Option<Signer<'info>>,
//     #[account(mut, seeds=[b"two_auth", wrapper_account.key().as_ref(), owner.key().as_ref()], bump)]
//     pub two_auth: Account<'info,TwoAuth>,
//     pub mint: InterfaceAccount<'info, Mint>,
//     pub system_program: Program<'info, System>,
// }


#[derive(Accounts)]
pub struct RecoverAccount<'info> {
    #[account(seeds = [b"recovery", owner.key().as_ref()], bump)]
    pub recovery_authority: Account<'info, RecoveryAuthorities>,
    #[account(seeds=[b"wrapper", approver.key().as_ref()], bump)]
    pub wrapper_account: Account<'info, WrapperAccount>,
    /// CHECK: The approver of the wrapper
    pub approver: UncheckedAccount<'info>,
    #[account(mut, token::mint = mint, token::authority = wrapper_account, seeds=[b"wrapped_token", wrapper_account.key().as_ref(), mint.key().as_ref(), owner.key().as_ref()], bump)]
    pub user_wrapped_token_account: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: Account to recover from
    pub owner: AccountInfo<'info>,
    #[account(init_if_needed, token::mint = mint, token::authority = wrapper_account, seeds=[b"wrapped_token", wrapper_account.key().as_ref(), mint.key().as_ref(), main_recovery_authority.key().as_ref()], bump, payer= main_recovery_authority)]
    pub recover_authority_wrapped_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub main_recovery_authority: Signer<'info>,
    #[account(seeds=[b"two_auth", wrapper_account.key().as_ref(), owner.key().as_ref()], bump)]
    pub two_auth: Account<'info,TwoAuth>,
    #[account(seeds = [b"identity", owner.key().as_ref()], bump)]
    pub idendity: Account<'info, IdAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}


pub fn _initialize_recovery(
    ctx: Context<InitializeRecovery>,
    recovery_delegates: Vec<RecoveryAuthority>,
) -> Result<()> {
    let recovery_authority = &mut ctx.accounts.recovery_authority;
    recovery_authority.authorities = recovery_delegates;

    Ok(())
}



pub fn _recover_account(ctx: Context<RecoverAccount>) -> Result<()> {
    let recovery_authority = &ctx.accounts.recovery_authority;
    let main_recovery_authority = &ctx.accounts.main_recovery_authority;
    let main_recovery = recovery_authority.authorities.iter().find(|recovery| recovery.authority == main_recovery_authority.key());

    if main_recovery.is_none() {
        return Err(RecoveryError::WrongMainRecoveryAuthority.into())
    }

    let main_recovery = main_recovery.unwrap();

    let last_tx = &ctx.accounts.two_auth.last_tx;
    let timestamp = Clock::get()?.unix_timestamp;
    if *last_tx + (main_recovery.min_duration as i64 ) < timestamp {
        return Err(RecoveryError::RecoveryTimeNotPassed.into());
    }

    if main_recovery.min_signatures > 1 {
        let mut number_of_signatures = 1;

        let signers: Vec<_> = ctx
        .remaining_accounts
        .iter()
        .filter(|account| account.is_signer && account.key != main_recovery_authority.key)
        .collect();

        for authority in recovery_authority.authorities.iter().filter(
            | authority| 
            authority.min_signatures <= main_recovery.min_signatures // Only the authorities that don't need more signatures
            && *last_tx + (authority.min_duration as i64)  < timestamp // Only those that can be use for this duration
        ) {
            if signers.iter().any(|signer| signer.key == &authority.authority) {
                number_of_signatures += 1
            }
            if number_of_signatures >= main_recovery.min_signatures {
                break;
            }
        }
    
        if number_of_signatures < main_recovery.min_signatures {
            return Err(RecoveryError::NotEnoughSignatures.into());
        }
    }

    let approver = ctx.accounts.approver.key();
    let bump = ctx.bumps.wrapper_account;
    let seed: &[&[&[u8]]]  = &[&[b"wrapper", approver.as_ref(), &[bump]]];

    // CPI to transfer tokens from user to recover_authority_wrapped_token_account
    let ix = spl_token::instruction::transfer(
        ctx.accounts.token_program.key,
        &ctx.accounts.user_wrapped_token_account.key(),
        &ctx.accounts.recover_authority_wrapped_token_account.key(),
        &ctx.accounts.wrapper_account.key(),
        &[&ctx.accounts.wrapper_account.key()],
        ctx.accounts.user_wrapped_token_account.amount,
    )?;
    program::invoke_signed(
        &ix,
        &[
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.user_wrapped_token_account.to_account_info(),
            ctx.accounts.recover_authority_wrapped_token_account.to_account_info(),
            ctx.accounts.wrapper_account.to_account_info(),
        ],
        seed
    )?;

    // Close account
    let ix = spl_token::instruction::close_account(
        ctx.accounts.token_program.key,
        &ctx.accounts.user_wrapped_token_account.key(),
        &ctx.accounts.main_recovery_authority.key(),
        &ctx.accounts.wrapper_account.key(),
        &[&ctx.accounts.wrapper_account.key()],
    )?;

    program::invoke_signed(
        &ix,
        &[
            ctx.accounts.user_wrapped_token_account.to_account_info(),
            ctx.accounts.main_recovery_authority.to_account_info(),
            ctx.accounts.wrapper_account.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
        ],
        seed,
    )?;

    Ok(())
}
