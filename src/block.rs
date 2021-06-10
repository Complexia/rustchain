pub struct Transaction {
    pub address: String,
    pub balance: f32
}

pub struct Block {
    pub list: Vec<Transaction>
}

impl Block {
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.list.push(transaction);
    }
}