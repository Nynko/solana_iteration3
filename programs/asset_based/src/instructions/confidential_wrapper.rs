use anchor_lang::{prelude::*, solana_program::program};
use anchor_spl::{
    token::spl_token, token_interface::{Mint, TokenAccount, TokenInterface}
};

use crate::{error::WrapperError, WrapperAccount};

#[derive(Accounts)]
#[instruction(list_issuer: Vec<Pubkey>, exit_regulators: Vec<Pubkey>)]
pub struct InitializeWrapper<'info> {
    #[account(init, seeds=[b"wrapper", approver.key().as_ref()], bump, payer=payer, space=WrapperAccount::get_init_len(&list_issuer,&exit_regulators))]
    pub wrapper_account: Account<'info, WrapperAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub approver: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddWrapperIssuer<'info> {
    #[account(mut, seeds=[b"wrapper", approver.key().as_ref()], bump, realloc=wrapper_account.get_len_add_address(), realloc::payer=payer, realloc::zero=false)]
    pub wrapper_account: Account<'info, WrapperAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub approver: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeleteWrapperIssuer<'info> {
    #[account(mut, seeds=[b"wrapper", approver.key().as_ref()], bump, realloc=wrapper_account.get_len_remove_address(), realloc::payer=payer, realloc::zero=true)]
    pub wrapper_account: Account<'info, WrapperAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub approver: Signer<'info>,
    /// CHECK: The issuer to be removed
    #[account(constraint = wrapper_account.id_issuers.contains(&issuer.key()))]
    pub issuer: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WrapTokens<'info> {
    #[account(seeds=[b"wrapper", approver.key().as_ref()], bump)]
    pub wrapper_account: Account<'info, WrapperAccount>,
    /// CHECK: The approver of the wrapper
    pub approver: UncheckedAccount<'info>,
    #[account(init_if_needed, token::authority = wrapper_account, token::mint = mint, seeds=[b"wrapped_token", wrapper_account.key().as_ref(), mint.key().as_ref(), owner_to_account.key().as_ref()], bump, payer=owner_from_token_account)]
    pub to_wrapped_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, token::authority = owner_from_token_account, token::mint = mint)]
    pub from_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub owner_from_token_account: Signer<'info>,
    /// CHECK: owner.key() == user_wrapped_token_account.owner but need to check after init if init happens --> Maybe not necessary ? 
    pub owner_to_account: AccountInfo<'info>,
    #[account(mint::token_program = token_program)]
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct UnWrapTokens<'info> {
    #[account(seeds=[b"wrapper", approver.key().as_ref()], bump)]
    pub wrapper_account: Account<'info, WrapperAccount>,
    /// CHECK: The approver of the wrapper
    pub approver: UncheckedAccount<'info>,
    /// CHECK: That the exit regulator is in the list of the wrapper_account
    pub exit_regulator: Signer<'info>,
    #[account(mut, token::authority = wrapper_account, token::mint = mint, seeds=[b"wrapped_token", wrapper_account.key().as_ref(), mint.key().as_ref(), owner.key().as_ref()], bump)]
    pub user_wrapped_token_account: InterfaceAccount<'info, TokenAccount>, // also constraints on owner field
    #[account(mut, token::authority = owner_token_account, token::mint = mint)]
    pub to_token_account: InterfaceAccount<'info, TokenAccount>,
    pub owner_token_account: AccountInfo<'info>,
    pub owner: Signer<'info>,
    #[account(mint::token_program = token_program)]
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn _initialize_wrapper(
    ctx: Context<InitializeWrapper>,
    list_issuer: Vec<Pubkey>,
    exit_regulators: Vec<Pubkey>
) -> Result<()> {
    let wrapper_account = &mut ctx.accounts.wrapper_account;
    wrapper_account.id_issuers = list_issuer;
    wrapper_account.exit_regulators = exit_regulators;
    wrapper_account.bump = ctx.bumps.wrapper_account;
    Ok(())
}

pub fn _add_issuers_wrapper(ctx: Context<AddWrapperIssuer>, issuer: Pubkey) -> Result<()> {
    let wrapper_account = &mut ctx.accounts.wrapper_account;
    wrapper_account.id_issuers.push(issuer);
    Ok(())
}

pub fn _remove_issuer_wrapper(ctx: Context<DeleteWrapperIssuer>) -> Result<()> {
    let wrapper_account = &mut ctx.accounts.wrapper_account;
    let index = wrapper_account
        .id_issuers
        .iter()
        .position(|x| *x == ctx.accounts.issuer.key())
        .unwrap();
    wrapper_account.id_issuers.remove(index);
    Ok(())
}


pub fn _wrap_tokens(ctx: Context<WrapTokens>, amount: u64, decimals: u8) -> Result<()> {
    let mint = &ctx.accounts.mint;

    if mint.decimals != decimals {
        return Err(WrapperError::InvalidDecimals.into());
    }

    // CPI to transfer tokens from from_user to wrapped_account of the user on this program
    let ix = spl_token::instruction::transfer(
        ctx.accounts.token_program.key,
        &ctx.accounts.from_token_account.key(),
        &ctx.accounts.to_wrapped_token_account.key(),
        ctx.accounts.owner_from_token_account.key,
        &[ctx.accounts.owner_from_token_account.key],
        amount,
    )?;
    program::invoke(
        &ix,
        &[
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.from_token_account.to_account_info(),
            ctx.accounts.to_wrapped_token_account.to_account_info(),
            ctx.accounts.owner_from_token_account.to_account_info(),
        ],
    )?;

    Ok(())
}



pub fn _unwrap_tokens(ctx: Context<UnWrapTokens>, amount: u64, decimals: u8) -> Result<()> {

    // CHECK: That the exit regulator is in the list of the wrapper_account
    let wrapper_account = &ctx.accounts.wrapper_account;
    let exit_regulator = &ctx.accounts.exit_regulator;

    if !wrapper_account.exit_regulators.contains(exit_regulator.key) {
        return Err(WrapperError::InvalidExitRegulator.into())
    }

    let mint = &ctx.accounts.mint;

    if mint.decimals != decimals {
        return Err(WrapperError::InvalidDecimals.into());
    }

    let approver = ctx.accounts.approver.key();
    let bump = ctx.bumps.wrapper_account;
    let seed: &[&[&[u8]]]  = &[&[b"wrapper", approver.as_ref(), &[bump]]];


    // CPI to transfer tokens from user to to_token_account
    let ix = spl_token::instruction::transfer(
        ctx.accounts.token_program.key,
        &ctx.accounts.user_wrapped_token_account.key(),
        &ctx.accounts.to_token_account.key(),
        &ctx.accounts.wrapper_account.key(),
        &[&ctx.accounts.wrapper_account.key()],
        amount,
    )?;
    program::invoke_signed(
        &ix,
        &[
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.user_wrapped_token_account.to_account_info(),
            ctx.accounts.to_token_account.to_account_info(),
            ctx.accounts.wrapper_account.to_account_info(),
        ],
        seed
    )?;
    
    Ok(())
}