use leptos::leptos_dom::logging::console_log;
use leptos::prelude::Update;
use reactive_stores::Store;
use crate::model::board::{Board, Case};

#[derive(Store, Debug, Clone)]
pub struct GameState {
    #[store(key:String = |case| case.get_key())]
    pub rows: Vec<IndexCase>,
}

#[derive(Debug, Store, Clone)]
pub struct IndexCase {
    pub idx: usize,
    pub case: Case,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            rows: Self::generate_grid(),
        }
    }

    fn generate_grid() -> Vec<IndexCase> {
        Board::new(10)
            .iter()
            .enumerate()
            .map(|(idx, case)| IndexCase { idx, case: case.clone() })
            .collect::<Vec<IndexCase>>()
    }

    pub fn reset(&mut self)  {
        self.rows = Self::generate_grid();
    }
}

impl IndexCase {
    pub fn get_key(&self) -> String {
        console_log(&format!("hahha key {}", self.idx.to_string() + &self.case.is_flagged().to_string() + &self.case.is_revealed().to_string()));
        self.idx.to_string() + &self.case.is_flagged().to_string() + &self.case.is_revealed().to_string()
    }
}

impl Update for IndexCase {
    type Value = IndexCase;

    fn try_maybe_update<U>(&self, fun: impl FnOnce(&mut Self::Value) -> (bool, U)) -> Option<U> {
        let (changed, res) = fun(&mut self.clone());
        if changed {
            Some(res)
        } else {
            None
        }
    }
}
