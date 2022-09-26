#[derive(Debug)]
pub struct State {
  pub cycle_no: u64,
  pub ch0: u32,
  pub ch1: u32
}

pub fn step_state(state: &mut State) {
  state.cycle_no += 1;
}