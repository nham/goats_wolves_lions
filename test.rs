use super::{Forest, parity_tuple, find_same_parity, combine};

#[test]
fn forest_parity_tuple() {
    let f1: Forest = [3, 5, 7];
    let p1 = parity_tuple(f1);

    assert!(p1[0] == 1);
    assert!(p1[1] == 1);
    assert!(p1[2] == 1);

    let f2: Forest = [2, 4, 6];
    let p2 = parity_tuple(f2);

    assert!(p2[0] == 0);
    assert!(p2[1] == 0);
    assert!(p2[2] == 0);


    let f3: Forest = [2, 5, 6];
    let p3 = parity_tuple(f3);

    assert!(p3[0] == 0);
    assert!(p3[1] == 1);
    assert!(p3[2] == 0);
}

#[test]
fn forest_same_parity() {
    let f1: Forest = [3,18,6];
    assert!(find_same_parity(f1) == (1u, 2u));

    let f2: Forest = [3,18,9];
    assert!(find_same_parity(f2) == (0u, 2u));

}

#[test]
fn forest_combine() {
    let mut f: Forest = [3, 18, 6];
    let mut res: Option<Forest> = None;

    res = combine(f, 0u, 1u, 2u);
    assert!(res.is_some() && res.unwrap() == [2, 17, 7]);
    f = res.unwrap();

    res = combine(f, 0u, 2u, 1u);
    assert!(res.is_some() && res.unwrap() == [1, 18, 6]);
    f = res.unwrap();

    res = combine(f, 1u, 2u, 0u);
    assert!(res.is_some() && res.unwrap() == [2, 17, 5]);
    f = res.unwrap();

    res = combine(f, 0u, 1u, 2u);
    assert!(res.is_some() && res.unwrap() == [1, 16, 6]);
    f = res.unwrap();

    res = combine(f, 1u, 0u, 2u);
    assert!(res.is_some() && res.unwrap() == [0, 15, 7]);
    f = res.unwrap();

    res = combine(f, 0u, 1u, 2u);
    assert!(res.is_none());
}
