// tests/blockchain_tests.rs

use nyxou_lib::blockchain::block::Block;
use nyxou_lib::blockchain::blockchain::Blockchain;
use nyxou_lib::blockchain::transaction::Transaction;

// ====================
// 1) Test du genesis block
// ====================
#[test]
fn test_blockchain_genesis_block() {
    let chain = Blockchain::new();

    assert_eq!(
        chain.chain.len(),
        1,
        "La blockchain devrait commencer avec 1 bloc (genesis)."
    );
    assert_eq!(
        chain.chain[0].index, 0,
        "L'index du bloc genésis devrait être 0."
    );
    assert_eq!(
        chain.chain[0].hash, "0",
        "Le bloc genésis a un hash par défaut '0'."
    );
}

// ====================
// 2) Test ajout d'un bloc
// ====================
#[test]
fn test_blockchain_add_block() {
    let mut chain = Blockchain::new();

    let tx = Transaction {
        from: "Alice".to_string(),
        to: "Bob".to_string(),
        amount: 100,
        public_key: None, // si tu as ces champs
        signature: None,
    };

    // Création d'un bloc
    let new_block = Block::new(
        1,
        1234567890,
        vec![tx],
        chain.chain.last().unwrap().hash.clone(),
        0,
    );
    chain.add_block(new_block);

    // Vérifie la taille de la chaîne
    assert_eq!(
        chain.chain.len(),
        2,
        "La blockchain devrait contenir 2 blocs."
    );

    // Vérifie le contenu du nouveau bloc
    assert_eq!(
        chain.chain[1].index, 1,
        "L'index du nouveau bloc devrait être 1."
    );

    // Vérifie que le previous_hash du bloc[1] correspond au hash du bloc[0]
    assert_eq!(
        chain.chain[1].previous_hash, chain.chain[0].hash,
        "Le previous_hash du bloc doit être égal au hash du bloc précédent."
    );
}

// ====================
// 3) Test ajout de plusieurs blocs
// ====================
#[test]
fn test_add_multiple_blocks() {
    let mut chain = Blockchain::new();

    // Exemple de transactions
    let tx1 = Transaction {
        from: "Alice".to_string(),
        to: "Bob".to_string(),
        amount: 50,
        public_key: None,
        signature: None,
    };
    let tx2 = Transaction {
        from: "Bob".to_string(),
        to: "Charlie".to_string(),
        amount: 30,
        public_key: None,
        signature: None,
    };

    // Premier bloc
    let block1 = Block::new(
        chain.chain.len() as u64, // index = 1 si la longueur actuelle = 1 (le genesis)
        1111111111,
        vec![tx1],
        chain.chain.last().unwrap().hash.clone(),
        0,
    );
    chain.add_block(block1);

    // Deuxième bloc
    let block2 = Block::new(
        chain.chain.len() as u64, // index = 2
        2222222222,
        vec![tx2],
        chain.chain.last().unwrap().hash.clone(),
        0,
    );
    chain.add_block(block2);

    // On s'attend maintenant à 3 blocs dans la chaîne (1 genesis + 2 ajoutés)
    assert_eq!(
        chain.chain.len(),
        3,
        "La blockchain devrait contenir 3 blocs après en avoir ajouté 2."
    );

    // Vérifie que le bloc[1] a pour previous_hash le hash du bloc[0]
    assert_eq!(
        chain.chain[1].previous_hash, chain.chain[0].hash,
        "Le previous_hash du bloc[1] devrait être le hash du genesis block."
    );

    // Vérifie que le bloc[2] a pour previous_hash le hash du bloc[1]
    assert_eq!(
        chain.chain[2].previous_hash, chain.chain[1].hash,
        "Le previous_hash du bloc[2] devrait être le hash du bloc[1]."
    );
}

// ====================
// 4) (Optionnel) Test signature post-quantique
// ====================
#[test]
fn test_pq_signed_transaction() {
    use pqcrypto_dilithium::dilithium2;

    // Générer une paire de clés Dilithium2
    let (pk, sk) = dilithium2::keypair();

    // Transaction à signer
    let mut tx = Transaction {
        from: "Alice".to_string(),
        to: "Bob".to_string(),
        amount: 999,
        public_key: None,
        signature: None,
    };

    // Signe la transaction (implémentation supposée dans transaction.rs)
    tx.sign_transaction(&sk, &pk);

    // Vérifie la validité
    assert!(
        tx.verify_transaction(),
        "La signature PQ de la transaction devrait être valide."
    );
}
