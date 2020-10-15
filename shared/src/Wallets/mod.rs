mod BasicWalletImpl;

pub mod WalletFactory;

use BasicWalletImpl::BasicWallet;

#[derive(Debug)]
pub enum WalletTypes {
    // file base storage
    Basic(BasicWallet),
    // using indysdk wallets
    Indy()
}

/*
    Wallet is used to store records for an agent.
*/
pub trait WalletTrait {
    fn create(&self);
    fn open(&self);
    fn close(&self);
    fn delete(&self);
}

pub struct WalletTypeFactory {}