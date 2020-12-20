use rust_base58::ToBase58;
use sodiumoxide::crypto::sign::ed25519::{
    gen_keypair,
    keypair_from_seed,
    PublicKey,
    Seed,
    SecretKey
};

/*
    did and ver_key confirm the standards defined in indysdk.
    and is modeled from
    https://github.com/hyperledger/indy-sdk/blob/7f4fbb055478181761347a981544fb823d6f47ba/libindy/src/services/crypto/mod.rs#L53
*/
// toThink() should type go above this mod?
pub struct Did
{
    pub did: String,
    pub ver_key: String,            // toThink() Indy calls pub key ver_key.  but maybe we should call it publicKey
    pub private_key: String,
}

impl Did
{
    pub fn new(seed: Option<String>) -> Did
    {
        let did: String;
        let ver_key: String;
        let private_key: String;

        let (v, s): (PublicKey, SecretKey) = match seed
        {
            None =>
                {
                    gen_keypair()
                },
            Some(seed_str) =>
                {
                    let seed_bytes: &[u8] = seed_str.as_bytes();
                    keypair_from_seed(&Seed::from_slice(seed_bytes).unwrap())
                }
        };

        did = v[0..16].to_vec().to_base58();
        ver_key = v[..].to_base58();
        private_key = s[..].to_base58();

        return Did
        {
            did,
            ver_key,
            private_key,
        };
    }
}