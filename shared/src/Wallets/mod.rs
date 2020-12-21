mod BasicWalletImpl;
mod Records;

use BasicWalletImpl::BasicWallet;
use Records::*;

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

    // save a connection invitation record, for now expecting
    // implementation to understand when its a new record or an update
    fn save_invitation(&self, record: &ConnectionRecord);
}

impl WalletTrait for WalletTypes
{
    fn create(&self) {
        unimplemented!()
    }

    fn open(&self) {
        unimplemented!()
    }

    fn close(&self) {
        unimplemented!()
    }

    fn delete(&self) {
        unimplemented!()
    }

    fn save_invitation(&self, record: &ConnectionRecord) {
        debug!("WalletTypes.save_invitation");
        match *self {
            WalletTypes::Basic(ref handler) => handler.save_invitation(record),
            WalletTypes::Indy() => unimplemented!()
        }
    }
}

pub fn get_wallet_handler(wallet_type: &str, wallet_config: &str) -> WalletTypes {
    match wallet_type.to_lowercase().as_str() {
        "indy" => WalletTypes::Indy(),
        _ => WalletTypes::Basic(BasicWallet::new(wallet_config))
    }
}