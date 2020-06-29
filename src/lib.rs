// TODO: lo/hi parameters for searching/sorting within a slice of `a`
// TODO: Alias bisect = bisect_right, insort=insort_right

pub fn insort_right<T>(a: &mut Vec<T>, x: T)
where
    T: PartialOrd,
{
    let lo = bisect_right(a, &x);
    a.insert(lo, x);
}

pub fn bisect_right<T>(a: &Vec<T>, x: &T) -> usize
where
    T: PartialOrd,
{
    let mut lo = 0;
    let mut hi = a.len();
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

pub fn insort_left<T>(a: &mut Vec<T>, x: T)
where
    T: PartialOrd,
{
    let lo = bisect_left(a, &x);
    a.insert(lo, x);
}

pub fn bisect_left<T>(a: &Vec<T>, x: &T) -> usize
where
    T: PartialOrd,
{
    let mut lo = 0;
    let mut hi = a.len();
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

#[cfg(test)]
mod tests {

    use super::*;
    use proptest::prelude::*;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    macro_rules! bisect_tests {
        ($($func:ident { $($name:ident: $value:expr,)*},)*) => {
            $(
                $(
                    #[test]
                    fn $name() {
                        let (a, x, expected) = $value;
                        assert_eq!(expected, $func(&a, &x))
                    }
                )*
            )*
        }
    }

    bisect_tests! {
        bisect_right {
            bisect_right_0: (vec![], 1, 0),
            bisect_right_1: (vec![1], 0, 0),
            bisect_right_2: (vec![1], 1, 1),
            bisect_right_3: (vec![1], 2, 1),
            bisect_right_4: (vec![1, 1], 0, 0),
            bisect_right_5: (vec![1, 1], 1, 2),
            bisect_right_6: (vec![1, 1], 2, 2),
            bisect_right_7: (vec![1, 1, 1], 0, 0),
            bisect_right_8: (vec![1, 1, 1], 1, 3),
            bisect_right_9: (vec![1, 1, 1], 2, 3),
            bisect_right_10: (vec![1, 1, 1, 1], 0, 0),
            bisect_right_11: (vec![1, 1, 1, 1], 1, 4),
            bisect_right_12: (vec![1, 1, 1, 1], 2, 4),
            bisect_right_13: (vec![1, 2], 0, 0),
            bisect_right_14: (vec![1, 2], 1, 1),
            bisect_right_15: (vec![1.0, 2.0], 1.5, 1),
            bisect_right_16: (vec![1, 2], 2, 2),
            bisect_right_17: (vec![1, 2], 3, 2),
            bisect_right_18: (vec![1, 1, 2, 2], 0, 0),
            bisect_right_19: (vec![1, 1, 2, 2], 1, 2),
            bisect_right_20: (vec![1.0, 1.0, 2.0, 2.0], 1.5, 2),
            bisect_right_21: (vec![1, 1, 2, 2], 2, 4),
            bisect_right_22: (vec![1, 1, 2, 2], 3, 4),
            bisect_right_23: (vec![1, 2, 3], 0, 0),
            bisect_right_24: (vec![1, 2, 3], 1, 1),
            bisect_right_25: (vec![1.0, 2.0, 3.0], 1.5, 1),
            bisect_right_26: (vec![1, 2, 3], 2, 2),
            bisect_right_27: (vec![1.0, 2.0, 3.0], 2.5, 2),
            bisect_right_28: (vec![1, 2, 3], 3, 3),
            bisect_right_29: (vec![1, 2, 3], 4, 3),
            bisect_right_30: (vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 0, 0),
            bisect_right_31: (vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 1, 1),
            bisect_right_32: (vec![1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 4.0], 1.5, 1),
            bisect_right_33: (vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 2, 3),
            bisect_right_34: (vec![1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 4.0], 2.5, 3),
            bisect_right_35: (vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 3, 6),
            bisect_right_36: (vec![1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 4.0], 3.0, 6),
            bisect_right_37: (vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 4, 10),
            bisect_right_38: (vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 5, 10),
        },

        bisect_left {
            bisect_left_0: (vec![], 1, 0),
            bisect_left_1: (vec![1], 0, 0),
            bisect_left_2: (vec![1], 1, 0),
            bisect_left_3: (vec![1], 2, 1),
            bisect_left_4: (vec![1, 1], 0, 0),
            bisect_left_5: (vec![1, 1], 1, 0),
            bisect_left_6: (vec![1, 1], 2, 2),
            bisect_left_7: (vec![1, 1, 1], 0, 0),
            bisect_left_8: (vec![1, 1, 1], 1, 0),
            bisect_left_9: (vec![1, 1, 1], 2, 3),
            bisect_left_10: (vec![1, 1, 1, 1], 0, 0),
            bisect_left_11: (vec![1, 1, 1, 1], 1, 0),
            bisect_left_12: (vec![1, 1, 1, 1], 2, 4),
            bisect_left_13: (vec![1, 2], 0, 0),
            bisect_left_14: (vec![1, 2], 1, 0),
            bisect_left_15: (vec![1.0, 2.0], 1.5, 1),
            bisect_left_16: (vec![1, 2], 2, 1),
            bisect_left_17: (vec![1, 2], 3, 2),
            bisect_left_18: (vec![1, 1, 2, 2], 0, 0),
            bisect_left_19: (vec![1, 1, 2, 2], 1, 0),
            bisect_left_20: (vec![1.0, 1.0, 2.0, 2.0], 1.5, 2),
            bisect_left_21: (vec![1, 1, 2, 2], 2, 2),
            bisect_left_22: (vec![1, 1, 2, 2], 3, 4),
            bisect_left_23: (vec![1, 2, 3], 0, 0),
            bisect_left_24: (vec![1, 2, 3], 1, 0),
            bisect_left_25: (vec![1.0, 2.0, 3.0], 1.5, 1),
            bisect_left_26: (vec![1, 2, 3], 2, 1),
            bisect_left_27: (vec![1.0, 2.0, 3.0], 2.5, 2),
            bisect_left_28: (vec![1, 2, 3], 3, 2),
            bisect_left_29: (vec![1, 2, 3], 4, 3),
            bisect_left_30: (vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 0, 0),
            bisect_left_31: (vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 1, 0),
            bisect_left_32: (vec![1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 4.0], 1.5, 1),
            bisect_left_33: (vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 2, 1),
            bisect_left_34: (vec![1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 4.0], 2.5, 3),
            bisect_left_35: (vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 3, 3),
            bisect_left_36: (vec![1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 4.0], 3.5, 6),
            bisect_left_37: (vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 4, 6),
            bisect_left_38: (vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 5, 10),
        },
    }

    proptest! {

        #[test]
        fn test_bisect_left(
            mut nums in prop::collection::vec(any::<u32>(), 0..500),
            num in any::<u32>()
        ) {
            nums.sort();

            let i = bisect_left(&nums, &num);

            assert!(nums[..i].iter().all(|&x| x < num));
            assert!(nums[i..].iter().all(|&x| x >= num));
        }

        #[test]
        fn test_bisect_right(
            mut nums in prop::collection::vec(any::<u32>(), 0..500),
            num in any::<u32>()
        ) {
            nums.sort();

            let i = bisect_right(&nums, &num);

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
