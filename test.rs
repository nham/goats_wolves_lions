use super::{Forest, find_same_parity};

#[test]
fn forest_parity_tuple() {
    let f1 = Forest::new(3, 5, 7);
    let p1 = f1.parity_tuple();

    assert!(p1[0] == 1);
    assert!(p1[1] == 1);
    assert!(p1[2] == 1);

    let f2 = Forest::new(2, 4, 6);
    let p2 = f2.parity_tuple();

    assert!(p2[0] == 0);
    assert!(p2[1] == 0);
    assert!(p2[2] == 0);


    let f3 = Forest::new(2, 5, 6);
    let p3 = f3.parity_tuple();

    assert!(p3[0] == 0);
    assert!(p3[1] == 1);
    assert!(p3[2] == 0);
}

#[test]
fn forest_same_parity() {
    let f1 = Forest::new(3,18,6);
    assert!(find_same_parity(f1) == (1u, 2u));

    let f2 = Forest::new(3,18,9);
    assert!(find_same_parity(f2) == (0u, 2u));

}

#[test]
fn forest_combine() {
    let mut f = Forest::new(3, 18, 6);
    let mut res = false;

    res = f.combine(0u, 1u, 2u);
    assert!(res && f == Forest::new(2,17, 7));

    res = f.combine(0u, 2u, 1u);
    assert!(res && f == Forest::new(1, 18, 6));

    res = f.combine(1u, 2u, 0u);
    assert!(res && f == Forest::new(2, 17, 5));

    res = f.combine(0u, 1u, 2u);
    assert!(res && f == Forest::new(1, 16, 6));

    res = f.combine(1u, 0u, 2u);
    assert!(res && f == Forest::new(0, 15, 7));

    res = f.combine(0u, 1u, 2u);
    assert!(!res);
}
