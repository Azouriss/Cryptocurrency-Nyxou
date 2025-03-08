// blockchain/transaction.rs
#[derive(Debug, Clone)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
}
