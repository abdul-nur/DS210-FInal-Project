#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_distance() {
        //general examples + edge cases
        let first = vec![1, 2, 3];
        let second = vec![4, 5, 6];
        assert_eq!(distance(&first, &second), 3);

        let first = vec![1, 2, 3];
        let second = vec![2, 3, 4];
        assert_eq!(distance(&first, &second), 2);

        // tests empty vec cases
        let first = vec![];
        let second = vec![4, 5, 6];
        assert_eq!(distance(&first, &second), 0);

        // tests duplicates
        let first = vec![1, 2, 3, 3];
        let second = vec![3, 4, 5, 5];
        assert_eq!(distance(&first, &second), 3);

        // tests negative inputs
        let first = vec![-1, -2, -3];
        let second = vec![-4, -5, -6];
        assert_eq!(distance(&first, &second), 3);

        // tests bounds for super large inputs
        let first = vec![1_000_000, 2_000_000, 3_000_000];
        let second = vec![4_000_000, 5_000_000, 6_000_000];
        assert_eq!(distance(&first, &second), 3);
    }
}
