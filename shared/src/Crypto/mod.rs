use rust_base58::ToBase58;
use sodiumoxide::crypto::sign::ed25519::{
    gen_keypair,
    keypair_from_seed,
    PublicKey,
    Seed,
    SecretKey
};

// toThink() should type go above this mod?
pub struct Did
{
    pub did: String,
    pub verKey: String,            // toThink() Indy calls pub key ver_key.  but maybe we should call it publicKey
    pub privateKey: String,
}

impl Did
{
    pub fn new(seed: Option<String>) -> Did
    {
        let did: String;
        let verKey: String;
        let privateKey: String;

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
        verKey = v[..].to_base58();
        privateKey = s[..].to_base58();

        return Did
        {
            did,
            verKey,
            privateKey,
        };
    }
}