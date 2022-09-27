use crate::state::state_dict::State;

pub fn step_state(state: &mut State) {
  state.cycle_no += 1;
}

pub fn print_state(state: &State) {
  println!("{:?}", state);
}


pub fn main_loop(state: &mut State) {
  for _ in 0..20 {
    step_state(state);
    print_state(state); 
  }
}