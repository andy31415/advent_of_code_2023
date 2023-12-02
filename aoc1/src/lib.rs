struct Mapping<'a> {
    prefixes: &'a [&'a str],
    value: i32,
}

static NAME_MAP: &[Mapping] = &[
    Mapping {
        prefixes: &["0", "zero"],
        value: 0,
    },
    Mapping {
        prefixes: &["1", "one"],
        value: 1,
    },
    Mapping {
        prefixes: &["2", "two"],
        value: 2,
    },
    Mapping {
        prefixes: &["3", "three"],
        value: 3,
    },
    Mapping {
        prefixes: &["4", "four"],
        value: 4,
    },
    Mapping {
        prefixes: &["5", "five"],
        value: 5,
    },
    Mapping {
        prefixes: &["6", "six"],
        value: 6,
    },
    Mapping {
        prefixes: &["7", "seven"],
        value: 7,
    },
    Mapping {
        prefixes: &["8", "eight"],
        value: 8,
    },
    Mapping {
        prefixes: &["9", "nine"],
        value: 9,
    },
];

pub struct DigitIterator<'a> {
    data: &'a str,
}

impl<'a> DigitIterator<'a> {
    pub fn new(data: &'a str) -> Self {
        DigitIterator { data }
    }

    /// Iterate digits within a string
    ///     
    /// Example:
    ///
    /// ```
    /// use aoc1::DigitIterator;
    /// use itertools::assert_equal;
    ///
    /// assert_equal(DigitIterator::new("1abc2").iterate_digits(), [1, 2]);
    /// assert_equal(DigitIterator::new("pqr3stu8vwx").iterate_digits(), [3, 8]);
    /// assert_equal(DigitIterator::new("a1b2c3d4e5f").iterate_digits(), [1,2,3,4,5]);
    /// assert_equal(DigitIterator::new("treb7ucet").iterate_digits(), [7]);
    ///
    /// // Digits spelled out also work
    /// assert_equal(DigitIterator::new("two1nine").iterate_digits(), [2, 1, 9]);
    /// assert_equal(DigitIterator::new("eightwothree").iterate_digits(), [8, 2, 3]);
    /// assert_equal(DigitIterator::new("abcone2threexyz").iterate_digits(), [1, 2, 3]);
    /// assert_equal(DigitIterator::new("xtwone3four").iterate_digits(), [2, 1, 3, 4]);
    /// assert_equal(DigitIterator::new("4nineeightseven2").iterate_digits(), [4, 9, 8, 7, 2]);
    /// assert_equal(DigitIterator::new("zoneight234").iterate_digits(), [1, 8, 2, 3, 4]);
    /// assert_equal(DigitIterator::new("7pqrstsixteen").iterate_digits(), [7, 6]);
    ///
    /// ```
    pub fn iterate_digits(self) -> impl Iterator<Item = i32> + 'a {
        self.data
            .char_indices()
            .map(|index| &self.data[index.0..])
            .filter_map(|tail| {
                for &Mapping { prefixes, value } in NAME_MAP {
                    if prefixes.iter().any(|p| tail.starts_with(p)) {
                        return Some(value);
                    }
                }
                None
            })
    }
}

/// Grab the first and last number from a list of numbers.
/// If list of numbers has only one element, first and last will be the
/// same value.
///     
/// Example:
///
/// ```
/// use aoc1::first_and_last;
///
/// assert_eq!(first_and_last([1, 2, 3]), Some((1, 3)));
/// assert_eq!(first_and_last([1, 3, 4, 2]), Some((1, 2)));
/// assert_eq!(first_and_last([10]), Some((10, 10)));
/// assert_eq!(first_and_last([]), None);
/// ```
pub fn first_and_last(i: impl IntoIterator<Item = i32>) -> Option<(i32, i32)> {
    let mut iter = i.into_iter();

    iter.next()
        .map(|first| (first, iter.last().unwrap_or(first)))
}

#[cfg(test)]
mod tests {
    use crate::{first_and_last, DigitIterator};

    #[test]
    fn test_mapping() {
        assert_eq!(
            first_and_last(
                DigitIterator::new("eight9fhstbssrplmdlncmmqqnklb39ninej").iterate_digits()
            ),
            Some((8, 9))
        );
    }
}
