use std::io::{self, BufRead};
use std::str::FromStr;

macro_rules! init_input {
    ($ii:ident) => {
        let stdin = std::io::stdin();
        let stdin_lock = stdin.lock();
        let mut $ii = In::new(stdin_lock);
    };
}

fn main() {
    init_input!(ii);

    let n: usize = ii.get_num();
    let vec: Vec<usize> = ii.get_vec();

    let (a, b) = ii.get_two();

}




///
/// HELPERS
///

struct In<'a>{
    it: io::Lines<io::StdinLock<'a>>
}

#[allow(dead_code)]
impl<'a> In<'a> {
    fn new(lock: io::StdinLock<'a>) -> Self {
        Self { it: lock.lines() }
    }

    fn get_num<T: FromStr>(&mut self) -> T
    where <T as FromStr>::Err: std::fmt::Debug
    {
        self.it.next().unwrap().unwrap().parse::<T>().unwrap()
    }

    fn get_two<T: FromStr>(&mut self) -> (T, T)
    where <T as FromStr>::Err: std::fmt::Debug
    {
        let vec = Self::get_vec();
        (vec[0].unwrap(), vec[1].unwrap())
    }

    fn get_three<T: FromStr>(&mut self) -> (T, T, T)
    where <T as FromStr>::Err: std::fmt::Debug
    {
        let vec = Self::get_vec();
        (vec[0].unwrap(), vec[1].unwrap(), vec[2].unwrap())
    }

    fn get_vec<T: FromStr>(&mut self) -> Vec<T>
    where <T as FromStr>::Err: std::fmt::Debug
    {
        let row = self.it.next().unwrap().unwrap();
        row.trim().split_whitespace()
            .map(|n| n.parse::<T>().unwrap())
            .collect::<Vec<_>>()
    }

    fn next_line(&mut self) -> Option<String> {
        self.it.next().map(|line| line.unwrap())
    }
}