mod BasicWalletImpl;
mod Records;

use BasicWalletImpl::BasicWallet;

#[derive(Debug, Clone)]
pub enum WalletTypes {
    // file base storage
    Basic(BasicWallet),
    // using indysdk wallets
    Indy()
}

/*
    Wallet is used to store records for an agent.
    TBD: this interface is being defined as the use cases arise
*/
pub trait WalletTrait {
    fn create(&self);
    fn open(&self);
    fn close(&self);
    fn delete(&self);
}

pub fn get_wallet_handler(wallet_type: &str, wallet_config: &str) -> WalletTypes {
    match wallet_type.to_lowercase().as_str() {
        "indy" => WalletTypes::Indy(),
        _ => WalletTypes::Basic(BasicWallet::new(wallet_config))
    }
}