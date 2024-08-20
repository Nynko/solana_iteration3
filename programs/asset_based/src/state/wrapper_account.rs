use anchor_lang::prelude::*;

#[account]
pub struct WrapperAccount {
    pub approver: Pubkey,
    pub id_issuers: Vec<Pubkey>,
    pub exit_regulators: Vec<Pubkey>,
    pub bump: u8
}

impl WrapperAccount {
    pub fn get_init_len(is_issuers: &Vec<Pubkey>, exit_regulators: &Vec<Pubkey>) -> usize {
        return 8 + 32 + 4 + 32 * is_issuers.len() + 4 + 32 * exit_regulators.len() + 1;
    }

    pub fn get_len_add_address(&self) -> usize {
        return Self::get_init_len(&self.id_issuers,&self.exit_regulators) + 32;
    }

    pub fn get_len_remove_address(&self) -> usize {
        return Self::get_init_len(&self.id_issuers,&self.exit_regulators) - 32;
    }
}
