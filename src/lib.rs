mod plus_one_66;
mod summary_ranges_228;
mod two_sum_1;
mod valid_parentheses_20;
#[allow(dead_code)]
struct Solution;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
