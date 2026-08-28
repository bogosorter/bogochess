use crate::core::model::{EngineState, TranspositionTable};



impl EngineState {
    pub fn new() -> EngineState {
        EngineState {
            game_state: None,
            tt: TranspositionTable::new(16 * 1024 * 1024)
        }
    }
}
