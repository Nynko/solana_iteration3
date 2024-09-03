use anchor_lang::prelude::*;

#[account]
pub struct SharedAccount { // 8 + 2 + 32 = 42
    pub owners : Vec<Pubkey>, // 4 + 32 * owners.len()
} 

impl SharedAccount{
    pub fn get_init_len(owners : &Vec<Pubkey>) -> usize {
        return 8 + 4 + owners.len() * 32;
    }
}