

use super::{
    BasicWalletImpl::BasicWallet,
    WalletTypes,
    WalletTypeFactory};

impl WalletTypeFactory {
    pub fn get_wallet_handler(wallet_type: &str, wallet_config: &str) -> WalletTypes {
        match wallet_type.to_lowercase().as_str() {
            "indy" => WalletTypes::Indy(),
            _ => WalletTypes::Basic(BasicWallet::new(wallet_config))
        }
    }
}