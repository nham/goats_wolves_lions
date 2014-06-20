use std::cmp::{min, max};
mod test;

type Forest = [u32, ..3];

fn same_parity(f: Forest, i: uint, j: uint) -> bool {
    // Assuming here that i != j. This will give nonsense otherwise
    let p = parity_tuple(f);
    p[i] == p[j]
}

fn parity_tuple(f: Forest) -> [u32, ..3] {
    [f[0] % 2, f[1] % 2, f[2] % 2]
}

fn find_same_parity(f: Forest) -> (uint, uint) {
    if same_parity(f, 0u, 1u) {
        (0u, 1u)
    } else if same_parity(f, 0u, 2u) {
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

// Combine components i and j if i and j are both nonzero, otherwise return None
fn combine(mut f: Forest, i: uint, j: uint, k: uint) -> Option<Forest> {
    if f[i] == 0 || f[j] == 0 {
        return None;
    }

    f[i] -= 1;
    f[j] -= 1;
    f[k] += 1;
    Some(f)
}


fn main() {
    let mut f: Forest = [55, 6, 17];
    let same = find_same_parity(f);

    let (i, j) = if f[same.val0()] <= f[same.val1()] {
        same
    } else {
        (same.val1(), same.val0())
    };


    let k = remaining_index(i, j).unwrap();

    while i != j {
        println!("{} {} {}", f[0], f[1], f[2]);
        if k == 0 {
           f = combine(f, i, j, k).unwrap();
        }

        f = combine(f, k, j , i).unwrap();
    }

    println!("{}", f[j] + f[k]);

}
