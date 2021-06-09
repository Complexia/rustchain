mod stateVM;
use stateVM::{ State };

fn main() {
    println!("Hello, world!");
    let mut state = State::new();
    state.update_state("address1".to_string(), 1000.0);
    state.update_state("address1".to_string(), 500.0);
    
    println!("{}", state.ledger["address1"]);
}
