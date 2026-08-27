use crate::core::model::{State, TranspositionTable};



impl State {
    pub fn new() -> State {
        State {
            position: None,
            tt: TranspositionTable::new(32 * 1024 * 1024)
        }
    }
}
