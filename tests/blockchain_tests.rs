use nyxou_lib::blockchain::block::Block;
use nyxou_lib::blockchain::blockchain::Blockchain;
use nyxou_lib::blockchain::transaction::Transaction;

#[test]
fn test_blockchain_add_block() {
    let mut chain = Blockchain::new();

    // Exemple de transactions
    let tx = Transaction {
        from: "Alice".to_string(),
        to: "Bob".to_string(),
        amount: 100,
    };

    let new_block = Block::new(
        1,
        1234567890,
        vec![tx],
        String::new(),
        0,
    );

    chain.add_block(new_block);

    assert_eq!(chain.chain.len(), 2, "La blockchain devrait contenir 2 blocs");
    assert_eq!(chain.chain[1].index, 1, "L'index du nouveau bloc devrait être 1");
}

#[test]
fn test_blockchain_genesis_block() {
    let chain = Blockchain::new();
    assert_eq!(chain.chain.len(), 1, "La blockchain devrait commencer avec 1 bloc (genesis)");
    assert_eq!(chain.chain[0].hash, "0", "Le bloc genésis a un hash par défaut '0'");
}
