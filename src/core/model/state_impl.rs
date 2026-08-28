use crate::core::model::{State, TranspositionTable};



impl State {
    pub fn new() -> State {
        State {
            position: None,
            tt: TranspositionTable::new(16 * 1024 * 1024)
        }
    }
}
