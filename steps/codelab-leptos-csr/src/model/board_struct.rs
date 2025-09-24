use std::cmp::PartialEq;
use std::collections::HashSet;
use rand::distr::{Distribution, Uniform};


#[derive(Copy,Clone,Debug, PartialEq, Eq, Hash)]
pub struct  Case {
    mines_around: u8,
    is_revealed: bool,
    is_flagged: bool,
    is_mine: bool,
}

impl Case {
    pub fn new() -> Case {
        Case {
            mines_around: 0,
            is_revealed: false,
            is_flagged: false,
            is_mine: false,
        }
    }

    pub fn new_mine() -> Case {
        Case {
            mines_around: 0,
            is_revealed: false,
            is_flagged: false,
            is_mine: true,
        }
    }

    pub fn increase_mines_around(&mut self) {
        if !self.is_mine {
            self.mines_around += 1;
        }
    }

    pub fn is_revealed(&self) -> bool {
        self.is_revealed
    }

    pub fn is_flagged(&self) -> bool {
        self.is_flagged
    }

    pub fn is_mine(&self) -> bool {
        self .is_mine
    }

    pub fn get_mines_around(&self) -> u8 {
        self.mines_around
    }

    pub fn reveal(&mut self) {
        self.is_revealed = true;
    }

    pub fn flag(&mut self, status: bool) {
        self.is_flagged = status;
    }
}

pub struct Board {
     cases: [[Case; 10]; 10],
}

impl Board {
    pub fn new(mines_count: usize) -> Board {
        let mut cases = [[Case::new(); 10]; 10];
        let mut mines_cases = HashSet::<(u8, u8)>::new();
        let step = Uniform::new(0, 10).unwrap();
        let mut rng = rand::thread_rng();

        while mines_cases.len() < mines_count {
            let x = step.sample(&mut rng);;
            let y = step.sample(&mut rng);;
            mines_cases.insert((x, y));
        }


        for (x, y) in mines_cases {
            cases[x as usize][y as usize] = Case::new_mine();
            for i in -1..=1 {
                for j in -1..=1 {
                    let nx = x as i8 + i;
                    let ny = y as i8 + j;
                    if nx >= 0 && nx < 10 && ny >= 0 && ny < 10 {
                        cases[nx as usize][ny as usize].increase_mines_around()
                    }
                }
            }
        }
        Board { cases: cases }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Case> {
        self.cases.iter().flatten()
    }

    pub fn set_case(&mut self,index: usize,value: Case) {
        let y = index % 10;
        let x = index / 10;
        self.cases[x][y] = value;
    }

    pub fn reveal_case(&mut self,x: usize,y: usize) -> &Case {
        if !self.cases[x][y].is_revealed() {
            if self.cases[x][y].is_mine() {
                self.cases[x][y];
            } else {
                self.cases[x][y].reveal();
                if self.cases[x][y].get_mines_around() == 0 {
                    for i in -1..=1 {
                        for j in -1..=1 {
                            let nx = x as i8 + i;
                            let ny = y as i8 + j;
                            if nx >= 0 && nx < 10 && ny >= 0 && ny < 10 {
                                self.reveal_case(nx as usize, ny as usize);
                            }
                        }
                    }
                }
            }
        };
        &self.cases[x][y]
    }
}