// TODO: Doctests

/// Insert `x` in `a[lo..hi]`, keeping it sorted assuming `a` is sorted.
/// If `a` contains `x`, insert it just *after* the *rightmost* occurence of `x`.
pub fn insort_right_slice<T>(a: &mut Vec<T>, x: T, lo: Option<usize>, hi: Option<usize>)
where
    T: Ord,
{
    let lo = bisect_right_slice(a, &x, lo, hi);
    a.insert(lo, x);
}

/// Insert `x` in `a`, keeping it sorted assuming `a` is sorted.
/// If `a` contains `x`, insert it just *after* the *rightmost* occurence of `x`.
pub fn insort_right<T>(a: &mut Vec<T>, x: T)
where
    T: Ord,
{
    insort_right_slice(a, x, None, None);
}

/// Return the index where `x` should be inserted in `a[lo..hi]`, assuming `a`
/// is sorted.
///
/// The return value `i` is such that all `e` in `a[..i]` have `e <= x`, and
/// all `e` in `a[i..]` have `e > x`.
/// - If `a` contains `x`, `a.insert(i, x)` will insert just *after* the
///   *rightmost* `x`.
pub fn bisect_right_slice<T>(a: &Vec<T>, x: &T, lo: Option<usize>, hi: Option<usize>) -> usize
where
    T: Ord,
{
    let mut lo = lo.unwrap_or(0);
    let mut hi = hi.unwrap_or(a.len());
    while lo < hi {
        let mid = (lo + hi) / 2;
        if x < &a[mid] {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

/// Return the index where `x` should be inserted in `a`, assuming `a` is sorted.
///
/// The return value `i` is such that all `e` in `a[..i]` have `e <= x`, and
/// all `e` in `a[i..]` have `e > x`.
/// - If `a` contains `x`, `a.insert(i, x)` will insert just *after* the
///   *rightmost* occurence of `x`.
pub fn bisect_right<T>(a: &Vec<T>, x: &T) -> usize
where
    T: Ord,
{
    bisect_right_slice(a, x, None, None)
}
/// Insert `x` in `a[lo..hi]`, keeping it sorted assuming `a` is sorted.
/// If `a` contains `x`, insert it just *before* the *leftmost* occurence of `x`.
pub fn insort_left_slice<T>(a: &mut Vec<T>, x: T, lo: Option<usize>, hi: Option<usize>)
where
    T: Ord,
{
    let lo = bisect_left_slice(a, &x, lo, hi);
    a.insert(lo, x);
}

/// Insert `x` in `a`, keeping it sorted assuming `a` is sorted.
/// If `a` contains `x`, insert it just *before* the *leftmost* occurence of `x`.
pub fn insort_left<T>(a: &mut Vec<T>, x: T)
where
    T: Ord,
{
    insort_left_slice(a, x, None, None);
}

/// Return the index where `x` should be inserted in `a[lo..hi]`, assuming `a`
/// is sorted.
///
/// The return value `i` is such that all `e` in `a[..i]` have `e < x`, and
/// all `e` in `a[i..]` have `e >= x`.
/// - If `a` contains `x`, `a.insert(i, x)` will insert just *before* the
///   *leftmost* `x`.
pub fn bisect_left_slice<T>(a: &Vec<T>, x: &T, lo: Option<usize>, hi: Option<usize>) -> usize
where
    T: Ord,
{
    let mut lo = lo.unwrap_or(0);
    let mut hi = hi.unwrap_or(a.len());
    while lo < hi {
        let mid = (lo + hi) / 2;
        if &a[mid] < x {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

/// Return the index where `x` should be inserted in `a`, assuming `a` is sorted.
///
/// The return value `i` is such that all `e` in `a[..i]` have `e < x`, and
/// all `e` in `a[i..]` have `e >= x`.
/// - If `a` contains `x`, `a.insert(i, x)` will insert just *before* the
///   *leftmost* `x`.
pub fn bisect_left<T>(a: &Vec<T>, x: &T) -> usize
where
    T: Ord,
{
    bisect_left_slice(a, x, None, None)
}

#[cfg(test)]
mod tests {

    use super::*;
    use proptest::prelude::*;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[derive(Clone, Debug)]
    struct BisectTest<T: 'static>
    where
        T: Ord,
    {
        name: &'static str,
        a: &'static [T],
        x: T,
        expected_index: usize,
    }

    #[derive(Debug, Clone)]
    enum TestDirection {
        Left,
        Right,
    }

    type TestCollection<T: Ord + Clone> = &'static [BisectTest<T>];

    macro_rules! t {
        ($name:ident, $a:expr, $x:expr, $expected_index:expr) => {
            BisectTest {
                name: stringify!($name),
                a: $a,
                x: $x,
                expected_index: $expected_index,
            }
        };
    }

    const RIGHT_INT_CASES: TestCollection<i32> = &[
        t!(ints_right_0, &[], 1, 0),
        t!(ints_right_1, &[1], 0, 0),
        t!(ints_right_2, &[1], 1, 1),
        t!(ints_right_3, &[1], 2, 1),
        t!(ints_right_4, &[1, 1], 0, 0),
        t!(ints_right_5, &[1, 1], 1, 2),
        t!(ints_right_6, &[1, 1], 2, 2),
        t!(ints_right_7, &[1, 1, 1], 0, 0),
        t!(ints_right_8, &[1, 1, 1], 1, 3),
        t!(ints_right_9, &[1, 1, 1], 2, 3),
        t!(ints_right_10, &[1, 1, 1, 1], 0, 0),
        t!(ints_right_11, &[1, 1, 1, 1], 1, 4),
        t!(ints_right_12, &[1, 1, 1, 1], 2, 4),
        t!(ints_right_13, &[1, 2], 0, 0),
        t!(ints_right_14, &[1, 2], 1, 1),
        t!(ints_right_15, &[1, 2], 2, 2),
        t!(ints_right_16, &[1, 2], 3, 2),
        t!(ints_right_17, &[1, 1, 2, 2], 0, 0),
        t!(ints_right_18, &[1, 1, 2, 2], 1, 2),
        t!(ints_right_19, &[1, 1, 2, 2], 2, 4),
        t!(ints_right_20, &[1, 1, 2, 2], 3, 4),
        t!(ints_right_21, &[1, 2, 3], 0, 0),
        t!(ints_right_22, &[1, 2, 3], 1, 1),
        t!(ints_right_23, &[1, 2, 3], 2, 2),
        t!(ints_right_24, &[1, 2, 3], 3, 3),
        t!(ints_right_25, &[1, 2, 3], 4, 3),
        t!(ints_right_26, &[1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 0, 0),
        t!(ints_right_27, &[1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 1, 1),
        t!(ints_right_28, &[1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 2, 3),
        t!(ints_right_29, &[1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 3, 6),
        t!(ints_right_30, &[1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 4, 10),
        t!(ints_right_31, &[1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 5, 10),
    ];

    const LEFT_INT_CASES: TestCollection<i32> = &[
        t!(ints_left_0, &[], 1, 0),
        t!(ints_left_1, &[1], 0, 0),
        t!(ints_left_2, &[1], 1, 0),
        t!(ints_left_3, &[1], 2, 1),
        t!(ints_left_4, &[1, 1], 0, 0),
        t!(ints_left_5, &[1, 1], 1, 0),
        t!(ints_left_6, &[1, 1], 2, 2),
        t!(ints_left_7, &[1, 1, 1], 0, 0),
        t!(ints_left_8, &[1, 1, 1], 1, 0),
        t!(ints_left_9, &[1, 1, 1], 2, 3),
        t!(ints_left_10, &[1, 1, 1, 1], 0, 0),
        t!(ints_left_11, &[1, 1, 1, 1], 1, 0),
        t!(ints_left_12, &[1, 1, 1, 1], 2, 4),
        t!(ints_left_13, &[1, 2], 0, 0),
        t!(ints_left_14, &[1, 2], 1, 0),
        t!(ints_left_15, &[1, 2], 2, 1),
        t!(ints_left_16, &[1, 2], 3, 2),
        t!(ints_left_17, &[1, 1, 2, 2], 0, 0),
        t!(ints_left_18, &[1, 1, 2, 2], 1, 0),
        t!(ints_left_19, &[1, 1, 2, 2], 2, 2),
        t!(ints_left_20, &[1, 1, 2, 2], 3, 4),
        t!(ints_left_21, &[1, 2, 3], 0, 0),
        t!(ints_left_22, &[1, 2, 3], 1, 0),
        t!(ints_left_23, &[1, 2, 3], 2, 1),
        t!(ints_left_24, &[1, 2, 3], 3, 2),
        t!(ints_left_25, &[1, 2, 3], 4, 3),
        t!(ints_left_26, &[1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 0, 0),
        t!(ints_left_27, &[1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 1, 0),
        t!(ints_left_28, &[1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 2, 1),
        t!(ints_left_29, &[1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 3, 3),
        t!(ints_left_30, &[1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 4, 6),
        t!(ints_left_31, &[1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 5, 10),
    ];

    #[test]
    fn bisect_right_precomputed() {
        run_bisect_tests(TestDirection::Right, RIGHT_INT_CASES);
    }

    #[test]
    fn bisect_left_precomputed() {
        run_bisect_tests(TestDirection::Left, LEFT_INT_CASES);
    }

    #[test]
    fn bisect_right_slice_precomputed() {
        run_bisect_slice_tests(TestDirection::Right, RIGHT_INT_CASES);
    }

    #[test]
    fn bisect_left_slice_precomputed() {
        run_bisect_slice_tests(TestDirection::Left, LEFT_INT_CASES);
    }

    // TODO: Just put function pointer in test cases?

    fn run_bisect_tests<T: Clone + Ord>(direction: TestDirection, test_cases: TestCollection<T>) {
        let bisect_func = match direction {
            TestDirection::Left => bisect_left,
            TestDirection::Right => bisect_right,
        };

        for test_case in test_cases {
            let data = test_case.a.to_vec();
            assert_eq!(test_case.expected_index, bisect_func(&data, &test_case.x));
        }
    }

    fn run_bisect_slice_tests<T: Clone + Ord>(
        direction: TestDirection,
        test_cases: TestCollection<T>,
    ) {
        let bisect_func = match direction {
            TestDirection::Left => bisect_left_slice,
            TestDirection::Right => bisect_right_slice,
        };

        for test_case in test_cases {
            let data = test_case.a.to_vec();
            for lo in 0..4 {
                for hi in 3..8 {
                    let hi = std::cmp::min(data.len(), hi);
                    let ip = bisect_func(&data, &test_case.x, Some(lo), Some(hi));

                    match direction {
                        TestDirection::Left => {
                            if ip < hi {
                                assert!(test_case.x <= data[ip]);
                            }

                            if ip > lo {
                                assert!(data[ip - 1] < test_case.x)
                            }
                        }
                        TestDirection::Right => {
                            if ip < hi {
                                assert!(test_case.x < data[ip]);
                            }

                            if ip > lo {
                                assert!(data[ip - 1] <= test_case.x)
                            }
                        }
                    }

                    assert_eq!(
                        ip,
                        std::cmp::max(lo, std::cmp::min(hi, test_case.expected_index))
                    );
                }
            }
        }
    }

    proptest! {

        #[test]
        fn test_bisect_left_index_property(
            mut nums in prop::collection::vec(any::<u32>(), 0..500),
            num in any::<u32>()
        ) {
            nums.sort();

            let i = bisect_left(&nums, &num);

            // See `bisect_left` docstring
            assert!(nums[..i].iter().all(|&x| x < num));
            assert!(nums[i..].iter().all(|&x| x >= num));
        }

        #[test]
        fn test_bisect_right_index_property(
            mut nums in prop::collection::vec(any::<u32>(), 0..500),
            num in any::<u32>()
        ) {
            nums.sort();

            let i = bisect_right(&nums, &num);

            // See `bisect_right` docstring
            assert!(nums[..i].iter().all(|&x| x <= num));
            assert!(nums[i..].iter().all(|&x| x > num));
        }

        #[test]
        fn test_insort_vs_vec_sort(
            digits in prop::collection::vec(0..10, 0..500)
        ) {
            let left_digits = HashSet::<i32>::from_iter(vec![0, 2, 4, 6, 8]);
            let mut insorted = vec![];

            for digit in digits {
                let f = if  left_digits.contains(&digit) {
                    insort_left
                } else {
                    insort_right
                };

                f(&mut insorted, digit);
            }

            let vec_sorted = {
                let mut v = insorted.clone();
                v.sort();
                v
            };

            assert_eq!(vec_sorted, insorted);
        }
    }
}
