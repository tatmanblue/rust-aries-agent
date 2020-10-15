
use std::io::Error;
use std::path::Path;
use super::WalletTrait;

lazy_static! {
	static ref WALLET_CONFIG: BasicWalletConfig = BasicWalletConfig{ file_name: "test.json".to_string(), reset: Some(true)};
}

//
// configures the basic wallet
//
#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
struct BasicWalletConfig {
    #[serde(rename = "fileName")]
    pub file_name: String,
    pub reset: Option<bool>
}

//
// this is the basic wallet "database"
// create will create a new empty database
// open will read from disk at BasicWalletConfig.file_name.  if it cannot find it, create it
//
#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicWallet {

}

// ----------------------------------------------------------------------------------
// Implementation for BasicWalletConfig

impl BasicWalletConfig {
    pub fn from_json(json: &str) -> Result<BasicWalletConfig, Error> {
        let config: BasicWalletConfig = serde_json::from_str(json)?;
        Ok(config)
    }

    pub fn loadBasicWallet(&self) -> BasicWallet {
        // will create a new BasicWallet on
        // 1 - reset = true
        // 2 - file does not exist
        // 3 - file fails to serialize
        let path: &Path = Path::new(&self.file_name);
        println!("wallet file is '{:?}'", path.to_str());

        BasicWallet {

        }
    }
}

// ----------------------------------------------------------------------------------
// Implementation for BasicWallet

impl BasicWallet {
    pub fn new(wallet_config: &str) -> BasicWallet {

        let config: BasicWalletConfig = match BasicWalletConfig::from_json(wallet_config) {
            Ok(success) => success,
            Err(e) => {
                println!("did not understand wallet-config, using default: {:?}", e);
                BasicWalletConfig{ file_name: "test.json".to_string(), reset: Some(true)}
            }
        };

        config.loadBasicWallet()
    }
}

impl WalletTrait for BasicWallet {
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
}