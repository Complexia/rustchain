use std::collections::HashMap;

fn update_state(mut ledger: HashMap<String, f32>) -> HashMap<String, f32> {
    ledger.insert("address1".to_string(), 1000.0);
    return ledger;
}

struct State {
    ledger: HashMap<String, f32>
}

impl State {
    fn new() -> Self {
        return State {
            ledger: HashMap::<String, f32>::new()
        }
    }

    fn main(&mut self) {
       //let mut ledger: HashMap<String, f32> = HashMap::<String, f32>::new();
       self.ledger.insert("address1".to_string(), 1000.0);
    }
}

fn main() {
    //let mut ledger: HashMap<String, f32> = HashMap::<String, f32>::new();
    let mut state = State::new();
    state.main();
    
    //ledger = update_state(ledger);
    println!("{}", state.ledger["address1"]);
    
    
}