// use crate::state::State;

mod state;

fn main() {
    let mut global_state = state::state_dict::State {
        cycle_no: 0,
        ch0: 0,
        ch1: 1
    };

    state::main_control_loop::main_loop(&mut global_state);
}
