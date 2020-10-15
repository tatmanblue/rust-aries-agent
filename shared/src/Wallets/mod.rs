pub mod WalletTrait;

#[derive(Debug)]
pub enum WalletTypes {
    // file base storage
    Basic(),
    // using indysdk wallets
    Indy()
}
pub struct WalletTypeFactory {}