use anchor_lang::prelude::*;

#[account]
pub struct PseudoAccount { // 8 + 2 + 32 = 42
    pub initialized: bool, // 1
    pub bump : u8, // 1
    pub owner : Pubkey // 32
} 

impl PseudoAccount{
    pub const LEN : usize = 8 + 2 + 32 ;
}