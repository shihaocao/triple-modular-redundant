// use crate::state::State;

mod state;

fn main() {
    let mut global_state = state::State {
        cycle_no: 0,
        ch0: 0,
        ch1: 1
    };

    for _ in 0..20 {
        state::step_state(&mut global_state);
        println!("{:?}", global_state)
    }
}
