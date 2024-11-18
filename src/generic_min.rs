use std::cmp::Ordering;

// TODO: implement the `min` function used in `main`.
// function min which is templated, but where the generic
// T must implement the Ord trait
fn min<T: Ord>(x: T, y: T) -> T {
    if x.cmp(&y) == Ordering::Less {
        x
    }
    else {
        y
    }
}

pub fn test_generic_min() {
    assert_eq!(min(0, 10), 0);
    assert_eq!(min(500, 123), 123);

    assert_eq!(min('a', 'z'), 'a');
    assert_eq!(min('7', '1'), '1');

    assert_eq!(min("hello", "goodbye"), "goodbye");
    assert_eq!(min("bat", "armadillo"), "armadillo");
}