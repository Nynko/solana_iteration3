use anchor_lang::prelude::*;

#[error_code]
pub enum WrapperError {
    #[msg("Decimal provided does not match the mint's decimal value")]
    InvalidDecimals,
    #[msg("Invalid exit regulator for the specified wrapper")]
    InvalidExitRegulator
}

#[error_code]
pub enum TransferError {
    #[msg("The source account does not have enough funds to transfer")]
    InsufficientFunds,
    #[msg("Decimal provided does not match the mint's decimal value")]
    InvalidDecimals,
    #[msg("Overflow when adding the amount to the destination account")]
    Overflow,
}

#[error_code]
pub enum IdendityError {
    #[msg("Idendity already exists")]
    IdendityAlreadyExists,
    #[msg("Idendity is not active")]
    IdendityNotActive,
    #[msg("Idendity expired")]
    IdendityExpired,
    #[msg("Idendity recovered")]
    IdendityRecovered,
    #[msg("Idendity already recovered")]
    IdendityAlreadyRecovered,
    #[msg("Issuer is not approved")]
    IssuerNotApproved,
    #[msg("No approved issuer found or inactive/expired issuer")]
    InvalidIdendity,
    #[msg("A pseudo already exist, please use the update method")]
    PseudoAlreadyExist,
    #[msg("The old pseudo doesn't exist, please use the add method")]
    PseudoDontExist
}



#[error_code]
pub enum TwoAuthError {
    #[msg("Not authorized to approve this transaction")]
    NotAuthorized,
    #[msg("Need the two auth entity approval")]
    NeedTwoAuthApproval,
    #[msg("The provided two_auth entity is wrong")]
    WrongApproval,
    #[msg("The Approval has expired")]
    ExpiredApproval,
}

#[error_code]
pub enum RecoveryError {
    #[msg("Recovery time not passed")]
    RecoveryTimeNotPassed,
    #[msg("Not enough signatures")]
    NotEnoughSignatures,
    #[msg("The main recovery authority is not in the list of recoverable authority")]
    WrongMainRecoveryAuthority
}