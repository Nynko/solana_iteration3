#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod constants {}
pub mod error {
    use anchor_lang::prelude::*;
    #[repr(u32)]
    pub enum WrapperError {
        InvalidDecimals,
        InvalidExitRegulator,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for WrapperError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    WrapperError::InvalidDecimals => "InvalidDecimals",
                    WrapperError::InvalidExitRegulator => "InvalidExitRegulator",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for WrapperError {
        #[inline]
        fn clone(&self) -> WrapperError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for WrapperError {}
    impl WrapperError {
        /// Gets the name of this [#enum_name].
        pub fn name(&self) -> String {
            match self {
                WrapperError::InvalidDecimals => "InvalidDecimals".to_string(),
                WrapperError::InvalidExitRegulator => "InvalidExitRegulator".to_string(),
            }
        }
    }
    impl From<WrapperError> for u32 {
        fn from(e: WrapperError) -> u32 {
            e as u32 + anchor_lang::error::ERROR_CODE_OFFSET
        }
    }
    impl From<WrapperError> for anchor_lang::error::Error {
        fn from(error_code: WrapperError) -> anchor_lang::error::Error {
            anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                error_name: error_code.name(),
                error_code_number: error_code.into(),
                error_msg: error_code.to_string(),
                error_origin: None,
                compared_values: None,
            })
        }
    }
    impl std::fmt::Display for WrapperError {
        fn fmt(
            &self,
            fmt: &mut std::fmt::Formatter<'_>,
        ) -> std::result::Result<(), std::fmt::Error> {
            match self {
                WrapperError::InvalidDecimals => {
                    fmt.write_fmt(
                        format_args!(
                            "Decimal provided does not match the mint\'s decimal value",
                        ),
                    )
                }
                WrapperError::InvalidExitRegulator => {
                    fmt.write_fmt(
                        format_args!("Invalid exit regulator for the specified wrapper"),
                    )
                }
            }
        }
    }
    #[repr(u32)]
    pub enum TransferError {
        InsufficientFunds,
        InvalidDecimals,
        Overflow,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TransferError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    TransferError::InsufficientFunds => "InsufficientFunds",
                    TransferError::InvalidDecimals => "InvalidDecimals",
                    TransferError::Overflow => "Overflow",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TransferError {
        #[inline]
        fn clone(&self) -> TransferError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for TransferError {}
    impl TransferError {
        /// Gets the name of this [#enum_name].
        pub fn name(&self) -> String {
            match self {
                TransferError::InsufficientFunds => "InsufficientFunds".to_string(),
                TransferError::InvalidDecimals => "InvalidDecimals".to_string(),
                TransferError::Overflow => "Overflow".to_string(),
            }
        }
    }
    impl From<TransferError> for u32 {
        fn from(e: TransferError) -> u32 {
            e as u32 + anchor_lang::error::ERROR_CODE_OFFSET
        }
    }
    impl From<TransferError> for anchor_lang::error::Error {
        fn from(error_code: TransferError) -> anchor_lang::error::Error {
            anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                error_name: error_code.name(),
                error_code_number: error_code.into(),
                error_msg: error_code.to_string(),
                error_origin: None,
                compared_values: None,
            })
        }
    }
    impl std::fmt::Display for TransferError {
        fn fmt(
            &self,
            fmt: &mut std::fmt::Formatter<'_>,
        ) -> std::result::Result<(), std::fmt::Error> {
            match self {
                TransferError::InsufficientFunds => {
                    fmt.write_fmt(
                        format_args!(
                            "The source account does not have enough funds to transfer",
                        ),
                    )
                }
                TransferError::InvalidDecimals => {
                    fmt.write_fmt(
                        format_args!(
                            "Decimal provided does not match the mint\'s decimal value",
                        ),
                    )
                }
                TransferError::Overflow => {
                    fmt.write_fmt(
                        format_args!(
                            "Overflow when adding the amount to the destination account",
                        ),
                    )
                }
            }
        }
    }
    #[repr(u32)]
    pub enum IdendityError {
        IdendityAlreadyExists,
        IdendityNotActive,
        IdendityExpired,
        IdendityRecovered,
        IdendityAlreadyRecovered,
        IssuerNotApproved,
        InvalidIdendity,
        PseudoAlreadyExist,
        PseudoDontExist,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for IdendityError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    IdendityError::IdendityAlreadyExists => "IdendityAlreadyExists",
                    IdendityError::IdendityNotActive => "IdendityNotActive",
                    IdendityError::IdendityExpired => "IdendityExpired",
                    IdendityError::IdendityRecovered => "IdendityRecovered",
                    IdendityError::IdendityAlreadyRecovered => "IdendityAlreadyRecovered",
                    IdendityError::IssuerNotApproved => "IssuerNotApproved",
                    IdendityError::InvalidIdendity => "InvalidIdendity",
                    IdendityError::PseudoAlreadyExist => "PseudoAlreadyExist",
                    IdendityError::PseudoDontExist => "PseudoDontExist",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for IdendityError {
        #[inline]
        fn clone(&self) -> IdendityError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for IdendityError {}
    impl IdendityError {
        /// Gets the name of this [#enum_name].
        pub fn name(&self) -> String {
            match self {
                IdendityError::IdendityAlreadyExists => {
                    "IdendityAlreadyExists".to_string()
                }
                IdendityError::IdendityNotActive => "IdendityNotActive".to_string(),
                IdendityError::IdendityExpired => "IdendityExpired".to_string(),
                IdendityError::IdendityRecovered => "IdendityRecovered".to_string(),
                IdendityError::IdendityAlreadyRecovered => {
                    "IdendityAlreadyRecovered".to_string()
                }
                IdendityError::IssuerNotApproved => "IssuerNotApproved".to_string(),
                IdendityError::InvalidIdendity => "InvalidIdendity".to_string(),
                IdendityError::PseudoAlreadyExist => "PseudoAlreadyExist".to_string(),
                IdendityError::PseudoDontExist => "PseudoDontExist".to_string(),
            }
        }
    }
    impl From<IdendityError> for u32 {
        fn from(e: IdendityError) -> u32 {
            e as u32 + anchor_lang::error::ERROR_CODE_OFFSET
        }
    }
    impl From<IdendityError> for anchor_lang::error::Error {
        fn from(error_code: IdendityError) -> anchor_lang::error::Error {
            anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                error_name: error_code.name(),
                error_code_number: error_code.into(),
                error_msg: error_code.to_string(),
                error_origin: None,
                compared_values: None,
            })
        }
    }
    impl std::fmt::Display for IdendityError {
        fn fmt(
            &self,
            fmt: &mut std::fmt::Formatter<'_>,
        ) -> std::result::Result<(), std::fmt::Error> {
            match self {
                IdendityError::IdendityAlreadyExists => {
                    fmt.write_fmt(format_args!("Idendity already exists"))
                }
                IdendityError::IdendityNotActive => {
                    fmt.write_fmt(format_args!("Idendity is not active"))
                }
                IdendityError::IdendityExpired => {
                    fmt.write_fmt(format_args!("Idendity expired"))
                }
                IdendityError::IdendityRecovered => {
                    fmt.write_fmt(format_args!("Idendity recovered"))
                }
                IdendityError::IdendityAlreadyRecovered => {
                    fmt.write_fmt(format_args!("Idendity already recovered"))
                }
                IdendityError::IssuerNotApproved => {
                    fmt.write_fmt(format_args!("Issuer is not approved"))
                }
                IdendityError::InvalidIdendity => {
                    fmt.write_fmt(
                        format_args!(
                            "No approved issuer found or inactive/expired issuer",
                        ),
                    )
                }
                IdendityError::PseudoAlreadyExist => {
                    fmt.write_fmt(
                        format_args!(
                            "A pseudo already exist, please use the update method",
                        ),
                    )
                }
                IdendityError::PseudoDontExist => {
                    fmt.write_fmt(
                        format_args!(
                            "The old pseudo doesn\'t exist, please use the add method",
                        ),
                    )
                }
            }
        }
    }
    #[repr(u32)]
    pub enum TwoAuthError {
        NotAuthorized,
        NeedTwoAuthApproval,
        WrongApproval,
        ExpiredApproval,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TwoAuthError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    TwoAuthError::NotAuthorized => "NotAuthorized",
                    TwoAuthError::NeedTwoAuthApproval => "NeedTwoAuthApproval",
                    TwoAuthError::WrongApproval => "WrongApproval",
                    TwoAuthError::ExpiredApproval => "ExpiredApproval",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TwoAuthError {
        #[inline]
        fn clone(&self) -> TwoAuthError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for TwoAuthError {}
    impl TwoAuthError {
        /// Gets the name of this [#enum_name].
        pub fn name(&self) -> String {
            match self {
                TwoAuthError::NotAuthorized => "NotAuthorized".to_string(),
                TwoAuthError::NeedTwoAuthApproval => "NeedTwoAuthApproval".to_string(),
                TwoAuthError::WrongApproval => "WrongApproval".to_string(),
                TwoAuthError::ExpiredApproval => "ExpiredApproval".to_string(),
            }
        }
    }
    impl From<TwoAuthError> for u32 {
        fn from(e: TwoAuthError) -> u32 {
            e as u32 + anchor_lang::error::ERROR_CODE_OFFSET
        }
    }
    impl From<TwoAuthError> for anchor_lang::error::Error {
        fn from(error_code: TwoAuthError) -> anchor_lang::error::Error {
            anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                error_name: error_code.name(),
                error_code_number: error_code.into(),
                error_msg: error_code.to_string(),
                error_origin: None,
                compared_values: None,
            })
        }
    }
    impl std::fmt::Display for TwoAuthError {
        fn fmt(
            &self,
            fmt: &mut std::fmt::Formatter<'_>,
        ) -> std::result::Result<(), std::fmt::Error> {
            match self {
                TwoAuthError::NotAuthorized => {
                    fmt.write_fmt(
                        format_args!("Not authorized to approve this transaction"),
                    )
                }
                TwoAuthError::NeedTwoAuthApproval => {
                    fmt.write_fmt(format_args!("Need the two auth entity approval"))
                }
                TwoAuthError::WrongApproval => {
                    fmt.write_fmt(format_args!("The provided two_auth entity is wrong"))
                }
                TwoAuthError::ExpiredApproval => {
                    fmt.write_fmt(format_args!("The Approval has expired"))
                }
            }
        }
    }
    #[repr(u32)]
    pub enum RecoveryError {
        RecoveryTimeNotPassed,
        NotEnoughSignatures,
        WrongMainRecoveryAuthority,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for RecoveryError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    RecoveryError::RecoveryTimeNotPassed => "RecoveryTimeNotPassed",
                    RecoveryError::NotEnoughSignatures => "NotEnoughSignatures",
                    RecoveryError::WrongMainRecoveryAuthority => {
                        "WrongMainRecoveryAuthority"
                    }
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for RecoveryError {
        #[inline]
        fn clone(&self) -> RecoveryError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for RecoveryError {}
    impl RecoveryError {
        /// Gets the name of this [#enum_name].
        pub fn name(&self) -> String {
            match self {
                RecoveryError::RecoveryTimeNotPassed => {
                    "RecoveryTimeNotPassed".to_string()
                }
                RecoveryError::NotEnoughSignatures => "NotEnoughSignatures".to_string(),
                RecoveryError::WrongMainRecoveryAuthority => {
                    "WrongMainRecoveryAuthority".to_string()
                }
            }
        }
    }
    impl From<RecoveryError> for u32 {
        fn from(e: RecoveryError) -> u32 {
            e as u32 + anchor_lang::error::ERROR_CODE_OFFSET
        }
    }
    impl From<RecoveryError> for anchor_lang::error::Error {
        fn from(error_code: RecoveryError) -> anchor_lang::error::Error {
            anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                error_name: error_code.name(),
                error_code_number: error_code.into(),
                error_msg: error_code.to_string(),
                error_origin: None,
                compared_values: None,
            })
        }
    }
    impl std::fmt::Display for RecoveryError {
        fn fmt(
            &self,
            fmt: &mut std::fmt::Formatter<'_>,
        ) -> std::result::Result<(), std::fmt::Error> {
            match self {
                RecoveryError::RecoveryTimeNotPassed => {
                    fmt.write_fmt(format_args!("Recovery time not passed"))
                }
                RecoveryError::NotEnoughSignatures => {
                    fmt.write_fmt(format_args!("Not enough signatures"))
                }
                RecoveryError::WrongMainRecoveryAuthority => {
                    fmt.write_fmt(
                        format_args!(
                            "The main recovery authority is not in the list of recoverable authority",
                        ),
                    )
                }
            }
        }
    }
}
pub mod instructions {
    pub mod subsystem {
        use anchor_lang::{prelude::*, solana_program::program};
        use anchor_spl::{
            token::spl_token, token_interface::{Mint, TokenAccount, TokenInterface},
        };
        use crate::{error::WrapperError, WrapperAccount};
        #[instruction(list_issuer:Vec<Pubkey>, exit_regulators:Vec<Pubkey>)]
        pub struct InitializeSubsystem<'info> {
            #[account(
                init,
                seeds = [b"wrapper",
                approver.key().as_ref()],
                bump,
                payer = payer,
                space = WrapperAccount::get_init_len(&list_issuer, &exit_regulators)
            )]
            pub wrapper_account: Account<'info, WrapperAccount>,
            #[account(mut)]
            pub payer: Signer<'info>,
            pub approver: Signer<'info>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, InitializeSubsystemBumps>
        for InitializeSubsystem<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut InitializeSubsystemBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let mut __ix_data = __ix_data;
                struct __Args {
                    list_issuer: Vec<Pubkey>,
                    exit_regulators: Vec<Pubkey>,
                }
                impl borsh::ser::BorshSerialize for __Args
                where
                    Vec<Pubkey>: borsh::ser::BorshSerialize,
                    Vec<Pubkey>: borsh::ser::BorshSerialize,
                {
                    fn serialize<W: borsh::maybestd::io::Write>(
                        &self,
                        writer: &mut W,
                    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                        borsh::BorshSerialize::serialize(&self.list_issuer, writer)?;
                        borsh::BorshSerialize::serialize(&self.exit_regulators, writer)?;
                        Ok(())
                    }
                }
                impl borsh::de::BorshDeserialize for __Args
                where
                    Vec<Pubkey>: borsh::BorshDeserialize,
                    Vec<Pubkey>: borsh::BorshDeserialize,
                {
                    fn deserialize_reader<R: borsh::maybestd::io::Read>(
                        reader: &mut R,
                    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                        Ok(Self {
                            list_issuer: borsh::BorshDeserialize::deserialize_reader(
                                reader,
                            )?,
                            exit_regulators: borsh::BorshDeserialize::deserialize_reader(
                                reader,
                            )?,
                        })
                    }
                }
                let __Args { list_issuer, exit_regulators } = __Args::deserialize(
                        &mut __ix_data,
                    )
                    .map_err(|_| {
                        anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                    })?;
                if __accounts.is_empty() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into(),
                    );
                }
                let wrapper_account = &__accounts[0];
                *__accounts = &__accounts[1..];
                let payer: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("payer"))?;
                let approver: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("approver"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                let __anchor_rent = Rent::get()?;
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[b"wrapper", approver.key().as_ref()],
                    __program_id,
                );
                __bumps.wrapper_account = __bump;
                if wrapper_account.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("wrapper_account")
                            .with_pubkeys((wrapper_account.key(), __pda_address)),
                    );
                }
                let wrapper_account = {
                    let actual_field = AsRef::<AccountInfo>::as_ref(&wrapper_account);
                    let actual_owner = actual_field.owner;
                    let space = WrapperAccount::get_init_len(
                        &list_issuer,
                        &exit_regulators,
                    );
                    let pa: anchor_lang::accounts::account::Account<WrapperAccount> = if !false
                        || actual_owner
                            == &anchor_lang::solana_program::system_program::ID
                    {
                        let __current_lamports = wrapper_account.lamports();
                        if __current_lamports == 0 {
                            let space = space;
                            let lamports = __anchor_rent.minimum_balance(space);
                            let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                from: payer.to_account_info(),
                                to: wrapper_account.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::create_account(
                                cpi_context
                                    .with_signer(
                                        &[&[b"wrapper", approver.key().as_ref(), &[__bump][..]][..]],
                                    ),
                                lamports,
                                space as u64,
                                __program_id,
                            )?;
                        } else {
                            if payer.key() == wrapper_account.key() {
                                return Err(
                                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                            error_name: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .name(),
                                            error_code_number: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .into(),
                                            error_msg: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .to_string(),
                                            error_origin: Some(
                                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                    filename: "programs/asset_based/src/instructions/subsystem.rs",
                                                    line: 8u32,
                                                }),
                                            ),
                                            compared_values: None,
                                        })
                                        .with_pubkeys((payer.key(), wrapper_account.key())),
                                );
                            }
                            let required_lamports = __anchor_rent
                                .minimum_balance(space)
                                .max(1)
                                .saturating_sub(__current_lamports);
                            if required_lamports > 0 {
                                let cpi_accounts = anchor_lang::system_program::Transfer {
                                    from: payer.to_account_info(),
                                    to: wrapper_account.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::transfer(
                                    cpi_context,
                                    required_lamports,
                                )?;
                            }
                            let cpi_accounts = anchor_lang::system_program::Allocate {
                                account_to_allocate: wrapper_account.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::allocate(
                                cpi_context
                                    .with_signer(
                                        &[&[b"wrapper", approver.key().as_ref(), &[__bump][..]][..]],
                                    ),
                                space as u64,
                            )?;
                            let cpi_accounts = anchor_lang::system_program::Assign {
                                account_to_assign: wrapper_account.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::assign(
                                cpi_context
                                    .with_signer(
                                        &[&[b"wrapper", approver.key().as_ref(), &[__bump][..]][..]],
                                    ),
                                __program_id,
                            )?;
                        }
                        match anchor_lang::accounts::account::Account::try_from_unchecked(
                            &wrapper_account,
                        ) {
                            Ok(val) => val,
                            Err(e) => return Err(e.with_account_name("wrapper_account")),
                        }
                    } else {
                        match anchor_lang::accounts::account::Account::try_from(
                            &wrapper_account,
                        ) {
                            Ok(val) => val,
                            Err(e) => return Err(e.with_account_name("wrapper_account")),
                        }
                    };
                    if false {
                        if space != actual_field.data_len() {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::ConstraintSpace,
                                    )
                                    .with_account_name("wrapper_account")
                                    .with_values((space, actual_field.data_len())),
                            );
                        }
                        if actual_owner != __program_id {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::ConstraintOwner,
                                    )
                                    .with_account_name("wrapper_account")
                                    .with_pubkeys((*actual_owner, *__program_id)),
                            );
                        }
                        {
                            let required_lamports = __anchor_rent.minimum_balance(space);
                            if pa.to_account_info().lamports() < required_lamports {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintRentExempt,
                                        )
                                        .with_account_name("wrapper_account"),
                                );
                            }
                        }
                    }
                    pa
                };
                if !AsRef::<AccountInfo>::as_ref(&wrapper_account).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("wrapper_account"),
                    );
                }
                if !__anchor_rent
                    .is_exempt(
                        wrapper_account.to_account_info().lamports(),
                        wrapper_account.to_account_info().try_data_len()?,
                    )
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                            )
                            .with_account_name("wrapper_account"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&payer).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("payer"),
                    );
                }
                Ok(InitializeSubsystem {
                    wrapper_account,
                    payer,
                    approver,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeSubsystem<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.wrapper_account.to_account_infos());
                account_infos.extend(self.payer.to_account_infos());
                account_infos.extend(self.approver.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for InitializeSubsystem<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.wrapper_account.to_account_metas(None));
                account_metas.extend(self.payer.to_account_metas(None));
                account_metas.extend(self.approver.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for InitializeSubsystem<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.wrapper_account, program_id)
                    .map_err(|e| e.with_account_name("wrapper_account"))?;
                anchor_lang::AccountsExit::exit(&self.payer, program_id)
                    .map_err(|e| e.with_account_name("payer"))?;
                Ok(())
            }
        }
        pub struct InitializeSubsystemBumps {
            pub wrapper_account: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for InitializeSubsystemBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "InitializeSubsystemBumps",
                    "wrapper_account",
                    &&self.wrapper_account,
                )
            }
        }
        impl Default for InitializeSubsystemBumps {
            fn default() -> Self {
                InitializeSubsystemBumps {
                    wrapper_account: u8::MAX,
                }
            }
        }
        impl<'info> anchor_lang::Bumps for InitializeSubsystem<'info>
        where
            'info: 'info,
        {
            type Bumps = InitializeSubsystemBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_initialize_subsystem {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`InitializeSubsystem`].
            pub struct InitializeSubsystem {
                pub wrapper_account: Pubkey,
                pub payer: Pubkey,
                pub approver: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for InitializeSubsystem
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.wrapper_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.payer, writer)?;
                    borsh::BorshSerialize::serialize(&self.approver, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for InitializeSubsystem {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.wrapper_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.payer,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.approver,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_initialize_subsystem {
            use super::*;
            /// Generated CPI struct of the accounts for [`InitializeSubsystem`].
            pub struct InitializeSubsystem<'info> {
                pub wrapper_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub payer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub approver: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for InitializeSubsystem<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.wrapper_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.payer),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.approver),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info>
            for InitializeSubsystem<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.wrapper_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.payer),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.approver),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct AddSubsystemIssuer<'info> {
            #[account(
                mut,
                seeds = [b"wrapper",
                approver.key().as_ref()],
                bump,
                realloc = wrapper_account.get_len_add_address(),
                realloc::payer = payer,
                realloc::zero = false
            )]
            pub wrapper_account: Account<'info, WrapperAccount>,
            #[account(mut)]
            pub payer: Signer<'info>,
            pub approver: Signer<'info>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, AddSubsystemIssuerBumps>
        for AddSubsystemIssuer<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut AddSubsystemIssuerBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let wrapper_account: anchor_lang::accounts::account::Account<
                    WrapperAccount,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("wrapper_account"))?;
                let payer: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("payer"))?;
                let approver: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("approver"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                if __reallocs.contains(&wrapper_account.key()) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::AccountDuplicateReallocs,
                            )
                            .with_account_name("wrapper_account"),
                    );
                }
                let __anchor_rent = anchor_lang::prelude::Rent::get()?;
                let __field_info = wrapper_account.to_account_info();
                let __new_rent_minimum = __anchor_rent
                    .minimum_balance(wrapper_account.get_len_add_address());
                let __delta_space = (::std::convert::TryInto::<
                    isize,
                >::try_into(wrapper_account.get_len_add_address())
                    .unwrap())
                    .checked_sub(
                        ::std::convert::TryInto::try_into(__field_info.data_len())
                            .unwrap(),
                    )
                    .unwrap();
                if __delta_space != 0 {
                    if __delta_space > 0 {
                        if ::std::convert::TryInto::<usize>::try_into(__delta_space)
                            .unwrap()
                            > anchor_lang::solana_program::entrypoint::MAX_PERMITTED_DATA_INCREASE
                        {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::AccountReallocExceedsLimit,
                                    )
                                    .with_account_name("wrapper_account"),
                            );
                        }
                        if __new_rent_minimum > __field_info.lamports() {
                            anchor_lang::system_program::transfer(
                                anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    anchor_lang::system_program::Transfer {
                                        from: payer.to_account_info(),
                                        to: __field_info.clone(),
                                    },
                                ),
                                __new_rent_minimum
                                    .checked_sub(__field_info.lamports())
                                    .unwrap(),
                            )?;
                        }
                    } else {
                        let __lamport_amt = __field_info
                            .lamports()
                            .checked_sub(__new_rent_minimum)
                            .unwrap();
                        **payer.to_account_info().lamports.borrow_mut() = payer
                            .to_account_info()
                            .lamports()
                            .checked_add(__lamport_amt)
                            .unwrap();
                        **__field_info.lamports.borrow_mut() = __field_info
                            .lamports()
                            .checked_sub(__lamport_amt)
                            .unwrap();
                    }
                    __field_info.realloc(wrapper_account.get_len_add_address(), false)?;
                    __reallocs.insert(wrapper_account.key());
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[b"wrapper", approver.key().as_ref()],
                    &__program_id,
                );
                __bumps.wrapper_account = __bump;
                if wrapper_account.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("wrapper_account")
                            .with_pubkeys((wrapper_account.key(), __pda_address)),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&wrapper_account).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("wrapper_account"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&payer).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("payer"),
                    );
                }
                Ok(AddSubsystemIssuer {
                    wrapper_account,
                    payer,
                    approver,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for AddSubsystemIssuer<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.wrapper_account.to_account_infos());
                account_infos.extend(self.payer.to_account_infos());
                account_infos.extend(self.approver.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for AddSubsystemIssuer<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.wrapper_account.to_account_metas(None));
                account_metas.extend(self.payer.to_account_metas(None));
                account_metas.extend(self.approver.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for AddSubsystemIssuer<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.wrapper_account, program_id)
                    .map_err(|e| e.with_account_name("wrapper_account"))?;
                anchor_lang::AccountsExit::exit(&self.payer, program_id)
                    .map_err(|e| e.with_account_name("payer"))?;
                Ok(())
            }
        }
        pub struct AddSubsystemIssuerBumps {
            pub wrapper_account: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AddSubsystemIssuerBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "AddSubsystemIssuerBumps",
                    "wrapper_account",
                    &&self.wrapper_account,
                )
            }
        }
        impl Default for AddSubsystemIssuerBumps {
            fn default() -> Self {
                AddSubsystemIssuerBumps {
                    wrapper_account: u8::MAX,
                }
            }
        }
        impl<'info> anchor_lang::Bumps for AddSubsystemIssuer<'info>
        where
            'info: 'info,
        {
            type Bumps = AddSubsystemIssuerBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_add_subsystem_issuer {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`AddSubsystemIssuer`].
            pub struct AddSubsystemIssuer {
                pub wrapper_account: Pubkey,
                pub payer: Pubkey,
                pub approver: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for AddSubsystemIssuer
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.wrapper_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.payer, writer)?;
                    borsh::BorshSerialize::serialize(&self.approver, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for AddSubsystemIssuer {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.wrapper_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.payer,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.approver,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_add_subsystem_issuer {
            use super::*;
            /// Generated CPI struct of the accounts for [`AddSubsystemIssuer`].
            pub struct AddSubsystemIssuer<'info> {
                pub wrapper_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub payer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub approver: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for AddSubsystemIssuer<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.wrapper_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.payer),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.approver),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info>
            for AddSubsystemIssuer<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.wrapper_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.payer),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.approver),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct DeleteSubsystemIssuer<'info> {
            #[account(
                mut,
                seeds = [b"wrapper",
                approver.key().as_ref()],
                bump,
                realloc = wrapper_account.get_len_remove_address(),
                realloc::payer = payer,
                realloc::zero = true
            )]
            pub wrapper_account: Account<'info, WrapperAccount>,
            #[account(mut)]
            pub payer: Signer<'info>,
            pub approver: Signer<'info>,
            /// CHECK: The issuer to be removed
            #[account(constraint = wrapper_account.id_issuers.contains(&issuer.key()))]
            pub issuer: UncheckedAccount<'info>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, DeleteSubsystemIssuerBumps>
        for DeleteSubsystemIssuer<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut DeleteSubsystemIssuerBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let wrapper_account: anchor_lang::accounts::account::Account<
                    WrapperAccount,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("wrapper_account"))?;
                let payer: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("payer"))?;
                let approver: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("approver"))?;
                let issuer: UncheckedAccount = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("issuer"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                if __reallocs.contains(&wrapper_account.key()) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::AccountDuplicateReallocs,
                            )
                            .with_account_name("wrapper_account"),
                    );
                }
                let __anchor_rent = anchor_lang::prelude::Rent::get()?;
                let __field_info = wrapper_account.to_account_info();
                let __new_rent_minimum = __anchor_rent
                    .minimum_balance(wrapper_account.get_len_remove_address());
                let __delta_space = (::std::convert::TryInto::<
                    isize,
                >::try_into(wrapper_account.get_len_remove_address())
                    .unwrap())
                    .checked_sub(
                        ::std::convert::TryInto::try_into(__field_info.data_len())
                            .unwrap(),
                    )
                    .unwrap();
                if __delta_space != 0 {
                    if __delta_space > 0 {
                        if ::std::convert::TryInto::<usize>::try_into(__delta_space)
                            .unwrap()
                            > anchor_lang::solana_program::entrypoint::MAX_PERMITTED_DATA_INCREASE
                        {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::AccountReallocExceedsLimit,
                                    )
                                    .with_account_name("wrapper_account"),
                            );
                        }
                        if __new_rent_minimum > __field_info.lamports() {
                            anchor_lang::system_program::transfer(
                                anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    anchor_lang::system_program::Transfer {
                                        from: payer.to_account_info(),
                                        to: __field_info.clone(),
                                    },
                                ),
                                __new_rent_minimum
                                    .checked_sub(__field_info.lamports())
                                    .unwrap(),
                            )?;
                        }
                    } else {
                        let __lamport_amt = __field_info
                            .lamports()
                            .checked_sub(__new_rent_minimum)
                            .unwrap();
                        **payer.to_account_info().lamports.borrow_mut() = payer
                            .to_account_info()
                            .lamports()
                            .checked_add(__lamport_amt)
                            .unwrap();
                        **__field_info.lamports.borrow_mut() = __field_info
                            .lamports()
                            .checked_sub(__lamport_amt)
                            .unwrap();
                    }
                    __field_info
                        .realloc(wrapper_account.get_len_remove_address(), true)?;
                    __reallocs.insert(wrapper_account.key());
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[b"wrapper", approver.key().as_ref()],
                    &__program_id,
                );
                __bumps.wrapper_account = __bump;
                if wrapper_account.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("wrapper_account")
                            .with_pubkeys((wrapper_account.key(), __pda_address)),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&wrapper_account).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("wrapper_account"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&payer).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("payer"),
                    );
                }
                if !(wrapper_account.id_issuers.contains(&issuer.key())) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("issuer"),
                    );
                }
                Ok(DeleteSubsystemIssuer {
                    wrapper_account,
                    payer,
                    approver,
                    issuer,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for DeleteSubsystemIssuer<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.wrapper_account.to_account_infos());
                account_infos.extend(self.payer.to_account_infos());
                account_infos.extend(self.approver.to_account_infos());
                account_infos.extend(self.issuer.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for DeleteSubsystemIssuer<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.wrapper_account.to_account_metas(None));
                account_metas.extend(self.payer.to_account_metas(None));
                account_metas.extend(self.approver.to_account_metas(None));
                account_metas.extend(self.issuer.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for DeleteSubsystemIssuer<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.wrapper_account, program_id)
                    .map_err(|e| e.with_account_name("wrapper_account"))?;
                anchor_lang::AccountsExit::exit(&self.payer, program_id)
                    .map_err(|e| e.with_account_name("payer"))?;
                Ok(())
            }
        }
        pub struct DeleteSubsystemIssuerBumps {
            pub wrapper_account: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DeleteSubsystemIssuerBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "DeleteSubsystemIssuerBumps",
                    "wrapper_account",
                    &&self.wrapper_account,
                )
            }
        }
        impl Default for DeleteSubsystemIssuerBumps {
            fn default() -> Self {
                DeleteSubsystemIssuerBumps {
                    wrapper_account: u8::MAX,
                }
            }
        }
        impl<'info> anchor_lang::Bumps for DeleteSubsystemIssuer<'info>
        where
            'info: 'info,
        {
            type Bumps = DeleteSubsystemIssuerBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_delete_subsystem_issuer {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`DeleteSubsystemIssuer`].
            pub struct DeleteSubsystemIssuer {
                pub wrapper_account: Pubkey,
                pub payer: Pubkey,
                pub approver: Pubkey,
                pub issuer: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for DeleteSubsystemIssuer
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.wrapper_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.payer, writer)?;
                    borsh::BorshSerialize::serialize(&self.approver, writer)?;
                    borsh::BorshSerialize::serialize(&self.issuer, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for DeleteSubsystemIssuer {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.wrapper_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.payer,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.approver,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.issuer,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_delete_subsystem_issuer {
            use super::*;
            /// Generated CPI struct of the accounts for [`DeleteSubsystemIssuer`].
            pub struct DeleteSubsystemIssuer<'info> {
                pub wrapper_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub payer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub approver: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub issuer: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for DeleteSubsystemIssuer<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.wrapper_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.payer),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.approver),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.issuer),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info>
            for DeleteSubsystemIssuer<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.wrapper_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.payer),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.approver),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.issuer),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct DepositTokensSubsystem<'info> {
            #[account(seeds = [b"wrapper", approver.key().as_ref()], bump)]
            pub wrapper_account: Account<'info, WrapperAccount>,
            /// CHECK: The approver of the wrapper
            pub approver: UncheckedAccount<'info>,
            #[account(
                init_if_needed,
                token::authority = wrapper_account,
                token::mint = mint,
                seeds = [b"wrapped_token",
                wrapper_account.key().as_ref(),
                mint.key().as_ref(),
                owner_to_account.key().as_ref()],
                bump,
                payer = owner_from_token_account
            )]
            pub to_wrapped_token_account: InterfaceAccount<'info, TokenAccount>,
            #[account(
                mut,
                token::authority = owner_from_token_account,
                token::mint = mint
            )]
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
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, DepositTokensSubsystemBumps>
        for DepositTokensSubsystem<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut DepositTokensSubsystemBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let wrapper_account: anchor_lang::accounts::account::Account<
                    WrapperAccount,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("wrapper_account"))?;
                let approver: UncheckedAccount = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("approver"))?;
                if __accounts.is_empty() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into(),
                    );
                }
                let to_wrapped_token_account = &__accounts[0];
                *__accounts = &__accounts[1..];
                let from_token_account: anchor_lang::accounts::interface_account::InterfaceAccount<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("from_token_account"))?;
                let owner_from_token_account: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("owner_from_token_account"))?;
                let owner_to_account: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("owner_to_account"))?;
                let mint: anchor_lang::accounts::interface_account::InterfaceAccount<
                    Mint,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("mint"))?;
                let token_program: anchor_lang::accounts::interface::Interface<
                    TokenInterface,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("token_program"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                let __anchor_rent = Rent::get()?;
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[
                        b"wrapped_token",
                        wrapper_account.key().as_ref(),
                        mint.key().as_ref(),
                        owner_to_account.key().as_ref(),
                    ],
                    __program_id,
                );
                __bumps.to_wrapped_token_account = __bump;
                if to_wrapped_token_account.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("to_wrapped_token_account")
                            .with_pubkeys((
                                to_wrapped_token_account.key(),
                                __pda_address,
                            )),
                    );
                }
                let to_wrapped_token_account: anchor_lang::accounts::interface_account::InterfaceAccount<
                    TokenAccount,
                > = {
                    let owner_program = AsRef::<
                        AccountInfo,
                    >::as_ref(&to_wrapped_token_account)
                        .owner;
                    if !true
                        || owner_program
                            == &anchor_lang::solana_program::system_program::ID
                    {
                        let __current_lamports = to_wrapped_token_account.lamports();
                        if __current_lamports == 0 {
                            let space = {
                                let mint_info = mint.to_account_info();
                                if *mint_info.owner
                                    == ::anchor_spl::token_2022::Token2022::id()
                                {
                                    use ::anchor_spl::token_2022::spl_token_2022::extension::{
                                        BaseStateWithExtensions, ExtensionType, StateWithExtensions,
                                    };
                                    use ::anchor_spl::token_2022::spl_token_2022::state::{
                                        Account, Mint,
                                    };
                                    let mint_data = mint_info.try_borrow_data()?;
                                    let mint_state = StateWithExtensions::<
                                        Mint,
                                    >::unpack(&mint_data)?;
                                    let mint_extensions = mint_state.get_extension_types()?;
                                    let required_extensions = ExtensionType::get_required_init_account_extensions(
                                        &mint_extensions,
                                    );
                                    ExtensionType::try_calculate_account_len::<
                                        Account,
                                    >(&required_extensions)?
                                } else {
                                    ::anchor_spl::token::TokenAccount::LEN
                                }
                            };
                            let lamports = __anchor_rent.minimum_balance(space);
                            let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                from: owner_from_token_account.to_account_info(),
                                to: to_wrapped_token_account.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::create_account(
                                cpi_context
                                    .with_signer(
                                        &[
                                            &[
                                                b"wrapped_token",
                                                wrapper_account.key().as_ref(),
                                                mint.key().as_ref(),
                                                owner_to_account.key().as_ref(),
                                                &[__bump][..],
                                            ][..],
                                        ],
                                    ),
                                lamports,
                                space as u64,
                                &token_program.key(),
                            )?;
                        } else {
                            if owner_from_token_account.key()
                                == to_wrapped_token_account.key()
                            {
                                return Err(
                                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                            error_name: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .name(),
                                            error_code_number: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .into(),
                                            error_msg: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .to_string(),
                                            error_origin: Some(
                                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                    filename: "programs/asset_based/src/instructions/subsystem.rs",
                                                    line: 42u32,
                                                }),
                                            ),
                                            compared_values: None,
                                        })
                                        .with_pubkeys((
                                            owner_from_token_account.key(),
                                            to_wrapped_token_account.key(),
                                        )),
                                );
                            }
                            let required_lamports = __anchor_rent
                                .minimum_balance({
                                    let mint_info = mint.to_account_info();
                                    if *mint_info.owner
                                        == ::anchor_spl::token_2022::Token2022::id()
                                    {
                                        use ::anchor_spl::token_2022::spl_token_2022::extension::{
                                            BaseStateWithExtensions, ExtensionType, StateWithExtensions,
                                        };
                                        use ::anchor_spl::token_2022::spl_token_2022::state::{
                                            Account, Mint,
                                        };
                                        let mint_data = mint_info.try_borrow_data()?;
                                        let mint_state = StateWithExtensions::<
                                            Mint,
                                        >::unpack(&mint_data)?;
                                        let mint_extensions = mint_state.get_extension_types()?;
                                        let required_extensions = ExtensionType::get_required_init_account_extensions(
                                            &mint_extensions,
                                        );
                                        ExtensionType::try_calculate_account_len::<
                                            Account,
                                        >(&required_extensions)?
                                    } else {
                                        ::anchor_spl::token::TokenAccount::LEN
                                    }
                                })
                                .max(1)
                                .saturating_sub(__current_lamports);
                            if required_lamports > 0 {
                                let cpi_accounts = anchor_lang::system_program::Transfer {
                                    from: owner_from_token_account.to_account_info(),
                                    to: to_wrapped_token_account.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::transfer(
                                    cpi_context,
                                    required_lamports,
                                )?;
                            }
                            let cpi_accounts = anchor_lang::system_program::Allocate {
                                account_to_allocate: to_wrapped_token_account
                                    .to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::allocate(
                                cpi_context
                                    .with_signer(
                                        &[
                                            &[
                                                b"wrapped_token",
                                                wrapper_account.key().as_ref(),
                                                mint.key().as_ref(),
                                                owner_to_account.key().as_ref(),
                                                &[__bump][..],
                                            ][..],
                                        ],
                                    ),
                                {
                                    let mint_info = mint.to_account_info();
                                    if *mint_info.owner
                                        == ::anchor_spl::token_2022::Token2022::id()
                                    {
                                        use ::anchor_spl::token_2022::spl_token_2022::extension::{
                                            BaseStateWithExtensions, ExtensionType, StateWithExtensions,
                                        };
                                        use ::anchor_spl::token_2022::spl_token_2022::state::{
                                            Account, Mint,
                                        };
                                        let mint_data = mint_info.try_borrow_data()?;
                                        let mint_state = StateWithExtensions::<
                                            Mint,
                                        >::unpack(&mint_data)?;
                                        let mint_extensions = mint_state.get_extension_types()?;
                                        let required_extensions = ExtensionType::get_required_init_account_extensions(
                                            &mint_extensions,
                                        );
                                        ExtensionType::try_calculate_account_len::<
                                            Account,
                                        >(&required_extensions)?
                                    } else {
                                        ::anchor_spl::token::TokenAccount::LEN
                                    }
                                } as u64,
                            )?;
                            let cpi_accounts = anchor_lang::system_program::Assign {
                                account_to_assign: to_wrapped_token_account
                                    .to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::assign(
                                cpi_context
                                    .with_signer(
                                        &[
                                            &[
                                                b"wrapped_token",
                                                wrapper_account.key().as_ref(),
                                                mint.key().as_ref(),
                                                owner_to_account.key().as_ref(),
                                                &[__bump][..],
                                            ][..],
                                        ],
                                    ),
                                &token_program.key(),
                            )?;
                        }
                        let cpi_program = token_program.to_account_info();
                        let accounts = ::anchor_spl::token_interface::InitializeAccount3 {
                            account: to_wrapped_token_account.to_account_info(),
                            mint: mint.to_account_info(),
                            authority: wrapper_account.to_account_info(),
                        };
                        let cpi_ctx = anchor_lang::context::CpiContext::new(
                            cpi_program,
                            accounts,
                        );
                        ::anchor_spl::token_interface::initialize_account3(cpi_ctx)?;
                    }
                    let pa: anchor_lang::accounts::interface_account::InterfaceAccount<
                        TokenAccount,
                    > = match anchor_lang::accounts::interface_account::InterfaceAccount::try_from_unchecked(
                        &to_wrapped_token_account,
                    ) {
                        Ok(val) => val,
                        Err(e) => {
                            return Err(e.with_account_name("to_wrapped_token_account"));
                        }
                    };
                    if true {
                        if pa.mint != mint.key() {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::ConstraintTokenMint,
                                    )
                                    .with_account_name("to_wrapped_token_account")
                                    .with_pubkeys((pa.mint, mint.key())),
                            );
                        }
                        if pa.owner != wrapper_account.key() {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                                    )
                                    .with_account_name("to_wrapped_token_account")
                                    .with_pubkeys((pa.owner, wrapper_account.key())),
                            );
                        }
                        if owner_program != &token_program.key() {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::ConstraintTokenTokenProgram,
                                    )
                                    .with_account_name("to_wrapped_token_account")
                                    .with_pubkeys((*owner_program, token_program.key())),
                            );
                        }
                    }
                    pa
                };
                if !AsRef::<AccountInfo>::as_ref(&to_wrapped_token_account).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("to_wrapped_token_account"),
                    );
                }
                if !__anchor_rent
                    .is_exempt(
                        to_wrapped_token_account.to_account_info().lamports(),
                        to_wrapped_token_account.to_account_info().try_data_len()?,
                    )
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                            )
                            .with_account_name("to_wrapped_token_account"),
                    );
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[b"wrapper", approver.key().as_ref()],
                    &__program_id,
                );
                __bumps.wrapper_account = __bump;
                if wrapper_account.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("wrapper_account")
                            .with_pubkeys((wrapper_account.key(), __pda_address)),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&from_token_account).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("from_token_account"),
                    );
                }
                {
                    if from_token_account.owner != owner_from_token_account.key() {
                        return Err(
                            anchor_lang::error::ErrorCode::ConstraintTokenOwner.into(),
                        );
                    }
                    if from_token_account.mint != mint.key() {
                        return Err(
                            anchor_lang::error::ErrorCode::ConstraintTokenMint.into(),
                        );
                    }
                }
                if !AsRef::<AccountInfo>::as_ref(&owner_from_token_account).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("owner_from_token_account"),
                    );
                }
                {
                    if AsRef::<AccountInfo>::as_ref(&mint).owner != &token_program.key()
                    {
                        return Err(
                            anchor_lang::error::ErrorCode::ConstraintMintTokenProgram
                                .into(),
                        );
                    }
                }
                Ok(DepositTokensSubsystem {
                    wrapper_account,
                    approver,
                    to_wrapped_token_account,
                    from_token_account,
                    owner_from_token_account,
                    owner_to_account,
                    mint,
                    token_program,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for DepositTokensSubsystem<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.wrapper_account.to_account_infos());
                account_infos.extend(self.approver.to_account_infos());
                account_infos.extend(self.to_wrapped_token_account.to_account_infos());
                account_infos.extend(self.from_token_account.to_account_infos());
                account_infos.extend(self.owner_from_token_account.to_account_infos());
                account_infos.extend(self.owner_to_account.to_account_infos());
                account_infos.extend(self.mint.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for DepositTokensSubsystem<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.wrapper_account.to_account_metas(None));
                account_metas.extend(self.approver.to_account_metas(None));
                account_metas
                    .extend(self.to_wrapped_token_account.to_account_metas(None));
                account_metas.extend(self.from_token_account.to_account_metas(None));
                account_metas
                    .extend(self.owner_from_token_account.to_account_metas(None));
                account_metas.extend(self.owner_to_account.to_account_metas(None));
                account_metas.extend(self.mint.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for DepositTokensSubsystem<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(
                        &self.to_wrapped_token_account,
                        program_id,
                    )
                    .map_err(|e| e.with_account_name("to_wrapped_token_account"))?;
                anchor_lang::AccountsExit::exit(&self.from_token_account, program_id)
                    .map_err(|e| e.with_account_name("from_token_account"))?;
                anchor_lang::AccountsExit::exit(
                        &self.owner_from_token_account,
                        program_id,
                    )
                    .map_err(|e| e.with_account_name("owner_from_token_account"))?;
                Ok(())
            }
        }
        pub struct DepositTokensSubsystemBumps {
            pub wrapper_account: u8,
            pub to_wrapped_token_account: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DepositTokensSubsystemBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "DepositTokensSubsystemBumps",
                    "wrapper_account",
                    &self.wrapper_account,
                    "to_wrapped_token_account",
                    &&self.to_wrapped_token_account,
                )
            }
        }
        impl Default for DepositTokensSubsystemBumps {
            fn default() -> Self {
                DepositTokensSubsystemBumps {
                    wrapper_account: u8::MAX,
                    to_wrapped_token_account: u8::MAX,
                }
            }
        }
        impl<'info> anchor_lang::Bumps for DepositTokensSubsystem<'info>
        where
            'info: 'info,
        {
            type Bumps = DepositTokensSubsystemBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_deposit_tokens_subsystem {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`DepositTokensSubsystem`].
            pub struct DepositTokensSubsystem {
                pub wrapper_account: Pubkey,
                pub approver: Pubkey,
                pub to_wrapped_token_account: Pubkey,
                pub from_token_account: Pubkey,
                pub owner_from_token_account: Pubkey,
                pub owner_to_account: Pubkey,
                pub mint: Pubkey,
                pub token_program: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for DepositTokensSubsystem
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.wrapper_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.approver, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.to_wrapped_token_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(&self.from_token_account, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.owner_from_token_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(&self.owner_to_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for DepositTokensSubsystem {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.wrapper_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.approver,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.to_wrapped_token_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.from_token_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.owner_from_token_account,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.owner_to_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.mint,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.token_program,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_deposit_tokens_subsystem {
            use super::*;
            /// Generated CPI struct of the accounts for [`DepositTokensSubsystem`].
            pub struct DepositTokensSubsystem<'info> {
                pub wrapper_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub approver: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub to_wrapped_token_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub from_token_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub owner_from_token_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub owner_to_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for DepositTokensSubsystem<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.wrapper_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.approver),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.to_wrapped_token_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.from_token_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.owner_from_token_account),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.owner_to_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.mint),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.token_program),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info>
            for DepositTokensSubsystem<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.wrapper_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.approver),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.to_wrapped_token_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.from_token_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.owner_from_token_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.owner_to_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.mint),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.token_program,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct WithdrawTokensSubsystem<'info> {
            #[account(seeds = [b"wrapper", approver.key().as_ref()], bump)]
            pub wrapper_account: Account<'info, WrapperAccount>,
            /// CHECK: The approver of the wrapper
            pub approver: UncheckedAccount<'info>,
            /// CHECK: That the exit regulator is in the list of the wrapper_account
            pub exit_regulator: Signer<'info>,
            #[account(
                mut,
                token::authority = wrapper_account,
                token::mint = mint,
                seeds = [b"wrapped_token",
                wrapper_account.key().as_ref(),
                mint.key().as_ref(),
                owner.key().as_ref()],
                bump
            )]
            pub user_wrapped_token_account: InterfaceAccount<'info, TokenAccount>,
            #[account(mut, token::authority = owner_token_account, token::mint = mint)]
            pub to_token_account: InterfaceAccount<'info, TokenAccount>,
            pub owner_token_account: AccountInfo<'info>,
            pub owner: Signer<'info>,
            #[account(mint::token_program = token_program)]
            pub mint: InterfaceAccount<'info, Mint>,
            pub token_program: Interface<'info, TokenInterface>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, WithdrawTokensSubsystemBumps>
        for WithdrawTokensSubsystem<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut WithdrawTokensSubsystemBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let wrapper_account: anchor_lang::accounts::account::Account<
                    WrapperAccount,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("wrapper_account"))?;
                let approver: UncheckedAccount = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("approver"))?;
                let exit_regulator: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("exit_regulator"))?;
                let user_wrapped_token_account: anchor_lang::accounts::interface_account::InterfaceAccount<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("user_wrapped_token_account"))?;
                let to_token_account: anchor_lang::accounts::interface_account::InterfaceAccount<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("to_token_account"))?;
                let owner_token_account: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("owner_token_account"))?;
                let owner: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("owner"))?;
                let mint: anchor_lang::accounts::interface_account::InterfaceAccount<
                    Mint,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("mint"))?;
                let token_program: anchor_lang::accounts::interface::Interface<
                    TokenInterface,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("token_program"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[b"wrapper", approver.key().as_ref()],
                    &__program_id,
                );
                __bumps.wrapper_account = __bump;
                if wrapper_account.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("wrapper_account")
                            .with_pubkeys((wrapper_account.key(), __pda_address)),
                    );
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[
                        b"wrapped_token",
                        wrapper_account.key().as_ref(),
                        mint.key().as_ref(),
                        owner.key().as_ref(),
                    ],
                    &__program_id,
                );
                __bumps.user_wrapped_token_account = __bump;
                if user_wrapped_token_account.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("user_wrapped_token_account")
                            .with_pubkeys((
                                user_wrapped_token_account.key(),
                                __pda_address,
                            )),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&user_wrapped_token_account).is_writable
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("user_wrapped_token_account"),
                    );
                }
                {
                    if user_wrapped_token_account.owner != wrapper_account.key() {
                        return Err(
                            anchor_lang::error::ErrorCode::ConstraintTokenOwner.into(),
                        );
                    }
                    if user_wrapped_token_account.mint != mint.key() {
                        return Err(
                            anchor_lang::error::ErrorCode::ConstraintTokenMint.into(),
                        );
                    }
                }
                if !AsRef::<AccountInfo>::as_ref(&to_token_account).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("to_token_account"),
                    );
                }
                {
                    if to_token_account.owner != owner_token_account.key() {
                        return Err(
                            anchor_lang::error::ErrorCode::ConstraintTokenOwner.into(),
                        );
                    }
                    if to_token_account.mint != mint.key() {
                        return Err(
                            anchor_lang::error::ErrorCode::ConstraintTokenMint.into(),
                        );
                    }
                }
                {
                    if AsRef::<AccountInfo>::as_ref(&mint).owner != &token_program.key()
                    {
                        return Err(
                            anchor_lang::error::ErrorCode::ConstraintMintTokenProgram
                                .into(),
                        );
                    }
                }
                Ok(WithdrawTokensSubsystem {
                    wrapper_account,
                    approver,
                    exit_regulator,
                    user_wrapped_token_account,
                    to_token_account,
                    owner_token_account,
                    owner,
                    mint,
                    token_program,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for WithdrawTokensSubsystem<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.wrapper_account.to_account_infos());
                account_infos.extend(self.approver.to_account_infos());
                account_infos.extend(self.exit_regulator.to_account_infos());
                account_infos.extend(self.user_wrapped_token_account.to_account_infos());
                account_infos.extend(self.to_token_account.to_account_infos());
                account_infos.extend(self.owner_token_account.to_account_infos());
                account_infos.extend(self.owner.to_account_infos());
                account_infos.extend(self.mint.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for WithdrawTokensSubsystem<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.wrapper_account.to_account_metas(None));
                account_metas.extend(self.approver.to_account_metas(None));
                account_metas.extend(self.exit_regulator.to_account_metas(None));
                account_metas
                    .extend(self.user_wrapped_token_account.to_account_metas(None));
                account_metas.extend(self.to_token_account.to_account_metas(None));
                account_metas.extend(self.owner_token_account.to_account_metas(None));
                account_metas.extend(self.owner.to_account_metas(None));
                account_metas.extend(self.mint.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for WithdrawTokensSubsystem<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(
                        &self.user_wrapped_token_account,
                        program_id,
                    )
                    .map_err(|e| e.with_account_name("user_wrapped_token_account"))?;
                anchor_lang::AccountsExit::exit(&self.to_token_account, program_id)
                    .map_err(|e| e.with_account_name("to_token_account"))?;
                Ok(())
            }
        }
        pub struct WithdrawTokensSubsystemBumps {
            pub wrapper_account: u8,
            pub user_wrapped_token_account: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for WithdrawTokensSubsystemBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "WithdrawTokensSubsystemBumps",
                    "wrapper_account",
                    &self.wrapper_account,
                    "user_wrapped_token_account",
                    &&self.user_wrapped_token_account,
                )
            }
        }
        impl Default for WithdrawTokensSubsystemBumps {
            fn default() -> Self {
                WithdrawTokensSubsystemBumps {
                    wrapper_account: u8::MAX,
                    user_wrapped_token_account: u8::MAX,
                }
            }
        }
        impl<'info> anchor_lang::Bumps for WithdrawTokensSubsystem<'info>
        where
            'info: 'info,
        {
            type Bumps = WithdrawTokensSubsystemBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_withdraw_tokens_subsystem {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`WithdrawTokensSubsystem`].
            pub struct WithdrawTokensSubsystem {
                pub wrapper_account: Pubkey,
                pub approver: Pubkey,
                pub exit_regulator: Pubkey,
                pub user_wrapped_token_account: Pubkey,
                pub to_token_account: Pubkey,
                pub owner_token_account: Pubkey,
                pub owner: Pubkey,
                pub mint: Pubkey,
                pub token_program: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for WithdrawTokensSubsystem
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.wrapper_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.approver, writer)?;
                    borsh::BorshSerialize::serialize(&self.exit_regulator, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.user_wrapped_token_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(&self.to_token_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.owner_token_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.owner, writer)?;
                    borsh::BorshSerialize::serialize(&self.mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for WithdrawTokensSubsystem {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.wrapper_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.approver,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.exit_regulator,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.user_wrapped_token_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.to_token_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.owner_token_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.owner,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.mint,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.token_program,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_withdraw_tokens_subsystem {
            use super::*;
            /// Generated CPI struct of the accounts for [`WithdrawTokensSubsystem`].
            pub struct WithdrawTokensSubsystem<'info> {
                pub wrapper_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub approver: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub exit_regulator: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub user_wrapped_token_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub to_token_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub owner_token_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub owner: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for WithdrawTokensSubsystem<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.wrapper_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.approver),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.exit_regulator),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.user_wrapped_token_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.to_token_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.owner_token_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.owner),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.mint),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.token_program),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info>
            for WithdrawTokensSubsystem<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.wrapper_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.approver),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.exit_regulator,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.user_wrapped_token_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.to_token_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.owner_token_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.owner),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.mint),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.token_program,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub fn _initialize_subsystem(
            ctx: Context<InitializeSubsystem>,
            list_issuer: Vec<Pubkey>,
            exit_regulators: Vec<Pubkey>,
        ) -> Result<()> {
            let wrapper_account = &mut ctx.accounts.wrapper_account;
            wrapper_account.id_issuers = list_issuer;
            wrapper_account.exit_regulators = exit_regulators;
            wrapper_account.bump = ctx.bumps.wrapper_account;
            Ok(())
        }
        pub fn _add_issuers_subsystem(
            ctx: Context<AddSubsystemIssuer>,
            issuer: Pubkey,
        ) -> Result<()> {
            let wrapper_account = &mut ctx.accounts.wrapper_account;
            wrapper_account.id_issuers.push(issuer);
            Ok(())
        }
        pub fn _remove_issuer_subsystem(
            ctx: Context<DeleteSubsystemIssuer>,
        ) -> Result<()> {
            let wrapper_account = &mut ctx.accounts.wrapper_account;
            let index = wrapper_account
                .id_issuers
                .iter()
                .position(|x| *x == ctx.accounts.issuer.key())
                .unwrap();
            wrapper_account.id_issuers.remove(index);
            Ok(())
        }
        pub fn _deposit_token_subsystem(
            ctx: Context<DepositTokensSubsystem>,
            amount: u64,
            decimals: u8,
        ) -> Result<()> {
            let mint = &ctx.accounts.mint;
            if mint.decimals != decimals {
                return Err(WrapperError::InvalidDecimals.into());
            }
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
        pub fn _withdraw_token_subsystem(
            ctx: Context<WithdrawTokensSubsystem>,
            amount: u64,
            decimals: u8,
        ) -> Result<()> {
            let wrapper_account = &ctx.accounts.wrapper_account;
            let exit_regulator = &ctx.accounts.exit_regulator;
            if !wrapper_account.exit_regulators.contains(exit_regulator.key) {
                return Err(WrapperError::InvalidExitRegulator.into());
            }
            let mint = &ctx.accounts.mint;
            if mint.decimals != decimals {
                return Err(WrapperError::InvalidDecimals.into());
            }
            let approver = ctx.accounts.approver.key();
            let bump = ctx.bumps.wrapper_account;
            let seed: &[&[&[u8]]] = &[&[b"wrapper", approver.as_ref(), &[bump]]];
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
                seed,
            )?;
            Ok(())
        }
    }
    pub use subsystem::*;
    pub mod confidential_wrapper {
        use anchor_lang::{prelude::*, solana_program::program};
        use anchor_spl::{
            token::spl_token, token_interface::{Mint, TokenAccount, TokenInterface},
        };
        use crate::{error::WrapperError, WrapperAccount};
        #[instruction(list_issuer:Vec<Pubkey>, exit_regulators:Vec<Pubkey>)]
        pub struct InitializeWrapper<'info> {
            #[account(
                init,
                seeds = [b"wrapper",
                approver.key().as_ref()],
                bump,
                payer = payer,
                space = WrapperAccount::get_init_len(&list_issuer, &exit_regulators)
            )]
            pub wrapper_account: Account<'info, WrapperAccount>,
            #[account(mut)]
            pub payer: Signer<'info>,
            pub approver: Signer<'info>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, InitializeWrapperBumps>
        for InitializeWrapper<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut InitializeWrapperBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let mut __ix_data = __ix_data;
                struct __Args {
                    list_issuer: Vec<Pubkey>,
                    exit_regulators: Vec<Pubkey>,
                }
                impl borsh::ser::BorshSerialize for __Args
                where
                    Vec<Pubkey>: borsh::ser::BorshSerialize,
                    Vec<Pubkey>: borsh::ser::BorshSerialize,
                {
                    fn serialize<W: borsh::maybestd::io::Write>(
                        &self,
                        writer: &mut W,
                    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                        borsh::BorshSerialize::serialize(&self.list_issuer, writer)?;
                        borsh::BorshSerialize::serialize(&self.exit_regulators, writer)?;
                        Ok(())
                    }
                }
                impl borsh::de::BorshDeserialize for __Args
                where
                    Vec<Pubkey>: borsh::BorshDeserialize,
                    Vec<Pubkey>: borsh::BorshDeserialize,
                {
                    fn deserialize_reader<R: borsh::maybestd::io::Read>(
                        reader: &mut R,
                    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                        Ok(Self {
                            list_issuer: borsh::BorshDeserialize::deserialize_reader(
                                reader,
                            )?,
                            exit_regulators: borsh::BorshDeserialize::deserialize_reader(
                                reader,
                            )?,
                        })
                    }
                }
                let __Args { list_issuer, exit_regulators } = __Args::deserialize(
                        &mut __ix_data,
                    )
                    .map_err(|_| {
                        anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                    })?;
                if __accounts.is_empty() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into(),
                    );
                }
                let wrapper_account = &__accounts[0];
                *__accounts = &__accounts[1..];
                let payer: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("payer"))?;
                let approver: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("approver"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                let __anchor_rent = Rent::get()?;
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[b"wrapper", approver.key().as_ref()],
                    __program_id,
                );
                __bumps.wrapper_account = __bump;
                if wrapper_account.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("wrapper_account")
                            .with_pubkeys((wrapper_account.key(), __pda_address)),
                    );
                }
                let wrapper_account = {
                    let actual_field = AsRef::<AccountInfo>::as_ref(&wrapper_account);
                    let actual_owner = actual_field.owner;
                    let space = WrapperAccount::get_init_len(
                        &list_issuer,
                        &exit_regulators,
                    );
                    let pa: anchor_lang::accounts::account::Account<WrapperAccount> = if !false
                        || actual_owner
                            == &anchor_lang::solana_program::system_program::ID
                    {
                        let __current_lamports = wrapper_account.lamports();
                        if __current_lamports == 0 {
                            let space = space;
                            let lamports = __anchor_rent.minimum_balance(space);
                            let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                from: payer.to_account_info(),
                                to: wrapper_account.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::create_account(
                                cpi_context
                                    .with_signer(
                                        &[&[b"wrapper", approver.key().as_ref(), &[__bump][..]][..]],
                                    ),
                                lamports,
                                space as u64,
                                __program_id,
                            )?;
                        } else {
                            if payer.key() == wrapper_account.key() {
                                return Err(
                                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                            error_name: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .name(),
                                            error_code_number: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .into(),
                                            error_msg: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .to_string(),
                                            error_origin: Some(
                                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                    filename: "programs/asset_based/src/instructions/confidential_wrapper.rs",
                                                    line: 8u32,
                                                }),
                                            ),
                                            compared_values: None,
                                        })
                                        .with_pubkeys((payer.key(), wrapper_account.key())),
                                );
                            }
                            let required_lamports = __anchor_rent
                                .minimum_balance(space)
                                .max(1)
                                .saturating_sub(__current_lamports);
                            if required_lamports > 0 {
                                let cpi_accounts = anchor_lang::system_program::Transfer {
                                    from: payer.to_account_info(),
                                    to: wrapper_account.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::transfer(
                                    cpi_context,
                                    required_lamports,
                                )?;
                            }
                            let cpi_accounts = anchor_lang::system_program::Allocate {
                                account_to_allocate: wrapper_account.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::allocate(
                                cpi_context
                                    .with_signer(
                                        &[&[b"wrapper", approver.key().as_ref(), &[__bump][..]][..]],
                                    ),
                                space as u64,
                            )?;
                            let cpi_accounts = anchor_lang::system_program::Assign {
                                account_to_assign: wrapper_account.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::assign(
                                cpi_context
                                    .with_signer(
                                        &[&[b"wrapper", approver.key().as_ref(), &[__bump][..]][..]],
                                    ),
                                __program_id,
                            )?;
                        }
                        match anchor_lang::accounts::account::Account::try_from_unchecked(
                            &wrapper_account,
                        ) {
                            Ok(val) => val,
                            Err(e) => return Err(e.with_account_name("wrapper_account")),
                        }
                    } else {
                        match anchor_lang::accounts::account::Account::try_from(
                            &wrapper_account,
                        ) {
                            Ok(val) => val,
                            Err(e) => return Err(e.with_account_name("wrapper_account")),
                        }
                    };
                    if false {
                        if space != actual_field.data_len() {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::ConstraintSpace,
                                    )
                                    .with_account_name("wrapper_account")
                                    .with_values((space, actual_field.data_len())),
                            );
                        }
                        if actual_owner != __program_id {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::ConstraintOwner,
                                    )
                                    .with_account_name("wrapper_account")
                                    .with_pubkeys((*actual_owner, *__program_id)),
                            );
                        }
                        {
                            let required_lamports = __anchor_rent.minimum_balance(space);
                            if pa.to_account_info().lamports() < required_lamports {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintRentExempt,
                                        )
                                        .with_account_name("wrapper_account"),
                                );
                            }
                        }
                    }
                    pa
                };
                if !AsRef::<AccountInfo>::as_ref(&wrapper_account).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("wrapper_account"),
                    );
                }
                if !__anchor_rent
                    .is_exempt(
                        wrapper_account.to_account_info().lamports(),
                        wrapper_account.to_account_info().try_data_len()?,
                    )
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                            )
                            .with_account_name("wrapper_account"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&payer).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("payer"),
                    );
                }
                Ok(InitializeWrapper {
                    wrapper_account,
                    payer,
                    approver,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeWrapper<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.wrapper_account.to_account_infos());
                account_infos.extend(self.payer.to_account_infos());
                account_infos.extend(self.approver.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for InitializeWrapper<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.wrapper_account.to_account_metas(None));
                account_metas.extend(self.payer.to_account_metas(None));
                account_metas.extend(self.approver.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for InitializeWrapper<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.wrapper_account, program_id)
                    .map_err(|e| e.with_account_name("wrapper_account"))?;
                anchor_lang::AccountsExit::exit(&self.payer, program_id)
                    .map_err(|e| e.with_account_name("payer"))?;
                Ok(())
            }
        }
        pub struct InitializeWrapperBumps {
            pub wrapper_account: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for InitializeWrapperBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "InitializeWrapperBumps",
                    "wrapper_account",
                    &&self.wrapper_account,
                )
            }
        }
        impl Default for InitializeWrapperBumps {
            fn default() -> Self {
                InitializeWrapperBumps {
                    wrapper_account: u8::MAX,
                }
            }
        }
        impl<'info> anchor_lang::Bumps for InitializeWrapper<'info>
        where
            'info: 'info,
        {
            type Bumps = InitializeWrapperBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_initialize_wrapper {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`InitializeWrapper`].
            pub struct InitializeWrapper {
                pub wrapper_account: Pubkey,
                pub payer: Pubkey,
                pub approver: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for InitializeWrapper
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.wrapper_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.payer, writer)?;
                    borsh::BorshSerialize::serialize(&self.approver, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for InitializeWrapper {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.wrapper_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.payer,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.approver,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_initialize_wrapper {
            use super::*;
            /// Generated CPI struct of the accounts for [`InitializeWrapper`].
            pub struct InitializeWrapper<'info> {
                pub wrapper_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub payer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub approver: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for InitializeWrapper<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.wrapper_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.payer),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.approver),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeWrapper<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.wrapper_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.payer),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.approver),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct AddWrapperIssuer<'info> {
            #[account(
                mut,
                seeds = [b"wrapper",
                approver.key().as_ref()],
                bump,
                realloc = wrapper_account.get_len_add_address(),
                realloc::payer = payer,
                realloc::zero = false
            )]
            pub wrapper_account: Account<'info, WrapperAccount>,
            #[account(mut)]
            pub payer: Signer<'info>,
            pub approver: Signer<'info>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, AddWrapperIssuerBumps>
        for AddWrapperIssuer<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut AddWrapperIssuerBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let wrapper_account: anchor_lang::accounts::account::Account<
                    WrapperAccount,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("wrapper_account"))?;
                let payer: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("payer"))?;
                let approver: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("approver"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                if __reallocs.contains(&wrapper_account.key()) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::AccountDuplicateReallocs,
                            )
                            .with_account_name("wrapper_account"),
                    );
                }
                let __anchor_rent = anchor_lang::prelude::Rent::get()?;
                let __field_info = wrapper_account.to_account_info();
                let __new_rent_minimum = __anchor_rent
                    .minimum_balance(wrapper_account.get_len_add_address());
                let __delta_space = (::std::convert::TryInto::<
                    isize,
                >::try_into(wrapper_account.get_len_add_address())
                    .unwrap())
                    .checked_sub(
                        ::std::convert::TryInto::try_into(__field_info.data_len())
                            .unwrap(),
                    )
                    .unwrap();
                if __delta_space != 0 {
                    if __delta_space > 0 {
                        if ::std::convert::TryInto::<usize>::try_into(__delta_space)
                            .unwrap()
                            > anchor_lang::solana_program::entrypoint::MAX_PERMITTED_DATA_INCREASE
                        {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::AccountReallocExceedsLimit,
                                    )
                                    .with_account_name("wrapper_account"),
                            );
                        }
                        if __new_rent_minimum > __field_info.lamports() {
                            anchor_lang::system_program::transfer(
                                anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    anchor_lang::system_program::Transfer {
                                        from: payer.to_account_info(),
                                        to: __field_info.clone(),
                                    },
                                ),
                                __new_rent_minimum
                                    .checked_sub(__field_info.lamports())
                                    .unwrap(),
                            )?;
                        }
                    } else {
                        let __lamport_amt = __field_info
                            .lamports()
                            .checked_sub(__new_rent_minimum)
                            .unwrap();
                        **payer.to_account_info().lamports.borrow_mut() = payer
                            .to_account_info()
                            .lamports()
                            .checked_add(__lamport_amt)
                            .unwrap();
                        **__field_info.lamports.borrow_mut() = __field_info
                            .lamports()
                            .checked_sub(__lamport_amt)
                            .unwrap();
                    }
                    __field_info.realloc(wrapper_account.get_len_add_address(), false)?;
                    __reallocs.insert(wrapper_account.key());
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[b"wrapper", approver.key().as_ref()],
                    &__program_id,
                );
                __bumps.wrapper_account = __bump;
                if wrapper_account.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("wrapper_account")
                            .with_pubkeys((wrapper_account.key(), __pda_address)),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&wrapper_account).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("wrapper_account"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&payer).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("payer"),
                    );
                }
                Ok(AddWrapperIssuer {
                    wrapper_account,
                    payer,
                    approver,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for AddWrapperIssuer<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.wrapper_account.to_account_infos());
                account_infos.extend(self.payer.to_account_infos());
                account_infos.extend(self.approver.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for AddWrapperIssuer<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.wrapper_account.to_account_metas(None));
                account_metas.extend(self.payer.to_account_metas(None));
                account_metas.extend(self.approver.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for AddWrapperIssuer<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.wrapper_account, program_id)
                    .map_err(|e| e.with_account_name("wrapper_account"))?;
                anchor_lang::AccountsExit::exit(&self.payer, program_id)
                    .map_err(|e| e.with_account_name("payer"))?;
                Ok(())
            }
        }
        pub struct AddWrapperIssuerBumps {
            pub wrapper_account: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AddWrapperIssuerBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "AddWrapperIssuerBumps",
                    "wrapper_account",
                    &&self.wrapper_account,
                )
            }
        }
        impl Default for AddWrapperIssuerBumps {
            fn default() -> Self {
                AddWrapperIssuerBumps {
                    wrapper_account: u8::MAX,
                }
            }
        }
        impl<'info> anchor_lang::Bumps for AddWrapperIssuer<'info>
        where
            'info: 'info,
        {
            type Bumps = AddWrapperIssuerBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_add_wrapper_issuer {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`AddWrapperIssuer`].
            pub struct AddWrapperIssuer {
                pub wrapper_account: Pubkey,
                pub payer: Pubkey,
                pub approver: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for AddWrapperIssuer
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.wrapper_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.payer, writer)?;
                    borsh::BorshSerialize::serialize(&self.approver, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for AddWrapperIssuer {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.wrapper_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.payer,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.approver,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_add_wrapper_issuer {
            use super::*;
            /// Generated CPI struct of the accounts for [`AddWrapperIssuer`].
            pub struct AddWrapperIssuer<'info> {
                pub wrapper_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub payer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub approver: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for AddWrapperIssuer<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.wrapper_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.payer),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.approver),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for AddWrapperIssuer<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.wrapper_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.payer),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.approver),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct DeleteWrapperIssuer<'info> {
            #[account(
                mut,
                seeds = [b"wrapper",
                approver.key().as_ref()],
                bump,
                realloc = wrapper_account.get_len_remove_address(),
                realloc::payer = payer,
                realloc::zero = true
            )]
            pub wrapper_account: Account<'info, WrapperAccount>,
            #[account(mut)]
            pub payer: Signer<'info>,
            pub approver: Signer<'info>,
            /// CHECK: The issuer to be removed
            #[account(constraint = wrapper_account.id_issuers.contains(&issuer.key()))]
            pub issuer: UncheckedAccount<'info>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, DeleteWrapperIssuerBumps>
        for DeleteWrapperIssuer<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut DeleteWrapperIssuerBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let wrapper_account: anchor_lang::accounts::account::Account<
                    WrapperAccount,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("wrapper_account"))?;
                let payer: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("payer"))?;
                let approver: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("approver"))?;
                let issuer: UncheckedAccount = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("issuer"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                if __reallocs.contains(&wrapper_account.key()) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::AccountDuplicateReallocs,
                            )
                            .with_account_name("wrapper_account"),
                    );
                }
                let __anchor_rent = anchor_lang::prelude::Rent::get()?;
                let __field_info = wrapper_account.to_account_info();
                let __new_rent_minimum = __anchor_rent
                    .minimum_balance(wrapper_account.get_len_remove_address());
                let __delta_space = (::std::convert::TryInto::<
                    isize,
                >::try_into(wrapper_account.get_len_remove_address())
                    .unwrap())
                    .checked_sub(
                        ::std::convert::TryInto::try_into(__field_info.data_len())
                            .unwrap(),
                    )
                    .unwrap();
                if __delta_space != 0 {
                    if __delta_space > 0 {
                        if ::std::convert::TryInto::<usize>::try_into(__delta_space)
                            .unwrap()
                            > anchor_lang::solana_program::entrypoint::MAX_PERMITTED_DATA_INCREASE
                        {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::AccountReallocExceedsLimit,
                                    )
                                    .with_account_name("wrapper_account"),
                            );
                        }
                        if __new_rent_minimum > __field_info.lamports() {
                            anchor_lang::system_program::transfer(
                                anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    anchor_lang::system_program::Transfer {
                                        from: payer.to_account_info(),
                                        to: __field_info.clone(),
                                    },
                                ),
                                __new_rent_minimum
                                    .checked_sub(__field_info.lamports())
                                    .unwrap(),
                            )?;
                        }
                    } else {
                        let __lamport_amt = __field_info
                            .lamports()
                            .checked_sub(__new_rent_minimum)
                            .unwrap();
                        **payer.to_account_info().lamports.borrow_mut() = payer
                            .to_account_info()
                            .lamports()
                            .checked_add(__lamport_amt)
                            .unwrap();
                        **__field_info.lamports.borrow_mut() = __field_info
                            .lamports()
                            .checked_sub(__lamport_amt)
                            .unwrap();
                    }
                    __field_info
                        .realloc(wrapper_account.get_len_remove_address(), true)?;
                    __reallocs.insert(wrapper_account.key());
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[b"wrapper", approver.key().as_ref()],
                    &__program_id,
                );
                __bumps.wrapper_account = __bump;
                if wrapper_account.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("wrapper_account")
                            .with_pubkeys((wrapper_account.key(), __pda_address)),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&wrapper_account).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("wrapper_account"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&payer).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("payer"),
                    );
                }
                if !(wrapper_account.id_issuers.contains(&issuer.key())) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("issuer"),
                    );
                }
                Ok(DeleteWrapperIssuer {
                    wrapper_account,
                    payer,
                    approver,
                    issuer,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for DeleteWrapperIssuer<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.wrapper_account.to_account_infos());
                account_infos.extend(self.payer.to_account_infos());
                account_infos.extend(self.approver.to_account_infos());
                account_infos.extend(self.issuer.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for DeleteWrapperIssuer<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.wrapper_account.to_account_metas(None));
                account_metas.extend(self.payer.to_account_metas(None));
                account_metas.extend(self.approver.to_account_metas(None));
                account_metas.extend(self.issuer.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for DeleteWrapperIssuer<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.wrapper_account, program_id)
                    .map_err(|e| e.with_account_name("wrapper_account"))?;
                anchor_lang::AccountsExit::exit(&self.payer, program_id)
                    .map_err(|e| e.with_account_name("payer"))?;
                Ok(())
            }
        }
        pub struct DeleteWrapperIssuerBumps {
            pub wrapper_account: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DeleteWrapperIssuerBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "DeleteWrapperIssuerBumps",
                    "wrapper_account",
                    &&self.wrapper_account,
                )
            }
        }
        impl Default for DeleteWrapperIssuerBumps {
            fn default() -> Self {
                DeleteWrapperIssuerBumps {
                    wrapper_account: u8::MAX,
                }
            }
        }
        impl<'info> anchor_lang::Bumps for DeleteWrapperIssuer<'info>
        where
            'info: 'info,
        {
            type Bumps = DeleteWrapperIssuerBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_delete_wrapper_issuer {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`DeleteWrapperIssuer`].
            pub struct DeleteWrapperIssuer {
                pub wrapper_account: Pubkey,
                pub payer: Pubkey,
                pub approver: Pubkey,
                pub issuer: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for DeleteWrapperIssuer
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.wrapper_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.payer, writer)?;
                    borsh::BorshSerialize::serialize(&self.approver, writer)?;
                    borsh::BorshSerialize::serialize(&self.issuer, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for DeleteWrapperIssuer {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.wrapper_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.payer,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.approver,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.issuer,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_delete_wrapper_issuer {
            use super::*;
            /// Generated CPI struct of the accounts for [`DeleteWrapperIssuer`].
            pub struct DeleteWrapperIssuer<'info> {
                pub wrapper_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub payer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub approver: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub issuer: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for DeleteWrapperIssuer<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.wrapper_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.payer),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.approver),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.issuer),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info>
            for DeleteWrapperIssuer<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.wrapper_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.payer),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.approver),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.issuer),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct WrapTokens<'info> {
            #[account(seeds = [b"wrapper", approver.key().as_ref()], bump)]
            pub wrapper_account: Account<'info, WrapperAccount>,
            /// CHECK: The approver of the wrapper
            pub approver: UncheckedAccount<'info>,
            #[account(
                init_if_needed,
                token::authority = wrapper_account,
                token::mint = mint,
                seeds = [b"wrapped_token",
                wrapper_account.key().as_ref(),
                mint.key().as_ref(),
                owner_to_account.key().as_ref()],
                bump,
                payer = owner_from_token_account
            )]
            pub to_wrapped_token_account: InterfaceAccount<'info, TokenAccount>,
            #[account(
                mut,
                token::authority = owner_from_token_account,
                token::mint = mint
            )]
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
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, WrapTokensBumps> for WrapTokens<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut WrapTokensBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let wrapper_account: anchor_lang::accounts::account::Account<
                    WrapperAccount,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("wrapper_account"))?;
                let approver: UncheckedAccount = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("approver"))?;
                if __accounts.is_empty() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into(),
                    );
                }
                let to_wrapped_token_account = &__accounts[0];
                *__accounts = &__accounts[1..];
                let from_token_account: anchor_lang::accounts::interface_account::InterfaceAccount<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("from_token_account"))?;
                let owner_from_token_account: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("owner_from_token_account"))?;
                let owner_to_account: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("owner_to_account"))?;
                let mint: anchor_lang::accounts::interface_account::InterfaceAccount<
                    Mint,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("mint"))?;
                let token_program: anchor_lang::accounts::interface::Interface<
                    TokenInterface,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("token_program"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                let __anchor_rent = Rent::get()?;
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[
                        b"wrapped_token",
                        wrapper_account.key().as_ref(),
                        mint.key().as_ref(),
                        owner_to_account.key().as_ref(),
                    ],
                    __program_id,
                );
                __bumps.to_wrapped_token_account = __bump;
                if to_wrapped_token_account.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("to_wrapped_token_account")
                            .with_pubkeys((
                                to_wrapped_token_account.key(),
                                __pda_address,
                            )),
                    );
                }
                let to_wrapped_token_account: anchor_lang::accounts::interface_account::InterfaceAccount<
                    TokenAccount,
                > = {
                    let owner_program = AsRef::<
                        AccountInfo,
                    >::as_ref(&to_wrapped_token_account)
                        .owner;
                    if !true
                        || owner_program
                            == &anchor_lang::solana_program::system_program::ID
                    {
                        let __current_lamports = to_wrapped_token_account.lamports();
                        if __current_lamports == 0 {
                            let space = {
                                let mint_info = mint.to_account_info();
                                if *mint_info.owner
                                    == ::anchor_spl::token_2022::Token2022::id()
                                {
                                    use ::anchor_spl::token_2022::spl_token_2022::extension::{
                                        BaseStateWithExtensions, ExtensionType, StateWithExtensions,
                                    };
                                    use ::anchor_spl::token_2022::spl_token_2022::state::{
                                        Account, Mint,
                                    };
                                    let mint_data = mint_info.try_borrow_data()?;
                                    let mint_state = StateWithExtensions::<
                                        Mint,
                                    >::unpack(&mint_data)?;
                                    let mint_extensions = mint_state.get_extension_types()?;
                                    let required_extensions = ExtensionType::get_required_init_account_extensions(
                                        &mint_extensions,
                                    );
                                    ExtensionType::try_calculate_account_len::<
                                        Account,
                                    >(&required_extensions)?
                                } else {
                                    ::anchor_spl::token::TokenAccount::LEN
                                }
                            };
                            let lamports = __anchor_rent.minimum_balance(space);
                            let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                from: owner_from_token_account.to_account_info(),
                                to: to_wrapped_token_account.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::create_account(
                                cpi_context
                                    .with_signer(
                                        &[
                                            &[
                                                b"wrapped_token",
                                                wrapper_account.key().as_ref(),
                                                mint.key().as_ref(),
                                                owner_to_account.key().as_ref(),
                                                &[__bump][..],
                                            ][..],
                                        ],
                                    ),
                                lamports,
                                space as u64,
                                &token_program.key(),
                            )?;
                        } else {
                            if owner_from_token_account.key()
                                == to_wrapped_token_account.key()
                            {
                                return Err(
                                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                            error_name: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .name(),
                                            error_code_number: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .into(),
                                            error_msg: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .to_string(),
                                            error_origin: Some(
                                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                    filename: "programs/asset_based/src/instructions/confidential_wrapper.rs",
                                                    line: 42u32,
                                                }),
                                            ),
                                            compared_values: None,
                                        })
                                        .with_pubkeys((
                                            owner_from_token_account.key(),
                                            to_wrapped_token_account.key(),
                                        )),
                                );
                            }
                            let required_lamports = __anchor_rent
                                .minimum_balance({
                                    let mint_info = mint.to_account_info();
                                    if *mint_info.owner
                                        == ::anchor_spl::token_2022::Token2022::id()
                                    {
                                        use ::anchor_spl::token_2022::spl_token_2022::extension::{
                                            BaseStateWithExtensions, ExtensionType, StateWithExtensions,
                                        };
                                        use ::anchor_spl::token_2022::spl_token_2022::state::{
                                            Account, Mint,
                                        };
                                        let mint_data = mint_info.try_borrow_data()?;
                                        let mint_state = StateWithExtensions::<
                                            Mint,
                                        >::unpack(&mint_data)?;
                                        let mint_extensions = mint_state.get_extension_types()?;
                                        let required_extensions = ExtensionType::get_required_init_account_extensions(
                                            &mint_extensions,
                                        );
                                        ExtensionType::try_calculate_account_len::<
                                            Account,
                                        >(&required_extensions)?
                                    } else {
                                        ::anchor_spl::token::TokenAccount::LEN
                                    }
                                })
                                .max(1)
                                .saturating_sub(__current_lamports);
                            if required_lamports > 0 {
                                let cpi_accounts = anchor_lang::system_program::Transfer {
                                    from: owner_from_token_account.to_account_info(),
                                    to: to_wrapped_token_account.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::transfer(
                                    cpi_context,
                                    required_lamports,
                                )?;
                            }
                            let cpi_accounts = anchor_lang::system_program::Allocate {
                                account_to_allocate: to_wrapped_token_account
                                    .to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::allocate(
                                cpi_context
                                    .with_signer(
                                        &[
                                            &[
                                                b"wrapped_token",
                                                wrapper_account.key().as_ref(),
                                                mint.key().as_ref(),
                                                owner_to_account.key().as_ref(),
                                                &[__bump][..],
                                            ][..],
                                        ],
                                    ),
                                {
                                    let mint_info = mint.to_account_info();
                                    if *mint_info.owner
                                        == ::anchor_spl::token_2022::Token2022::id()
                                    {
                                        use ::anchor_spl::token_2022::spl_token_2022::extension::{
                                            BaseStateWithExtensions, ExtensionType, StateWithExtensions,
                                        };
                                        use ::anchor_spl::token_2022::spl_token_2022::state::{
                                            Account, Mint,
                                        };
                                        let mint_data = mint_info.try_borrow_data()?;
                                        let mint_state = StateWithExtensions::<
                                            Mint,
                                        >::unpack(&mint_data)?;
                                        let mint_extensions = mint_state.get_extension_types()?;
                                        let required_extensions = ExtensionType::get_required_init_account_extensions(
                                            &mint_extensions,
                                        );
                                        ExtensionType::try_calculate_account_len::<
                                            Account,
                                        >(&required_extensions)?
                                    } else {
                                        ::anchor_spl::token::TokenAccount::LEN
                                    }
                                } as u64,
                            )?;
                            let cpi_accounts = anchor_lang::system_program::Assign {
                                account_to_assign: to_wrapped_token_account
                                    .to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::assign(
                                cpi_context
                                    .with_signer(
                                        &[
                                            &[
                                                b"wrapped_token",
                                                wrapper_account.key().as_ref(),
                                                mint.key().as_ref(),
                                                owner_to_account.key().as_ref(),
                                                &[__bump][..],
                                            ][..],
                                        ],
                                    ),
                                &token_program.key(),
                            )?;
                        }
                        let cpi_program = token_program.to_account_info();
                        let accounts = ::anchor_spl::token_interface::InitializeAccount3 {
                            account: to_wrapped_token_account.to_account_info(),
                            mint: mint.to_account_info(),
                            authority: wrapper_account.to_account_info(),
                        };
                        let cpi_ctx = anchor_lang::context::CpiContext::new(
                            cpi_program,
                            accounts,
                        );
                        ::anchor_spl::token_interface::initialize_account3(cpi_ctx)?;
                    }
                    let pa: anchor_lang::accounts::interface_account::InterfaceAccount<
                        TokenAccount,
                    > = match anchor_lang::accounts::interface_account::InterfaceAccount::try_from_unchecked(
                        &to_wrapped_token_account,
                    ) {
                        Ok(val) => val,
                        Err(e) => {
                            return Err(e.with_account_name("to_wrapped_token_account"));
                        }
                    };
                    if true {
                        if pa.mint != mint.key() {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::ConstraintTokenMint,
                                    )
                                    .with_account_name("to_wrapped_token_account")
                                    .with_pubkeys((pa.mint, mint.key())),
                            );
                        }
                        if pa.owner != wrapper_account.key() {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                                    )
                                    .with_account_name("to_wrapped_token_account")
                                    .with_pubkeys((pa.owner, wrapper_account.key())),
                            );
                        }
                        if owner_program != &token_program.key() {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::ConstraintTokenTokenProgram,
                                    )
                                    .with_account_name("to_wrapped_token_account")
                                    .with_pubkeys((*owner_program, token_program.key())),
                            );
                        }
                    }
                    pa
                };
                if !AsRef::<AccountInfo>::as_ref(&to_wrapped_token_account).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("to_wrapped_token_account"),
                    );
                }
                if !__anchor_rent
                    .is_exempt(
                        to_wrapped_token_account.to_account_info().lamports(),
                        to_wrapped_token_account.to_account_info().try_data_len()?,
                    )
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                            )
                            .with_account_name("to_wrapped_token_account"),
                    );
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[b"wrapper", approver.key().as_ref()],
                    &__program_id,
                );
                __bumps.wrapper_account = __bump;
                if wrapper_account.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("wrapper_account")
                            .with_pubkeys((wrapper_account.key(), __pda_address)),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&from_token_account).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("from_token_account"),
                    );
                }
                {
                    if from_token_account.owner != owner_from_token_account.key() {
                        return Err(
                            anchor_lang::error::ErrorCode::ConstraintTokenOwner.into(),
                        );
                    }
                    if from_token_account.mint != mint.key() {
                        return Err(
                            anchor_lang::error::ErrorCode::ConstraintTokenMint.into(),
                        );
                    }
                }
                if !AsRef::<AccountInfo>::as_ref(&owner_from_token_account).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("owner_from_token_account"),
                    );
                }
                {
                    if AsRef::<AccountInfo>::as_ref(&mint).owner != &token_program.key()
                    {
                        return Err(
                            anchor_lang::error::ErrorCode::ConstraintMintTokenProgram
                                .into(),
                        );
                    }
                }
                Ok(WrapTokens {
                    wrapper_account,
                    approver,
                    to_wrapped_token_account,
                    from_token_account,
                    owner_from_token_account,
                    owner_to_account,
                    mint,
                    token_program,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for WrapTokens<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.wrapper_account.to_account_infos());
                account_infos.extend(self.approver.to_account_infos());
                account_infos.extend(self.to_wrapped_token_account.to_account_infos());
                account_infos.extend(self.from_token_account.to_account_infos());
                account_infos.extend(self.owner_from_token_account.to_account_infos());
                account_infos.extend(self.owner_to_account.to_account_infos());
                account_infos.extend(self.mint.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for WrapTokens<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.wrapper_account.to_account_metas(None));
                account_metas.extend(self.approver.to_account_metas(None));
                account_metas
                    .extend(self.to_wrapped_token_account.to_account_metas(None));
                account_metas.extend(self.from_token_account.to_account_metas(None));
                account_metas
                    .extend(self.owner_from_token_account.to_account_metas(None));
                account_metas.extend(self.owner_to_account.to_account_metas(None));
                account_metas.extend(self.mint.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for WrapTokens<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(
                        &self.to_wrapped_token_account,
                        program_id,
                    )
                    .map_err(|e| e.with_account_name("to_wrapped_token_account"))?;
                anchor_lang::AccountsExit::exit(&self.from_token_account, program_id)
                    .map_err(|e| e.with_account_name("from_token_account"))?;
                anchor_lang::AccountsExit::exit(
                        &self.owner_from_token_account,
                        program_id,
                    )
                    .map_err(|e| e.with_account_name("owner_from_token_account"))?;
                Ok(())
            }
        }
        pub struct WrapTokensBumps {
            pub wrapper_account: u8,
            pub to_wrapped_token_account: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for WrapTokensBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "WrapTokensBumps",
                    "wrapper_account",
                    &self.wrapper_account,
                    "to_wrapped_token_account",
                    &&self.to_wrapped_token_account,
                )
            }
        }
        impl Default for WrapTokensBumps {
            fn default() -> Self {
                WrapTokensBumps {
                    wrapper_account: u8::MAX,
                    to_wrapped_token_account: u8::MAX,
                }
            }
        }
        impl<'info> anchor_lang::Bumps for WrapTokens<'info>
        where
            'info: 'info,
        {
            type Bumps = WrapTokensBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_wrap_tokens {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`WrapTokens`].
            pub struct WrapTokens {
                pub wrapper_account: Pubkey,
                pub approver: Pubkey,
                pub to_wrapped_token_account: Pubkey,
                pub from_token_account: Pubkey,
                pub owner_from_token_account: Pubkey,
                pub owner_to_account: Pubkey,
                pub mint: Pubkey,
                pub token_program: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for WrapTokens
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.wrapper_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.approver, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.to_wrapped_token_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(&self.from_token_account, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.owner_from_token_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(&self.owner_to_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for WrapTokens {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.wrapper_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.approver,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.to_wrapped_token_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.from_token_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.owner_from_token_account,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.owner_to_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.mint,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.token_program,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_wrap_tokens {
            use super::*;
            /// Generated CPI struct of the accounts for [`WrapTokens`].
            pub struct WrapTokens<'info> {
                pub wrapper_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub approver: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub to_wrapped_token_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub from_token_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub owner_from_token_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub owner_to_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for WrapTokens<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.wrapper_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.approver),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.to_wrapped_token_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.from_token_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.owner_from_token_account),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.owner_to_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.mint),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.token_program),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for WrapTokens<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.wrapper_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.approver),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.to_wrapped_token_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.from_token_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.owner_from_token_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.owner_to_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.mint),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.token_program,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct UnWrapTokens<'info> {
            #[account(seeds = [b"wrapper", approver.key().as_ref()], bump)]
            pub wrapper_account: Account<'info, WrapperAccount>,
            /// CHECK: The approver of the wrapper
            pub approver: UncheckedAccount<'info>,
            /// CHECK: That the exit regulator is in the list of the wrapper_account
            pub exit_regulator: Signer<'info>,
            #[account(
                mut,
                token::authority = wrapper_account,
                token::mint = mint,
                seeds = [b"wrapped_token",
                wrapper_account.key().as_ref(),
                mint.key().as_ref(),
                owner.key().as_ref()],
                bump
            )]
            pub user_wrapped_token_account: InterfaceAccount<'info, TokenAccount>,
            #[account(mut, token::authority = owner_token_account, token::mint = mint)]
            pub to_token_account: InterfaceAccount<'info, TokenAccount>,
            pub owner_token_account: AccountInfo<'info>,
            pub owner: Signer<'info>,
            #[account(mint::token_program = token_program)]
            pub mint: InterfaceAccount<'info, Mint>,
            pub token_program: Interface<'info, TokenInterface>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, UnWrapTokensBumps>
        for UnWrapTokens<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut UnWrapTokensBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let wrapper_account: anchor_lang::accounts::account::Account<
                    WrapperAccount,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("wrapper_account"))?;
                let approver: UncheckedAccount = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("approver"))?;
                let exit_regulator: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("exit_regulator"))?;
                let user_wrapped_token_account: anchor_lang::accounts::interface_account::InterfaceAccount<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("user_wrapped_token_account"))?;
                let to_token_account: anchor_lang::accounts::interface_account::InterfaceAccount<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("to_token_account"))?;
                let owner_token_account: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("owner_token_account"))?;
                let owner: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("owner"))?;
                let mint: anchor_lang::accounts::interface_account::InterfaceAccount<
                    Mint,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("mint"))?;
                let token_program: anchor_lang::accounts::interface::Interface<
                    TokenInterface,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("token_program"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[b"wrapper", approver.key().as_ref()],
                    &__program_id,
                );
                __bumps.wrapper_account = __bump;
                if wrapper_account.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("wrapper_account")
                            .with_pubkeys((wrapper_account.key(), __pda_address)),
                    );
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[
                        b"wrapped_token",
                        wrapper_account.key().as_ref(),
                        mint.key().as_ref(),
                        owner.key().as_ref(),
                    ],
                    &__program_id,
                );
                __bumps.user_wrapped_token_account = __bump;
                if user_wrapped_token_account.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("user_wrapped_token_account")
                            .with_pubkeys((
                                user_wrapped_token_account.key(),
                                __pda_address,
                            )),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&user_wrapped_token_account).is_writable
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("user_wrapped_token_account"),
                    );
                }
                {
                    if user_wrapped_token_account.owner != wrapper_account.key() {
                        return Err(
                            anchor_lang::error::ErrorCode::ConstraintTokenOwner.into(),
                        );
                    }
                    if user_wrapped_token_account.mint != mint.key() {
                        return Err(
                            anchor_lang::error::ErrorCode::ConstraintTokenMint.into(),
                        );
                    }
                }
                if !AsRef::<AccountInfo>::as_ref(&to_token_account).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("to_token_account"),
                    );
                }
                {
                    if to_token_account.owner != owner_token_account.key() {
                        return Err(
                            anchor_lang::error::ErrorCode::ConstraintTokenOwner.into(),
                        );
                    }
                    if to_token_account.mint != mint.key() {
                        return Err(
                            anchor_lang::error::ErrorCode::ConstraintTokenMint.into(),
                        );
                    }
                }
                {
                    if AsRef::<AccountInfo>::as_ref(&mint).owner != &token_program.key()
                    {
                        return Err(
                            anchor_lang::error::ErrorCode::ConstraintMintTokenProgram
                                .into(),
                        );
                    }
                }
                Ok(UnWrapTokens {
                    wrapper_account,
                    approver,
                    exit_regulator,
                    user_wrapped_token_account,
                    to_token_account,
                    owner_token_account,
                    owner,
                    mint,
                    token_program,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for UnWrapTokens<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.wrapper_account.to_account_infos());
                account_infos.extend(self.approver.to_account_infos());
                account_infos.extend(self.exit_regulator.to_account_infos());
                account_infos.extend(self.user_wrapped_token_account.to_account_infos());
                account_infos.extend(self.to_token_account.to_account_infos());
                account_infos.extend(self.owner_token_account.to_account_infos());
                account_infos.extend(self.owner.to_account_infos());
                account_infos.extend(self.mint.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for UnWrapTokens<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.wrapper_account.to_account_metas(None));
                account_metas.extend(self.approver.to_account_metas(None));
                account_metas.extend(self.exit_regulator.to_account_metas(None));
                account_metas
                    .extend(self.user_wrapped_token_account.to_account_metas(None));
                account_metas.extend(self.to_token_account.to_account_metas(None));
                account_metas.extend(self.owner_token_account.to_account_metas(None));
                account_metas.extend(self.owner.to_account_metas(None));
                account_metas.extend(self.mint.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for UnWrapTokens<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(
                        &self.user_wrapped_token_account,
                        program_id,
                    )
                    .map_err(|e| e.with_account_name("user_wrapped_token_account"))?;
                anchor_lang::AccountsExit::exit(&self.to_token_account, program_id)
                    .map_err(|e| e.with_account_name("to_token_account"))?;
                Ok(())
            }
        }
        pub struct UnWrapTokensBumps {
            pub wrapper_account: u8,
            pub user_wrapped_token_account: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for UnWrapTokensBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "UnWrapTokensBumps",
                    "wrapper_account",
                    &self.wrapper_account,
                    "user_wrapped_token_account",
                    &&self.user_wrapped_token_account,
                )
            }
        }
        impl Default for UnWrapTokensBumps {
            fn default() -> Self {
                UnWrapTokensBumps {
                    wrapper_account: u8::MAX,
                    user_wrapped_token_account: u8::MAX,
                }
            }
        }
        impl<'info> anchor_lang::Bumps for UnWrapTokens<'info>
        where
            'info: 'info,
        {
            type Bumps = UnWrapTokensBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_un_wrap_tokens {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`UnWrapTokens`].
            pub struct UnWrapTokens {
                pub wrapper_account: Pubkey,
                pub approver: Pubkey,
                pub exit_regulator: Pubkey,
                pub user_wrapped_token_account: Pubkey,
                pub to_token_account: Pubkey,
                pub owner_token_account: Pubkey,
                pub owner: Pubkey,
                pub mint: Pubkey,
                pub token_program: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for UnWrapTokens
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.wrapper_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.approver, writer)?;
                    borsh::BorshSerialize::serialize(&self.exit_regulator, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.user_wrapped_token_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(&self.to_token_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.owner_token_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.owner, writer)?;
                    borsh::BorshSerialize::serialize(&self.mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for UnWrapTokens {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.wrapper_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.approver,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.exit_regulator,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.user_wrapped_token_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.to_token_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.owner_token_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.owner,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.mint,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.token_program,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_un_wrap_tokens {
            use super::*;
            /// Generated CPI struct of the accounts for [`UnWrapTokens`].
            pub struct UnWrapTokens<'info> {
                pub wrapper_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub approver: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub exit_regulator: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub user_wrapped_token_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub to_token_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub owner_token_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub owner: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for UnWrapTokens<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.wrapper_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.approver),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.exit_regulator),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.user_wrapped_token_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.to_token_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.owner_token_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.owner),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.mint),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.token_program),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for UnWrapTokens<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.wrapper_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.approver),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.exit_regulator,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.user_wrapped_token_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.to_token_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.owner_token_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.owner),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.mint),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.token_program,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub fn _initialize_wrapper(
            ctx: Context<InitializeWrapper>,
            list_issuer: Vec<Pubkey>,
            exit_regulators: Vec<Pubkey>,
        ) -> Result<()> {
            let wrapper_account = &mut ctx.accounts.wrapper_account;
            wrapper_account.id_issuers = list_issuer;
            wrapper_account.exit_regulators = exit_regulators;
            wrapper_account.bump = ctx.bumps.wrapper_account;
            Ok(())
        }
        pub fn _add_issuers_wrapper(
            ctx: Context<AddWrapperIssuer>,
            issuer: Pubkey,
        ) -> Result<()> {
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
        pub fn _wrap_tokens(
            ctx: Context<WrapTokens>,
            amount: u64,
            decimals: u8,
        ) -> Result<()> {
            let mint = &ctx.accounts.mint;
            if mint.decimals != decimals {
                return Err(WrapperError::InvalidDecimals.into());
            }
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
        pub fn _unwrap_tokens(
            ctx: Context<UnWrapTokens>,
            amount: u64,
            decimals: u8,
        ) -> Result<()> {
            let wrapper_account = &ctx.accounts.wrapper_account;
            let exit_regulator = &ctx.accounts.exit_regulator;
            if !wrapper_account.exit_regulators.contains(exit_regulator.key) {
                return Err(WrapperError::InvalidExitRegulator.into());
            }
            let mint = &ctx.accounts.mint;
            if mint.decimals != decimals {
                return Err(WrapperError::InvalidDecimals.into());
            }
            let approver = ctx.accounts.approver.key();
            let bump = ctx.bumps.wrapper_account;
            let seed: &[&[&[u8]]] = &[&[b"wrapper", approver.as_ref(), &[bump]]];
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
                seed,
            )?;
            Ok(())
        }
    }
    pub use confidential_wrapper::*;
    pub mod transfer {
        use anchor_lang::{prelude::*, solana_program::program};
        use anchor_spl::{
            token::spl_token, token_interface::{Mint, TokenAccount, TokenInterface},
        };
        use crate::{
            check_idendity_not_recovered, error::{IdendityError, TwoAuthError},
            two_auth, IdAccount, Issuer, TwoAuth, TwoAuthParameters, WrapperAccount,
        };
        pub struct Transfer<'info> {
            #[account(
                mut,
                token::authority = wrapper_account,
                seeds = [b"wrapped_token",
                wrapper_account.key().as_ref(),
                mint.key().as_ref(),
                source_owner.key().as_ref()],
                bump
            )]
            pub source_wrapped_account: InterfaceAccount<'info, TokenAccount>,
            #[account(mut)]
            pub source_owner: Signer<'info>,
            #[account(
                seeds = [b"identity",
                source_owner.key().as_ref()],
                bump = idendity_sender.bump
            )]
            pub idendity_sender: Box<Account<'info, IdAccount>>,
            #[account(
                mut,
                seeds = [b"two_auth",
                wrapper_account.key().as_ref(),
                source_owner.key().as_ref()],
                bump = two_auth.bump
            )]
            pub two_auth: Account<'info, TwoAuth>,
            #[account(
                init_if_needed,
                token::mint = mint,
                token::authority = wrapper_account,
                seeds = [b"wrapped_token",
                wrapper_account.key().as_ref(),
                mint.key().as_ref(),
                destination_owner.key().as_ref()],
                bump,
                payer = payer
            )]
            pub destination_wrapped_account: InterfaceAccount<'info, TokenAccount>,
            /// CHECK: The owner of the destination account
            pub destination_owner: AccountInfo<'info>,
            #[account(seeds = [b"identity", destination_owner.key().as_ref()], bump)]
            pub idendity_receiver: Box<Account<'info, IdAccount>>,
            pub two_auth_signer: Option<Signer<'info>>,
            #[account(mut)]
            pub payer: Signer<'info>,
            #[account(
                seeds = [b"wrapper",
                approver.key().as_ref()],
                bump = wrapper_account.bump
            )]
            pub wrapper_account: Account<'info, WrapperAccount>,
            /// CHECK: The approver of the wrapper
            pub approver: UncheckedAccount<'info>,
            #[account(mint::token_program = token_program)]
            pub mint: InterfaceAccount<'info, Mint>,
            pub token_program: Interface<'info, TokenInterface>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, TransferBumps> for Transfer<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut TransferBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let source_wrapped_account: anchor_lang::accounts::interface_account::InterfaceAccount<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("source_wrapped_account"))?;
                let source_owner: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("source_owner"))?;
                let idendity_sender: Box<
                    anchor_lang::accounts::account::Account<IdAccount>,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("idendity_sender"))?;
                let two_auth: anchor_lang::accounts::account::Account<TwoAuth> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("two_auth"))?;
                if __accounts.is_empty() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into(),
                    );
                }
                let destination_wrapped_account = &__accounts[0];
                *__accounts = &__accounts[1..];
                let destination_owner: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("destination_owner"))?;
                let idendity_receiver: Box<
                    anchor_lang::accounts::account::Account<IdAccount>,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("idendity_receiver"))?;
                let two_auth_signer: Option<Signer> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("two_auth_signer"))?;
                let payer: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("payer"))?;
                let wrapper_account: anchor_lang::accounts::account::Account<
                    WrapperAccount,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("wrapper_account"))?;
                let approver: UncheckedAccount = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("approver"))?;
                let mint: anchor_lang::accounts::interface_account::InterfaceAccount<
                    Mint,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("mint"))?;
                let token_program: anchor_lang::accounts::interface::Interface<
                    TokenInterface,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("token_program"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                let __anchor_rent = Rent::get()?;
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[
                        b"wrapped_token",
                        wrapper_account.key().as_ref(),
                        mint.key().as_ref(),
                        destination_owner.key().as_ref(),
                    ],
                    __program_id,
                );
                __bumps.destination_wrapped_account = __bump;
                if destination_wrapped_account.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("destination_wrapped_account")
                            .with_pubkeys((
                                destination_wrapped_account.key(),
                                __pda_address,
                            )),
                    );
                }
                let destination_wrapped_account: anchor_lang::accounts::interface_account::InterfaceAccount<
                    TokenAccount,
                > = {
                    let owner_program = AsRef::<
                        AccountInfo,
                    >::as_ref(&destination_wrapped_account)
                        .owner;
                    if !true
                        || owner_program
                            == &anchor_lang::solana_program::system_program::ID
                    {
                        let __current_lamports = destination_wrapped_account.lamports();
                        if __current_lamports == 0 {
                            let space = {
                                let mint_info = mint.to_account_info();
                                if *mint_info.owner
                                    == ::anchor_spl::token_2022::Token2022::id()
                                {
                                    use ::anchor_spl::token_2022::spl_token_2022::extension::{
                                        BaseStateWithExtensions, ExtensionType, StateWithExtensions,
                                    };
                                    use ::anchor_spl::token_2022::spl_token_2022::state::{
                                        Account, Mint,
                                    };
                                    let mint_data = mint_info.try_borrow_data()?;
                                    let mint_state = StateWithExtensions::<
                                        Mint,
                                    >::unpack(&mint_data)?;
                                    let mint_extensions = mint_state.get_extension_types()?;
                                    let required_extensions = ExtensionType::get_required_init_account_extensions(
                                        &mint_extensions,
                                    );
                                    ExtensionType::try_calculate_account_len::<
                                        Account,
                                    >(&required_extensions)?
                                } else {
                                    ::anchor_spl::token::TokenAccount::LEN
                                }
                            };
                            let lamports = __anchor_rent.minimum_balance(space);
                            let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                from: payer.to_account_info(),
                                to: destination_wrapped_account.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::create_account(
                                cpi_context
                                    .with_signer(
                                        &[
                                            &[
                                                b"wrapped_token",
                                                wrapper_account.key().as_ref(),
                                                mint.key().as_ref(),
                                                destination_owner.key().as_ref(),
                                                &[__bump][..],
                                            ][..],
                                        ],
                                    ),
                                lamports,
                                space as u64,
                                &token_program.key(),
                            )?;
                        } else {
                            if payer.key() == destination_wrapped_account.key() {
                                return Err(
                                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                            error_name: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .name(),
                                            error_code_number: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .into(),
                                            error_msg: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .to_string(),
                                            error_origin: Some(
                                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                    filename: "programs/asset_based/src/instructions/transfer.rs",
                                                    line: 8u32,
                                                }),
                                            ),
                                            compared_values: None,
                                        })
                                        .with_pubkeys((
                                            payer.key(),
                                            destination_wrapped_account.key(),
                                        )),
                                );
                            }
                            let required_lamports = __anchor_rent
                                .minimum_balance({
                                    let mint_info = mint.to_account_info();
                                    if *mint_info.owner
                                        == ::anchor_spl::token_2022::Token2022::id()
                                    {
                                        use ::anchor_spl::token_2022::spl_token_2022::extension::{
                                            BaseStateWithExtensions, ExtensionType, StateWithExtensions,
                                        };
                                        use ::anchor_spl::token_2022::spl_token_2022::state::{
                                            Account, Mint,
                                        };
                                        let mint_data = mint_info.try_borrow_data()?;
                                        let mint_state = StateWithExtensions::<
                                            Mint,
                                        >::unpack(&mint_data)?;
                                        let mint_extensions = mint_state.get_extension_types()?;
                                        let required_extensions = ExtensionType::get_required_init_account_extensions(
                                            &mint_extensions,
                                        );
                                        ExtensionType::try_calculate_account_len::<
                                            Account,
                                        >(&required_extensions)?
                                    } else {
                                        ::anchor_spl::token::TokenAccount::LEN
                                    }
                                })
                                .max(1)
                                .saturating_sub(__current_lamports);
                            if required_lamports > 0 {
                                let cpi_accounts = anchor_lang::system_program::Transfer {
                                    from: payer.to_account_info(),
                                    to: destination_wrapped_account.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::transfer(
                                    cpi_context,
                                    required_lamports,
                                )?;
                            }
                            let cpi_accounts = anchor_lang::system_program::Allocate {
                                account_to_allocate: destination_wrapped_account
                                    .to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::allocate(
                                cpi_context
                                    .with_signer(
                                        &[
                                            &[
                                                b"wrapped_token",
                                                wrapper_account.key().as_ref(),
                                                mint.key().as_ref(),
                                                destination_owner.key().as_ref(),
                                                &[__bump][..],
                                            ][..],
                                        ],
                                    ),
                                {
                                    let mint_info = mint.to_account_info();
                                    if *mint_info.owner
                                        == ::anchor_spl::token_2022::Token2022::id()
                                    {
                                        use ::anchor_spl::token_2022::spl_token_2022::extension::{
                                            BaseStateWithExtensions, ExtensionType, StateWithExtensions,
                                        };
                                        use ::anchor_spl::token_2022::spl_token_2022::state::{
                                            Account, Mint,
                                        };
                                        let mint_data = mint_info.try_borrow_data()?;
                                        let mint_state = StateWithExtensions::<
                                            Mint,
                                        >::unpack(&mint_data)?;
                                        let mint_extensions = mint_state.get_extension_types()?;
                                        let required_extensions = ExtensionType::get_required_init_account_extensions(
                                            &mint_extensions,
                                        );
                                        ExtensionType::try_calculate_account_len::<
                                            Account,
                                        >(&required_extensions)?
                                    } else {
                                        ::anchor_spl::token::TokenAccount::LEN
                                    }
                                } as u64,
                            )?;
                            let cpi_accounts = anchor_lang::system_program::Assign {
                                account_to_assign: destination_wrapped_account
                                    .to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::assign(
                                cpi_context
                                    .with_signer(
                                        &[
                                            &[
                                                b"wrapped_token",
                                                wrapper_account.key().as_ref(),
                                                mint.key().as_ref(),
                                                destination_owner.key().as_ref(),
                                                &[__bump][..],
                                            ][..],
                                        ],
                                    ),
                                &token_program.key(),
                            )?;
                        }
                        let cpi_program = token_program.to_account_info();
                        let accounts = ::anchor_spl::token_interface::InitializeAccount3 {
                            account: destination_wrapped_account.to_account_info(),
                            mint: mint.to_account_info(),
                            authority: wrapper_account.to_account_info(),
                        };
                        let cpi_ctx = anchor_lang::context::CpiContext::new(
                            cpi_program,
                            accounts,
                        );
                        ::anchor_spl::token_interface::initialize_account3(cpi_ctx)?;
                    }
                    let pa: anchor_lang::accounts::interface_account::InterfaceAccount<
                        TokenAccount,
                    > = match anchor_lang::accounts::interface_account::InterfaceAccount::try_from_unchecked(
                        &destination_wrapped_account,
                    ) {
                        Ok(val) => val,
                        Err(e) => {
                            return Err(
                                e.with_account_name("destination_wrapped_account"),
                            );
                        }
                    };
                    if true {
                        if pa.mint != mint.key() {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::ConstraintTokenMint,
                                    )
                                    .with_account_name("destination_wrapped_account")
                                    .with_pubkeys((pa.mint, mint.key())),
                            );
                        }
                        if pa.owner != wrapper_account.key() {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                                    )
                                    .with_account_name("destination_wrapped_account")
                                    .with_pubkeys((pa.owner, wrapper_account.key())),
                            );
                        }
                        if owner_program != &token_program.key() {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::ConstraintTokenTokenProgram,
                                    )
                                    .with_account_name("destination_wrapped_account")
                                    .with_pubkeys((*owner_program, token_program.key())),
                            );
                        }
                    }
                    pa
                };
                if !AsRef::<AccountInfo>::as_ref(&destination_wrapped_account)
                    .is_writable
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("destination_wrapped_account"),
                    );
                }
                if !__anchor_rent
                    .is_exempt(
                        destination_wrapped_account.to_account_info().lamports(),
                        destination_wrapped_account.to_account_info().try_data_len()?,
                    )
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                            )
                            .with_account_name("destination_wrapped_account"),
                    );
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[
                        b"wrapped_token",
                        wrapper_account.key().as_ref(),
                        mint.key().as_ref(),
                        source_owner.key().as_ref(),
                    ],
                    &__program_id,
                );
                __bumps.source_wrapped_account = __bump;
                if source_wrapped_account.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("source_wrapped_account")
                            .with_pubkeys((source_wrapped_account.key(), __pda_address)),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&source_wrapped_account).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("source_wrapped_account"),
                    );
                }
                {
                    if source_wrapped_account.owner != wrapper_account.key() {
                        return Err(
                            anchor_lang::error::ErrorCode::ConstraintTokenOwner.into(),
                        );
                    }
                }
                if !AsRef::<AccountInfo>::as_ref(&source_owner).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("source_owner"),
                    );
                }
                let __pda_address = Pubkey::create_program_address(
                        &[
                            b"identity",
                            source_owner.key().as_ref(),
                            &[idendity_sender.bump][..],
                        ],
                        &__program_id,
                    )
                    .map_err(|_| {
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("idendity_sender")
                    })?;
                if idendity_sender.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("idendity_sender")
                            .with_pubkeys((idendity_sender.key(), __pda_address)),
                    );
                }
                let __pda_address = Pubkey::create_program_address(
                        &[
                            b"two_auth",
                            wrapper_account.key().as_ref(),
                            source_owner.key().as_ref(),
                            &[two_auth.bump][..],
                        ],
                        &__program_id,
                    )
                    .map_err(|_| {
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("two_auth")
                    })?;
                if two_auth.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("two_auth")
                            .with_pubkeys((two_auth.key(), __pda_address)),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&two_auth).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("two_auth"),
                    );
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[b"identity", destination_owner.key().as_ref()],
                    &__program_id,
                );
                __bumps.idendity_receiver = __bump;
                if idendity_receiver.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("idendity_receiver")
                            .with_pubkeys((idendity_receiver.key(), __pda_address)),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&payer).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("payer"),
                    );
                }
                let __pda_address = Pubkey::create_program_address(
                        &[
                            b"wrapper",
                            approver.key().as_ref(),
                            &[wrapper_account.bump][..],
                        ],
                        &__program_id,
                    )
                    .map_err(|_| {
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("wrapper_account")
                    })?;
                if wrapper_account.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("wrapper_account")
                            .with_pubkeys((wrapper_account.key(), __pda_address)),
                    );
                }
                {
                    if AsRef::<AccountInfo>::as_ref(&mint).owner != &token_program.key()
                    {
                        return Err(
                            anchor_lang::error::ErrorCode::ConstraintMintTokenProgram
                                .into(),
                        );
                    }
                }
                Ok(Transfer {
                    source_wrapped_account,
                    source_owner,
                    idendity_sender,
                    two_auth,
                    destination_wrapped_account,
                    destination_owner,
                    idendity_receiver,
                    two_auth_signer,
                    payer,
                    wrapper_account,
                    approver,
                    mint,
                    token_program,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for Transfer<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.source_wrapped_account.to_account_infos());
                account_infos.extend(self.source_owner.to_account_infos());
                account_infos.extend(self.idendity_sender.to_account_infos());
                account_infos.extend(self.two_auth.to_account_infos());
                account_infos
                    .extend(self.destination_wrapped_account.to_account_infos());
                account_infos.extend(self.destination_owner.to_account_infos());
                account_infos.extend(self.idendity_receiver.to_account_infos());
                account_infos.extend(self.two_auth_signer.to_account_infos());
                account_infos.extend(self.payer.to_account_infos());
                account_infos.extend(self.wrapper_account.to_account_infos());
                account_infos.extend(self.approver.to_account_infos());
                account_infos.extend(self.mint.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for Transfer<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.source_wrapped_account.to_account_metas(None));
                account_metas.extend(self.source_owner.to_account_metas(None));
                account_metas.extend(self.idendity_sender.to_account_metas(None));
                account_metas.extend(self.two_auth.to_account_metas(None));
                account_metas
                    .extend(self.destination_wrapped_account.to_account_metas(None));
                account_metas.extend(self.destination_owner.to_account_metas(None));
                account_metas.extend(self.idendity_receiver.to_account_metas(None));
                if let Some(two_auth_signer) = &self.two_auth_signer {
                    account_metas.extend(two_auth_signer.to_account_metas(None));
                } else {
                    account_metas.push(AccountMeta::new_readonly(crate::ID, false));
                }
                account_metas.extend(self.payer.to_account_metas(None));
                account_metas.extend(self.wrapper_account.to_account_metas(None));
                account_metas.extend(self.approver.to_account_metas(None));
                account_metas.extend(self.mint.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for Transfer<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.source_wrapped_account, program_id)
                    .map_err(|e| e.with_account_name("source_wrapped_account"))?;
                anchor_lang::AccountsExit::exit(&self.source_owner, program_id)
                    .map_err(|e| e.with_account_name("source_owner"))?;
                anchor_lang::AccountsExit::exit(&self.two_auth, program_id)
                    .map_err(|e| e.with_account_name("two_auth"))?;
                anchor_lang::AccountsExit::exit(
                        &self.destination_wrapped_account,
                        program_id,
                    )
                    .map_err(|e| e.with_account_name("destination_wrapped_account"))?;
                anchor_lang::AccountsExit::exit(&self.payer, program_id)
                    .map_err(|e| e.with_account_name("payer"))?;
                Ok(())
            }
        }
        pub struct TransferBumps {
            pub source_wrapped_account: u8,
            pub destination_wrapped_account: u8,
            pub idendity_receiver: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TransferBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "TransferBumps",
                    "source_wrapped_account",
                    &self.source_wrapped_account,
                    "destination_wrapped_account",
                    &self.destination_wrapped_account,
                    "idendity_receiver",
                    &&self.idendity_receiver,
                )
            }
        }
        impl Default for TransferBumps {
            fn default() -> Self {
                TransferBumps {
                    source_wrapped_account: u8::MAX,
                    destination_wrapped_account: u8::MAX,
                    idendity_receiver: u8::MAX,
                }
            }
        }
        impl<'info> anchor_lang::Bumps for Transfer<'info>
        where
            'info: 'info,
        {
            type Bumps = TransferBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_transfer {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`Transfer`].
            pub struct Transfer {
                pub source_wrapped_account: Pubkey,
                pub source_owner: Pubkey,
                pub idendity_sender: Pubkey,
                pub two_auth: Pubkey,
                pub destination_wrapped_account: Pubkey,
                pub destination_owner: Pubkey,
                pub idendity_receiver: Pubkey,
                pub two_auth_signer: Option<Pubkey>,
                pub payer: Pubkey,
                pub wrapper_account: Pubkey,
                pub approver: Pubkey,
                pub mint: Pubkey,
                pub token_program: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for Transfer
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Option<Pubkey>: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(
                        &self.source_wrapped_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(&self.source_owner, writer)?;
                    borsh::BorshSerialize::serialize(&self.idendity_sender, writer)?;
                    borsh::BorshSerialize::serialize(&self.two_auth, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.destination_wrapped_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(&self.destination_owner, writer)?;
                    borsh::BorshSerialize::serialize(&self.idendity_receiver, writer)?;
                    borsh::BorshSerialize::serialize(&self.two_auth_signer, writer)?;
                    borsh::BorshSerialize::serialize(&self.payer, writer)?;
                    borsh::BorshSerialize::serialize(&self.wrapper_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.approver, writer)?;
                    borsh::BorshSerialize::serialize(&self.mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for Transfer {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.source_wrapped_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.source_owner,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.idendity_sender,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.two_auth,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.destination_wrapped_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.destination_owner,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.idendity_receiver,
                                false,
                            ),
                        );
                    if let Some(two_auth_signer) = &self.two_auth_signer {
                        account_metas
                            .push(
                                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                    *two_auth_signer,
                                    true,
                                ),
                            );
                    } else {
                        account_metas
                            .push(
                                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                    crate::ID,
                                    false,
                                ),
                            );
                    }
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.payer,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.wrapper_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.approver,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.mint,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.token_program,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_transfer {
            use super::*;
            /// Generated CPI struct of the accounts for [`Transfer`].
            pub struct Transfer<'info> {
                pub source_wrapped_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub source_owner: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub idendity_sender: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub two_auth: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub destination_wrapped_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub destination_owner: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub idendity_receiver: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub two_auth_signer: Option<
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                >,
                pub payer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub wrapper_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub approver: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for Transfer<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.source_wrapped_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.source_owner),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.idendity_sender),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.two_auth),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.destination_wrapped_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.destination_owner),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.idendity_receiver),
                                false,
                            ),
                        );
                    if let Some(two_auth_signer) = &self.two_auth_signer {
                        account_metas
                            .push(
                                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                    anchor_lang::Key::key(two_auth_signer),
                                    true,
                                ),
                            );
                    } else {
                        account_metas
                            .push(
                                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                    crate::ID,
                                    false,
                                ),
                            );
                    }
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.payer),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.wrapper_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.approver),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.mint),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.token_program),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for Transfer<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.source_wrapped_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.source_owner,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.idendity_sender,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.two_auth),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.destination_wrapped_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.destination_owner,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.idendity_receiver,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.two_auth_signer,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.payer),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.wrapper_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.approver),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.mint),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.token_program,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub fn _transfer_public(
            ctx: Context<Transfer>,
            amount: u64,
            decimals: u8,
        ) -> Result<()> {
            let source = &mut ctx.accounts.source_wrapped_account;
            let destination = &mut ctx.accounts.destination_wrapped_account;
            let self_transfer = source.key() == destination.key();
            check_idendity_not_recovered(&ctx.accounts.idendity_sender)?;
            if !self_transfer {
                check_idendity_not_recovered(&ctx.accounts.idendity_receiver)?;
            }
            let _two_auth = &mut ctx.accounts.two_auth;
            let two_auth = &mut _two_auth.two_auth;
            let two_auth_signer = &ctx.accounts.two_auth_signer;
            let current_time = Clock::get()?.unix_timestamp;
            if !self_transfer {
                check_two_auth(
                    two_auth,
                    two_auth_signer,
                    amount,
                    current_time,
                    ctx.accounts.idendity_receiver.key(),
                )?;
            }
            let sender_issuers = &ctx.accounts.idendity_sender.issuers;
            let receiver_issuers = &ctx.accounts.idendity_receiver.issuers;
            let allowed_issuers = &ctx.accounts.wrapper_account.id_issuers;
            check_idendities(sender_issuers, allowed_issuers, current_time)?;
            if !self_transfer {
                check_idendities(receiver_issuers, allowed_issuers, current_time)?;
            }
            _two_auth.last_tx = current_time;
            if self_transfer {
                return Ok(());
            }
            let approver = ctx.accounts.approver.key();
            let bump = ctx.accounts.wrapper_account.bump;
            let seed: &[&[&[u8]]] = &[&[b"wrapper", approver.as_ref(), &[bump]]];
            let ix = spl_token::instruction::transfer_checked(
                ctx.accounts.token_program.key,
                &ctx.accounts.source_wrapped_account.key(),
                &ctx.accounts.mint.key(),
                &ctx.accounts.destination_wrapped_account.key(),
                &ctx.accounts.wrapper_account.key(),
                &[&ctx.accounts.wrapper_account.key()],
                amount,
                decimals,
            )?;
            program::invoke_signed(
                &ix,
                &[
                    ctx.accounts.token_program.to_account_info(),
                    ctx.accounts.source_wrapped_account.to_account_info(),
                    ctx.accounts.destination_wrapped_account.to_account_info(),
                    ctx.accounts.wrapper_account.to_account_info(),
                    ctx.accounts.mint.to_account_info(),
                ],
                seed,
            )?;
            Ok(())
        }
        #[inline(always)]
        pub fn check_idendities(
            user_issuers: &Vec<Issuer>,
            allowed_issuers: &Vec<Pubkey>,
            current_time: i64,
        ) -> Result<()> {
            for issuer in user_issuers {
                if allowed_issuers.contains(&issuer.key) && issuer.active
                    && issuer.expires_at > current_time
                {
                    return Ok(());
                }
            }
            return Err(IdendityError::InvalidIdendity.into());
        }
        #[inline(always)]
        pub fn check_two_auth(
            two_auth: &mut Option<TwoAuthParameters>,
            two_auth_signer: &Option<Signer>,
            amount: u64,
            current_time: i64,
            receiver: Pubkey,
        ) -> Result<()> {
            if two_auth.is_some() {
                let two_auth_parameters = two_auth.as_mut().unwrap();
                let functions = &mut two_auth_parameters.functions;
                match two_auth_signer {
                    Some(signer) => {
                        if two_auth_parameters.two_auth_entity != signer.key() {
                            return Err(TwoAuthError::WrongApproval.into());
                        }
                    }
                    None => {
                        if two_auth::apply_two_auth_functions(
                            amount,
                            functions,
                            current_time,
                            receiver,
                        ) {
                            return Err(TwoAuthError::NeedTwoAuthApproval.into());
                        }
                        return Ok(());
                    }
                }
            }
            Ok(())
        }
    }
    pub use transfer::*;
    pub mod idendity {
        use anchor_lang::prelude::*;
        use crate::{error::IdendityError, IdAccount, Issuer};
        pub struct InitializeId<'info> {
            pub issuer: Signer<'info>,
            #[account(
                init,
                seeds = [b"identity",
                owner.key().as_ref()],
                bump,
                payer = payer,
                space = IdAccount::INIT_LEN
            )]
            pub idendity: Account<'info, IdAccount>,
            pub owner: Signer<'info>,
            #[account(mut)]
            pub payer: Signer<'info>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, InitializeIdBumps>
        for InitializeId<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut InitializeIdBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let issuer: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("issuer"))?;
                if __accounts.is_empty() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into(),
                    );
                }
                let idendity = &__accounts[0];
                *__accounts = &__accounts[1..];
                let owner: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("owner"))?;
                let payer: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("payer"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                let __anchor_rent = Rent::get()?;
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[b"identity", owner.key().as_ref()],
                    __program_id,
                );
                __bumps.idendity = __bump;
                if idendity.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("idendity")
                            .with_pubkeys((idendity.key(), __pda_address)),
                    );
                }
                let idendity = {
                    let actual_field = AsRef::<AccountInfo>::as_ref(&idendity);
                    let actual_owner = actual_field.owner;
                    let space = IdAccount::INIT_LEN;
                    let pa: anchor_lang::accounts::account::Account<IdAccount> = if !false
                        || actual_owner
                            == &anchor_lang::solana_program::system_program::ID
                    {
                        let __current_lamports = idendity.lamports();
                        if __current_lamports == 0 {
                            let space = space;
                            let lamports = __anchor_rent.minimum_balance(space);
                            let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                from: payer.to_account_info(),
                                to: idendity.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::create_account(
                                cpi_context
                                    .with_signer(
                                        &[&[b"identity", owner.key().as_ref(), &[__bump][..]][..]],
                                    ),
                                lamports,
                                space as u64,
                                __program_id,
                            )?;
                        } else {
                            if payer.key() == idendity.key() {
                                return Err(
                                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                            error_name: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .name(),
                                            error_code_number: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .into(),
                                            error_msg: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .to_string(),
                                            error_origin: Some(
                                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                    filename: "programs/asset_based/src/instructions/idendity.rs",
                                                    line: 5u32,
                                                }),
                                            ),
                                            compared_values: None,
                                        })
                                        .with_pubkeys((payer.key(), idendity.key())),
                                );
                            }
                            let required_lamports = __anchor_rent
                                .minimum_balance(space)
                                .max(1)
                                .saturating_sub(__current_lamports);
                            if required_lamports > 0 {
                                let cpi_accounts = anchor_lang::system_program::Transfer {
                                    from: payer.to_account_info(),
                                    to: idendity.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::transfer(
                                    cpi_context,
                                    required_lamports,
                                )?;
                            }
                            let cpi_accounts = anchor_lang::system_program::Allocate {
                                account_to_allocate: idendity.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::allocate(
                                cpi_context
                                    .with_signer(
                                        &[&[b"identity", owner.key().as_ref(), &[__bump][..]][..]],
                                    ),
                                space as u64,
                            )?;
                            let cpi_accounts = anchor_lang::system_program::Assign {
                                account_to_assign: idendity.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::assign(
                                cpi_context
                                    .with_signer(
                                        &[&[b"identity", owner.key().as_ref(), &[__bump][..]][..]],
                                    ),
                                __program_id,
                            )?;
                        }
                        match anchor_lang::accounts::account::Account::try_from_unchecked(
                            &idendity,
                        ) {
                            Ok(val) => val,
                            Err(e) => return Err(e.with_account_name("idendity")),
                        }
                    } else {
                        match anchor_lang::accounts::account::Account::try_from(
                            &idendity,
                        ) {
                            Ok(val) => val,
                            Err(e) => return Err(e.with_account_name("idendity")),
                        }
                    };
                    if false {
                        if space != actual_field.data_len() {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::ConstraintSpace,
                                    )
                                    .with_account_name("idendity")
                                    .with_values((space, actual_field.data_len())),
                            );
                        }
                        if actual_owner != __program_id {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::ConstraintOwner,
                                    )
                                    .with_account_name("idendity")
                                    .with_pubkeys((*actual_owner, *__program_id)),
                            );
                        }
                        {
                            let required_lamports = __anchor_rent.minimum_balance(space);
                            if pa.to_account_info().lamports() < required_lamports {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintRentExempt,
                                        )
                                        .with_account_name("idendity"),
                                );
                            }
                        }
                    }
                    pa
                };
                if !AsRef::<AccountInfo>::as_ref(&idendity).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("idendity"),
                    );
                }
                if !__anchor_rent
                    .is_exempt(
                        idendity.to_account_info().lamports(),
                        idendity.to_account_info().try_data_len()?,
                    )
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                            )
                            .with_account_name("idendity"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&payer).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("payer"),
                    );
                }
                Ok(InitializeId {
                    issuer,
                    idendity,
                    owner,
                    payer,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeId<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.issuer.to_account_infos());
                account_infos.extend(self.idendity.to_account_infos());
                account_infos.extend(self.owner.to_account_infos());
                account_infos.extend(self.payer.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for InitializeId<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.issuer.to_account_metas(None));
                account_metas.extend(self.idendity.to_account_metas(None));
                account_metas.extend(self.owner.to_account_metas(None));
                account_metas.extend(self.payer.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for InitializeId<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.idendity, program_id)
                    .map_err(|e| e.with_account_name("idendity"))?;
                anchor_lang::AccountsExit::exit(&self.payer, program_id)
                    .map_err(|e| e.with_account_name("payer"))?;
                Ok(())
            }
        }
        pub struct InitializeIdBumps {
            pub idendity: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for InitializeIdBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "InitializeIdBumps",
                    "idendity",
                    &&self.idendity,
                )
            }
        }
        impl Default for InitializeIdBumps {
            fn default() -> Self {
                InitializeIdBumps {
                    idendity: u8::MAX,
                }
            }
        }
        impl<'info> anchor_lang::Bumps for InitializeId<'info>
        where
            'info: 'info,
        {
            type Bumps = InitializeIdBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_initialize_id {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`InitializeId`].
            pub struct InitializeId {
                pub issuer: Pubkey,
                pub idendity: Pubkey,
                pub owner: Pubkey,
                pub payer: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for InitializeId
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.issuer, writer)?;
                    borsh::BorshSerialize::serialize(&self.idendity, writer)?;
                    borsh::BorshSerialize::serialize(&self.owner, writer)?;
                    borsh::BorshSerialize::serialize(&self.payer, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for InitializeId {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.issuer,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.idendity,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.owner,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.payer,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_initialize_id {
            use super::*;
            /// Generated CPI struct of the accounts for [`InitializeId`].
            pub struct InitializeId<'info> {
                pub issuer: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub idendity: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub owner: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub payer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for InitializeId<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.issuer),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.idendity),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.owner),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.payer),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeId<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.issuer),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.idendity),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.owner),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.payer),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct AddIssuer<'info> {
            #[account(mut)]
            pub issuer: Signer<'info>,
            #[account(
                mut,
                seeds = [b"identity",
                owner.key().as_ref()],
                bump,
                realloc = idendity.get_add_issuer_len(),
                realloc::payer = owner,
                realloc::zero = false
            )]
            pub idendity: Account<'info, IdAccount>,
            #[account(mut)]
            pub owner: Signer<'info>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, AddIssuerBumps> for AddIssuer<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut AddIssuerBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let issuer: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("issuer"))?;
                let idendity: anchor_lang::accounts::account::Account<IdAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("idendity"))?;
                let owner: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("owner"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                if !AsRef::<AccountInfo>::as_ref(&issuer).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("issuer"),
                    );
                }
                if __reallocs.contains(&idendity.key()) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::AccountDuplicateReallocs,
                            )
                            .with_account_name("idendity"),
                    );
                }
                let __anchor_rent = anchor_lang::prelude::Rent::get()?;
                let __field_info = idendity.to_account_info();
                let __new_rent_minimum = __anchor_rent
                    .minimum_balance(idendity.get_add_issuer_len());
                let __delta_space = (::std::convert::TryInto::<
                    isize,
                >::try_into(idendity.get_add_issuer_len())
                    .unwrap())
                    .checked_sub(
                        ::std::convert::TryInto::try_into(__field_info.data_len())
                            .unwrap(),
                    )
                    .unwrap();
                if __delta_space != 0 {
                    if __delta_space > 0 {
                        if ::std::convert::TryInto::<usize>::try_into(__delta_space)
                            .unwrap()
                            > anchor_lang::solana_program::entrypoint::MAX_PERMITTED_DATA_INCREASE
                        {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::AccountReallocExceedsLimit,
                                    )
                                    .with_account_name("idendity"),
                            );
                        }
                        if __new_rent_minimum > __field_info.lamports() {
                            anchor_lang::system_program::transfer(
                                anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    anchor_lang::system_program::Transfer {
                                        from: owner.to_account_info(),
                                        to: __field_info.clone(),
                                    },
                                ),
                                __new_rent_minimum
                                    .checked_sub(__field_info.lamports())
                                    .unwrap(),
                            )?;
                        }
                    } else {
                        let __lamport_amt = __field_info
                            .lamports()
                            .checked_sub(__new_rent_minimum)
                            .unwrap();
                        **owner.to_account_info().lamports.borrow_mut() = owner
                            .to_account_info()
                            .lamports()
                            .checked_add(__lamport_amt)
                            .unwrap();
                        **__field_info.lamports.borrow_mut() = __field_info
                            .lamports()
                            .checked_sub(__lamport_amt)
                            .unwrap();
                    }
                    __field_info.realloc(idendity.get_add_issuer_len(), false)?;
                    __reallocs.insert(idendity.key());
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[b"identity", owner.key().as_ref()],
                    &__program_id,
                );
                __bumps.idendity = __bump;
                if idendity.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("idendity")
                            .with_pubkeys((idendity.key(), __pda_address)),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&idendity).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("idendity"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&owner).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("owner"),
                    );
                }
                Ok(AddIssuer {
                    issuer,
                    idendity,
                    owner,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for AddIssuer<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.issuer.to_account_infos());
                account_infos.extend(self.idendity.to_account_infos());
                account_infos.extend(self.owner.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for AddIssuer<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.issuer.to_account_metas(None));
                account_metas.extend(self.idendity.to_account_metas(None));
                account_metas.extend(self.owner.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for AddIssuer<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.issuer, program_id)
                    .map_err(|e| e.with_account_name("issuer"))?;
                anchor_lang::AccountsExit::exit(&self.idendity, program_id)
                    .map_err(|e| e.with_account_name("idendity"))?;
                anchor_lang::AccountsExit::exit(&self.owner, program_id)
                    .map_err(|e| e.with_account_name("owner"))?;
                Ok(())
            }
        }
        pub struct AddIssuerBumps {
            pub idendity: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AddIssuerBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "AddIssuerBumps",
                    "idendity",
                    &&self.idendity,
                )
            }
        }
        impl Default for AddIssuerBumps {
            fn default() -> Self {
                AddIssuerBumps {
                    idendity: u8::MAX,
                }
            }
        }
        impl<'info> anchor_lang::Bumps for AddIssuer<'info>
        where
            'info: 'info,
        {
            type Bumps = AddIssuerBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_add_issuer {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`AddIssuer`].
            pub struct AddIssuer {
                pub issuer: Pubkey,
                pub idendity: Pubkey,
                pub owner: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for AddIssuer
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.issuer, writer)?;
                    borsh::BorshSerialize::serialize(&self.idendity, writer)?;
                    borsh::BorshSerialize::serialize(&self.owner, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for AddIssuer {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.issuer,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.idendity,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.owner,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_add_issuer {
            use super::*;
            /// Generated CPI struct of the accounts for [`AddIssuer`].
            pub struct AddIssuer<'info> {
                pub issuer: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub idendity: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub owner: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for AddIssuer<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.issuer),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.idendity),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.owner),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for AddIssuer<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.issuer),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.idendity),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.owner),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub fn _initialize_id(
            ctx: Context<InitializeId>,
            id_validity_duration: i64,
        ) -> Result<()> {
            let clock = Clock::get()?;
            let idendity = &mut ctx.accounts.idendity;
            idendity.owner = ctx.accounts.owner.key().clone();
            let issuer = Issuer {
                key: ctx.accounts.issuer.key().clone(),
                last_modified: clock.unix_timestamp,
                expires_at: clock.unix_timestamp + id_validity_duration,
                active: true,
            };
            idendity.issuers = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([issuer]),
            );
            idendity.bump = ctx.bumps.idendity;
            idendity.associated_pseudo = None;
            Ok(())
        }
        pub fn _add_issuer_to_id(
            ctx: Context<AddIssuer>,
            id_validity_duration: i64,
        ) -> Result<()> {
            if ctx.accounts.idendity.recovered_address.is_some() {
                return Err(IdendityError::IdendityRecovered.into());
            }
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
            if idendity.recovered_address.is_some() {
                return Err(IdendityError::IdendityRecovered.into());
            }
            Ok(())
        }
    }
    pub use idendity::*;
    pub mod two_auth {
        use anchor_lang::prelude::*;
        use crate::{
            check_idendity_not_recovered, error::TwoAuthError, CircularTimeWindow,
            IdAccount, TwoAuth, TwoAuthArgs, TwoAuthFunction, TwoAuthParameters,
            WrapperAccount,
        };
        #[instruction(two_auth_args:Option<TwoAuthArgs>)]
        pub struct InitTwoAuth<'info> {
            #[account(seeds = [b"identity", owner.key().as_ref()], bump)]
            pub idendity: Account<'info, IdAccount>,
            #[account(seeds = [b"wrapper", approver.key().as_ref()], bump)]
            pub wrapper_account: Account<'info, WrapperAccount>,
            /// CHECK: The approver of the wrapper
            pub approver: UncheckedAccount<'info>,
            #[account(
                init,
                seeds = [b"two_auth",
                wrapper_account.key().as_ref(),
                owner.key().as_ref()],
                bump,
                payer = payer,
                space = TwoAuth::get_init_len(&two_auth_args)
            )]
            pub two_auth: Account<'info, TwoAuth>,
            pub two_auth_entity: Option<Signer<'info>>,
            #[account(mut)]
            pub payer: Signer<'info>,
            pub owner: Signer<'info>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, InitTwoAuthBumps> for InitTwoAuth<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut InitTwoAuthBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let mut __ix_data = __ix_data;
                struct __Args {
                    two_auth_args: Option<TwoAuthArgs>,
                }
                impl borsh::ser::BorshSerialize for __Args
                where
                    Option<TwoAuthArgs>: borsh::ser::BorshSerialize,
                {
                    fn serialize<W: borsh::maybestd::io::Write>(
                        &self,
                        writer: &mut W,
                    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                        borsh::BorshSerialize::serialize(&self.two_auth_args, writer)?;
                        Ok(())
                    }
                }
                impl borsh::de::BorshDeserialize for __Args
                where
                    Option<TwoAuthArgs>: borsh::BorshDeserialize,
                {
                    fn deserialize_reader<R: borsh::maybestd::io::Read>(
                        reader: &mut R,
                    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                        Ok(Self {
                            two_auth_args: borsh::BorshDeserialize::deserialize_reader(
                                reader,
                            )?,
                        })
                    }
                }
                let __Args { two_auth_args } = __Args::deserialize(&mut __ix_data)
                    .map_err(|_| {
                        anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                    })?;
                let idendity: anchor_lang::accounts::account::Account<IdAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("idendity"))?;
                let wrapper_account: anchor_lang::accounts::account::Account<
                    WrapperAccount,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("wrapper_account"))?;
                let approver: UncheckedAccount = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("approver"))?;
                if __accounts.is_empty() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into(),
                    );
                }
                let two_auth = &__accounts[0];
                *__accounts = &__accounts[1..];
                let two_auth_entity: Option<Signer> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("two_auth_entity"))?;
                let payer: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("payer"))?;
                let owner: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("owner"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                let __anchor_rent = Rent::get()?;
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[b"two_auth", wrapper_account.key().as_ref(), owner.key().as_ref()],
                    __program_id,
                );
                __bumps.two_auth = __bump;
                if two_auth.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("two_auth")
                            .with_pubkeys((two_auth.key(), __pda_address)),
                    );
                }
                let two_auth = {
                    let actual_field = AsRef::<AccountInfo>::as_ref(&two_auth);
                    let actual_owner = actual_field.owner;
                    let space = TwoAuth::get_init_len(&two_auth_args);
                    let pa: anchor_lang::accounts::account::Account<TwoAuth> = if !false
                        || actual_owner
                            == &anchor_lang::solana_program::system_program::ID
                    {
                        let __current_lamports = two_auth.lamports();
                        if __current_lamports == 0 {
                            let space = space;
                            let lamports = __anchor_rent.minimum_balance(space);
                            let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                from: payer.to_account_info(),
                                to: two_auth.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::create_account(
                                cpi_context
                                    .with_signer(
                                        &[
                                            &[
                                                b"two_auth",
                                                wrapper_account.key().as_ref(),
                                                owner.key().as_ref(),
                                                &[__bump][..],
                                            ][..],
                                        ],
                                    ),
                                lamports,
                                space as u64,
                                __program_id,
                            )?;
                        } else {
                            if payer.key() == two_auth.key() {
                                return Err(
                                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                            error_name: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .name(),
                                            error_code_number: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .into(),
                                            error_msg: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .to_string(),
                                            error_origin: Some(
                                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                    filename: "programs/asset_based/src/instructions/two_auth.rs",
                                                    line: 5u32,
                                                }),
                                            ),
                                            compared_values: None,
                                        })
                                        .with_pubkeys((payer.key(), two_auth.key())),
                                );
                            }
                            let required_lamports = __anchor_rent
                                .minimum_balance(space)
                                .max(1)
                                .saturating_sub(__current_lamports);
                            if required_lamports > 0 {
                                let cpi_accounts = anchor_lang::system_program::Transfer {
                                    from: payer.to_account_info(),
                                    to: two_auth.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::transfer(
                                    cpi_context,
                                    required_lamports,
                                )?;
                            }
                            let cpi_accounts = anchor_lang::system_program::Allocate {
                                account_to_allocate: two_auth.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::allocate(
                                cpi_context
                                    .with_signer(
                                        &[
                                            &[
                                                b"two_auth",
                                                wrapper_account.key().as_ref(),
                                                owner.key().as_ref(),
                                                &[__bump][..],
                                            ][..],
                                        ],
                                    ),
                                space as u64,
                            )?;
                            let cpi_accounts = anchor_lang::system_program::Assign {
                                account_to_assign: two_auth.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::assign(
                                cpi_context
                                    .with_signer(
                                        &[
                                            &[
                                                b"two_auth",
                                                wrapper_account.key().as_ref(),
                                                owner.key().as_ref(),
                                                &[__bump][..],
                                            ][..],
                                        ],
                                    ),
                                __program_id,
                            )?;
                        }
                        match anchor_lang::accounts::account::Account::try_from_unchecked(
                            &two_auth,
                        ) {
                            Ok(val) => val,
                            Err(e) => return Err(e.with_account_name("two_auth")),
                        }
                    } else {
                        match anchor_lang::accounts::account::Account::try_from(
                            &two_auth,
                        ) {
                            Ok(val) => val,
                            Err(e) => return Err(e.with_account_name("two_auth")),
                        }
                    };
                    if false {
                        if space != actual_field.data_len() {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::ConstraintSpace,
                                    )
                                    .with_account_name("two_auth")
                                    .with_values((space, actual_field.data_len())),
                            );
                        }
                        if actual_owner != __program_id {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::ConstraintOwner,
                                    )
                                    .with_account_name("two_auth")
                                    .with_pubkeys((*actual_owner, *__program_id)),
                            );
                        }
                        {
                            let required_lamports = __anchor_rent.minimum_balance(space);
                            if pa.to_account_info().lamports() < required_lamports {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintRentExempt,
                                        )
                                        .with_account_name("two_auth"),
                                );
                            }
                        }
                    }
                    pa
                };
                if !AsRef::<AccountInfo>::as_ref(&two_auth).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("two_auth"),
                    );
                }
                if !__anchor_rent
                    .is_exempt(
                        two_auth.to_account_info().lamports(),
                        two_auth.to_account_info().try_data_len()?,
                    )
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                            )
                            .with_account_name("two_auth"),
                    );
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[b"identity", owner.key().as_ref()],
                    &__program_id,
                );
                __bumps.idendity = __bump;
                if idendity.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("idendity")
                            .with_pubkeys((idendity.key(), __pda_address)),
                    );
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[b"wrapper", approver.key().as_ref()],
                    &__program_id,
                );
                __bumps.wrapper_account = __bump;
                if wrapper_account.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("wrapper_account")
                            .with_pubkeys((wrapper_account.key(), __pda_address)),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&payer).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("payer"),
                    );
                }
                Ok(InitTwoAuth {
                    idendity,
                    wrapper_account,
                    approver,
                    two_auth,
                    two_auth_entity,
                    payer,
                    owner,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for InitTwoAuth<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.idendity.to_account_infos());
                account_infos.extend(self.wrapper_account.to_account_infos());
                account_infos.extend(self.approver.to_account_infos());
                account_infos.extend(self.two_auth.to_account_infos());
                account_infos.extend(self.two_auth_entity.to_account_infos());
                account_infos.extend(self.payer.to_account_infos());
                account_infos.extend(self.owner.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for InitTwoAuth<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.idendity.to_account_metas(None));
                account_metas.extend(self.wrapper_account.to_account_metas(None));
                account_metas.extend(self.approver.to_account_metas(None));
                account_metas.extend(self.two_auth.to_account_metas(None));
                if let Some(two_auth_entity) = &self.two_auth_entity {
                    account_metas.extend(two_auth_entity.to_account_metas(None));
                } else {
                    account_metas.push(AccountMeta::new_readonly(crate::ID, false));
                }
                account_metas.extend(self.payer.to_account_metas(None));
                account_metas.extend(self.owner.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for InitTwoAuth<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.two_auth, program_id)
                    .map_err(|e| e.with_account_name("two_auth"))?;
                anchor_lang::AccountsExit::exit(&self.payer, program_id)
                    .map_err(|e| e.with_account_name("payer"))?;
                Ok(())
            }
        }
        pub struct InitTwoAuthBumps {
            pub idendity: u8,
            pub wrapper_account: u8,
            pub two_auth: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for InitTwoAuthBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "InitTwoAuthBumps",
                    "idendity",
                    &self.idendity,
                    "wrapper_account",
                    &self.wrapper_account,
                    "two_auth",
                    &&self.two_auth,
                )
            }
        }
        impl Default for InitTwoAuthBumps {
            fn default() -> Self {
                InitTwoAuthBumps {
                    idendity: u8::MAX,
                    wrapper_account: u8::MAX,
                    two_auth: u8::MAX,
                }
            }
        }
        impl<'info> anchor_lang::Bumps for InitTwoAuth<'info>
        where
            'info: 'info,
        {
            type Bumps = InitTwoAuthBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_init_two_auth {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`InitTwoAuth`].
            pub struct InitTwoAuth {
                pub idendity: Pubkey,
                pub wrapper_account: Pubkey,
                pub approver: Pubkey,
                pub two_auth: Pubkey,
                pub two_auth_entity: Option<Pubkey>,
                pub payer: Pubkey,
                pub owner: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for InitTwoAuth
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Option<Pubkey>: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.idendity, writer)?;
                    borsh::BorshSerialize::serialize(&self.wrapper_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.approver, writer)?;
                    borsh::BorshSerialize::serialize(&self.two_auth, writer)?;
                    borsh::BorshSerialize::serialize(&self.two_auth_entity, writer)?;
                    borsh::BorshSerialize::serialize(&self.payer, writer)?;
                    borsh::BorshSerialize::serialize(&self.owner, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for InitTwoAuth {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.idendity,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.wrapper_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.approver,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.two_auth,
                                false,
                            ),
                        );
                    if let Some(two_auth_entity) = &self.two_auth_entity {
                        account_metas
                            .push(
                                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                    *two_auth_entity,
                                    true,
                                ),
                            );
                    } else {
                        account_metas
                            .push(
                                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                    crate::ID,
                                    false,
                                ),
                            );
                    }
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.payer,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.owner,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_init_two_auth {
            use super::*;
            /// Generated CPI struct of the accounts for [`InitTwoAuth`].
            pub struct InitTwoAuth<'info> {
                pub idendity: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub wrapper_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub approver: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub two_auth: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub two_auth_entity: Option<
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                >,
                pub payer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub owner: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for InitTwoAuth<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.idendity),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.wrapper_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.approver),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.two_auth),
                                false,
                            ),
                        );
                    if let Some(two_auth_entity) = &self.two_auth_entity {
                        account_metas
                            .push(
                                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                    anchor_lang::Key::key(two_auth_entity),
                                    true,
                                ),
                            );
                    } else {
                        account_metas
                            .push(
                                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                    crate::ID,
                                    false,
                                ),
                            );
                    }
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.payer),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.owner),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for InitTwoAuth<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.idendity),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.wrapper_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.approver),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.two_auth),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.two_auth_entity,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.payer),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.owner),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        #[instruction(two_auth_args:Option<TwoAuthArgs>)]
        pub struct UpdateTwoAuth<'info> {
            #[account(seeds = [b"identity", owner.key().as_ref()], bump)]
            pub idendity: Account<'info, IdAccount>,
            #[account(seeds = [b"wrapper", approver.key().as_ref()], bump)]
            pub wrapper_account: Account<'info, WrapperAccount>,
            /// CHECK: The approver of the wrapper
            pub approver: UncheckedAccount<'info>,
            #[account(
                mut,
                seeds = [b"two_auth",
                wrapper_account.key().as_ref(),
                owner.key().as_ref()],
                bump,
                realloc = TwoAuth::get_init_len(&two_auth_args),
                realloc::payer = owner,
                realloc::zero = true
            )]
            pub two_auth: Account<'info, TwoAuth>,
            pub two_auth_entity: Option<Signer<'info>>,
            pub old_two_auth_entity: Option<Signer<'info>>,
            #[account(mut)]
            pub owner: Signer<'info>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, UpdateTwoAuthBumps>
        for UpdateTwoAuth<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut UpdateTwoAuthBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let mut __ix_data = __ix_data;
                struct __Args {
                    two_auth_args: Option<TwoAuthArgs>,
                }
                impl borsh::ser::BorshSerialize for __Args
                where
                    Option<TwoAuthArgs>: borsh::ser::BorshSerialize,
                {
                    fn serialize<W: borsh::maybestd::io::Write>(
                        &self,
                        writer: &mut W,
                    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                        borsh::BorshSerialize::serialize(&self.two_auth_args, writer)?;
                        Ok(())
                    }
                }
                impl borsh::de::BorshDeserialize for __Args
                where
                    Option<TwoAuthArgs>: borsh::BorshDeserialize,
                {
                    fn deserialize_reader<R: borsh::maybestd::io::Read>(
                        reader: &mut R,
                    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                        Ok(Self {
                            two_auth_args: borsh::BorshDeserialize::deserialize_reader(
                                reader,
                            )?,
                        })
                    }
                }
                let __Args { two_auth_args } = __Args::deserialize(&mut __ix_data)
                    .map_err(|_| {
                        anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                    })?;
                let idendity: anchor_lang::accounts::account::Account<IdAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("idendity"))?;
                let wrapper_account: anchor_lang::accounts::account::Account<
                    WrapperAccount,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("wrapper_account"))?;
                let approver: UncheckedAccount = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("approver"))?;
                let two_auth: anchor_lang::accounts::account::Account<TwoAuth> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("two_auth"))?;
                let two_auth_entity: Option<Signer> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("two_auth_entity"))?;
                let old_two_auth_entity: Option<Signer> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("old_two_auth_entity"))?;
                let owner: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("owner"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[b"identity", owner.key().as_ref()],
                    &__program_id,
                );
                __bumps.idendity = __bump;
                if idendity.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("idendity")
                            .with_pubkeys((idendity.key(), __pda_address)),
                    );
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[b"wrapper", approver.key().as_ref()],
                    &__program_id,
                );
                __bumps.wrapper_account = __bump;
                if wrapper_account.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("wrapper_account")
                            .with_pubkeys((wrapper_account.key(), __pda_address)),
                    );
                }
                if __reallocs.contains(&two_auth.key()) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::AccountDuplicateReallocs,
                            )
                            .with_account_name("two_auth"),
                    );
                }
                let __anchor_rent = anchor_lang::prelude::Rent::get()?;
                let __field_info = two_auth.to_account_info();
                let __new_rent_minimum = __anchor_rent
                    .minimum_balance(TwoAuth::get_init_len(&two_auth_args));
                let __delta_space = (::std::convert::TryInto::<
                    isize,
                >::try_into(TwoAuth::get_init_len(&two_auth_args))
                    .unwrap())
                    .checked_sub(
                        ::std::convert::TryInto::try_into(__field_info.data_len())
                            .unwrap(),
                    )
                    .unwrap();
                if __delta_space != 0 {
                    if __delta_space > 0 {
                        if ::std::convert::TryInto::<usize>::try_into(__delta_space)
                            .unwrap()
                            > anchor_lang::solana_program::entrypoint::MAX_PERMITTED_DATA_INCREASE
                        {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::AccountReallocExceedsLimit,
                                    )
                                    .with_account_name("two_auth"),
                            );
                        }
                        if __new_rent_minimum > __field_info.lamports() {
                            anchor_lang::system_program::transfer(
                                anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    anchor_lang::system_program::Transfer {
                                        from: owner.to_account_info(),
                                        to: __field_info.clone(),
                                    },
                                ),
                                __new_rent_minimum
                                    .checked_sub(__field_info.lamports())
                                    .unwrap(),
                            )?;
                        }
                    } else {
                        let __lamport_amt = __field_info
                            .lamports()
                            .checked_sub(__new_rent_minimum)
                            .unwrap();
                        **owner.to_account_info().lamports.borrow_mut() = owner
                            .to_account_info()
                            .lamports()
                            .checked_add(__lamport_amt)
                            .unwrap();
                        **__field_info.lamports.borrow_mut() = __field_info
                            .lamports()
                            .checked_sub(__lamport_amt)
                            .unwrap();
                    }
                    __field_info.realloc(TwoAuth::get_init_len(&two_auth_args), true)?;
                    __reallocs.insert(two_auth.key());
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[b"two_auth", wrapper_account.key().as_ref(), owner.key().as_ref()],
                    &__program_id,
                );
                __bumps.two_auth = __bump;
                if two_auth.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("two_auth")
                            .with_pubkeys((two_auth.key(), __pda_address)),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&two_auth).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("two_auth"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&owner).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("owner"),
                    );
                }
                Ok(UpdateTwoAuth {
                    idendity,
                    wrapper_account,
                    approver,
                    two_auth,
                    two_auth_entity,
                    old_two_auth_entity,
                    owner,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for UpdateTwoAuth<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.idendity.to_account_infos());
                account_infos.extend(self.wrapper_account.to_account_infos());
                account_infos.extend(self.approver.to_account_infos());
                account_infos.extend(self.two_auth.to_account_infos());
                account_infos.extend(self.two_auth_entity.to_account_infos());
                account_infos.extend(self.old_two_auth_entity.to_account_infos());
                account_infos.extend(self.owner.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for UpdateTwoAuth<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.idendity.to_account_metas(None));
                account_metas.extend(self.wrapper_account.to_account_metas(None));
                account_metas.extend(self.approver.to_account_metas(None));
                account_metas.extend(self.two_auth.to_account_metas(None));
                if let Some(two_auth_entity) = &self.two_auth_entity {
                    account_metas.extend(two_auth_entity.to_account_metas(None));
                } else {
                    account_metas.push(AccountMeta::new_readonly(crate::ID, false));
                }
                if let Some(old_two_auth_entity) = &self.old_two_auth_entity {
                    account_metas.extend(old_two_auth_entity.to_account_metas(None));
                } else {
                    account_metas.push(AccountMeta::new_readonly(crate::ID, false));
                }
                account_metas.extend(self.owner.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for UpdateTwoAuth<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.two_auth, program_id)
                    .map_err(|e| e.with_account_name("two_auth"))?;
                anchor_lang::AccountsExit::exit(&self.owner, program_id)
                    .map_err(|e| e.with_account_name("owner"))?;
                Ok(())
            }
        }
        pub struct UpdateTwoAuthBumps {
            pub idendity: u8,
            pub wrapper_account: u8,
            pub two_auth: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for UpdateTwoAuthBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "UpdateTwoAuthBumps",
                    "idendity",
                    &self.idendity,
                    "wrapper_account",
                    &self.wrapper_account,
                    "two_auth",
                    &&self.two_auth,
                )
            }
        }
        impl Default for UpdateTwoAuthBumps {
            fn default() -> Self {
                UpdateTwoAuthBumps {
                    idendity: u8::MAX,
                    wrapper_account: u8::MAX,
                    two_auth: u8::MAX,
                }
            }
        }
        impl<'info> anchor_lang::Bumps for UpdateTwoAuth<'info>
        where
            'info: 'info,
        {
            type Bumps = UpdateTwoAuthBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_update_two_auth {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`UpdateTwoAuth`].
            pub struct UpdateTwoAuth {
                pub idendity: Pubkey,
                pub wrapper_account: Pubkey,
                pub approver: Pubkey,
                pub two_auth: Pubkey,
                pub two_auth_entity: Option<Pubkey>,
                pub old_two_auth_entity: Option<Pubkey>,
                pub owner: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for UpdateTwoAuth
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Option<Pubkey>: borsh::ser::BorshSerialize,
                Option<Pubkey>: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.idendity, writer)?;
                    borsh::BorshSerialize::serialize(&self.wrapper_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.approver, writer)?;
                    borsh::BorshSerialize::serialize(&self.two_auth, writer)?;
                    borsh::BorshSerialize::serialize(&self.two_auth_entity, writer)?;
                    borsh::BorshSerialize::serialize(&self.old_two_auth_entity, writer)?;
                    borsh::BorshSerialize::serialize(&self.owner, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for UpdateTwoAuth {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.idendity,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.wrapper_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.approver,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.two_auth,
                                false,
                            ),
                        );
                    if let Some(two_auth_entity) = &self.two_auth_entity {
                        account_metas
                            .push(
                                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                    *two_auth_entity,
                                    true,
                                ),
                            );
                    } else {
                        account_metas
                            .push(
                                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                    crate::ID,
                                    false,
                                ),
                            );
                    }
                    if let Some(old_two_auth_entity) = &self.old_two_auth_entity {
                        account_metas
                            .push(
                                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                    *old_two_auth_entity,
                                    true,
                                ),
                            );
                    } else {
                        account_metas
                            .push(
                                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                    crate::ID,
                                    false,
                                ),
                            );
                    }
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.owner,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_update_two_auth {
            use super::*;
            /// Generated CPI struct of the accounts for [`UpdateTwoAuth`].
            pub struct UpdateTwoAuth<'info> {
                pub idendity: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub wrapper_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub approver: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub two_auth: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub two_auth_entity: Option<
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                >,
                pub old_two_auth_entity: Option<
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                >,
                pub owner: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for UpdateTwoAuth<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.idendity),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.wrapper_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.approver),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.two_auth),
                                false,
                            ),
                        );
                    if let Some(two_auth_entity) = &self.two_auth_entity {
                        account_metas
                            .push(
                                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                    anchor_lang::Key::key(two_auth_entity),
                                    true,
                                ),
                            );
                    } else {
                        account_metas
                            .push(
                                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                    crate::ID,
                                    false,
                                ),
                            );
                    }
                    if let Some(old_two_auth_entity) = &self.old_two_auth_entity {
                        account_metas
                            .push(
                                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                    anchor_lang::Key::key(old_two_auth_entity),
                                    true,
                                ),
                            );
                    } else {
                        account_metas
                            .push(
                                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                    crate::ID,
                                    false,
                                ),
                            );
                    }
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.owner),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for UpdateTwoAuth<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.idendity),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.wrapper_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.approver),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.two_auth),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.two_auth_entity,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.old_two_auth_entity,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.owner),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub fn _initialize_two_auth(
            ctx: Context<InitTwoAuth>,
            two_auth_args: Option<TwoAuthArgs>,
        ) -> Result<()> {
            let idendity = &ctx.accounts.idendity;
            check_idendity_not_recovered(idendity)?;
            let two_auth = &mut ctx.accounts.two_auth;
            match two_auth_args {
                Some(two_auth_args) => {
                    let two_auth_entity = &ctx.accounts.two_auth_entity;
                    if two_auth_entity.is_none() {
                        return Err(TwoAuthError::NeedTwoAuthApproval.into());
                    }
                    let time = Clock::get()?.unix_timestamp;
                    let two_auth_parameters = TwoAuthParameters {
                        functions: two_auth_args
                            .functions
                            .iter()
                            .map(|function| init_functions(function, time))
                            .collect(),
                        two_auth_entity: two_auth_entity.as_ref().unwrap().key(),
                        allowed_issuers: two_auth_args.allowed_issuers.clone(),
                    };
                    two_auth.two_auth = Some(two_auth_parameters);
                }
                None => {
                    two_auth.two_auth = None;
                }
            }
            two_auth.last_tx = Clock::get()?.unix_timestamp;
            two_auth.bump = ctx.bumps.two_auth;
            Ok(())
        }
        pub fn _update_two_auth(
            ctx: Context<UpdateTwoAuth>,
            two_auth_args: Option<TwoAuthArgs>,
        ) -> Result<()> {
            let idendity = &ctx.accounts.idendity;
            check_idendity_not_recovered(idendity)?;
            let two_auth = &mut ctx.accounts.two_auth;
            let old_two_auth_entity = &ctx.accounts.old_two_auth_entity;
            check_authorization_old_two_auth_entity(old_two_auth_entity, two_auth)?;
            match two_auth_args {
                Some(two_auth_args) => {
                    let time = Clock::get()?.unix_timestamp;
                    let two_auth_entity = &ctx.accounts.two_auth_entity;
                    if two_auth_entity.is_none() {
                        return Err(TwoAuthError::NeedTwoAuthApproval.into());
                    }
                    let two_auth_parameters = TwoAuthParameters {
                        functions: two_auth_args
                            .functions
                            .iter()
                            .map(|function| init_functions(function, time))
                            .collect(),
                        two_auth_entity: two_auth_entity.as_ref().unwrap().key(),
                        allowed_issuers: two_auth_args.allowed_issuers.clone(),
                    };
                    two_auth.two_auth = Some(two_auth_parameters);
                }
                None => {
                    two_auth.two_auth = None;
                }
            }
            Ok(())
        }
        #[inline(always)]
        pub fn check_authorization_old_two_auth_entity(
            old_two_auth_entity: &Option<Signer>,
            two_auth: &TwoAuth,
        ) -> Result<()> {
            if two_auth.two_auth.is_some() {
                let two_auth_parameters = &two_auth.two_auth.as_ref().unwrap();
                match old_two_auth_entity {
                    Some(old_auth_entity) => {
                        if two_auth_parameters.two_auth_entity != old_auth_entity.key() {
                            return Err(TwoAuthError::NotAuthorized.into());
                        }
                    }
                    None => {
                        return Err(TwoAuthError::NeedTwoAuthApproval.into());
                    }
                }
            }
            Ok(())
        }
        #[inline(always)]
        pub fn init_functions(function: &TwoAuthFunction, time: i64) -> TwoAuthFunction {
            match function {
                TwoAuthFunction::CounterResetOnMax { counter: _, max } => {
                    TwoAuthFunction::CounterResetOnMax {
                        counter: 0,
                        max: *max,
                    }
                }
                TwoAuthFunction::CounterResetOnTime {
                    counter: _,
                    last_reset_time: _,
                    max,
                    duration,
                } => {
                    TwoAuthFunction::CounterResetOnTime {
                        counter: 0,
                        last_reset_time: time,
                        max: *max,
                        duration: duration.clone(),
                    }
                }
                TwoAuthFunction::CounterWithTimeWindow { window, max } => {
                    let duration = window.get_duration();
                    TwoAuthFunction::CounterWithTimeWindow {
                        window: CircularTimeWindow::new(duration, time),
                        max: *max,
                    }
                }
                _ => function.clone(),
            }
        }
        pub fn apply_two_auth_functions(
            amount: u64,
            functions: &mut Vec<TwoAuthFunction>,
            time: i64,
            receiver: Pubkey,
        ) -> bool {
            let mut need_two_auth;
            for function in functions.iter_mut() {
                need_two_auth = match_functions(amount, function, time, receiver);
                if need_two_auth.is_some() {
                    return need_two_auth.unwrap();
                }
            }
            return false;
        }
        pub fn match_functions(
            amount: u64,
            function: &mut TwoAuthFunction,
            time: i64,
            receiver: Pubkey,
        ) -> Option<bool> {
            match function {
                TwoAuthFunction::Always => Some(true),
                TwoAuthFunction::OnMax { max } => {
                    if amount >= *max { Some(true) } else { None }
                }
                TwoAuthFunction::CounterResetOnMax { max, counter } => {
                    let new_counter = counter.checked_add(amount);
                    if new_counter.is_none()
                        || new_counter.is_some() && new_counter.unwrap() >= *max
                    {
                        *counter = 0;
                        return Some(true);
                    }
                    return None;
                }
                TwoAuthFunction::CounterResetOnTime {
                    max,
                    duration,
                    counter,
                    last_reset_time,
                } => {
                    let diff = time.checked_sub(*last_reset_time);
                    if diff.is_some() && diff.unwrap() > duration.get() as i64 {
                        *counter = 0;
                        *last_reset_time = time;
                    }
                    let new_counter = counter.checked_add(amount);
                    if new_counter.is_some() && new_counter.unwrap() >= *max {
                        *counter = new_counter.unwrap();
                        return Some(true);
                    } else if new_counter.is_none() {
                        *counter = 0;
                        return Some(true);
                    }
                    return None;
                }
                TwoAuthFunction::CounterWithTimeWindow { max, window } => {
                    window.add(time, amount);
                    let count = window.get_count();
                    if count >= *max {
                        return Some(true);
                    }
                    return None;
                }
                TwoAuthFunction::DeactivateForUserSpecificWhiteList { white_list } => {
                    if white_list.contains(&receiver) {
                        return Some(false);
                    }
                    return None;
                }
            }
        }
    }
    pub use two_auth::*;
    pub mod recovery {
        use anchor_lang::{prelude::*, solana_program::program};
        use anchor_spl::{
            token::{spl_token, Token},
            token_interface::{Mint, TokenAccount},
        };
        use crate::{
            error::RecoveryError, IdAccount, RecoveryAuthorities, RecoveryAuthority,
            TwoAuth, WrapperAccount,
        };
        #[instruction(recovery_delegates:Vec<RecoveryAuthority>)]
        pub struct InitializeRecovery<'info> {
            #[account(
                init,
                seeds = [b"recovery",
                owner.key().as_ref()],
                bump,
                payer = payer,
                space = RecoveryAuthorities::get_init_len(&recovery_delegates)
            )]
            pub recovery_authority: Account<'info, RecoveryAuthorities>,
            #[account(mut)]
            pub payer: Signer<'info>,
            pub owner: Signer<'info>,
            pub mint: InterfaceAccount<'info, Mint>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, InitializeRecoveryBumps>
        for InitializeRecovery<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut InitializeRecoveryBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let mut __ix_data = __ix_data;
                struct __Args {
                    recovery_delegates: Vec<RecoveryAuthority>,
                }
                impl borsh::ser::BorshSerialize for __Args
                where
                    Vec<RecoveryAuthority>: borsh::ser::BorshSerialize,
                {
                    fn serialize<W: borsh::maybestd::io::Write>(
                        &self,
                        writer: &mut W,
                    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                        borsh::BorshSerialize::serialize(
                            &self.recovery_delegates,
                            writer,
                        )?;
                        Ok(())
                    }
                }
                impl borsh::de::BorshDeserialize for __Args
                where
                    Vec<RecoveryAuthority>: borsh::BorshDeserialize,
                {
                    fn deserialize_reader<R: borsh::maybestd::io::Read>(
                        reader: &mut R,
                    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                        Ok(Self {
                            recovery_delegates: borsh::BorshDeserialize::deserialize_reader(
                                reader,
                            )?,
                        })
                    }
                }
                let __Args { recovery_delegates } = __Args::deserialize(&mut __ix_data)
                    .map_err(|_| {
                        anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                    })?;
                if __accounts.is_empty() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into(),
                    );
                }
                let recovery_authority = &__accounts[0];
                *__accounts = &__accounts[1..];
                let payer: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("payer"))?;
                let owner: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("owner"))?;
                let mint: anchor_lang::accounts::interface_account::InterfaceAccount<
                    Mint,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("mint"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                let __anchor_rent = Rent::get()?;
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[b"recovery", owner.key().as_ref()],
                    __program_id,
                );
                __bumps.recovery_authority = __bump;
                if recovery_authority.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("recovery_authority")
                            .with_pubkeys((recovery_authority.key(), __pda_address)),
                    );
                }
                let recovery_authority = {
                    let actual_field = AsRef::<AccountInfo>::as_ref(&recovery_authority);
                    let actual_owner = actual_field.owner;
                    let space = RecoveryAuthorities::get_init_len(&recovery_delegates);
                    let pa: anchor_lang::accounts::account::Account<
                        RecoveryAuthorities,
                    > = if !false
                        || actual_owner
                            == &anchor_lang::solana_program::system_program::ID
                    {
                        let __current_lamports = recovery_authority.lamports();
                        if __current_lamports == 0 {
                            let space = space;
                            let lamports = __anchor_rent.minimum_balance(space);
                            let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                from: payer.to_account_info(),
                                to: recovery_authority.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::create_account(
                                cpi_context
                                    .with_signer(
                                        &[&[b"recovery", owner.key().as_ref(), &[__bump][..]][..]],
                                    ),
                                lamports,
                                space as u64,
                                __program_id,
                            )?;
                        } else {
                            if payer.key() == recovery_authority.key() {
                                return Err(
                                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                            error_name: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .name(),
                                            error_code_number: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .into(),
                                            error_msg: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .to_string(),
                                            error_origin: Some(
                                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                    filename: "programs/asset_based/src/instructions/recovery.rs",
                                                    line: 7u32,
                                                }),
                                            ),
                                            compared_values: None,
                                        })
                                        .with_pubkeys((payer.key(), recovery_authority.key())),
                                );
                            }
                            let required_lamports = __anchor_rent
                                .minimum_balance(space)
                                .max(1)
                                .saturating_sub(__current_lamports);
                            if required_lamports > 0 {
                                let cpi_accounts = anchor_lang::system_program::Transfer {
                                    from: payer.to_account_info(),
                                    to: recovery_authority.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::transfer(
                                    cpi_context,
                                    required_lamports,
                                )?;
                            }
                            let cpi_accounts = anchor_lang::system_program::Allocate {
                                account_to_allocate: recovery_authority.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::allocate(
                                cpi_context
                                    .with_signer(
                                        &[&[b"recovery", owner.key().as_ref(), &[__bump][..]][..]],
                                    ),
                                space as u64,
                            )?;
                            let cpi_accounts = anchor_lang::system_program::Assign {
                                account_to_assign: recovery_authority.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::assign(
                                cpi_context
                                    .with_signer(
                                        &[&[b"recovery", owner.key().as_ref(), &[__bump][..]][..]],
                                    ),
                                __program_id,
                            )?;
                        }
                        match anchor_lang::accounts::account::Account::try_from_unchecked(
                            &recovery_authority,
                        ) {
                            Ok(val) => val,
                            Err(e) => {
                                return Err(e.with_account_name("recovery_authority"));
                            }
                        }
                    } else {
                        match anchor_lang::accounts::account::Account::try_from(
                            &recovery_authority,
                        ) {
                            Ok(val) => val,
                            Err(e) => {
                                return Err(e.with_account_name("recovery_authority"));
                            }
                        }
                    };
                    if false {
                        if space != actual_field.data_len() {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::ConstraintSpace,
                                    )
                                    .with_account_name("recovery_authority")
                                    .with_values((space, actual_field.data_len())),
                            );
                        }
                        if actual_owner != __program_id {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::ConstraintOwner,
                                    )
                                    .with_account_name("recovery_authority")
                                    .with_pubkeys((*actual_owner, *__program_id)),
                            );
                        }
                        {
                            let required_lamports = __anchor_rent.minimum_balance(space);
                            if pa.to_account_info().lamports() < required_lamports {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintRentExempt,
                                        )
                                        .with_account_name("recovery_authority"),
                                );
                            }
                        }
                    }
                    pa
                };
                if !AsRef::<AccountInfo>::as_ref(&recovery_authority).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("recovery_authority"),
                    );
                }
                if !__anchor_rent
                    .is_exempt(
                        recovery_authority.to_account_info().lamports(),
                        recovery_authority.to_account_info().try_data_len()?,
                    )
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                            )
                            .with_account_name("recovery_authority"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&payer).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("payer"),
                    );
                }
                Ok(InitializeRecovery {
                    recovery_authority,
                    payer,
                    owner,
                    mint,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeRecovery<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.recovery_authority.to_account_infos());
                account_infos.extend(self.payer.to_account_infos());
                account_infos.extend(self.owner.to_account_infos());
                account_infos.extend(self.mint.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for InitializeRecovery<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.recovery_authority.to_account_metas(None));
                account_metas.extend(self.payer.to_account_metas(None));
                account_metas.extend(self.owner.to_account_metas(None));
                account_metas.extend(self.mint.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for InitializeRecovery<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.recovery_authority, program_id)
                    .map_err(|e| e.with_account_name("recovery_authority"))?;
                anchor_lang::AccountsExit::exit(&self.payer, program_id)
                    .map_err(|e| e.with_account_name("payer"))?;
                Ok(())
            }
        }
        pub struct InitializeRecoveryBumps {
            pub recovery_authority: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for InitializeRecoveryBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "InitializeRecoveryBumps",
                    "recovery_authority",
                    &&self.recovery_authority,
                )
            }
        }
        impl Default for InitializeRecoveryBumps {
            fn default() -> Self {
                InitializeRecoveryBumps {
                    recovery_authority: u8::MAX,
                }
            }
        }
        impl<'info> anchor_lang::Bumps for InitializeRecovery<'info>
        where
            'info: 'info,
        {
            type Bumps = InitializeRecoveryBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_initialize_recovery {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`InitializeRecovery`].
            pub struct InitializeRecovery {
                pub recovery_authority: Pubkey,
                pub payer: Pubkey,
                pub owner: Pubkey,
                pub mint: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for InitializeRecovery
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.recovery_authority, writer)?;
                    borsh::BorshSerialize::serialize(&self.payer, writer)?;
                    borsh::BorshSerialize::serialize(&self.owner, writer)?;
                    borsh::BorshSerialize::serialize(&self.mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for InitializeRecovery {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.recovery_authority,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.payer,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.owner,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.mint,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_initialize_recovery {
            use super::*;
            /// Generated CPI struct of the accounts for [`InitializeRecovery`].
            pub struct InitializeRecovery<'info> {
                pub recovery_authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub payer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub owner: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for InitializeRecovery<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.recovery_authority),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.payer),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.owner),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.mint),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info>
            for InitializeRecovery<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.recovery_authority,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.payer),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.owner),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.mint),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct RecoverAccount<'info> {
            #[account(seeds = [b"recovery", owner.key().as_ref()], bump)]
            pub recovery_authority: Account<'info, RecoveryAuthorities>,
            #[account(seeds = [b"wrapper", approver.key().as_ref()], bump)]
            pub wrapper_account: Account<'info, WrapperAccount>,
            /// CHECK: The approver of the wrapper
            pub approver: UncheckedAccount<'info>,
            #[account(
                mut,
                token::mint = mint,
                token::authority = wrapper_account,
                seeds = [b"wrapped_token",
                wrapper_account.key().as_ref(),
                mint.key().as_ref(),
                owner.key().as_ref()],
                bump
            )]
            pub user_wrapped_token_account: InterfaceAccount<'info, TokenAccount>,
            /// CHECK: Account to recover from
            pub owner: AccountInfo<'info>,
            #[account(
                init_if_needed,
                token::mint = mint,
                token::authority = wrapper_account,
                seeds = [b"wrapped_token",
                wrapper_account.key().as_ref(),
                mint.key().as_ref(),
                main_recovery_authority.key().as_ref()],
                bump,
                payer = main_recovery_authority
            )]
            pub recover_authority_wrapped_token_account: InterfaceAccount<
                'info,
                TokenAccount,
            >,
            #[account(mut)]
            pub main_recovery_authority: Signer<'info>,
            #[account(
                seeds = [b"two_auth",
                wrapper_account.key().as_ref(),
                owner.key().as_ref()],
                bump
            )]
            pub two_auth: Account<'info, TwoAuth>,
            #[account(seeds = [b"identity", owner.key().as_ref()], bump)]
            pub idendity: Account<'info, IdAccount>,
            pub mint: InterfaceAccount<'info, Mint>,
            pub token_program: Program<'info, Token>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, RecoverAccountBumps>
        for RecoverAccount<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut RecoverAccountBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let recovery_authority: anchor_lang::accounts::account::Account<
                    RecoveryAuthorities,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("recovery_authority"))?;
                let wrapper_account: anchor_lang::accounts::account::Account<
                    WrapperAccount,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("wrapper_account"))?;
                let approver: UncheckedAccount = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("approver"))?;
                let user_wrapped_token_account: anchor_lang::accounts::interface_account::InterfaceAccount<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("user_wrapped_token_account"))?;
                let owner: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("owner"))?;
                if __accounts.is_empty() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into(),
                    );
                }
                let recover_authority_wrapped_token_account = &__accounts[0];
                *__accounts = &__accounts[1..];
                let main_recovery_authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("main_recovery_authority"))?;
                let two_auth: anchor_lang::accounts::account::Account<TwoAuth> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("two_auth"))?;
                let idendity: anchor_lang::accounts::account::Account<IdAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("idendity"))?;
                let mint: anchor_lang::accounts::interface_account::InterfaceAccount<
                    Mint,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("mint"))?;
                let token_program: anchor_lang::accounts::program::Program<Token> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("token_program"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                let __anchor_rent = Rent::get()?;
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[
                        b"wrapped_token",
                        wrapper_account.key().as_ref(),
                        mint.key().as_ref(),
                        main_recovery_authority.key().as_ref(),
                    ],
                    __program_id,
                );
                __bumps.recover_authority_wrapped_token_account = __bump;
                if recover_authority_wrapped_token_account.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("recover_authority_wrapped_token_account")
                            .with_pubkeys((
                                recover_authority_wrapped_token_account.key(),
                                __pda_address,
                            )),
                    );
                }
                let recover_authority_wrapped_token_account: anchor_lang::accounts::interface_account::InterfaceAccount<
                    TokenAccount,
                > = {
                    let owner_program = AsRef::<
                        AccountInfo,
                    >::as_ref(&recover_authority_wrapped_token_account)
                        .owner;
                    if !true
                        || owner_program
                            == &anchor_lang::solana_program::system_program::ID
                    {
                        let __current_lamports = recover_authority_wrapped_token_account
                            .lamports();
                        if __current_lamports == 0 {
                            let space = {
                                let mint_info = mint.to_account_info();
                                if *mint_info.owner
                                    == ::anchor_spl::token_2022::Token2022::id()
                                {
                                    use ::anchor_spl::token_2022::spl_token_2022::extension::{
                                        BaseStateWithExtensions, ExtensionType, StateWithExtensions,
                                    };
                                    use ::anchor_spl::token_2022::spl_token_2022::state::{
                                        Account, Mint,
                                    };
                                    let mint_data = mint_info.try_borrow_data()?;
                                    let mint_state = StateWithExtensions::<
                                        Mint,
                                    >::unpack(&mint_data)?;
                                    let mint_extensions = mint_state.get_extension_types()?;
                                    let required_extensions = ExtensionType::get_required_init_account_extensions(
                                        &mint_extensions,
                                    );
                                    ExtensionType::try_calculate_account_len::<
                                        Account,
                                    >(&required_extensions)?
                                } else {
                                    ::anchor_spl::token::TokenAccount::LEN
                                }
                            };
                            let lamports = __anchor_rent.minimum_balance(space);
                            let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                from: main_recovery_authority.to_account_info(),
                                to: recover_authority_wrapped_token_account
                                    .to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::create_account(
                                cpi_context
                                    .with_signer(
                                        &[
                                            &[
                                                b"wrapped_token",
                                                wrapper_account.key().as_ref(),
                                                mint.key().as_ref(),
                                                main_recovery_authority.key().as_ref(),
                                                &[__bump][..],
                                            ][..],
                                        ],
                                    ),
                                lamports,
                                space as u64,
                                &token_program.key(),
                            )?;
                        } else {
                            if main_recovery_authority.key()
                                == recover_authority_wrapped_token_account.key()
                            {
                                return Err(
                                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                            error_name: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .name(),
                                            error_code_number: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .into(),
                                            error_msg: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .to_string(),
                                            error_origin: Some(
                                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                    filename: "programs/asset_based/src/instructions/recovery.rs",
                                                    line: 41u32,
                                                }),
                                            ),
                                            compared_values: None,
                                        })
                                        .with_pubkeys((
                                            main_recovery_authority.key(),
                                            recover_authority_wrapped_token_account.key(),
                                        )),
                                );
                            }
                            let required_lamports = __anchor_rent
                                .minimum_balance({
                                    let mint_info = mint.to_account_info();
                                    if *mint_info.owner
                                        == ::anchor_spl::token_2022::Token2022::id()
                                    {
                                        use ::anchor_spl::token_2022::spl_token_2022::extension::{
                                            BaseStateWithExtensions, ExtensionType, StateWithExtensions,
                                        };
                                        use ::anchor_spl::token_2022::spl_token_2022::state::{
                                            Account, Mint,
                                        };
                                        let mint_data = mint_info.try_borrow_data()?;
                                        let mint_state = StateWithExtensions::<
                                            Mint,
                                        >::unpack(&mint_data)?;
                                        let mint_extensions = mint_state.get_extension_types()?;
                                        let required_extensions = ExtensionType::get_required_init_account_extensions(
                                            &mint_extensions,
                                        );
                                        ExtensionType::try_calculate_account_len::<
                                            Account,
                                        >(&required_extensions)?
                                    } else {
                                        ::anchor_spl::token::TokenAccount::LEN
                                    }
                                })
                                .max(1)
                                .saturating_sub(__current_lamports);
                            if required_lamports > 0 {
                                let cpi_accounts = anchor_lang::system_program::Transfer {
                                    from: main_recovery_authority.to_account_info(),
                                    to: recover_authority_wrapped_token_account
                                        .to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::transfer(
                                    cpi_context,
                                    required_lamports,
                                )?;
                            }
                            let cpi_accounts = anchor_lang::system_program::Allocate {
                                account_to_allocate: recover_authority_wrapped_token_account
                                    .to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::allocate(
                                cpi_context
                                    .with_signer(
                                        &[
                                            &[
                                                b"wrapped_token",
                                                wrapper_account.key().as_ref(),
                                                mint.key().as_ref(),
                                                main_recovery_authority.key().as_ref(),
                                                &[__bump][..],
                                            ][..],
                                        ],
                                    ),
                                {
                                    let mint_info = mint.to_account_info();
                                    if *mint_info.owner
                                        == ::anchor_spl::token_2022::Token2022::id()
                                    {
                                        use ::anchor_spl::token_2022::spl_token_2022::extension::{
                                            BaseStateWithExtensions, ExtensionType, StateWithExtensions,
                                        };
                                        use ::anchor_spl::token_2022::spl_token_2022::state::{
                                            Account, Mint,
                                        };
                                        let mint_data = mint_info.try_borrow_data()?;
                                        let mint_state = StateWithExtensions::<
                                            Mint,
                                        >::unpack(&mint_data)?;
                                        let mint_extensions = mint_state.get_extension_types()?;
                                        let required_extensions = ExtensionType::get_required_init_account_extensions(
                                            &mint_extensions,
                                        );
                                        ExtensionType::try_calculate_account_len::<
                                            Account,
                                        >(&required_extensions)?
                                    } else {
                                        ::anchor_spl::token::TokenAccount::LEN
                                    }
                                } as u64,
                            )?;
                            let cpi_accounts = anchor_lang::system_program::Assign {
                                account_to_assign: recover_authority_wrapped_token_account
                                    .to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::assign(
                                cpi_context
                                    .with_signer(
                                        &[
                                            &[
                                                b"wrapped_token",
                                                wrapper_account.key().as_ref(),
                                                mint.key().as_ref(),
                                                main_recovery_authority.key().as_ref(),
                                                &[__bump][..],
                                            ][..],
                                        ],
                                    ),
                                &token_program.key(),
                            )?;
                        }
                        let cpi_program = token_program.to_account_info();
                        let accounts = ::anchor_spl::token_interface::InitializeAccount3 {
                            account: recover_authority_wrapped_token_account
                                .to_account_info(),
                            mint: mint.to_account_info(),
                            authority: wrapper_account.to_account_info(),
                        };
                        let cpi_ctx = anchor_lang::context::CpiContext::new(
                            cpi_program,
                            accounts,
                        );
                        ::anchor_spl::token_interface::initialize_account3(cpi_ctx)?;
                    }
                    let pa: anchor_lang::accounts::interface_account::InterfaceAccount<
                        TokenAccount,
                    > = match anchor_lang::accounts::interface_account::InterfaceAccount::try_from_unchecked(
                        &recover_authority_wrapped_token_account,
                    ) {
                        Ok(val) => val,
                        Err(e) => {
                            return Err(
                                e
                                    .with_account_name(
                                        "recover_authority_wrapped_token_account",
                                    ),
                            );
                        }
                    };
                    if true {
                        if pa.mint != mint.key() {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::ConstraintTokenMint,
                                    )
                                    .with_account_name(
                                        "recover_authority_wrapped_token_account",
                                    )
                                    .with_pubkeys((pa.mint, mint.key())),
                            );
                        }
                        if pa.owner != wrapper_account.key() {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                                    )
                                    .with_account_name(
                                        "recover_authority_wrapped_token_account",
                                    )
                                    .with_pubkeys((pa.owner, wrapper_account.key())),
                            );
                        }
                        if owner_program != &token_program.key() {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::ConstraintTokenTokenProgram,
                                    )
                                    .with_account_name(
                                        "recover_authority_wrapped_token_account",
                                    )
                                    .with_pubkeys((*owner_program, token_program.key())),
                            );
                        }
                    }
                    pa
                };
                if !AsRef::<
                    AccountInfo,
                >::as_ref(&recover_authority_wrapped_token_account)
                    .is_writable
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("recover_authority_wrapped_token_account"),
                    );
                }
                if !__anchor_rent
                    .is_exempt(
                        recover_authority_wrapped_token_account
                            .to_account_info()
                            .lamports(),
                        recover_authority_wrapped_token_account
                            .to_account_info()
                            .try_data_len()?,
                    )
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                            )
                            .with_account_name("recover_authority_wrapped_token_account"),
                    );
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[b"recovery", owner.key().as_ref()],
                    &__program_id,
                );
                __bumps.recovery_authority = __bump;
                if recovery_authority.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("recovery_authority")
                            .with_pubkeys((recovery_authority.key(), __pda_address)),
                    );
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[b"wrapper", approver.key().as_ref()],
                    &__program_id,
                );
                __bumps.wrapper_account = __bump;
                if wrapper_account.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("wrapper_account")
                            .with_pubkeys((wrapper_account.key(), __pda_address)),
                    );
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[
                        b"wrapped_token",
                        wrapper_account.key().as_ref(),
                        mint.key().as_ref(),
                        owner.key().as_ref(),
                    ],
                    &__program_id,
                );
                __bumps.user_wrapped_token_account = __bump;
                if user_wrapped_token_account.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("user_wrapped_token_account")
                            .with_pubkeys((
                                user_wrapped_token_account.key(),
                                __pda_address,
                            )),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&user_wrapped_token_account).is_writable
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("user_wrapped_token_account"),
                    );
                }
                {
                    if user_wrapped_token_account.owner != wrapper_account.key() {
                        return Err(
                            anchor_lang::error::ErrorCode::ConstraintTokenOwner.into(),
                        );
                    }
                    if user_wrapped_token_account.mint != mint.key() {
                        return Err(
                            anchor_lang::error::ErrorCode::ConstraintTokenMint.into(),
                        );
                    }
                }
                if !AsRef::<AccountInfo>::as_ref(&main_recovery_authority).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("main_recovery_authority"),
                    );
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[b"two_auth", wrapper_account.key().as_ref(), owner.key().as_ref()],
                    &__program_id,
                );
                __bumps.two_auth = __bump;
                if two_auth.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("two_auth")
                            .with_pubkeys((two_auth.key(), __pda_address)),
                    );
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[b"identity", owner.key().as_ref()],
                    &__program_id,
                );
                __bumps.idendity = __bump;
                if idendity.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("idendity")
                            .with_pubkeys((idendity.key(), __pda_address)),
                    );
                }
                Ok(RecoverAccount {
                    recovery_authority,
                    wrapper_account,
                    approver,
                    user_wrapped_token_account,
                    owner,
                    recover_authority_wrapped_token_account,
                    main_recovery_authority,
                    two_auth,
                    idendity,
                    mint,
                    token_program,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for RecoverAccount<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.recovery_authority.to_account_infos());
                account_infos.extend(self.wrapper_account.to_account_infos());
                account_infos.extend(self.approver.to_account_infos());
                account_infos.extend(self.user_wrapped_token_account.to_account_infos());
                account_infos.extend(self.owner.to_account_infos());
                account_infos
                    .extend(
                        self.recover_authority_wrapped_token_account.to_account_infos(),
                    );
                account_infos.extend(self.main_recovery_authority.to_account_infos());
                account_infos.extend(self.two_auth.to_account_infos());
                account_infos.extend(self.idendity.to_account_infos());
                account_infos.extend(self.mint.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for RecoverAccount<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.recovery_authority.to_account_metas(None));
                account_metas.extend(self.wrapper_account.to_account_metas(None));
                account_metas.extend(self.approver.to_account_metas(None));
                account_metas
                    .extend(self.user_wrapped_token_account.to_account_metas(None));
                account_metas.extend(self.owner.to_account_metas(None));
                account_metas
                    .extend(
                        self
                            .recover_authority_wrapped_token_account
                            .to_account_metas(None),
                    );
                account_metas
                    .extend(self.main_recovery_authority.to_account_metas(None));
                account_metas.extend(self.two_auth.to_account_metas(None));
                account_metas.extend(self.idendity.to_account_metas(None));
                account_metas.extend(self.mint.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for RecoverAccount<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(
                        &self.user_wrapped_token_account,
                        program_id,
                    )
                    .map_err(|e| e.with_account_name("user_wrapped_token_account"))?;
                anchor_lang::AccountsExit::exit(
                        &self.recover_authority_wrapped_token_account,
                        program_id,
                    )
                    .map_err(|e| {
                        e.with_account_name("recover_authority_wrapped_token_account")
                    })?;
                anchor_lang::AccountsExit::exit(
                        &self.main_recovery_authority,
                        program_id,
                    )
                    .map_err(|e| e.with_account_name("main_recovery_authority"))?;
                Ok(())
            }
        }
        pub struct RecoverAccountBumps {
            pub recovery_authority: u8,
            pub wrapper_account: u8,
            pub user_wrapped_token_account: u8,
            pub recover_authority_wrapped_token_account: u8,
            pub two_auth: u8,
            pub idendity: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for RecoverAccountBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "recovery_authority",
                    "wrapper_account",
                    "user_wrapped_token_account",
                    "recover_authority_wrapped_token_account",
                    "two_auth",
                    "idendity",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.recovery_authority,
                    &self.wrapper_account,
                    &self.user_wrapped_token_account,
                    &self.recover_authority_wrapped_token_account,
                    &self.two_auth,
                    &&self.idendity,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "RecoverAccountBumps",
                    names,
                    values,
                )
            }
        }
        impl Default for RecoverAccountBumps {
            fn default() -> Self {
                RecoverAccountBumps {
                    recovery_authority: u8::MAX,
                    wrapper_account: u8::MAX,
                    user_wrapped_token_account: u8::MAX,
                    recover_authority_wrapped_token_account: u8::MAX,
                    two_auth: u8::MAX,
                    idendity: u8::MAX,
                }
            }
        }
        impl<'info> anchor_lang::Bumps for RecoverAccount<'info>
        where
            'info: 'info,
        {
            type Bumps = RecoverAccountBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_recover_account {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`RecoverAccount`].
            pub struct RecoverAccount {
                pub recovery_authority: Pubkey,
                pub wrapper_account: Pubkey,
                pub approver: Pubkey,
                pub user_wrapped_token_account: Pubkey,
                pub owner: Pubkey,
                pub recover_authority_wrapped_token_account: Pubkey,
                pub main_recovery_authority: Pubkey,
                pub two_auth: Pubkey,
                pub idendity: Pubkey,
                pub mint: Pubkey,
                pub token_program: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for RecoverAccount
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.recovery_authority, writer)?;
                    borsh::BorshSerialize::serialize(&self.wrapper_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.approver, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.user_wrapped_token_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(&self.owner, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.recover_authority_wrapped_token_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(
                        &self.main_recovery_authority,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(&self.two_auth, writer)?;
                    borsh::BorshSerialize::serialize(&self.idendity, writer)?;
                    borsh::BorshSerialize::serialize(&self.mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for RecoverAccount {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.recovery_authority,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.wrapper_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.approver,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.user_wrapped_token_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.owner,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.recover_authority_wrapped_token_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.main_recovery_authority,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.two_auth,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.idendity,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.mint,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.token_program,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_recover_account {
            use super::*;
            /// Generated CPI struct of the accounts for [`RecoverAccount`].
            pub struct RecoverAccount<'info> {
                pub recovery_authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub wrapper_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub approver: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub user_wrapped_token_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub owner: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub recover_authority_wrapped_token_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub main_recovery_authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub two_auth: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub idendity: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for RecoverAccount<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.recovery_authority),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.wrapper_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.approver),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.user_wrapped_token_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.owner),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(
                                    &self.recover_authority_wrapped_token_account,
                                ),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.main_recovery_authority),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.two_auth),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.idendity),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.mint),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.token_program),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for RecoverAccount<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.recovery_authority,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.wrapper_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.approver),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.user_wrapped_token_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.owner),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.recover_authority_wrapped_token_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.main_recovery_authority,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.two_auth),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.idendity),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.mint),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.token_program,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
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
            let main_recovery = recovery_authority
                .authorities
                .iter()
                .find(|recovery| recovery.authority == main_recovery_authority.key());
            if main_recovery.is_none() {
                return Err(RecoveryError::WrongMainRecoveryAuthority.into());
            }
            let main_recovery = main_recovery.unwrap();
            let last_tx = &ctx.accounts.two_auth.last_tx;
            let timestamp = Clock::get()?.unix_timestamp;
            if *last_tx + (main_recovery.min_duration as i64) < timestamp {
                return Err(RecoveryError::RecoveryTimeNotPassed.into());
            }
            if main_recovery.min_signatures > 1 {
                let mut number_of_signatures = 1;
                let signers: Vec<_> = ctx
                    .remaining_accounts
                    .iter()
                    .filter(|account| {
                        account.is_signer && account.key != main_recovery_authority.key
                    })
                    .collect();
                for authority in recovery_authority
                    .authorities
                    .iter()
                    .filter(|authority| {
                        authority.min_signatures <= main_recovery.min_signatures
                            && *last_tx + (authority.min_duration as i64) < timestamp
                    })
                {
                    if signers.iter().any(|signer| signer.key == &authority.authority) {
                        number_of_signatures += 1;
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
            let seed: &[&[&[u8]]] = &[&[b"wrapper", approver.as_ref(), &[bump]]];
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
                    ctx
                        .accounts
                        .recover_authority_wrapped_token_account
                        .to_account_info(),
                    ctx.accounts.wrapper_account.to_account_info(),
                ],
                seed,
            )?;
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
    }
    pub use recovery::*;
    pub mod pseudo {
        use anchor_lang::prelude::*;
        use crate::{error::IdendityError, IdAccount, PseudoAccount};
        #[instruction(pseudo:String)]
        pub struct AddPseudo<'info> {
            #[account(
                mut,
                seeds = [b"identity",
                owner.key().as_ref()],
                bump = idendity.bump
            )]
            pub idendity: Account<'info, IdAccount>,
            #[account(
                init,
                seeds = [b"pseudo",
                pseudo.as_bytes()],
                bump,
                payer = payer,
                space = PseudoAccount::get_init_len(&pseudo)
            )]
            pub pseudo_account: Account<'info, PseudoAccount>,
            #[account(mut)]
            pub payer: Signer<'info>,
            #[account(mut)]
            pub owner: Signer<'info>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, AddPseudoBumps> for AddPseudo<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut AddPseudoBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let mut __ix_data = __ix_data;
                struct __Args {
                    pseudo: String,
                }
                impl borsh::ser::BorshSerialize for __Args
                where
                    String: borsh::ser::BorshSerialize,
                {
                    fn serialize<W: borsh::maybestd::io::Write>(
                        &self,
                        writer: &mut W,
                    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                        borsh::BorshSerialize::serialize(&self.pseudo, writer)?;
                        Ok(())
                    }
                }
                impl borsh::de::BorshDeserialize for __Args
                where
                    String: borsh::BorshDeserialize,
                {
                    fn deserialize_reader<R: borsh::maybestd::io::Read>(
                        reader: &mut R,
                    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                        Ok(Self {
                            pseudo: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        })
                    }
                }
                let __Args { pseudo } = __Args::deserialize(&mut __ix_data)
                    .map_err(|_| {
                        anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                    })?;
                let idendity: anchor_lang::accounts::account::Account<IdAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("idendity"))?;
                if __accounts.is_empty() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into(),
                    );
                }
                let pseudo_account = &__accounts[0];
                *__accounts = &__accounts[1..];
                let payer: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("payer"))?;
                let owner: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("owner"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                let __anchor_rent = Rent::get()?;
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[b"pseudo", pseudo.as_bytes()],
                    __program_id,
                );
                __bumps.pseudo_account = __bump;
                if pseudo_account.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("pseudo_account")
                            .with_pubkeys((pseudo_account.key(), __pda_address)),
                    );
                }
                let pseudo_account = {
                    let actual_field = AsRef::<AccountInfo>::as_ref(&pseudo_account);
                    let actual_owner = actual_field.owner;
                    let space = PseudoAccount::get_init_len(&pseudo);
                    let pa: anchor_lang::accounts::account::Account<PseudoAccount> = if !false
                        || actual_owner
                            == &anchor_lang::solana_program::system_program::ID
                    {
                        let __current_lamports = pseudo_account.lamports();
                        if __current_lamports == 0 {
                            let space = space;
                            let lamports = __anchor_rent.minimum_balance(space);
                            let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                from: payer.to_account_info(),
                                to: pseudo_account.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::create_account(
                                cpi_context
                                    .with_signer(
                                        &[&[b"pseudo", pseudo.as_bytes(), &[__bump][..]][..]],
                                    ),
                                lamports,
                                space as u64,
                                __program_id,
                            )?;
                        } else {
                            if payer.key() == pseudo_account.key() {
                                return Err(
                                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                            error_name: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .name(),
                                            error_code_number: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .into(),
                                            error_msg: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .to_string(),
                                            error_origin: Some(
                                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                    filename: "programs/asset_based/src/instructions/pseudo.rs",
                                                    line: 5u32,
                                                }),
                                            ),
                                            compared_values: None,
                                        })
                                        .with_pubkeys((payer.key(), pseudo_account.key())),
                                );
                            }
                            let required_lamports = __anchor_rent
                                .minimum_balance(space)
                                .max(1)
                                .saturating_sub(__current_lamports);
                            if required_lamports > 0 {
                                let cpi_accounts = anchor_lang::system_program::Transfer {
                                    from: payer.to_account_info(),
                                    to: pseudo_account.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::transfer(
                                    cpi_context,
                                    required_lamports,
                                )?;
                            }
                            let cpi_accounts = anchor_lang::system_program::Allocate {
                                account_to_allocate: pseudo_account.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::allocate(
                                cpi_context
                                    .with_signer(
                                        &[&[b"pseudo", pseudo.as_bytes(), &[__bump][..]][..]],
                                    ),
                                space as u64,
                            )?;
                            let cpi_accounts = anchor_lang::system_program::Assign {
                                account_to_assign: pseudo_account.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::assign(
                                cpi_context
                                    .with_signer(
                                        &[&[b"pseudo", pseudo.as_bytes(), &[__bump][..]][..]],
                                    ),
                                __program_id,
                            )?;
                        }
                        match anchor_lang::accounts::account::Account::try_from_unchecked(
                            &pseudo_account,
                        ) {
                            Ok(val) => val,
                            Err(e) => return Err(e.with_account_name("pseudo_account")),
                        }
                    } else {
                        match anchor_lang::accounts::account::Account::try_from(
                            &pseudo_account,
                        ) {
                            Ok(val) => val,
                            Err(e) => return Err(e.with_account_name("pseudo_account")),
                        }
                    };
                    if false {
                        if space != actual_field.data_len() {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::ConstraintSpace,
                                    )
                                    .with_account_name("pseudo_account")
                                    .with_values((space, actual_field.data_len())),
                            );
                        }
                        if actual_owner != __program_id {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::ConstraintOwner,
                                    )
                                    .with_account_name("pseudo_account")
                                    .with_pubkeys((*actual_owner, *__program_id)),
                            );
                        }
                        {
                            let required_lamports = __anchor_rent.minimum_balance(space);
                            if pa.to_account_info().lamports() < required_lamports {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintRentExempt,
                                        )
                                        .with_account_name("pseudo_account"),
                                );
                            }
                        }
                    }
                    pa
                };
                if !AsRef::<AccountInfo>::as_ref(&pseudo_account).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("pseudo_account"),
                    );
                }
                if !__anchor_rent
                    .is_exempt(
                        pseudo_account.to_account_info().lamports(),
                        pseudo_account.to_account_info().try_data_len()?,
                    )
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                            )
                            .with_account_name("pseudo_account"),
                    );
                }
                let __pda_address = Pubkey::create_program_address(
                        &[b"identity", owner.key().as_ref(), &[idendity.bump][..]],
                        &__program_id,
                    )
                    .map_err(|_| {
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("idendity")
                    })?;
                if idendity.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("idendity")
                            .with_pubkeys((idendity.key(), __pda_address)),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&idendity).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("idendity"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&payer).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("payer"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&owner).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("owner"),
                    );
                }
                Ok(AddPseudo {
                    idendity,
                    pseudo_account,
                    payer,
                    owner,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for AddPseudo<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.idendity.to_account_infos());
                account_infos.extend(self.pseudo_account.to_account_infos());
                account_infos.extend(self.payer.to_account_infos());
                account_infos.extend(self.owner.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for AddPseudo<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.idendity.to_account_metas(None));
                account_metas.extend(self.pseudo_account.to_account_metas(None));
                account_metas.extend(self.payer.to_account_metas(None));
                account_metas.extend(self.owner.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for AddPseudo<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.idendity, program_id)
                    .map_err(|e| e.with_account_name("idendity"))?;
                anchor_lang::AccountsExit::exit(&self.pseudo_account, program_id)
                    .map_err(|e| e.with_account_name("pseudo_account"))?;
                anchor_lang::AccountsExit::exit(&self.payer, program_id)
                    .map_err(|e| e.with_account_name("payer"))?;
                anchor_lang::AccountsExit::exit(&self.owner, program_id)
                    .map_err(|e| e.with_account_name("owner"))?;
                Ok(())
            }
        }
        pub struct AddPseudoBumps {
            pub pseudo_account: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AddPseudoBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "AddPseudoBumps",
                    "pseudo_account",
                    &&self.pseudo_account,
                )
            }
        }
        impl Default for AddPseudoBumps {
            fn default() -> Self {
                AddPseudoBumps {
                    pseudo_account: u8::MAX,
                }
            }
        }
        impl<'info> anchor_lang::Bumps for AddPseudo<'info>
        where
            'info: 'info,
        {
            type Bumps = AddPseudoBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_add_pseudo {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`AddPseudo`].
            pub struct AddPseudo {
                pub idendity: Pubkey,
                pub pseudo_account: Pubkey,
                pub payer: Pubkey,
                pub owner: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for AddPseudo
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.idendity, writer)?;
                    borsh::BorshSerialize::serialize(&self.pseudo_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.payer, writer)?;
                    borsh::BorshSerialize::serialize(&self.owner, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for AddPseudo {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.idendity,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.pseudo_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.payer,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.owner,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_add_pseudo {
            use super::*;
            /// Generated CPI struct of the accounts for [`AddPseudo`].
            pub struct AddPseudo<'info> {
                pub idendity: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub pseudo_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub payer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub owner: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for AddPseudo<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.idendity),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.pseudo_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.payer),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.owner),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for AddPseudo<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.idendity),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.pseudo_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.payer),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.owner),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        #[instruction(pseudo:String)]
        pub struct UpdatePseudo<'info> {
            #[account(
                mut,
                seeds = [b"identity",
                owner.key().as_ref()],
                bump = idendity.bump
            )]
            pub idendity: Account<'info, IdAccount>,
            #[account(
                init,
                seeds = [b"pseudo",
                pseudo.as_bytes()],
                bump,
                payer = owner,
                space = PseudoAccount::get_init_len(&pseudo)
            )]
            pub new_pseudo_account: Account<'info, PseudoAccount>,
            #[account(
                mut,
                close = owner,
                constraint = old_pseudo_account.key(

                )= = idendity.associated_pseudo.ok_or(
                    IdendityError::PseudoDontExist
                )?.key()
            )]
            pub old_pseudo_account: Account<'info, PseudoAccount>,
            #[account(mut)]
            pub owner: Signer<'info>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, UpdatePseudoBumps>
        for UpdatePseudo<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut UpdatePseudoBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let mut __ix_data = __ix_data;
                struct __Args {
                    pseudo: String,
                }
                impl borsh::ser::BorshSerialize for __Args
                where
                    String: borsh::ser::BorshSerialize,
                {
                    fn serialize<W: borsh::maybestd::io::Write>(
                        &self,
                        writer: &mut W,
                    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                        borsh::BorshSerialize::serialize(&self.pseudo, writer)?;
                        Ok(())
                    }
                }
                impl borsh::de::BorshDeserialize for __Args
                where
                    String: borsh::BorshDeserialize,
                {
                    fn deserialize_reader<R: borsh::maybestd::io::Read>(
                        reader: &mut R,
                    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                        Ok(Self {
                            pseudo: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        })
                    }
                }
                let __Args { pseudo } = __Args::deserialize(&mut __ix_data)
                    .map_err(|_| {
                        anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                    })?;
                let idendity: anchor_lang::accounts::account::Account<IdAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("idendity"))?;
                if __accounts.is_empty() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into(),
                    );
                }
                let new_pseudo_account = &__accounts[0];
                *__accounts = &__accounts[1..];
                let old_pseudo_account: anchor_lang::accounts::account::Account<
                    PseudoAccount,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("old_pseudo_account"))?;
                let owner: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("owner"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                let __anchor_rent = Rent::get()?;
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[b"pseudo", pseudo.as_bytes()],
                    __program_id,
                );
                __bumps.new_pseudo_account = __bump;
                if new_pseudo_account.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("new_pseudo_account")
                            .with_pubkeys((new_pseudo_account.key(), __pda_address)),
                    );
                }
                let new_pseudo_account = {
                    let actual_field = AsRef::<AccountInfo>::as_ref(&new_pseudo_account);
                    let actual_owner = actual_field.owner;
                    let space = PseudoAccount::get_init_len(&pseudo);
                    let pa: anchor_lang::accounts::account::Account<PseudoAccount> = if !false
                        || actual_owner
                            == &anchor_lang::solana_program::system_program::ID
                    {
                        let __current_lamports = new_pseudo_account.lamports();
                        if __current_lamports == 0 {
                            let space = space;
                            let lamports = __anchor_rent.minimum_balance(space);
                            let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                from: owner.to_account_info(),
                                to: new_pseudo_account.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::create_account(
                                cpi_context
                                    .with_signer(
                                        &[&[b"pseudo", pseudo.as_bytes(), &[__bump][..]][..]],
                                    ),
                                lamports,
                                space as u64,
                                __program_id,
                            )?;
                        } else {
                            if owner.key() == new_pseudo_account.key() {
                                return Err(
                                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                            error_name: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .name(),
                                            error_code_number: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .into(),
                                            error_msg: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                .to_string(),
                                            error_origin: Some(
                                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                    filename: "programs/asset_based/src/instructions/pseudo.rs",
                                                    line: 19u32,
                                                }),
                                            ),
                                            compared_values: None,
                                        })
                                        .with_pubkeys((owner.key(), new_pseudo_account.key())),
                                );
                            }
                            let required_lamports = __anchor_rent
                                .minimum_balance(space)
                                .max(1)
                                .saturating_sub(__current_lamports);
                            if required_lamports > 0 {
                                let cpi_accounts = anchor_lang::system_program::Transfer {
                                    from: owner.to_account_info(),
                                    to: new_pseudo_account.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::transfer(
                                    cpi_context,
                                    required_lamports,
                                )?;
                            }
                            let cpi_accounts = anchor_lang::system_program::Allocate {
                                account_to_allocate: new_pseudo_account.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::allocate(
                                cpi_context
                                    .with_signer(
                                        &[&[b"pseudo", pseudo.as_bytes(), &[__bump][..]][..]],
                                    ),
                                space as u64,
                            )?;
                            let cpi_accounts = anchor_lang::system_program::Assign {
                                account_to_assign: new_pseudo_account.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::assign(
                                cpi_context
                                    .with_signer(
                                        &[&[b"pseudo", pseudo.as_bytes(), &[__bump][..]][..]],
                                    ),
                                __program_id,
                            )?;
                        }
                        match anchor_lang::accounts::account::Account::try_from_unchecked(
                            &new_pseudo_account,
                        ) {
                            Ok(val) => val,
                            Err(e) => {
                                return Err(e.with_account_name("new_pseudo_account"));
                            }
                        }
                    } else {
                        match anchor_lang::accounts::account::Account::try_from(
                            &new_pseudo_account,
                        ) {
                            Ok(val) => val,
                            Err(e) => {
                                return Err(e.with_account_name("new_pseudo_account"));
                            }
                        }
                    };
                    if false {
                        if space != actual_field.data_len() {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::ConstraintSpace,
                                    )
                                    .with_account_name("new_pseudo_account")
                                    .with_values((space, actual_field.data_len())),
                            );
                        }
                        if actual_owner != __program_id {
                            return Err(
                                anchor_lang::error::Error::from(
                                        anchor_lang::error::ErrorCode::ConstraintOwner,
                                    )
                                    .with_account_name("new_pseudo_account")
                                    .with_pubkeys((*actual_owner, *__program_id)),
                            );
                        }
                        {
                            let required_lamports = __anchor_rent.minimum_balance(space);
                            if pa.to_account_info().lamports() < required_lamports {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintRentExempt,
                                        )
                                        .with_account_name("new_pseudo_account"),
                                );
                            }
                        }
                    }
                    pa
                };
                if !AsRef::<AccountInfo>::as_ref(&new_pseudo_account).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("new_pseudo_account"),
                    );
                }
                if !__anchor_rent
                    .is_exempt(
                        new_pseudo_account.to_account_info().lamports(),
                        new_pseudo_account.to_account_info().try_data_len()?,
                    )
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                            )
                            .with_account_name("new_pseudo_account"),
                    );
                }
                let __pda_address = Pubkey::create_program_address(
                        &[b"identity", owner.key().as_ref(), &[idendity.bump][..]],
                        &__program_id,
                    )
                    .map_err(|_| {
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("idendity")
                    })?;
                if idendity.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("idendity")
                            .with_pubkeys((idendity.key(), __pda_address)),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&idendity).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("idendity"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&old_pseudo_account).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("old_pseudo_account"),
                    );
                }
                if !(old_pseudo_account.key()
                    == idendity
                        .associated_pseudo
                        .ok_or(IdendityError::PseudoDontExist)?
                        .key())
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("old_pseudo_account"),
                    );
                }
                {
                    if old_pseudo_account.key() == owner.key() {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintClose,
                                )
                                .with_account_name("old_pseudo_account"),
                        );
                    }
                }
                if !AsRef::<AccountInfo>::as_ref(&owner).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("owner"),
                    );
                }
                Ok(UpdatePseudo {
                    idendity,
                    new_pseudo_account,
                    old_pseudo_account,
                    owner,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for UpdatePseudo<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.idendity.to_account_infos());
                account_infos.extend(self.new_pseudo_account.to_account_infos());
                account_infos.extend(self.old_pseudo_account.to_account_infos());
                account_infos.extend(self.owner.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for UpdatePseudo<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.idendity.to_account_metas(None));
                account_metas.extend(self.new_pseudo_account.to_account_metas(None));
                account_metas.extend(self.old_pseudo_account.to_account_metas(None));
                account_metas.extend(self.owner.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for UpdatePseudo<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.idendity, program_id)
                    .map_err(|e| e.with_account_name("idendity"))?;
                anchor_lang::AccountsExit::exit(&self.new_pseudo_account, program_id)
                    .map_err(|e| e.with_account_name("new_pseudo_account"))?;
                {
                    let owner = &self.owner;
                    anchor_lang::AccountsClose::close(
                            &self.old_pseudo_account,
                            owner.to_account_info(),
                        )
                        .map_err(|e| e.with_account_name("old_pseudo_account"))?;
                }
                anchor_lang::AccountsExit::exit(&self.owner, program_id)
                    .map_err(|e| e.with_account_name("owner"))?;
                Ok(())
            }
        }
        pub struct UpdatePseudoBumps {
            pub new_pseudo_account: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for UpdatePseudoBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "UpdatePseudoBumps",
                    "new_pseudo_account",
                    &&self.new_pseudo_account,
                )
            }
        }
        impl Default for UpdatePseudoBumps {
            fn default() -> Self {
                UpdatePseudoBumps {
                    new_pseudo_account: u8::MAX,
                }
            }
        }
        impl<'info> anchor_lang::Bumps for UpdatePseudo<'info>
        where
            'info: 'info,
        {
            type Bumps = UpdatePseudoBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_update_pseudo {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`UpdatePseudo`].
            pub struct UpdatePseudo {
                pub idendity: Pubkey,
                pub new_pseudo_account: Pubkey,
                pub old_pseudo_account: Pubkey,
                pub owner: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for UpdatePseudo
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.idendity, writer)?;
                    borsh::BorshSerialize::serialize(&self.new_pseudo_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.old_pseudo_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.owner, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for UpdatePseudo {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.idendity,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.new_pseudo_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.old_pseudo_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.owner,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_update_pseudo {
            use super::*;
            /// Generated CPI struct of the accounts for [`UpdatePseudo`].
            pub struct UpdatePseudo<'info> {
                pub idendity: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub new_pseudo_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub old_pseudo_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub owner: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for UpdatePseudo<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.idendity),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.new_pseudo_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.old_pseudo_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.owner),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for UpdatePseudo<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.idendity),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.new_pseudo_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.old_pseudo_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.owner),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub fn _add_pseudo(ctx: Context<AddPseudo>, pseudo: String) -> Result<()> {
            let idendity_account = &mut ctx.accounts.idendity;
            let pseudo_account = &mut ctx.accounts.pseudo_account;
            if idendity_account.associated_pseudo.is_some() {
                return Err(IdendityError::PseudoAlreadyExist.into());
            } else {
                idendity_account.associated_pseudo = Some(pseudo_account.key());
                pseudo_account.initialized = true;
                pseudo_account.bump = ctx.bumps.pseudo_account;
                pseudo_account.owner = ctx.accounts.owner.key();
                pseudo_account.pseudo = pseudo;
                Ok(())
            }
        }
        pub fn _update_pseudo(ctx: Context<UpdatePseudo>, pseudo: String) -> Result<()> {
            let idendity_account = &mut ctx.accounts.idendity;
            idendity_account.associated_pseudo = Some(
                ctx.accounts.new_pseudo_account.key(),
            );
            let new_pseudo_account = &mut ctx.accounts.new_pseudo_account;
            new_pseudo_account.initialized = true;
            new_pseudo_account.bump = ctx.bumps.new_pseudo_account;
            new_pseudo_account.owner = ctx.accounts.owner.key();
            new_pseudo_account.pseudo = pseudo;
            Ok(())
        }
    }
    pub use pseudo::*;
}
pub mod state {
    pub mod idendity_account {
        use anchor_lang::prelude::*;
        pub struct IdAccount {
            pub owner: Pubkey,
            pub issuers: Vec<Issuer>,
            pub recovered_address: Option<Pubkey>,
            pub associated_pseudo: Option<Pubkey>,
            pub bump: u8,
        }
        impl borsh::ser::BorshSerialize for IdAccount
        where
            Pubkey: borsh::ser::BorshSerialize,
            Vec<Issuer>: borsh::ser::BorshSerialize,
            Option<Pubkey>: borsh::ser::BorshSerialize,
            Option<Pubkey>: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.owner, writer)?;
                borsh::BorshSerialize::serialize(&self.issuers, writer)?;
                borsh::BorshSerialize::serialize(&self.recovered_address, writer)?;
                borsh::BorshSerialize::serialize(&self.associated_pseudo, writer)?;
                borsh::BorshSerialize::serialize(&self.bump, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for IdAccount
        where
            Pubkey: borsh::BorshDeserialize,
            Vec<Issuer>: borsh::BorshDeserialize,
            Option<Pubkey>: borsh::BorshDeserialize,
            Option<Pubkey>: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    owner: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    issuers: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    recovered_address: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    associated_pseudo: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    bump: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for IdAccount {
            #[inline]
            fn clone(&self) -> IdAccount {
                IdAccount {
                    owner: ::core::clone::Clone::clone(&self.owner),
                    issuers: ::core::clone::Clone::clone(&self.issuers),
                    recovered_address: ::core::clone::Clone::clone(
                        &self.recovered_address,
                    ),
                    associated_pseudo: ::core::clone::Clone::clone(
                        &self.associated_pseudo,
                    ),
                    bump: ::core::clone::Clone::clone(&self.bump),
                }
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountSerialize for IdAccount {
            fn try_serialize<W: std::io::Write>(
                &self,
                writer: &mut W,
            ) -> anchor_lang::Result<()> {
                if writer.write_all(&[97, 148, 16, 183, 235, 126, 146, 150]).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                if AnchorSerialize::serialize(self, writer).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountDeserialize for IdAccount {
            fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                if buf.len() < [97, 148, 16, 183, 235, 126, 146, 150].len() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound
                            .into(),
                    );
                }
                let given_disc = &buf[..8];
                if &[97, 148, 16, 183, 235, 126, 146, 150] != given_disc {
                    return Err(
                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .name(),
                                error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .into(),
                                error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .to_string(),
                                error_origin: Some(
                                    anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                        filename: "programs/asset_based/src/state/idendity_account.rs",
                                        line: 3u32,
                                    }),
                                ),
                                compared_values: None,
                            })
                            .with_account_name("IdAccount"),
                    );
                }
                Self::try_deserialize_unchecked(buf)
            }
            fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                let mut data: &[u8] = &buf[8..];
                AnchorDeserialize::deserialize(&mut data)
                    .map_err(|_| {
                        anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                    })
            }
        }
        #[automatically_derived]
        impl anchor_lang::Discriminator for IdAccount {
            const DISCRIMINATOR: [u8; 8] = [97, 148, 16, 183, 235, 126, 146, 150];
        }
        #[automatically_derived]
        impl anchor_lang::Owner for IdAccount {
            fn owner() -> Pubkey {
                crate::ID
            }
        }
        impl IdAccount {
            pub const INIT_LEN: usize = 8 + 32 + 32 + 4 + 49 + 1 + (1 + 32) + 1;
            pub fn get_add_issuer_len(&self) -> usize {
                return 8 + 32 + 4 + self.issuers.len() * 49 + 49 + 1 + (1 + 32) + 1;
            }
            pub fn get_recover_len(&self) -> usize {
                return 8 + 32 + 4 + self.issuers.len() * 49 + 1 + 32 + (1 + 32) + 1;
            }
        }
        pub struct Issuer {
            pub key: Pubkey,
            pub last_modified: i64,
            pub expires_at: i64,
            pub active: bool,
        }
        impl borsh::ser::BorshSerialize for Issuer
        where
            Pubkey: borsh::ser::BorshSerialize,
            i64: borsh::ser::BorshSerialize,
            i64: borsh::ser::BorshSerialize,
            bool: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.key, writer)?;
                borsh::BorshSerialize::serialize(&self.last_modified, writer)?;
                borsh::BorshSerialize::serialize(&self.expires_at, writer)?;
                borsh::BorshSerialize::serialize(&self.active, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for Issuer
        where
            Pubkey: borsh::BorshDeserialize,
            i64: borsh::BorshDeserialize,
            i64: borsh::BorshDeserialize,
            bool: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    key: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    last_modified: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    expires_at: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    active: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Issuer {
            #[inline]
            fn clone(&self) -> Issuer {
                Issuer {
                    key: ::core::clone::Clone::clone(&self.key),
                    last_modified: ::core::clone::Clone::clone(&self.last_modified),
                    expires_at: ::core::clone::Clone::clone(&self.expires_at),
                    active: ::core::clone::Clone::clone(&self.active),
                }
            }
        }
    }
    pub use idendity_account::*;
    pub mod two_auth_account {
        use anchor_lang::prelude::*;
        pub struct TwoAuthParameters {
            pub functions: Vec<TwoAuthFunction>,
            pub two_auth_entity: Pubkey,
            pub allowed_issuers: Vec<Pubkey>,
        }
        impl borsh::ser::BorshSerialize for TwoAuthParameters
        where
            Vec<TwoAuthFunction>: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Vec<Pubkey>: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.functions, writer)?;
                borsh::BorshSerialize::serialize(&self.two_auth_entity, writer)?;
                borsh::BorshSerialize::serialize(&self.allowed_issuers, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for TwoAuthParameters
        where
            Vec<TwoAuthFunction>: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            Vec<Pubkey>: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    functions: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    two_auth_entity: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    allowed_issuers: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TwoAuthParameters {
            #[inline]
            fn clone(&self) -> TwoAuthParameters {
                TwoAuthParameters {
                    functions: ::core::clone::Clone::clone(&self.functions),
                    two_auth_entity: ::core::clone::Clone::clone(&self.two_auth_entity),
                    allowed_issuers: ::core::clone::Clone::clone(&self.allowed_issuers),
                }
            }
        }
        pub struct TwoAuthArgs {
            pub functions: Vec<TwoAuthFunction>,
            pub allowed_issuers: Vec<Pubkey>,
        }
        impl borsh::ser::BorshSerialize for TwoAuthArgs
        where
            Vec<TwoAuthFunction>: borsh::ser::BorshSerialize,
            Vec<Pubkey>: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.functions, writer)?;
                borsh::BorshSerialize::serialize(&self.allowed_issuers, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for TwoAuthArgs
        where
            Vec<TwoAuthFunction>: borsh::BorshDeserialize,
            Vec<Pubkey>: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    functions: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    allowed_issuers: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TwoAuthArgs {
            #[inline]
            fn clone(&self) -> TwoAuthArgs {
                TwoAuthArgs {
                    functions: ::core::clone::Clone::clone(&self.functions),
                    allowed_issuers: ::core::clone::Clone::clone(&self.allowed_issuers),
                }
            }
        }
        pub struct TwoAuth {
            pub two_auth: Option<TwoAuthParameters>,
            pub last_tx: i64,
            pub bump: u8,
        }
        impl borsh::ser::BorshSerialize for TwoAuth
        where
            Option<TwoAuthParameters>: borsh::ser::BorshSerialize,
            i64: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.two_auth, writer)?;
                borsh::BorshSerialize::serialize(&self.last_tx, writer)?;
                borsh::BorshSerialize::serialize(&self.bump, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for TwoAuth
        where
            Option<TwoAuthParameters>: borsh::BorshDeserialize,
            i64: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    two_auth: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    last_tx: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    bump: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TwoAuth {
            #[inline]
            fn clone(&self) -> TwoAuth {
                TwoAuth {
                    two_auth: ::core::clone::Clone::clone(&self.two_auth),
                    last_tx: ::core::clone::Clone::clone(&self.last_tx),
                    bump: ::core::clone::Clone::clone(&self.bump),
                }
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountSerialize for TwoAuth {
            fn try_serialize<W: std::io::Write>(
                &self,
                writer: &mut W,
            ) -> anchor_lang::Result<()> {
                if writer.write_all(&[130, 227, 165, 108, 170, 222, 193, 225]).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                if AnchorSerialize::serialize(self, writer).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountDeserialize for TwoAuth {
            fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                if buf.len() < [130, 227, 165, 108, 170, 222, 193, 225].len() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound
                            .into(),
                    );
                }
                let given_disc = &buf[..8];
                if &[130, 227, 165, 108, 170, 222, 193, 225] != given_disc {
                    return Err(
                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .name(),
                                error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .into(),
                                error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .to_string(),
                                error_origin: Some(
                                    anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                        filename: "programs/asset_based/src/state/two_auth_account.rs",
                                        line: 16u32,
                                    }),
                                ),
                                compared_values: None,
                            })
                            .with_account_name("TwoAuth"),
                    );
                }
                Self::try_deserialize_unchecked(buf)
            }
            fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                let mut data: &[u8] = &buf[8..];
                AnchorDeserialize::deserialize(&mut data)
                    .map_err(|_| {
                        anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                    })
            }
        }
        #[automatically_derived]
        impl anchor_lang::Discriminator for TwoAuth {
            const DISCRIMINATOR: [u8; 8] = [130, 227, 165, 108, 170, 222, 193, 225];
        }
        #[automatically_derived]
        impl anchor_lang::Owner for TwoAuth {
            fn owner() -> Pubkey {
                crate::ID
            }
        }
        impl TwoAuth {
            pub fn get_init_len(two_auth_args: &Option<TwoAuthArgs>) -> usize {
                match two_auth_args {
                    Some(TwoAuthArgs { functions, allowed_issuers }) => {
                        let functions_space = functions
                            .iter()
                            .map(|f| f.get_init_len())
                            .sum::<usize>();
                        return 8 + 1 + 4 + functions_space + 32 + 4
                            + 32 * allowed_issuers.len() + 8 + 1;
                    }
                    None => 8 + 1 + 8 + 1,
                }
            }
        }
        pub enum TwoAuthFunction {
            Always,
            OnMax { max: u64 },
            CounterResetOnMax { max: u64, counter: u64 },
            CounterResetOnTime {
                max: u64,
                duration: Duration,
                counter: u64,
                last_reset_time: i64,
            },
            CounterWithTimeWindow { max: u64, window: CircularTimeWindow },
            DeactivateForUserSpecificWhiteList { white_list: Vec<Pubkey> },
        }
        impl borsh::ser::BorshSerialize for TwoAuthFunction
        where
            u64: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            Duration: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            i64: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            CircularTimeWindow: borsh::ser::BorshSerialize,
            Vec<Pubkey>: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                let variant_idx: u8 = match self {
                    TwoAuthFunction::Always => 0u8,
                    TwoAuthFunction::OnMax { .. } => 1u8,
                    TwoAuthFunction::CounterResetOnMax { .. } => 2u8,
                    TwoAuthFunction::CounterResetOnTime { .. } => 3u8,
                    TwoAuthFunction::CounterWithTimeWindow { .. } => 4u8,
                    TwoAuthFunction::DeactivateForUserSpecificWhiteList { .. } => 5u8,
                };
                writer.write_all(&variant_idx.to_le_bytes())?;
                match self {
                    TwoAuthFunction::Always => {}
                    TwoAuthFunction::OnMax { max } => {
                        borsh::BorshSerialize::serialize(max, writer)?;
                    }
                    TwoAuthFunction::CounterResetOnMax { max, counter } => {
                        borsh::BorshSerialize::serialize(max, writer)?;
                        borsh::BorshSerialize::serialize(counter, writer)?;
                    }
                    TwoAuthFunction::CounterResetOnTime {
                        max,
                        duration,
                        counter,
                        last_reset_time,
                    } => {
                        borsh::BorshSerialize::serialize(max, writer)?;
                        borsh::BorshSerialize::serialize(duration, writer)?;
                        borsh::BorshSerialize::serialize(counter, writer)?;
                        borsh::BorshSerialize::serialize(last_reset_time, writer)?;
                    }
                    TwoAuthFunction::CounterWithTimeWindow { max, window } => {
                        borsh::BorshSerialize::serialize(max, writer)?;
                        borsh::BorshSerialize::serialize(window, writer)?;
                    }
                    TwoAuthFunction::DeactivateForUserSpecificWhiteList {
                        white_list,
                    } => {
                        borsh::BorshSerialize::serialize(white_list, writer)?;
                    }
                }
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for TwoAuthFunction
        where
            u64: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            Duration: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            i64: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            CircularTimeWindow: borsh::BorshDeserialize,
            Vec<Pubkey>: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                let tag = <u8 as borsh::de::BorshDeserialize>::deserialize_reader(
                    reader,
                )?;
                <Self as borsh::de::EnumExt>::deserialize_variant(reader, tag)
            }
        }
        impl borsh::de::EnumExt for TwoAuthFunction
        where
            u64: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            Duration: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            i64: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            CircularTimeWindow: borsh::BorshDeserialize,
            Vec<Pubkey>: borsh::BorshDeserialize,
        {
            fn deserialize_variant<R: borsh::maybestd::io::Read>(
                reader: &mut R,
                variant_idx: u8,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                let mut return_value = match variant_idx {
                    0u8 => TwoAuthFunction::Always,
                    1u8 => {
                        TwoAuthFunction::OnMax {
                            max: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        }
                    }
                    2u8 => {
                        TwoAuthFunction::CounterResetOnMax {
                            max: borsh::BorshDeserialize::deserialize_reader(reader)?,
                            counter: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        }
                    }
                    3u8 => {
                        TwoAuthFunction::CounterResetOnTime {
                            max: borsh::BorshDeserialize::deserialize_reader(reader)?,
                            duration: borsh::BorshDeserialize::deserialize_reader(
                                reader,
                            )?,
                            counter: borsh::BorshDeserialize::deserialize_reader(
                                reader,
                            )?,
                            last_reset_time: borsh::BorshDeserialize::deserialize_reader(
                                reader,
                            )?,
                        }
                    }
                    4u8 => {
                        TwoAuthFunction::CounterWithTimeWindow {
                            max: borsh::BorshDeserialize::deserialize_reader(reader)?,
                            window: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        }
                    }
                    5u8 => {
                        TwoAuthFunction::DeactivateForUserSpecificWhiteList {
                            white_list: borsh::BorshDeserialize::deserialize_reader(
                                reader,
                            )?,
                        }
                    }
                    _ => {
                        return Err(
                            borsh::maybestd::io::Error::new(
                                borsh::maybestd::io::ErrorKind::InvalidInput,
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!("Unexpected variant index: {0:?}", variant_idx),
                                    );
                                    res
                                },
                            ),
                        );
                    }
                };
                Ok(return_value)
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TwoAuthFunction {
            #[inline]
            fn clone(&self) -> TwoAuthFunction {
                match self {
                    TwoAuthFunction::Always => TwoAuthFunction::Always,
                    TwoAuthFunction::OnMax { max: __self_0 } => {
                        TwoAuthFunction::OnMax {
                            max: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    TwoAuthFunction::CounterResetOnMax {
                        max: __self_0,
                        counter: __self_1,
                    } => {
                        TwoAuthFunction::CounterResetOnMax {
                            max: ::core::clone::Clone::clone(__self_0),
                            counter: ::core::clone::Clone::clone(__self_1),
                        }
                    }
                    TwoAuthFunction::CounterResetOnTime {
                        max: __self_0,
                        duration: __self_1,
                        counter: __self_2,
                        last_reset_time: __self_3,
                    } => {
                        TwoAuthFunction::CounterResetOnTime {
                            max: ::core::clone::Clone::clone(__self_0),
                            duration: ::core::clone::Clone::clone(__self_1),
                            counter: ::core::clone::Clone::clone(__self_2),
                            last_reset_time: ::core::clone::Clone::clone(__self_3),
                        }
                    }
                    TwoAuthFunction::CounterWithTimeWindow {
                        max: __self_0,
                        window: __self_1,
                    } => {
                        TwoAuthFunction::CounterWithTimeWindow {
                            max: ::core::clone::Clone::clone(__self_0),
                            window: ::core::clone::Clone::clone(__self_1),
                        }
                    }
                    TwoAuthFunction::DeactivateForUserSpecificWhiteList {
                        white_list: __self_0,
                    } => {
                        TwoAuthFunction::DeactivateForUserSpecificWhiteList {
                            white_list: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                }
            }
        }
        impl TwoAuthFunction {
            pub fn get_init_len(&self) -> usize {
                match self {
                    TwoAuthFunction::Always => 1,
                    TwoAuthFunction::OnMax { .. } => 1 + 8,
                    TwoAuthFunction::CounterResetOnMax { .. } => 1 + 8 + 8,
                    TwoAuthFunction::CounterResetOnTime { .. } => {
                        1 + 8 + Duration::LEN + 8 + 8
                    }
                    TwoAuthFunction::CounterWithTimeWindow { max: _, window } => {
                        1 + 8 + window.get_init_len()
                    }
                    TwoAuthFunction::DeactivateForUserSpecificWhiteList {
                        white_list,
                    } => 1 + 4 + 32 * white_list.len(),
                }
            }
        }
        pub struct CircularTimeWindow {
            start_index: u8,
            window: Vec<u64>,
            duration: Duration,
            last_value_time: i64,
        }
        impl borsh::ser::BorshSerialize for CircularTimeWindow
        where
            u8: borsh::ser::BorshSerialize,
            Vec<u64>: borsh::ser::BorshSerialize,
            Duration: borsh::ser::BorshSerialize,
            i64: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.start_index, writer)?;
                borsh::BorshSerialize::serialize(&self.window, writer)?;
                borsh::BorshSerialize::serialize(&self.duration, writer)?;
                borsh::BorshSerialize::serialize(&self.last_value_time, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for CircularTimeWindow
        where
            u8: borsh::BorshDeserialize,
            Vec<u64>: borsh::BorshDeserialize,
            Duration: borsh::BorshDeserialize,
            i64: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    start_index: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    window: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    duration: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    last_value_time: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CircularTimeWindow {
            #[inline]
            fn clone(&self) -> CircularTimeWindow {
                CircularTimeWindow {
                    start_index: ::core::clone::Clone::clone(&self.start_index),
                    window: ::core::clone::Clone::clone(&self.window),
                    duration: ::core::clone::Clone::clone(&self.duration),
                    last_value_time: ::core::clone::Clone::clone(&self.last_value_time),
                }
            }
        }
        impl CircularTimeWindow {
            pub fn get_init_len(&self) -> usize {
                return 4 + 1 + 4 + (self.duration.get() as usize) * 8 + Duration::LEN
                    + 8;
            }
            pub fn new(duration: Duration, time: i64) -> Self {
                CircularTimeWindow {
                    window: ::alloc::vec::from_elem(0, duration.get() as usize),
                    start_index: 0,
                    duration: duration,
                    last_value_time: time,
                }
            }
            pub fn get_duration(&self) -> Duration {
                self.duration.clone()
            }
            pub fn add(&mut self, time: i64, value: u64) {
                let diff = self.get_time_difference_duration(time);
                if diff == 0 {
                    self.window[self.start_index as usize] += value;
                } else {
                    let new_index = (self.start_index as usize + diff as usize)
                        % self.window.len();
                    self.circular_reset_values_between_indexes(
                        self.start_index as usize,
                        new_index,
                    );
                    self.start_index = new_index as u8;
                    self.window[self.start_index as usize] = value;
                }
                self.last_value_time = time;
            }
            pub fn get(&self, index: u8) -> u64 {
                self.window[(self.start_index + index) as usize % self.window.len()]
            }
            pub fn get_count(&self) -> u64 {
                return self.window.iter().sum();
            }
            fn circular_reset_values_between_indexes(
                &mut self,
                index1: usize,
                index2: usize,
            ) {
                if index1 < index2 {
                    for i in (index1 + 1)..index2 {
                        self.window[i] = 0;
                    }
                } else {
                    for i in (index1 + 1)..self.window.len() {
                        self.window[i] = 0;
                    }
                    for i in 0..index2 {
                        self.window[i] = 0;
                    }
                }
            }
            fn get_time_difference_duration(&self, time: i64) -> u8 {
                let diff = time - self.last_value_time;
                if diff < 0 {
                    return 0;
                }
                match self.duration {
                    Duration::Seconds(t) => Self::u8_with_overflow(diff, t),
                    Duration::Minutes(t) => Self::u8_with_overflow(diff / 60, t),
                    Duration::Hours(t) => Self::u8_with_overflow(diff / 3600, t),
                    Duration::Days(t) => Self::u8_with_overflow(diff / 86400, t),
                    Duration::Weeks(t) => Self::u8_with_overflow(diff / 604800, t),
                }
            }
            fn u8_with_overflow(time_diff: i64, overflow_value: u8) -> u8 {
                if time_diff > overflow_value as i64 {
                    return overflow_value;
                } else {
                    return time_diff as u8;
                }
            }
        }
        pub enum Duration {
            Seconds(u8),
            Minutes(u8),
            Hours(u8),
            Days(u8),
            Weeks(u8),
        }
        impl borsh::ser::BorshSerialize for Duration
        where
            u8: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                let variant_idx: u8 = match self {
                    Duration::Seconds(..) => 0u8,
                    Duration::Minutes(..) => 1u8,
                    Duration::Hours(..) => 2u8,
                    Duration::Days(..) => 3u8,
                    Duration::Weeks(..) => 4u8,
                };
                writer.write_all(&variant_idx.to_le_bytes())?;
                match self {
                    Duration::Seconds(id0) => {
                        borsh::BorshSerialize::serialize(id0, writer)?;
                    }
                    Duration::Minutes(id0) => {
                        borsh::BorshSerialize::serialize(id0, writer)?;
                    }
                    Duration::Hours(id0) => {
                        borsh::BorshSerialize::serialize(id0, writer)?;
                    }
                    Duration::Days(id0) => {
                        borsh::BorshSerialize::serialize(id0, writer)?;
                    }
                    Duration::Weeks(id0) => {
                        borsh::BorshSerialize::serialize(id0, writer)?;
                    }
                }
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for Duration
        where
            u8: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                let tag = <u8 as borsh::de::BorshDeserialize>::deserialize_reader(
                    reader,
                )?;
                <Self as borsh::de::EnumExt>::deserialize_variant(reader, tag)
            }
        }
        impl borsh::de::EnumExt for Duration
        where
            u8: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
        {
            fn deserialize_variant<R: borsh::maybestd::io::Read>(
                reader: &mut R,
                variant_idx: u8,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                let mut return_value = match variant_idx {
                    0u8 => {
                        Duration::Seconds(
                            borsh::BorshDeserialize::deserialize_reader(reader)?,
                        )
                    }
                    1u8 => {
                        Duration::Minutes(
                            borsh::BorshDeserialize::deserialize_reader(reader)?,
                        )
                    }
                    2u8 => {
                        Duration::Hours(
                            borsh::BorshDeserialize::deserialize_reader(reader)?,
                        )
                    }
                    3u8 => {
                        Duration::Days(
                            borsh::BorshDeserialize::deserialize_reader(reader)?,
                        )
                    }
                    4u8 => {
                        Duration::Weeks(
                            borsh::BorshDeserialize::deserialize_reader(reader)?,
                        )
                    }
                    _ => {
                        return Err(
                            borsh::maybestd::io::Error::new(
                                borsh::maybestd::io::ErrorKind::InvalidInput,
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!("Unexpected variant index: {0:?}", variant_idx),
                                    );
                                    res
                                },
                            ),
                        );
                    }
                };
                Ok(return_value)
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Duration {
            #[inline]
            fn clone(&self) -> Duration {
                match self {
                    Duration::Seconds(__self_0) => {
                        Duration::Seconds(::core::clone::Clone::clone(__self_0))
                    }
                    Duration::Minutes(__self_0) => {
                        Duration::Minutes(::core::clone::Clone::clone(__self_0))
                    }
                    Duration::Hours(__self_0) => {
                        Duration::Hours(::core::clone::Clone::clone(__self_0))
                    }
                    Duration::Days(__self_0) => {
                        Duration::Days(::core::clone::Clone::clone(__self_0))
                    }
                    Duration::Weeks(__self_0) => {
                        Duration::Weeks(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        impl Duration {
            pub const LEN: usize = 2;
            pub fn get(&self) -> u8 {
                match self {
                    Duration::Seconds(t)
                    | Duration::Minutes(t)
                    | Duration::Hours(t)
                    | Duration::Days(t)
                    | Duration::Weeks(t) => *t,
                }
            }
        }
    }
    pub use two_auth_account::*;
    pub mod wrapper_account {
        use anchor_lang::prelude::*;
        pub struct WrapperAccount {
            pub approver: Pubkey,
            pub id_issuers: Vec<Pubkey>,
            pub exit_regulators: Vec<Pubkey>,
            pub bump: u8,
        }
        impl borsh::ser::BorshSerialize for WrapperAccount
        where
            Pubkey: borsh::ser::BorshSerialize,
            Vec<Pubkey>: borsh::ser::BorshSerialize,
            Vec<Pubkey>: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.approver, writer)?;
                borsh::BorshSerialize::serialize(&self.id_issuers, writer)?;
                borsh::BorshSerialize::serialize(&self.exit_regulators, writer)?;
                borsh::BorshSerialize::serialize(&self.bump, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for WrapperAccount
        where
            Pubkey: borsh::BorshDeserialize,
            Vec<Pubkey>: borsh::BorshDeserialize,
            Vec<Pubkey>: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    approver: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    id_issuers: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    exit_regulators: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    bump: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for WrapperAccount {
            #[inline]
            fn clone(&self) -> WrapperAccount {
                WrapperAccount {
                    approver: ::core::clone::Clone::clone(&self.approver),
                    id_issuers: ::core::clone::Clone::clone(&self.id_issuers),
                    exit_regulators: ::core::clone::Clone::clone(&self.exit_regulators),
                    bump: ::core::clone::Clone::clone(&self.bump),
                }
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountSerialize for WrapperAccount {
            fn try_serialize<W: std::io::Write>(
                &self,
                writer: &mut W,
            ) -> anchor_lang::Result<()> {
                if writer.write_all(&[66, 159, 143, 36, 181, 246, 145, 205]).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                if AnchorSerialize::serialize(self, writer).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountDeserialize for WrapperAccount {
            fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                if buf.len() < [66, 159, 143, 36, 181, 246, 145, 205].len() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound
                            .into(),
                    );
                }
                let given_disc = &buf[..8];
                if &[66, 159, 143, 36, 181, 246, 145, 205] != given_disc {
                    return Err(
                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .name(),
                                error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .into(),
                                error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .to_string(),
                                error_origin: Some(
                                    anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                        filename: "programs/asset_based/src/state/wrapper_account.rs",
                                        line: 3u32,
                                    }),
                                ),
                                compared_values: None,
                            })
                            .with_account_name("WrapperAccount"),
                    );
                }
                Self::try_deserialize_unchecked(buf)
            }
            fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                let mut data: &[u8] = &buf[8..];
                AnchorDeserialize::deserialize(&mut data)
                    .map_err(|_| {
                        anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                    })
            }
        }
        #[automatically_derived]
        impl anchor_lang::Discriminator for WrapperAccount {
            const DISCRIMINATOR: [u8; 8] = [66, 159, 143, 36, 181, 246, 145, 205];
        }
        #[automatically_derived]
        impl anchor_lang::Owner for WrapperAccount {
            fn owner() -> Pubkey {
                crate::ID
            }
        }
        impl WrapperAccount {
            pub fn get_init_len(
                is_issuers: &Vec<Pubkey>,
                exit_regulators: &Vec<Pubkey>,
            ) -> usize {
                return 8 + 32 + 4 + 32 * is_issuers.len() + 4
                    + 32 * exit_regulators.len() + 1;
            }
            pub fn get_len_add_address(&self) -> usize {
                return Self::get_init_len(&self.id_issuers, &self.exit_regulators) + 32;
            }
            pub fn get_len_remove_address(&self) -> usize {
                return Self::get_init_len(&self.id_issuers, &self.exit_regulators) - 32;
            }
        }
    }
    pub use wrapper_account::*;
    pub mod recovery_account {
        use anchor_lang::prelude::*;
        pub struct RecoveryAuthorities {
            pub authorities: Vec<RecoveryAuthority>,
        }
        impl borsh::ser::BorshSerialize for RecoveryAuthorities
        where
            Vec<RecoveryAuthority>: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.authorities, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for RecoveryAuthorities
        where
            Vec<RecoveryAuthority>: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    authorities: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for RecoveryAuthorities {
            #[inline]
            fn clone(&self) -> RecoveryAuthorities {
                RecoveryAuthorities {
                    authorities: ::core::clone::Clone::clone(&self.authorities),
                }
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountSerialize for RecoveryAuthorities {
            fn try_serialize<W: std::io::Write>(
                &self,
                writer: &mut W,
            ) -> anchor_lang::Result<()> {
                if writer.write_all(&[123, 17, 210, 150, 42, 52, 233, 39]).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                if AnchorSerialize::serialize(self, writer).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountDeserialize for RecoveryAuthorities {
            fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                if buf.len() < [123, 17, 210, 150, 42, 52, 233, 39].len() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound
                            .into(),
                    );
                }
                let given_disc = &buf[..8];
                if &[123, 17, 210, 150, 42, 52, 233, 39] != given_disc {
                    return Err(
                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .name(),
                                error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .into(),
                                error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .to_string(),
                                error_origin: Some(
                                    anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                        filename: "programs/asset_based/src/state/recovery_account.rs",
                                        line: 6u32,
                                    }),
                                ),
                                compared_values: None,
                            })
                            .with_account_name("RecoveryAuthorities"),
                    );
                }
                Self::try_deserialize_unchecked(buf)
            }
            fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                let mut data: &[u8] = &buf[8..];
                AnchorDeserialize::deserialize(&mut data)
                    .map_err(|_| {
                        anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                    })
            }
        }
        #[automatically_derived]
        impl anchor_lang::Discriminator for RecoveryAuthorities {
            const DISCRIMINATOR: [u8; 8] = [123, 17, 210, 150, 42, 52, 233, 39];
        }
        #[automatically_derived]
        impl anchor_lang::Owner for RecoveryAuthorities {
            fn owner() -> Pubkey {
                crate::ID
            }
        }
        pub struct RecoveryAuthority {
            pub authority: Pubkey,
            pub min_signatures: u8,
            pub min_duration: u32,
        }
        impl borsh::ser::BorshSerialize for RecoveryAuthority
        where
            Pubkey: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
            u32: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.authority, writer)?;
                borsh::BorshSerialize::serialize(&self.min_signatures, writer)?;
                borsh::BorshSerialize::serialize(&self.min_duration, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for RecoveryAuthority
        where
            Pubkey: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            u32: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    authority: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    min_signatures: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    min_duration: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for RecoveryAuthority {
            #[inline]
            fn clone(&self) -> RecoveryAuthority {
                RecoveryAuthority {
                    authority: ::core::clone::Clone::clone(&self.authority),
                    min_signatures: ::core::clone::Clone::clone(&self.min_signatures),
                    min_duration: ::core::clone::Clone::clone(&self.min_duration),
                }
            }
        }
        impl RecoveryAuthorities {
            pub fn get_init_len(recovery_authorities: &Vec<RecoveryAuthority>) -> usize {
                return 8 + 4 + recovery_authorities.len() * (32 + 1 + 4);
            }
        }
    }
    pub use recovery_account::*;
    pub mod pseudo_account {
        use anchor_lang::prelude::*;
        pub struct PseudoAccount {
            pub initialized: bool,
            pub bump: u8,
            pub owner: Pubkey,
            pub pseudo: String,
        }
        impl borsh::ser::BorshSerialize for PseudoAccount
        where
            bool: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            String: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.initialized, writer)?;
                borsh::BorshSerialize::serialize(&self.bump, writer)?;
                borsh::BorshSerialize::serialize(&self.owner, writer)?;
                borsh::BorshSerialize::serialize(&self.pseudo, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for PseudoAccount
        where
            bool: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            String: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    initialized: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    bump: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    owner: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    pseudo: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for PseudoAccount {
            #[inline]
            fn clone(&self) -> PseudoAccount {
                PseudoAccount {
                    initialized: ::core::clone::Clone::clone(&self.initialized),
                    bump: ::core::clone::Clone::clone(&self.bump),
                    owner: ::core::clone::Clone::clone(&self.owner),
                    pseudo: ::core::clone::Clone::clone(&self.pseudo),
                }
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountSerialize for PseudoAccount {
            fn try_serialize<W: std::io::Write>(
                &self,
                writer: &mut W,
            ) -> anchor_lang::Result<()> {
                if writer.write_all(&[115, 176, 208, 64, 36, 83, 249, 61]).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                if AnchorSerialize::serialize(self, writer).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountDeserialize for PseudoAccount {
            fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                if buf.len() < [115, 176, 208, 64, 36, 83, 249, 61].len() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound
                            .into(),
                    );
                }
                let given_disc = &buf[..8];
                if &[115, 176, 208, 64, 36, 83, 249, 61] != given_disc {
                    return Err(
                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .name(),
                                error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .into(),
                                error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .to_string(),
                                error_origin: Some(
                                    anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                        filename: "programs/asset_based/src/state/pseudo_account.rs",
                                        line: 3u32,
                                    }),
                                ),
                                compared_values: None,
                            })
                            .with_account_name("PseudoAccount"),
                    );
                }
                Self::try_deserialize_unchecked(buf)
            }
            fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                let mut data: &[u8] = &buf[8..];
                AnchorDeserialize::deserialize(&mut data)
                    .map_err(|_| {
                        anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                    })
            }
        }
        #[automatically_derived]
        impl anchor_lang::Discriminator for PseudoAccount {
            const DISCRIMINATOR: [u8; 8] = [115, 176, 208, 64, 36, 83, 249, 61];
        }
        #[automatically_derived]
        impl anchor_lang::Owner for PseudoAccount {
            fn owner() -> Pubkey {
                crate::ID
            }
        }
        impl PseudoAccount {
            pub fn get_init_len(pseudo: &String) -> usize {
                return 8 + 1 + 1 + 32 + 4 + pseudo.len();
            }
        }
    }
    pub use pseudo_account::*;
    pub mod shared_account {
        use anchor_lang::prelude::*;
        pub struct SharedAccount {
            pub owners: Vec<Pubkey>,
        }
        impl borsh::ser::BorshSerialize for SharedAccount
        where
            Vec<Pubkey>: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.owners, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for SharedAccount
        where
            Vec<Pubkey>: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    owners: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for SharedAccount {
            #[inline]
            fn clone(&self) -> SharedAccount {
                SharedAccount {
                    owners: ::core::clone::Clone::clone(&self.owners),
                }
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountSerialize for SharedAccount {
            fn try_serialize<W: std::io::Write>(
                &self,
                writer: &mut W,
            ) -> anchor_lang::Result<()> {
                if writer.write_all(&[184, 4, 4, 114, 152, 128, 233, 178]).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                if AnchorSerialize::serialize(self, writer).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountDeserialize for SharedAccount {
            fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                if buf.len() < [184, 4, 4, 114, 152, 128, 233, 178].len() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound
                            .into(),
                    );
                }
                let given_disc = &buf[..8];
                if &[184, 4, 4, 114, 152, 128, 233, 178] != given_disc {
                    return Err(
                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .name(),
                                error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .into(),
                                error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .to_string(),
                                error_origin: Some(
                                    anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                        filename: "programs/asset_based/src/state/shared_account.rs",
                                        line: 3u32,
                                    }),
                                ),
                                compared_values: None,
                            })
                            .with_account_name("SharedAccount"),
                    );
                }
                Self::try_deserialize_unchecked(buf)
            }
            fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                let mut data: &[u8] = &buf[8..];
                AnchorDeserialize::deserialize(&mut data)
                    .map_err(|_| {
                        anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                    })
            }
        }
        #[automatically_derived]
        impl anchor_lang::Discriminator for SharedAccount {
            const DISCRIMINATOR: [u8; 8] = [184, 4, 4, 114, 152, 128, 233, 178];
        }
        #[automatically_derived]
        impl anchor_lang::Owner for SharedAccount {
            fn owner() -> Pubkey {
                crate::ID
            }
        }
        impl SharedAccount {
            pub fn get_init_len(owners: &Vec<Pubkey>) -> usize {
                return 8 + 4 + owners.len() * 32;
            }
        }
    }
    pub use shared_account::*;
}
use anchor_lang::prelude::*;
pub use instructions::*;
pub use state::*;
/// The static program ID
pub static ID: anchor_lang::solana_program::pubkey::Pubkey = anchor_lang::solana_program::pubkey::Pubkey::new_from_array([
    230u8,
    117u8,
    115u8,
    171u8,
    183u8,
    244u8,
    244u8,
    236u8,
    78u8,
    116u8,
    59u8,
    192u8,
    178u8,
    95u8,
    15u8,
    87u8,
    239u8,
    31u8,
    212u8,
    68u8,
    13u8,
    138u8,
    227u8,
    195u8,
    165u8,
    13u8,
    211u8,
    0u8,
    210u8,
    85u8,
    75u8,
    71u8,
]);
/// Confirms that a given pubkey is equivalent to the program ID
pub fn check_id(id: &anchor_lang::solana_program::pubkey::Pubkey) -> bool {
    id == &ID
}
/// Returns the program ID
pub fn id() -> anchor_lang::solana_program::pubkey::Pubkey {
    ID
}
use self::asset_based::*;
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
    let (program_id, accounts, instruction_data) = unsafe {
        ::solana_program::entrypoint::deserialize(input)
    };
    match entry(&program_id, &accounts, &instruction_data) {
        Ok(()) => ::solana_program::entrypoint::SUCCESS,
        Err(error) => error.into(),
    }
}
/// The Anchor codegen exposes a programming model where a user defines
/// a set of methods inside of a `#[program]` module in a way similar
/// to writing RPC request handlers. The macro then generates a bunch of
/// code wrapping these user defined methods into something that can be
/// executed on Solana.
///
/// These methods fall into one category for now.
///
/// Global methods - regular methods inside of the `#[program]`.
///
/// Care must be taken by the codegen to prevent collisions between
/// methods in these different namespaces. For this reason, Anchor uses
/// a variant of sighash to perform method dispatch, rather than
/// something like a simple enum variant discriminator.
///
/// The execution flow of the generated code can be roughly outlined:
///
/// * Start program via the entrypoint.
/// * Strip method identifier off the first 8 bytes of the instruction
///   data and invoke the identified method. The method identifier
///   is a variant of sighash. See docs.rs for `anchor_lang` for details.
/// * If the method identifier is an IDL identifier, execute the IDL
///   instructions, which are a special set of hardcoded instructions
///   baked into every Anchor program. Then exit.
/// * Otherwise, the method identifier is for a user defined
///   instruction, i.e., one of the methods in the user defined
///   `#[program]` module. Perform method dispatch, i.e., execute the
///   big match statement mapping method identifier to method handler
///   wrapper.
/// * Run the method handler wrapper. This wraps the code the user
///   actually wrote, deserializing the accounts, constructing the
///   context, invoking the user's code, and finally running the exit
///   routine, which typically persists account changes.
///
/// The `entry` function here, defines the standard entry to a Solana
/// program, where execution begins.
pub fn entry<'info>(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    data: &[u8],
) -> anchor_lang::solana_program::entrypoint::ProgramResult {
    try_entry(program_id, accounts, data)
        .map_err(|e| {
            e.log();
            e.into()
        })
}
fn try_entry<'info>(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    data: &[u8],
) -> anchor_lang::Result<()> {
    if *program_id != ID {
        return Err(anchor_lang::error::ErrorCode::DeclaredProgramIdMismatch.into());
    }
    if data.len() < 8 {
        return Err(anchor_lang::error::ErrorCode::InstructionMissing.into());
    }
    dispatch(program_id, accounts, data)
}
/// Module representing the program.
pub mod program {
    use super::*;
    /// Type representing the program.
    pub struct AssetBased;
    #[automatically_derived]
    impl ::core::clone::Clone for AssetBased {
        #[inline]
        fn clone(&self) -> AssetBased {
            AssetBased
        }
    }
    impl anchor_lang::Id for AssetBased {
        fn id() -> Pubkey {
            ID
        }
    }
}
/// Performs method dispatch.
///
/// Each method in an anchor program is uniquely defined by a namespace
/// and a rust identifier (i.e., the name given to the method). These
/// two pieces can be combined to create a method identifier,
/// specifically, Anchor uses
///
/// Sha256("<namespace>:<rust-identifier>")[..8],
///
/// where the namespace can be one type. "global" for a
/// regular instruction.
///
/// With this 8 byte identifier, Anchor performs method dispatch,
/// matching the given 8 byte identifier to the associated method
/// handler, which leads to user defined code being eventually invoked.
fn dispatch<'info>(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    data: &[u8],
) -> anchor_lang::Result<()> {
    let mut ix_data: &[u8] = data;
    let sighash: [u8; 8] = {
        let mut sighash: [u8; 8] = [0; 8];
        sighash.copy_from_slice(&ix_data[..8]);
        ix_data = &ix_data[8..];
        sighash
    };
    use anchor_lang::Discriminator;
    match sighash {
        instruction::InitializeSubsystem::DISCRIMINATOR => {
            __private::__global::initialize_subsystem(program_id, accounts, ix_data)
        }
        instruction::AddIssuersWrapper::DISCRIMINATOR => {
            __private::__global::add_issuers_wrapper(program_id, accounts, ix_data)
        }
        instruction::RemoveIssuerWrapper::DISCRIMINATOR => {
            __private::__global::remove_issuer_wrapper(program_id, accounts, ix_data)
        }
        instruction::DepositTokenSubsystem::DISCRIMINATOR => {
            __private::__global::deposit_token_subsystem(program_id, accounts, ix_data)
        }
        instruction::WithdrawTokenSubsystem::DISCRIMINATOR => {
            __private::__global::withdraw_token_subsystem(program_id, accounts, ix_data)
        }
        instruction::InitializeId::DISCRIMINATOR => {
            __private::__global::initialize_id(program_id, accounts, ix_data)
        }
        instruction::AddIssuerToId::DISCRIMINATOR => {
            __private::__global::add_issuer_to_id(program_id, accounts, ix_data)
        }
        instruction::AddPseudo::DISCRIMINATOR => {
            __private::__global::add_pseudo(program_id, accounts, ix_data)
        }
        instruction::UpdatePseudo::DISCRIMINATOR => {
            __private::__global::update_pseudo(program_id, accounts, ix_data)
        }
        instruction::InitializeTwoAuth::DISCRIMINATOR => {
            __private::__global::initialize_two_auth(program_id, accounts, ix_data)
        }
        instruction::UpdateTwoAuth::DISCRIMINATOR => {
            __private::__global::update_two_auth(program_id, accounts, ix_data)
        }
        instruction::InitializeRecovery::DISCRIMINATOR => {
            __private::__global::initialize_recovery(program_id, accounts, ix_data)
        }
        instruction::RecoverAccount::DISCRIMINATOR => {
            __private::__global::recover_account(program_id, accounts, ix_data)
        }
        instruction::TransferPublic::DISCRIMINATOR => {
            __private::__global::transfer_public(program_id, accounts, ix_data)
        }
        anchor_lang::idl::IDL_IX_TAG_LE => {
            __private::__idl::__idl_dispatch(program_id, accounts, &ix_data)
        }
        anchor_lang::event::EVENT_IX_TAG_LE => {
            Err(anchor_lang::error::ErrorCode::EventInstructionStub.into())
        }
        _ => Err(anchor_lang::error::ErrorCode::InstructionFallbackNotFound.into()),
    }
}
/// Create a private module to not clutter the program's namespace.
/// Defines an entrypoint for each individual instruction handler
/// wrapper.
mod __private {
    use super::*;
    /// __idl mod defines handlers for injected Anchor IDL instructions.
    pub mod __idl {
        use super::*;
        #[inline(never)]
        #[cfg(not(feature = "no-idl"))]
        pub fn __idl_dispatch<'info>(
            program_id: &Pubkey,
            accounts: &'info [AccountInfo<'info>],
            idl_ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            let mut accounts = accounts;
            let mut data: &[u8] = idl_ix_data;
            let ix = anchor_lang::idl::IdlInstruction::deserialize(&mut data)
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            match ix {
                anchor_lang::idl::IdlInstruction::Create { data_len } => {
                    let mut bumps = <IdlCreateAccounts as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlCreateAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_create_account(program_id, &mut accounts, data_len)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Resize { data_len } => {
                    let mut bumps = <IdlResizeAccount as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlResizeAccount::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_resize_account(program_id, &mut accounts, data_len)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Close => {
                    let mut bumps = <IdlCloseAccount as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlCloseAccount::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_close_account(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::CreateBuffer => {
                    let mut bumps = <IdlCreateBuffer as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlCreateBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_create_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Write { data } => {
                    let mut bumps = <IdlAccounts as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_write(program_id, &mut accounts, data)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetAuthority { new_authority } => {
                    let mut bumps = <IdlAccounts as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_set_authority(program_id, &mut accounts, new_authority)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetBuffer => {
                    let mut bumps = <IdlSetBuffer as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlSetBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_set_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
            }
            Ok(())
        }
        use anchor_lang::idl::ERASED_AUTHORITY;
        pub struct IdlAccount {
            pub authority: Pubkey,
            pub data_len: u32,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlAccount {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "IdlAccount",
                    "authority",
                    &self.authority,
                    "data_len",
                    &&self.data_len,
                )
            }
        }
        impl borsh::ser::BorshSerialize for IdlAccount
        where
            Pubkey: borsh::ser::BorshSerialize,
            u32: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.authority, writer)?;
                borsh::BorshSerialize::serialize(&self.data_len, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for IdlAccount
        where
            Pubkey: borsh::BorshDeserialize,
            u32: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    authority: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    data_len: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for IdlAccount {
            #[inline]
            fn clone(&self) -> IdlAccount {
                IdlAccount {
                    authority: ::core::clone::Clone::clone(&self.authority),
                    data_len: ::core::clone::Clone::clone(&self.data_len),
                }
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountSerialize for IdlAccount {
            fn try_serialize<W: std::io::Write>(
                &self,
                writer: &mut W,
            ) -> anchor_lang::Result<()> {
                if writer.write_all(&[24, 70, 98, 191, 58, 144, 123, 158]).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                if AnchorSerialize::serialize(self, writer).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountDeserialize for IdlAccount {
            fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                if buf.len() < [24, 70, 98, 191, 58, 144, 123, 158].len() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound
                            .into(),
                    );
                }
                let given_disc = &buf[..8];
                if &[24, 70, 98, 191, 58, 144, 123, 158] != given_disc {
                    return Err(
                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .name(),
                                error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .into(),
                                error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .to_string(),
                                error_origin: Some(
                                    anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                        filename: "programs/asset_based/src/lib.rs",
                                        line: 14u32,
                                    }),
                                ),
                                compared_values: None,
                            })
                            .with_account_name("IdlAccount"),
                    );
                }
                Self::try_deserialize_unchecked(buf)
            }
            fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                let mut data: &[u8] = &buf[8..];
                AnchorDeserialize::deserialize(&mut data)
                    .map_err(|_| {
                        anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                    })
            }
        }
        #[automatically_derived]
        impl anchor_lang::Discriminator for IdlAccount {
            const DISCRIMINATOR: [u8; 8] = [24, 70, 98, 191, 58, 144, 123, 158];
        }
        impl IdlAccount {
            pub fn address(program_id: &Pubkey) -> Pubkey {
                let program_signer = Pubkey::find_program_address(&[], program_id).0;
                Pubkey::create_with_seed(&program_signer, IdlAccount::seed(), program_id)
                    .expect("Seed is always valid")
            }
            pub fn seed() -> &'static str {
                "anchor:idl"
            }
        }
        impl anchor_lang::Owner for IdlAccount {
            fn owner() -> Pubkey {
                crate::ID
            }
        }
        pub struct IdlCreateAccounts<'info> {
            #[account(signer)]
            pub from: AccountInfo<'info>,
            #[account(mut)]
            pub to: AccountInfo<'info>,
            #[account(seeds = [], bump)]
            pub base: AccountInfo<'info>,
            pub system_program: Program<'info, System>,
            #[account(executable)]
            pub program: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlCreateAccountsBumps>
        for IdlCreateAccounts<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlCreateAccountsBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let from: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("from"))?;
                let to: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("to"))?;
                let base: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("base"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                let program: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("program"))?;
                if !&from.is_signer {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSigner,
                            )
                            .with_account_name("from"),
                    );
                }
                if !&to.is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("to"),
                    );
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[],
                    &__program_id,
                );
                __bumps.base = __bump;
                if base.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("base")
                            .with_pubkeys((base.key(), __pda_address)),
                    );
                }
                if !&program.executable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintExecutable,
                            )
                            .with_account_name("program"),
                    );
                }
                Ok(IdlCreateAccounts {
                    from,
                    to,
                    base,
                    system_program,
                    program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCreateAccounts<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.from.to_account_infos());
                account_infos.extend(self.to.to_account_infos());
                account_infos.extend(self.base.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos.extend(self.program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlCreateAccounts<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.from.to_account_metas(Some(true)));
                account_metas.extend(self.to.to_account_metas(None));
                account_metas.extend(self.base.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas.extend(self.program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlCreateAccounts<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.to, program_id)
                    .map_err(|e| e.with_account_name("to"))?;
                Ok(())
            }
        }
        pub struct IdlCreateAccountsBumps {
            pub base: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlCreateAccountsBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "IdlCreateAccountsBumps",
                    "base",
                    &&self.base,
                )
            }
        }
        impl Default for IdlCreateAccountsBumps {
            fn default() -> Self {
                IdlCreateAccountsBumps {
                    base: u8::MAX,
                }
            }
        }
        impl<'info> anchor_lang::Bumps for IdlCreateAccounts<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlCreateAccountsBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_create_accounts {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlCreateAccounts`].
            pub struct IdlCreateAccounts {
                pub from: Pubkey,
                pub to: Pubkey,
                pub base: Pubkey,
                pub system_program: Pubkey,
                pub program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlCreateAccounts
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.from, writer)?;
                    borsh::BorshSerialize::serialize(&self.to, writer)?;
                    borsh::BorshSerialize::serialize(&self.base, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlCreateAccounts {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.from,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.to,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.base,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_create_accounts {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlCreateAccounts`].
            pub struct IdlCreateAccounts<'info> {
                pub from: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub to: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub base: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlCreateAccounts<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.from),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.to),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.base),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCreateAccounts<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.from),
                        );
                    account_infos
                        .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.to));
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.base),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.program),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlAccounts<'info> {
            #[account(mut, has_one = authority)]
            pub idl: Account<'info, IdlAccount>,
            #[account(constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlAccountsBumps> for IdlAccounts<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlAccountsBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let idl: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("idl"))?;
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                if !AsRef::<AccountInfo>::as_ref(&idl).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("idl"),
                    );
                }
                {
                    let my_key = idl.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                                )
                                .with_account_name("idl")
                                .with_pubkeys((my_key, target_key)),
                        );
                    }
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                Ok(IdlAccounts { idl, authority })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlAccounts<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.idl.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlAccounts<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.idl.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlAccounts<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.idl, program_id)
                    .map_err(|e| e.with_account_name("idl"))?;
                Ok(())
            }
        }
        pub struct IdlAccountsBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlAccountsBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlAccountsBumps")
            }
        }
        impl Default for IdlAccountsBumps {
            fn default() -> Self {
                IdlAccountsBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for IdlAccounts<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlAccountsBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_accounts {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlAccounts`].
            pub struct IdlAccounts {
                pub idl: Pubkey,
                pub authority: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlAccounts
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.idl, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlAccounts {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.idl,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.authority,
                                true,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_accounts {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlAccounts`].
            pub struct IdlAccounts<'info> {
                pub idl: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlAccounts<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.idl),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlAccounts<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.idl),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlResizeAccount<'info> {
            #[account(mut, has_one = authority)]
            pub idl: Account<'info, IdlAccount>,
            #[account(mut, constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlResizeAccountBumps>
        for IdlResizeAccount<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlResizeAccountBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let idl: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("idl"))?;
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                if !AsRef::<AccountInfo>::as_ref(&idl).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("idl"),
                    );
                }
                {
                    let my_key = idl.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                                )
                                .with_account_name("idl")
                                .with_pubkeys((my_key, target_key)),
                        );
                    }
                }
                if !AsRef::<AccountInfo>::as_ref(&authority).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("authority"),
                    );
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                Ok(IdlResizeAccount {
                    idl,
                    authority,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlResizeAccount<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.idl.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlResizeAccount<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.idl.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlResizeAccount<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.idl, program_id)
                    .map_err(|e| e.with_account_name("idl"))?;
                anchor_lang::AccountsExit::exit(&self.authority, program_id)
                    .map_err(|e| e.with_account_name("authority"))?;
                Ok(())
            }
        }
        pub struct IdlResizeAccountBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlResizeAccountBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlResizeAccountBumps")
            }
        }
        impl Default for IdlResizeAccountBumps {
            fn default() -> Self {
                IdlResizeAccountBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for IdlResizeAccount<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlResizeAccountBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_resize_account {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlResizeAccount`].
            pub struct IdlResizeAccount {
                pub idl: Pubkey,
                pub authority: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlResizeAccount
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.idl, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlResizeAccount {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.idl,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.authority,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_resize_account {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlResizeAccount`].
            pub struct IdlResizeAccount<'info> {
                pub idl: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlResizeAccount<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.idl),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlResizeAccount<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.idl),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlCreateBuffer<'info> {
            #[account(zero)]
            pub buffer: Account<'info, IdlAccount>,
            #[account(constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlCreateBufferBumps>
        for IdlCreateBuffer<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlCreateBufferBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                if __accounts.is_empty() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into(),
                    );
                }
                let buffer = &__accounts[0];
                *__accounts = &__accounts[1..];
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                let __anchor_rent = Rent::get()?;
                let buffer: anchor_lang::accounts::account::Account<IdlAccount> = {
                    let mut __data: &[u8] = &buffer.try_borrow_data()?;
                    let mut __disc_bytes = [0u8; 8];
                    __disc_bytes.copy_from_slice(&__data[..8]);
                    let __discriminator = u64::from_le_bytes(__disc_bytes);
                    if __discriminator != 0 {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintZero,
                                )
                                .with_account_name("buffer"),
                        );
                    }
                    match anchor_lang::accounts::account::Account::try_from_unchecked(
                        &buffer,
                    ) {
                        Ok(val) => val,
                        Err(e) => return Err(e.with_account_name("buffer")),
                    }
                };
                if !AsRef::<AccountInfo>::as_ref(&buffer).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("buffer"),
                    );
                }
                if !__anchor_rent
                    .is_exempt(
                        buffer.to_account_info().lamports(),
                        buffer.to_account_info().try_data_len()?,
                    )
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                            )
                            .with_account_name("buffer"),
                    );
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                Ok(IdlCreateBuffer {
                    buffer,
                    authority,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCreateBuffer<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.buffer.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlCreateBuffer<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.buffer.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlCreateBuffer<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.buffer, program_id)
                    .map_err(|e| e.with_account_name("buffer"))?;
                Ok(())
            }
        }
        pub struct IdlCreateBufferBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlCreateBufferBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlCreateBufferBumps")
            }
        }
        impl Default for IdlCreateBufferBumps {
            fn default() -> Self {
                IdlCreateBufferBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for IdlCreateBuffer<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlCreateBufferBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_create_buffer {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlCreateBuffer`].
            pub struct IdlCreateBuffer {
                pub buffer: Pubkey,
                pub authority: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlCreateBuffer
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.buffer, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlCreateBuffer {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.buffer,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.authority,
                                true,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_create_buffer {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlCreateBuffer`].
            pub struct IdlCreateBuffer<'info> {
                pub buffer: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlCreateBuffer<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.buffer),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCreateBuffer<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.buffer),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlSetBuffer<'info> {
            #[account(mut, constraint = buffer.authority = = idl.authority)]
            pub buffer: Account<'info, IdlAccount>,
            #[account(mut, has_one = authority)]
            pub idl: Account<'info, IdlAccount>,
            #[account(constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlSetBufferBumps>
        for IdlSetBuffer<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlSetBufferBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let buffer: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("buffer"))?;
                let idl: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("idl"))?;
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                if !AsRef::<AccountInfo>::as_ref(&buffer).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("buffer"),
                    );
                }
                if !(buffer.authority == idl.authority) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("buffer"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&idl).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("idl"),
                    );
                }
                {
                    let my_key = idl.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                                )
                                .with_account_name("idl")
                                .with_pubkeys((my_key, target_key)),
                        );
                    }
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                Ok(IdlSetBuffer {
                    buffer,
                    idl,
                    authority,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlSetBuffer<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.buffer.to_account_infos());
                account_infos.extend(self.idl.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlSetBuffer<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.buffer.to_account_metas(None));
                account_metas.extend(self.idl.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlSetBuffer<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.buffer, program_id)
                    .map_err(|e| e.with_account_name("buffer"))?;
                anchor_lang::AccountsExit::exit(&self.idl, program_id)
                    .map_err(|e| e.with_account_name("idl"))?;
                Ok(())
            }
        }
        pub struct IdlSetBufferBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlSetBufferBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlSetBufferBumps")
            }
        }
        impl Default for IdlSetBufferBumps {
            fn default() -> Self {
                IdlSetBufferBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for IdlSetBuffer<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlSetBufferBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_set_buffer {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlSetBuffer`].
            pub struct IdlSetBuffer {
                pub buffer: Pubkey,
                pub idl: Pubkey,
                pub authority: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlSetBuffer
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.buffer, writer)?;
                    borsh::BorshSerialize::serialize(&self.idl, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlSetBuffer {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.buffer,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.idl,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.authority,
                                true,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_set_buffer {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlSetBuffer`].
            pub struct IdlSetBuffer<'info> {
                pub buffer: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub idl: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlSetBuffer<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.buffer),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.idl),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlSetBuffer<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.buffer),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.idl),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlCloseAccount<'info> {
            #[account(mut, has_one = authority, close = sol_destination)]
            pub account: Account<'info, IdlAccount>,
            #[account(constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
            #[account(mut)]
            pub sol_destination: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlCloseAccountBumps>
        for IdlCloseAccount<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlCloseAccountBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let account: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("account"))?;
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                let sol_destination: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("sol_destination"))?;
                if !AsRef::<AccountInfo>::as_ref(&account).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("account"),
                    );
                }
                {
                    let my_key = account.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                                )
                                .with_account_name("account")
                                .with_pubkeys((my_key, target_key)),
                        );
                    }
                }
                {
                    if account.key() == sol_destination.key() {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintClose,
                                )
                                .with_account_name("account"),
                        );
                    }
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                if !&sol_destination.is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("sol_destination"),
                    );
                }
                Ok(IdlCloseAccount {
                    account,
                    authority,
                    sol_destination,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCloseAccount<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.account.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos.extend(self.sol_destination.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlCloseAccount<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.account.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas.extend(self.sol_destination.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlCloseAccount<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                {
                    let sol_destination = &self.sol_destination;
                    anchor_lang::AccountsClose::close(
                            &self.account,
                            sol_destination.to_account_info(),
                        )
                        .map_err(|e| e.with_account_name("account"))?;
                }
                anchor_lang::AccountsExit::exit(&self.sol_destination, program_id)
                    .map_err(|e| e.with_account_name("sol_destination"))?;
                Ok(())
            }
        }
        pub struct IdlCloseAccountBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlCloseAccountBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlCloseAccountBumps")
            }
        }
        impl Default for IdlCloseAccountBumps {
            fn default() -> Self {
                IdlCloseAccountBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for IdlCloseAccount<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlCloseAccountBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_close_account {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlCloseAccount`].
            pub struct IdlCloseAccount {
                pub account: Pubkey,
                pub authority: Pubkey,
                pub sol_destination: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlCloseAccount
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.account, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    borsh::BorshSerialize::serialize(&self.sol_destination, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlCloseAccount {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.authority,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.sol_destination,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_close_account {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlCloseAccount`].
            pub struct IdlCloseAccount<'info> {
                pub account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub sol_destination: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlCloseAccount<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.sol_destination),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCloseAccount<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.account),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.sol_destination,
                            ),
                        );
                    account_infos
                }
            }
        }
        use std::cell::{Ref, RefMut};
        pub trait IdlTrailingData<'info> {
            fn trailing_data(self) -> Ref<'info, [u8]>;
            fn trailing_data_mut(self) -> RefMut<'info, [u8]>;
        }
        impl<'a, 'info: 'a> IdlTrailingData<'a> for &'a Account<'info, IdlAccount> {
            fn trailing_data(self) -> Ref<'a, [u8]> {
                let info: &AccountInfo<'info> = self.as_ref();
                Ref::map(info.try_borrow_data().unwrap(), |d| &d[44..])
            }
            fn trailing_data_mut(self) -> RefMut<'a, [u8]> {
                let info: &AccountInfo<'info> = self.as_ref();
                RefMut::map(info.try_borrow_mut_data().unwrap(), |d| &mut d[44..])
            }
        }
        #[inline(never)]
        pub fn __idl_create_account(
            program_id: &Pubkey,
            accounts: &mut IdlCreateAccounts,
            data_len: u64,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCreateAccount");
            if program_id != accounts.program.key {
                return Err(
                    anchor_lang::error::ErrorCode::IdlInstructionInvalidProgram.into(),
                );
            }
            let from = accounts.from.key;
            let (base, nonce) = Pubkey::find_program_address(&[], program_id);
            let seed = IdlAccount::seed();
            let owner = accounts.program.key;
            let to = Pubkey::create_with_seed(&base, seed, owner).unwrap();
            let space = std::cmp::min(8 + 32 + 4 + data_len as usize, 10_000);
            let rent = Rent::get()?;
            let lamports = rent.minimum_balance(space);
            let seeds = &[&[nonce][..]];
            let ix = anchor_lang::solana_program::system_instruction::create_account_with_seed(
                from,
                &to,
                &base,
                seed,
                lamports,
                space as u64,
                owner,
            );
            anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &[
                    accounts.from.clone(),
                    accounts.to.clone(),
                    accounts.base.clone(),
                    accounts.system_program.to_account_info(),
                ],
                &[seeds],
            )?;
            let mut idl_account = {
                let mut account_data = accounts.to.try_borrow_data()?;
                let mut account_data_slice: &[u8] = &account_data;
                IdlAccount::try_deserialize_unchecked(&mut account_data_slice)?
            };
            idl_account.authority = *accounts.from.key;
            let mut data = accounts.to.try_borrow_mut_data()?;
            let dst: &mut [u8] = &mut data;
            let mut cursor = std::io::Cursor::new(dst);
            idl_account.try_serialize(&mut cursor)?;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_resize_account(
            program_id: &Pubkey,
            accounts: &mut IdlResizeAccount,
            data_len: u64,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlResizeAccount");
            let data_len: usize = data_len as usize;
            if accounts.idl.data_len != 0 {
                return Err(anchor_lang::error::ErrorCode::IdlAccountNotEmpty.into());
            }
            let idl_ref = AsRef::<AccountInfo>::as_ref(&accounts.idl);
            let new_account_space = idl_ref
                .data_len()
                .checked_add(
                    std::cmp::min(
                        data_len
                            .checked_sub(idl_ref.data_len())
                            .expect(
                                "data_len should always be >= the current account space",
                            ),
                        10_000,
                    ),
                )
                .unwrap();
            if new_account_space > idl_ref.data_len() {
                let sysvar_rent = Rent::get()?;
                let new_rent_minimum = sysvar_rent.minimum_balance(new_account_space);
                anchor_lang::system_program::transfer(
                    anchor_lang::context::CpiContext::new(
                        accounts.system_program.to_account_info(),
                        anchor_lang::system_program::Transfer {
                            from: accounts.authority.to_account_info(),
                            to: accounts.idl.to_account_info(),
                        },
                    ),
                    new_rent_minimum.checked_sub(idl_ref.lamports()).unwrap(),
                )?;
                idl_ref.realloc(new_account_space, false)?;
            }
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_close_account(
            program_id: &Pubkey,
            accounts: &mut IdlCloseAccount,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCloseAccount");
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_buffer(
            program_id: &Pubkey,
            accounts: &mut IdlCreateBuffer,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCreateBuffer");
            let mut buffer = &mut accounts.buffer;
            buffer.authority = *accounts.authority.key;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_write(
            program_id: &Pubkey,
            accounts: &mut IdlAccounts,
            idl_data: Vec<u8>,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlWrite");
            let prev_len: usize = ::std::convert::TryInto::<
                usize,
            >::try_into(accounts.idl.data_len)
                .unwrap();
            let new_len: usize = prev_len.checked_add(idl_data.len()).unwrap() as usize;
            accounts.idl.data_len = accounts
                .idl
                .data_len
                .checked_add(
                    ::std::convert::TryInto::<u32>::try_into(idl_data.len()).unwrap(),
                )
                .unwrap();
            use IdlTrailingData;
            let mut idl_bytes = accounts.idl.trailing_data_mut();
            let idl_expansion = &mut idl_bytes[prev_len..new_len];
            if idl_expansion.len() != idl_data.len() {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: anchor_lang::error::ErrorCode::RequireEqViolated
                                .name(),
                            error_code_number: anchor_lang::error::ErrorCode::RequireEqViolated
                                .into(),
                            error_msg: anchor_lang::error::ErrorCode::RequireEqViolated
                                .to_string(),
                            error_origin: Some(
                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                    filename: "programs/asset_based/src/lib.rs",
                                    line: 14u32,
                                }),
                            ),
                            compared_values: None,
                        })
                        .with_values((idl_expansion.len(), idl_data.len())),
                );
            }
            idl_expansion.copy_from_slice(&idl_data[..]);
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_authority(
            program_id: &Pubkey,
            accounts: &mut IdlAccounts,
            new_authority: Pubkey,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlSetAuthority");
            accounts.idl.authority = new_authority;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_buffer(
            program_id: &Pubkey,
            accounts: &mut IdlSetBuffer,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlSetBuffer");
            accounts.idl.data_len = accounts.buffer.data_len;
            use IdlTrailingData;
            let buffer_len = ::std::convert::TryInto::<
                usize,
            >::try_into(accounts.buffer.data_len)
                .unwrap();
            let mut target = accounts.idl.trailing_data_mut();
            let source = &accounts.buffer.trailing_data()[..buffer_len];
            if target.len() < buffer_len {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: anchor_lang::error::ErrorCode::RequireGteViolated
                                .name(),
                            error_code_number: anchor_lang::error::ErrorCode::RequireGteViolated
                                .into(),
                            error_msg: anchor_lang::error::ErrorCode::RequireGteViolated
                                .to_string(),
                            error_origin: Some(
                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                    filename: "programs/asset_based/src/lib.rs",
                                    line: 14u32,
                                }),
                            ),
                            compared_values: None,
                        })
                        .with_values((target.len(), buffer_len)),
                );
            }
            target[..buffer_len].copy_from_slice(source);
            Ok(())
        }
    }
    /// __global mod defines wrapped handlers for global instructions.
    pub mod __global {
        use super::*;
        #[inline(never)]
        pub fn initialize_subsystem<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: InitializeSubsystem");
            let ix = instruction::InitializeSubsystem::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::InitializeSubsystem { list_issuer, exit_regulators } = ix;
            let mut __bumps = <InitializeSubsystem as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = InitializeSubsystem::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = asset_based::initialize_subsystem(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                list_issuer,
                exit_regulators,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn add_issuers_wrapper<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: AddIssuersWrapper");
            let ix = instruction::AddIssuersWrapper::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::AddIssuersWrapper { issuer } = ix;
            let mut __bumps = <AddSubsystemIssuer as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = AddSubsystemIssuer::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = asset_based::add_issuers_wrapper(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                issuer,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn remove_issuer_wrapper<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: RemoveIssuerWrapper");
            let ix = instruction::RemoveIssuerWrapper::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::RemoveIssuerWrapper = ix;
            let mut __bumps = <DeleteSubsystemIssuer as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = DeleteSubsystemIssuer::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = asset_based::remove_issuer_wrapper(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn deposit_token_subsystem<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: DepositTokenSubsystem");
            let ix = instruction::DepositTokenSubsystem::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::DepositTokenSubsystem { amount, decimals } = ix;
            let mut __bumps = <DepositTokensSubsystem as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = DepositTokensSubsystem::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = asset_based::deposit_token_subsystem(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                amount,
                decimals,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn withdraw_token_subsystem<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: WithdrawTokenSubsystem");
            let ix = instruction::WithdrawTokenSubsystem::deserialize(
                    &mut &__ix_data[..],
                )
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::WithdrawTokenSubsystem { amount, decimals } = ix;
            let mut __bumps = <WithdrawTokensSubsystem as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = WithdrawTokensSubsystem::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = asset_based::withdraw_token_subsystem(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                amount,
                decimals,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn initialize_id<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: InitializeId");
            let ix = instruction::InitializeId::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::InitializeId { id_validity_duration } = ix;
            let mut __bumps = <InitializeId as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = InitializeId::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = asset_based::initialize_id(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                id_validity_duration,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn add_issuer_to_id<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: AddIssuerToId");
            let ix = instruction::AddIssuerToId::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::AddIssuerToId { id_validity_duration } = ix;
            let mut __bumps = <AddIssuer as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = AddIssuer::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = asset_based::add_issuer_to_id(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                id_validity_duration,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn add_pseudo<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: AddPseudo");
            let ix = instruction::AddPseudo::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::AddPseudo { _pseudo } = ix;
            let mut __bumps = <AddPseudo as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = AddPseudo::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = asset_based::add_pseudo(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                _pseudo,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn update_pseudo<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: UpdatePseudo");
            let ix = instruction::UpdatePseudo::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::UpdatePseudo { _pseudo } = ix;
            let mut __bumps = <UpdatePseudo as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = UpdatePseudo::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = asset_based::update_pseudo(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                _pseudo,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn initialize_two_auth<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: InitializeTwoAuth");
            let ix = instruction::InitializeTwoAuth::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::InitializeTwoAuth { two_auth } = ix;
            let mut __bumps = <InitTwoAuth as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = InitTwoAuth::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = asset_based::initialize_two_auth(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                two_auth,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn update_two_auth<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: UpdateTwoAuth");
            let ix = instruction::UpdateTwoAuth::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::UpdateTwoAuth { two_auth } = ix;
            let mut __bumps = <UpdateTwoAuth as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = UpdateTwoAuth::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = asset_based::update_two_auth(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                two_auth,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn initialize_recovery<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: InitializeRecovery");
            let ix = instruction::InitializeRecovery::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::InitializeRecovery { recovery_delegates } = ix;
            let mut __bumps = <InitializeRecovery as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = InitializeRecovery::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = asset_based::initialize_recovery(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                recovery_delegates,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn recover_account<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: RecoverAccount");
            let ix = instruction::RecoverAccount::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::RecoverAccount = ix;
            let mut __bumps = <RecoverAccount as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = RecoverAccount::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = asset_based::recover_account(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn transfer_public<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: TransferPublic");
            let ix = instruction::TransferPublic::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::TransferPublic { amount, decimals } = ix;
            let mut __bumps = <Transfer as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = Transfer::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = asset_based::transfer_public(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                amount,
                decimals,
            )?;
            __accounts.exit(__program_id)
        }
    }
}
pub mod asset_based {
    use super::*;
    pub fn initialize_subsystem(
        ctx: Context<InitializeSubsystem>,
        list_issuer: Vec<Pubkey>,
        exit_regulators: Vec<Pubkey>,
    ) -> Result<()> {
        subsystem::_initialize_subsystem(ctx, list_issuer, exit_regulators)
    }
    pub fn add_issuers_wrapper(
        ctx: Context<AddSubsystemIssuer>,
        issuer: Pubkey,
    ) -> Result<()> {
        subsystem::_add_issuers_subsystem(ctx, issuer)
    }
    pub fn remove_issuer_wrapper(ctx: Context<DeleteSubsystemIssuer>) -> Result<()> {
        subsystem::_remove_issuer_subsystem(ctx)
    }
    pub fn deposit_token_subsystem(
        ctx: Context<DepositTokensSubsystem>,
        amount: u64,
        decimals: u8,
    ) -> Result<()> {
        subsystem::_deposit_token_subsystem(ctx, amount, decimals)
    }
    pub fn withdraw_token_subsystem(
        ctx: Context<WithdrawTokensSubsystem>,
        amount: u64,
        decimals: u8,
    ) -> Result<()> {
        subsystem::_withdraw_token_subsystem(ctx, amount, decimals)
    }
    pub fn initialize_id(
        ctx: Context<InitializeId>,
        id_validity_duration: i64,
    ) -> Result<()> {
        idendity::_initialize_id(ctx, id_validity_duration)
    }
    pub fn add_issuer_to_id(
        ctx: Context<AddIssuer>,
        id_validity_duration: i64,
    ) -> Result<()> {
        idendity::_add_issuer_to_id(ctx, id_validity_duration)
    }
    pub fn add_pseudo(ctx: Context<AddPseudo>, _pseudo: String) -> Result<()> {
        pseudo::_add_pseudo(ctx, _pseudo)
    }
    pub fn update_pseudo(ctx: Context<UpdatePseudo>, _pseudo: String) -> Result<()> {
        pseudo::_update_pseudo(ctx, _pseudo)
    }
    pub fn initialize_two_auth(
        ctx: Context<InitTwoAuth>,
        two_auth: Option<TwoAuthArgs>,
    ) -> Result<()> {
        two_auth::_initialize_two_auth(ctx, two_auth)
    }
    pub fn update_two_auth(
        ctx: Context<UpdateTwoAuth>,
        two_auth: Option<TwoAuthArgs>,
    ) -> Result<()> {
        two_auth::_update_two_auth(ctx, two_auth)
    }
    pub fn initialize_recovery(
        ctx: Context<InitializeRecovery>,
        recovery_delegates: Vec<RecoveryAuthority>,
    ) -> Result<()> {
        recovery::_initialize_recovery(ctx, recovery_delegates)
    }
    pub fn recover_account(ctx: Context<RecoverAccount>) -> Result<()> {
        recovery::_recover_account(ctx)
    }
    pub fn transfer_public(
        ctx: Context<Transfer>,
        amount: u64,
        decimals: u8,
    ) -> Result<()> {
        transfer::_transfer_public(ctx, amount, decimals)
    }
}
/// An Anchor generated module containing the program's set of
/// instructions, where each method handler in the `#[program]` mod is
/// associated with a struct defining the input arguments to the
/// method. These should be used directly, when one wants to serialize
/// Anchor instruction data, for example, when speciying
/// instructions on a client.
pub mod instruction {
    use super::*;
    /// Instruction.
    pub struct InitializeSubsystem {
        pub list_issuer: Vec<Pubkey>,
        pub exit_regulators: Vec<Pubkey>,
    }
    impl borsh::ser::BorshSerialize for InitializeSubsystem
    where
        Vec<Pubkey>: borsh::ser::BorshSerialize,
        Vec<Pubkey>: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.list_issuer, writer)?;
            borsh::BorshSerialize::serialize(&self.exit_regulators, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for InitializeSubsystem
    where
        Vec<Pubkey>: borsh::BorshDeserialize,
        Vec<Pubkey>: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                list_issuer: borsh::BorshDeserialize::deserialize_reader(reader)?,
                exit_regulators: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for InitializeSubsystem {
        const DISCRIMINATOR: [u8; 8] = [29, 106, 70, 233, 182, 177, 72, 243];
    }
    impl anchor_lang::InstructionData for InitializeSubsystem {}
    impl anchor_lang::Owner for InitializeSubsystem {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct AddIssuersWrapper {
        pub issuer: Pubkey,
    }
    impl borsh::ser::BorshSerialize for AddIssuersWrapper
    where
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.issuer, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for AddIssuersWrapper
    where
        Pubkey: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                issuer: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for AddIssuersWrapper {
        const DISCRIMINATOR: [u8; 8] = [90, 150, 120, 185, 80, 51, 41, 200];
    }
    impl anchor_lang::InstructionData for AddIssuersWrapper {}
    impl anchor_lang::Owner for AddIssuersWrapper {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct RemoveIssuerWrapper;
    impl borsh::ser::BorshSerialize for RemoveIssuerWrapper {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for RemoveIssuerWrapper {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::Discriminator for RemoveIssuerWrapper {
        const DISCRIMINATOR: [u8; 8] = [29, 234, 10, 98, 23, 35, 22, 183];
    }
    impl anchor_lang::InstructionData for RemoveIssuerWrapper {}
    impl anchor_lang::Owner for RemoveIssuerWrapper {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct DepositTokenSubsystem {
        pub amount: u64,
        pub decimals: u8,
    }
    impl borsh::ser::BorshSerialize for DepositTokenSubsystem
    where
        u64: borsh::ser::BorshSerialize,
        u8: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.amount, writer)?;
            borsh::BorshSerialize::serialize(&self.decimals, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for DepositTokenSubsystem
    where
        u64: borsh::BorshDeserialize,
        u8: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                decimals: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for DepositTokenSubsystem {
        const DISCRIMINATOR: [u8; 8] = [170, 121, 151, 13, 240, 190, 238, 3];
    }
    impl anchor_lang::InstructionData for DepositTokenSubsystem {}
    impl anchor_lang::Owner for DepositTokenSubsystem {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct WithdrawTokenSubsystem {
        pub amount: u64,
        pub decimals: u8,
    }
    impl borsh::ser::BorshSerialize for WithdrawTokenSubsystem
    where
        u64: borsh::ser::BorshSerialize,
        u8: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.amount, writer)?;
            borsh::BorshSerialize::serialize(&self.decimals, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for WithdrawTokenSubsystem
    where
        u64: borsh::BorshDeserialize,
        u8: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                decimals: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for WithdrawTokenSubsystem {
        const DISCRIMINATOR: [u8; 8] = [10, 52, 24, 46, 67, 143, 10, 43];
    }
    impl anchor_lang::InstructionData for WithdrawTokenSubsystem {}
    impl anchor_lang::Owner for WithdrawTokenSubsystem {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct InitializeId {
        pub id_validity_duration: i64,
    }
    impl borsh::ser::BorshSerialize for InitializeId
    where
        i64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.id_validity_duration, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for InitializeId
    where
        i64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                id_validity_duration: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
            })
        }
    }
    impl anchor_lang::Discriminator for InitializeId {
        const DISCRIMINATOR: [u8; 8] = [63, 161, 143, 94, 240, 8, 194, 92];
    }
    impl anchor_lang::InstructionData for InitializeId {}
    impl anchor_lang::Owner for InitializeId {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct AddIssuerToId {
        pub id_validity_duration: i64,
    }
    impl borsh::ser::BorshSerialize for AddIssuerToId
    where
        i64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.id_validity_duration, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for AddIssuerToId
    where
        i64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                id_validity_duration: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
            })
        }
    }
    impl anchor_lang::Discriminator for AddIssuerToId {
        const DISCRIMINATOR: [u8; 8] = [213, 147, 148, 84, 40, 47, 21, 188];
    }
    impl anchor_lang::InstructionData for AddIssuerToId {}
    impl anchor_lang::Owner for AddIssuerToId {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct AddPseudo {
        pub _pseudo: String,
    }
    impl borsh::ser::BorshSerialize for AddPseudo
    where
        String: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self._pseudo, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for AddPseudo
    where
        String: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                _pseudo: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for AddPseudo {
        const DISCRIMINATOR: [u8; 8] = [9, 252, 66, 94, 22, 207, 241, 108];
    }
    impl anchor_lang::InstructionData for AddPseudo {}
    impl anchor_lang::Owner for AddPseudo {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct UpdatePseudo {
        pub _pseudo: String,
    }
    impl borsh::ser::BorshSerialize for UpdatePseudo
    where
        String: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self._pseudo, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for UpdatePseudo
    where
        String: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                _pseudo: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for UpdatePseudo {
        const DISCRIMINATOR: [u8; 8] = [123, 25, 227, 244, 182, 144, 60, 124];
    }
    impl anchor_lang::InstructionData for UpdatePseudo {}
    impl anchor_lang::Owner for UpdatePseudo {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct InitializeTwoAuth {
        pub two_auth: Option<TwoAuthArgs>,
    }
    impl borsh::ser::BorshSerialize for InitializeTwoAuth
    where
        Option<TwoAuthArgs>: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.two_auth, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for InitializeTwoAuth
    where
        Option<TwoAuthArgs>: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                two_auth: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for InitializeTwoAuth {
        const DISCRIMINATOR: [u8; 8] = [240, 201, 133, 234, 112, 48, 218, 210];
    }
    impl anchor_lang::InstructionData for InitializeTwoAuth {}
    impl anchor_lang::Owner for InitializeTwoAuth {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct UpdateTwoAuth {
        pub two_auth: Option<TwoAuthArgs>,
    }
    impl borsh::ser::BorshSerialize for UpdateTwoAuth
    where
        Option<TwoAuthArgs>: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.two_auth, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for UpdateTwoAuth
    where
        Option<TwoAuthArgs>: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                two_auth: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for UpdateTwoAuth {
        const DISCRIMINATOR: [u8; 8] = [222, 98, 57, 40, 232, 148, 90, 198];
    }
    impl anchor_lang::InstructionData for UpdateTwoAuth {}
    impl anchor_lang::Owner for UpdateTwoAuth {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct InitializeRecovery {
        pub recovery_delegates: Vec<RecoveryAuthority>,
    }
    impl borsh::ser::BorshSerialize for InitializeRecovery
    where
        Vec<RecoveryAuthority>: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.recovery_delegates, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for InitializeRecovery
    where
        Vec<RecoveryAuthority>: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                recovery_delegates: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for InitializeRecovery {
        const DISCRIMINATOR: [u8; 8] = [219, 233, 33, 218, 155, 25, 55, 18];
    }
    impl anchor_lang::InstructionData for InitializeRecovery {}
    impl anchor_lang::Owner for InitializeRecovery {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct RecoverAccount;
    impl borsh::ser::BorshSerialize for RecoverAccount {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for RecoverAccount {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::Discriminator for RecoverAccount {
        const DISCRIMINATOR: [u8; 8] = [240, 223, 246, 118, 26, 121, 34, 128];
    }
    impl anchor_lang::InstructionData for RecoverAccount {}
    impl anchor_lang::Owner for RecoverAccount {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct TransferPublic {
        pub amount: u64,
        pub decimals: u8,
    }
    impl borsh::ser::BorshSerialize for TransferPublic
    where
        u64: borsh::ser::BorshSerialize,
        u8: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.amount, writer)?;
            borsh::BorshSerialize::serialize(&self.decimals, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for TransferPublic
    where
        u64: borsh::BorshDeserialize,
        u8: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                decimals: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for TransferPublic {
        const DISCRIMINATOR: [u8; 8] = [75, 0, 8, 78, 143, 28, 131, 250];
    }
    impl anchor_lang::InstructionData for TransferPublic {}
    impl anchor_lang::Owner for TransferPublic {
        fn owner() -> Pubkey {
            ID
        }
    }
}
/// An Anchor generated module, providing a set of structs
/// mirroring the structs deriving `Accounts`, where each field is
/// a `Pubkey`. This is useful for specifying accounts for a client.
pub mod accounts {
    pub use crate::__client_accounts_recover_account::*;
    pub use crate::__client_accounts_transfer::*;
    pub use crate::__client_accounts_add_subsystem_issuer::*;
    pub use crate::__client_accounts_initialize_id::*;
    pub use crate::__client_accounts_add_pseudo::*;
    pub use crate::__client_accounts_deposit_tokens_subsystem::*;
    pub use crate::__client_accounts_initialize_recovery::*;
    pub use crate::__client_accounts_initialize_subsystem::*;
    pub use crate::__client_accounts_update_pseudo::*;
    pub use crate::__client_accounts_add_issuer::*;
    pub use crate::__client_accounts_withdraw_tokens_subsystem::*;
    pub use crate::__client_accounts_delete_subsystem_issuer::*;
    pub use crate::__client_accounts_init_two_auth::*;
    pub use crate::__client_accounts_update_two_auth::*;
}
