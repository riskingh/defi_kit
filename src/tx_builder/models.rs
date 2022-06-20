#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Transaction {
    pub chain_id: String,
    pub data: String,
    pub from: String,
    pub gas: u128,
    pub to: String,
    pub value: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct TransactResponse {
    pub transaction: Transaction,
}
