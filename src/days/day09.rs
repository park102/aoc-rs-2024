use crate::{Solution, SolutionPair};
use std::fs::read_to_string;


///////////////////////////////////////////////////////////////////////////////
fn ans(input: String) -> Vec<usize> {
    let sol1: usize = 0;
    let sol2: usize = 0;
    vec![sol1,sol2]

}


pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol = ans(input.unwrap());
        (Solution::from(sol[0]), Solution::from(sol[1]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = ans(read_to_string("input/day1.test").unwrap())[0];
        assert_eq!(result, 142);
    }
}


