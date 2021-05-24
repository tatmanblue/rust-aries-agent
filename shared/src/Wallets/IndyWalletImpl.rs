use indyrs::{
    INVALID_WALLET_HANDLE,
    future::Future,
    wallet,
    WalletHandle
};
use std::io::Error;

use super::WalletTrait;
use super::Records::*;
use indyrs::ErrorCode::WalletAlreadyExistsError;

static RECORD_TYPE_RECORD: &'static str = "record_type";

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
    pub configuration: IndyWalletConfig,
    pub wallet_handle: WalletHandle,
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

        let wallet: IndyWallet = IndyWallet {
            configuration: config,
            wallet_handle: INVALID_WALLET_HANDLE
        };

        wallet
    }
}

// ----------------------------------------------------------------------------------
// Implementation WalletTrait for BasicWallet

impl WalletTrait for IndyWallet {
    fn open(&mut self) {

        let config_data:String = json!({
            "id": self.configuration.id
        }).to_string();

        let credentials:String = json!({
            "key": self.configuration.key
        }).to_string();

        match wallet::create_wallet(&config_data, &credentials).wait() {
            Ok(_s) => {},
            Err(e) => {
                if e.error_code != WalletAlreadyExistsError {
                    panic!("error creating indy wallet {:?}", e);
                }
            }
        }


        match wallet::open_wallet(&config_data, &credentials).wait() {
            Ok(wallet) => {
                self.wallet_handle = wallet;
            },
            Err(e) => {
                panic!("indy wallet open failed {:?}", e)
            }
        }

    }

    fn close(&self) {
        match wallet::close_wallet(self.wallet_handle).wait() {
            Ok(_s) => debug!("wallet closed"),
            Err(e) => debug!("error closing indy wallet {:?}", e)
        }
    }

    fn delete(&self) {

        let config_data:String = json!({
            "id": self.configuration.id
        }).to_string();

        let credentials:String = json!({
            "key": self.configuration.key
        }).to_string();

        match wallet::delete_wallet(&config_data, &credentials).wait() {
            Ok(_s) => info!("wallet deleted"),
            Err(e) => panic!("deleting wallet failed {:?}", e)
        }
    }

    //
    //    Connection record is saved by connection_id and tagged with "connections"
    //    this allows retrieval by connection_id and searches on connections
    //
    fn save_invitation(&self, record: &ConnectionRecord) {

        match wallet::get_wallet_record(self.wallet_handle, RECORD_TYPE_RECORD, &record.id.to_string(), "{}").wait() {
            Ok(_s) => {
                debug!("updating record")
            },
            Err(_e) => {
                debug!("saving new record")
            }
        };

        unimplemented!()
    }
}