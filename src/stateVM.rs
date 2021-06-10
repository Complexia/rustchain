use std::collections::HashMap;

//Holds all of the state data. To be replaced by a Merkle Tree
pub struct State {
    pub ledger: HashMap<String, f32>
}

impl State {
    //Constructor for new clean state
    pub fn new() -> Self {
        return State {
            ledger: HashMap::<String, f32>::new()
        }
    }
    //Adds an address and a balance to state
    pub fn update_state(&mut self, address: String, balance: f32) {
        //if address exists, change balance of the address
        if self.ledger.contains_key(&address) {
            self.ledger.remove(&address);
            self.ledger.insert(address.to_string(), balance);
            println!("Hello");
        }
        else {
            //if address doesn't exist add address and balance
            self.ledger.insert(address.to_string(), balance);
        }
        
    }
}

// fn main() {
//     let mut state = State::new();
//     state.update_state();
    
//     println!("{}", state.ledger["address1"]);
    
    
// }