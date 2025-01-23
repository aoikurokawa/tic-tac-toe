use anchor_lang::prelude::Pubkey;

pub struct Config {
    /// The Restaking program's NCN admin is the signer to create and update this account
    pub ncn: Pubkey,
}
