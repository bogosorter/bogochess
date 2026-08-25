use crate::core::search::SearchStatistics;

impl SearchStatistics {
    pub fn new() -> SearchStatistics {
        // Search algorithms, for simplicity, will add one node for the root,
        // but that node is traditionally not counted. As such, we resort to
        // using -1 as the start value
        SearchStatistics { depth: 0, selective_depth: 0, nodes: -1, score: 0.0, search_time: 0 }
    }
}
