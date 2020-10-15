

use super::{WalletTypes, WalletTypeFactory};

/*
    Wallet is used to store records for an agent.
*/
pub trait WalletTrait {
    fn create(&self);
    fn open(&self);
    fn close(&self);
}

impl WalletTypeFactory {
    pub fn get_wallet_handler(wallet_type: &str) -> WalletTypes {
        match wallet_type.to_lowercase().as_str() {
            "indy" => WalletTypes::Indy(),
            _ => WalletTypes::Basic()
        }
    }
}