use crate::core::model::{State, TranspositionTable};



impl State {
    pub fn new() -> State {
        State {
            position: None,
            tt: TranspositionTable::new(64 * 1024 * 1024)
        }
    }
}
