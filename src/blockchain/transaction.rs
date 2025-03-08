// blockchain/transaction.rs
use pqcrypto_dilithium::dilithium2::{PublicKey, SignedMessage};
use pqcrypto_traits::sign::PublicKey as _; // pour pouvoir utiliser as_bytes() sur PublicKey
use pqcrypto_traits::sign::SignedMessage as _; // pour pouvoir utiliser as_bytes() sur SignedMessage
use std::fmt;

#[derive(Clone)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub public_key: Option<PublicKey>,
    pub signature: Option<SignedMessage>,
}

impl Transaction {
    // Signe la transaction avec la clé secrète et stocke la clé publique
    pub fn sign_transaction(
        &mut self,
        secret_key: &pqcrypto_dilithium::dilithium2::SecretKey,
        public_key: &pqcrypto_dilithium::dilithium2::PublicKey,
    ) {
        // Préparer la donnée à signer
        let data = format!("{}:{}:{}", self.from, self.to, self.amount);
        let data_bytes = data.as_bytes();

        // Signer -> retourne un "SignedMessage"
        let signed_msg = pqcrypto_dilithium::dilithium2::sign(data_bytes, secret_key);

        // Stocker la signature et la clé publique dans la transaction
        self.signature = Some(signed_msg);
        self.public_key = Some(public_key.clone());
        // clone() car PublicKey n'implémente pas forcément Copy
    }

    // Vérifie la signature
    pub fn verify_transaction(&self) -> bool {
        // Vérifie qu'on a bien une signature ET une clé publique
        if let (Some(pk), Some(sig)) = (&self.public_key, &self.signature) {
            // Reconstruit le même message
            let data = format!("{}:{}:{}", self.from, self.to, self.amount);
            let data_bytes = data.as_bytes();

            // dilithium2::open(...) -> Result<Vec<u8>, Error>, déchiffre le message
            match pqcrypto_dilithium::dilithium2::open(sig, pk) {
                Ok(verified_msg) => verified_msg == data_bytes,
                Err(_) => false,
            }
        } else {
            false
        }
    }
}

// Implémentation manuelle du Debug
impl fmt::Debug for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // On peut afficher un résumé de la clé ou un placeholder
        let pk_str = if let Some(ref pk) = self.public_key {
            // as_bytes() est maintenant disponible grâce à l'import du trait
            format!("PublicKey ({} bytes)", pk.as_bytes().len())
        } else {
            "None".to_string()
        };

        let sig_str = if let Some(ref sig) = self.signature {
            format!("SignedMessage ({} bytes)", sig.as_bytes().len())
        } else {
            "None".to_string()
        };

        f.debug_struct("Transaction")
            .field("from", &self.from)
            .field("to", &self.to)
            .field("amount", &self.amount)
            .field("public_key", &pk_str)
            .field("signature", &sig_str)
            .finish()
    }
}
