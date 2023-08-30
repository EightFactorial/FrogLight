use super::{BlockState, StatesMap};

pub trait StatesMapFn {
    fn get_state(&self, state_id: &u32) -> &BlockState;
}

impl StatesMapFn for StatesMap {
    fn get_state(&self, state_id: &u32) -> &BlockState {
        self.get(state_id).unwrap_or(&self[&u32::MAX])
    }
}
