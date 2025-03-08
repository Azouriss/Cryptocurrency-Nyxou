// tests/security_tests.rs
use nyxou_lib::consensus::quantum_security;
use pqcrypto_dilithium::dilithium2;
use pqcrypto_traits::sign::SignedMessage;

#[test]
fn test_apply_quantum_security() {
    // Vérifie simplement que la fonction ne panique pas
    quantum_security::apply_quantum_security();
}

/// Test positif : signature et vérification réussies sur un message légitime.
#[test]
fn test_postquantum_sign_verify_success() {
    // Générer une paire de clés
    let (public_key, secret_key) = dilithium2::keypair();

    let message = "Bonjour, test PQ sign/verify !".as_bytes();

    // Signature du message
    let signed_message = dilithium2::sign(message, &secret_key);

    // Vérifier la signature
    let opened_result = dilithium2::open(&signed_message, &public_key)
        .expect("La vérification de signature PQ aurait dû réussir.");

    // Comparer le message obtenu à l'original
    assert_eq!(
        opened_result, message,
        "Le message vérifié devrait correspondre à l'original."
    );
}

/// Test négatif : vérifie qu'une signature échoue si on utilise une mauvaise clé publique.
#[test]
fn test_postquantum_sign_verify_fail_wrong_pubkey() {
    // Génère deux paires de clés
    let (_pk1, sk1) = dilithium2::keypair();
    let (pk2, _sk2) = dilithium2::keypair();

    let message = "Message secret a signer.".as_bytes();
    let signed_msg = dilithium2::sign(message, &sk1);

    // Tente de vérifier avec la deuxième clé publique (mauvaise clé)
    let opened_result = dilithium2::open(&signed_msg, &pk2);

    // La vérification doit échouer
    assert!(
        opened_result.is_err(),
        "La vérification aurait dû échouer avec la mauvaise clé publique."
    );
}

/// Test négatif : altération de la signature pour vérifier que la modification est détectée.
#[test]
fn test_postquantum_sign_verify_fail_tampered_data() {
    // Génère une paire de clés
    let (pk, sk) = dilithium2::keypair();

    // Message original à signer
    let original_message = "Message original 123".as_bytes();
    let signed_msg = dilithium2::sign(original_message, &sk);

    let mut tampered = signed_msg.as_bytes().to_vec();

    // Altère un octet (si la signature n'est pas vide)
    if !tampered.is_empty() {
        tampered[0] = tampered[0].wrapping_add(1);
    }

    // Recrée un SignedMessage altéré à partir des octets modifiés
    let tampered_signed_msg = SignedMessage::from_bytes(&tampered)
        .expect("Impossible de recréer la signature altérée à partir des octets.");

    // Tente de vérifier avec la bonne clé publique
    let opened_result = dilithium2::open(&tampered_signed_msg, &pk);

    // La vérification doit échouer
    assert!(
        opened_result.is_err(),
        "La vérification aurait dû échouer après altération de la signature."
    );
}
