use anchor_lang::prelude::*;

#[account]
pub struct PseudoAccount { // 8 + 2 + 32 = 42
    pub initialized: bool, // 1
    pub bump : u8, // 1
    pub owner : Pubkey, // 32
    pub pseudo : String // 4 + len(pseudo)
} 

impl PseudoAccount{
    pub fn get_init_len(pseudo : &String) -> usize {
        return 8 + 1 + 1 + 32 + 4 + pseudo.len();
    }
}