mod BasicWalletImpl;
mod IndyWalletImpl;
pub mod Records;

use BasicWalletImpl::BasicWallet;
use IndyWalletImpl::IndyWallet;
use Records::*;

#[derive(Debug, Clone)]
pub enum WalletTypes {
    // file base storage
    Basic(BasicWallet),
    // using indysdk wallets
    Indy(IndyWallet)
}

/*
    Wallet is used to store records for an agent.
    TBD: this interface is being defined as the use cases arise
*/
pub trait WalletTrait {
    fn open(&mut self);
    fn close(&self);
    fn delete(&self);

    // save a connection invitation record, for now expecting
    // implementation to understand when its a new record or an update
    fn save_invitation(&self, record: &ConnectionRecord);
}

impl WalletTrait for WalletTypes
{
    fn open(&mut self) {
        debug!("WalletTypes.open");
        match *self {
            WalletTypes::Basic(ref mut handler) => handler.open(),
            WalletTypes::Indy(ref mut handler) => handler.open(),
        }
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
            WalletTypes::Indy(ref handler) => handler.save_invitation(record),
        }
    }
}

pub fn get_wallet_handler(wallet_type: &str, wallet_config: &str) -> WalletTypes {
    match wallet_type.to_lowercase().as_str() {
        "indy" => WalletTypes::Indy(IndyWallet::new(wallet_config)),
        _ => WalletTypes::Basic(BasicWallet::new(wallet_config))
    }
}