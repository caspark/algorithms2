pub struct CircularSuffixArray {
    indexes: Vec<usize>,
}

impl CircularSuffixArray {
    pub fn new(input: &[u8]) -> CircularSuffixArray {
        unimplemented!(); //TODO actually implement building CSA
    }

    pub fn len(&self) -> usize {
        self.indexes.len()
    }

    pub fn index(&self, i: usize) -> usize {
        self.indexes[i]
    }
}

#[cfg(test)]
mod tests {
    use super::CircularSuffixArray;

    #[test]
    fn matches_behaviour_of_example_from_spec() {
        let sample_input = "ABRACADABRA!";
        let a = CircularSuffixArray::new(sample_input.as_ref());
        assert_eq!(a.len(), sample_input.len());
        let expected_indexes = vec![11, 10, 7, 0, 3, 5, 8, 1, 4, 6, 9, 2];
        for (i, &expected_index) in expected_indexes.iter().enumerate() {
            assert!(a.index(i) == expected_index, format!("Expected CSA to say index({}) is {}", i, expected_index));
        }
    }
}
