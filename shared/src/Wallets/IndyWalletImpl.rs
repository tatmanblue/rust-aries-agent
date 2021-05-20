use indy::wallet;
use std::io::Error;

use super::WalletTrait;
use super::Records::*;


//
// configures the Indy wallet
//
#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndyWalletConfig {
    pub id: String,
    pub key: String,
    pub pool_file_name: String
}

//
// this is the indy wallet "database".
//
// create will create a new wallet
#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndyWallet {
    pub configuration: IndyWalletConfig
}

// ----------------------------------------------------------------------------------
// Implementation for IndyWalletConfig

impl IndyWalletConfig {
    pub fn from_json(json: &str) -> Result<IndyWalletConfig, Error> {
        let config: IndyWalletConfig = serde_json::from_str(json)?;
        Ok(config)
    }
}

impl IndyWallet
{
    pub fn new(wallet_config: &str) -> IndyWallet {
        let config: IndyWalletConfig = match IndyWalletConfig::from_json(wallet_config) {
            Ok(success) => success,
            Err(e) => {
                // the system cannot continue at this point
                panic!("did not understand wallet-config, cannot continue {:?}", e);
            }
        };

        let config_data:String = json!({
            "id": config.id
        }).to_string();

        let credentials:String = json!({
            "key": config.key
        }).to_string();

        let result = wallet::create_wallet(&config_data, &credentials).wait();

        let wallet: IndyWallet = IndyWallet {
            configuration: config
        };

        wallet
    }
}

// ----------------------------------------------------------------------------------
// Implementation WalletTrait for BasicWallet

impl WalletTrait for IndyWallet {
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
        unimplemented!()
    }
}