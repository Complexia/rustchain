use std::collections::HashMap;

//Holds all of the state data
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
    pub fn update_state(&mut self) {
       self.ledger.insert("address1".to_string(), 1000.0);
    }
}

fn main() {
    let mut state = State::new();
    state.update_state();
    
    println!("{}", state.ledger["address1"]);
    
    
}