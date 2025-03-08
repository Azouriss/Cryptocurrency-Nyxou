// consensus/quantum_security.rs
use pqcrypto_dilithium::dilithium2;

pub fn apply_quantum_security() {
    println!("Applying post-quantum security measures...");

    // 1) Génération d'une paire de clés
    let (public_key, secret_key) = dilithium2::keypair();

    // 2) On signe un message (ex: "Hello Quantum World")
    let message = b"Hello Quantum World";
    let signature = dilithium2::sign(message, &secret_key);

    // 3) Vérification
    match dilithium2::open(&signature, &public_key) {
        Ok(verified_msg) => {
            println!(
                "Signature PQ valide. Message: {:?}",
                String::from_utf8_lossy(&verified_msg)
            );
        }
        Err(_) => {
            println!("Signature PQ invalide ou corrompue !");
        }
    }
}
