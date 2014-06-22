use std::cmp::{min, max};
mod test;

#[deriving(Eq)]
struct Forest {
    state: [u32, ..3],
}

impl Forest {
    fn new(i: u32, j: u32, k: u32) -> Forest {
        Forest { state: [i, j, k] }
    }

    fn parity_tuple(&self) -> [u32, ..3] {
        [self.state[0] % 2, self.state[1] % 2, self.state[2] % 2]
    }

    fn same_parity(&self, i: uint, j: uint) -> bool {
        // Assuming here that i != j. This will give nonsense otherwise
        let p = self.parity_tuple();
        p[i] == p[j]
    }

    // Combine components i and j if i and j are both nonzero. return false otherwise
    fn combine(&mut self, i: uint, j: uint, k: uint) -> bool {
        if self.state[i] == 0 || self.state[j] == 0 {
            return false;
        }

        self.state[i] -= 1;
        self.state[j] -= 1;
        self.state[k] += 1;
        true
    }
}


impl PartialEq for Forest {
    fn eq(&self, other: &Forest) -> bool {
        self.state[0] == other.state[0] 
        && self.state[1] == other.state[1] 
        && self.state[2] == other.state[2]
    }
}



enum Parity {
    Pair(uint, uint),
    All
}

fn find_same_parity(f: Forest) -> (uint, uint) {
    if f.same_parity(0u, 1u) {
        (0u, 1u)
    } else if f.same_parity(0u, 2u) {
        (0u, 2u)
    } else {
        (1u, 2u)
    }
}

fn remaining_index(i:  uint, j: uint) -> Result<uint, String> {
    match (min(i, j), max(i, j)) {
        (0u, 1u) => Ok(2u),
        (0u, 2u) => Ok(1u),
        (1u, 2u) => Ok(0u),
        _        => Err("Indices must be distinct.".to_string())

    }

}



fn main() {
    let mut f = Forest::new(55, 6, 17);
    let same = find_same_parity(f);

    let (i, j) = if f.state[same.val0()] <= f.state[same.val1()] {
        same
    } else {
        (same.val1(), same.val0())
    };


    let k = remaining_index(i, j).unwrap();

    while f.state[i] != f.state[j] {
        //println!("{} {} {}", f[0], f[1], f[2]);
        if f.state[k] == 0 {
           f.combine(i, j, k);
        }

        f.combine(k, j , i);
    }

    println!("{}", f.state[j] + f.state[k]);

}
