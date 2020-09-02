use bisection::{bisect_left, bisect_right};

/// Locate the leftmost value exactly equal to `x`
fn index<T: Ord>(a: &[T], x: &T) -> Option<usize> {
    let i = bisect_left(a, x);
    if i != a.len() && a[i] == *x {
        Some(i)
    } else {
        None
    }
}

/// Find the rightmost value less than `x`
fn find_lt<'a, T: Ord>(a: &'a [T], x: &T) -> Option<&'a T> {
    let i = bisect_left(a, x);
    if i != 0 {
        Some(&a[i - 1])
    } else {
        None
    }
}

/// Find the rightmost value less than or equal to `x`
fn find_le<'a, T: Ord>(a: &'a [T], x: &T) -> Option<&'a T> {
    let i = bisect_right(a, x);
    if i != 0 {
        Some(&a[i - 1])
    } else {
        None
    }
}

/// Find the leftmost value greater than `x`
fn find_gt<'a, T: Ord>(a: &'a [T], x: &T) -> Option<&'a T> {
    let i = bisect_right(a, x);
    if i != a.len() {
        Some(&a[i - 1])
    } else {
        None
    }
}

/// Find the leftmost value greater than or equal to `x`
fn find_ge<'a, T: Ord>(a: &'a [T], x: &T) -> Option<&'a T> {
    let i = bisect_left(a, x);
    if i != a.len() {
        Some(&a[i - 1])
    } else {
        None
    }
}

fn main() {
    let a = [2, 4, 6, 8, 10, 12];

    println!("{:?}", index(&a, &2));
    println!("{:?}", index(&a, &15));

    println!("{:?}", find_lt(&a, &8));
    println!("{:?}", find_lt(&a, &2));

    println!("{:?}", find_le(&a, &8));
    println!("{:?}", find_le(&a, &1));

    println!("{:?}", find_gt(&a, &8));
    println!("{:?}", find_gt(&a, &12));

    println!("{:?}", find_ge(&a, &12));
    println!("{:?}", find_ge(&a, &13));
}
