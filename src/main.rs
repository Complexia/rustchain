mod stateVM;
use stateVM::{ State };

fn main() {
    println!("Hello, world!");
    let mut state = State::new();
    state.update_state();
    
    println!("{}", state.ledger["address1"]);
}
