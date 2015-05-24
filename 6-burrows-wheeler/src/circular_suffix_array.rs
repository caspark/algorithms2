pub struct CircularSuffixArray(Vec<usize>);

pub fn create(input: &[u8]) -> CircularSuffixArray {
    let mut indexes = (0..input.len()).collect::<Vec<usize>>();
    three_way_circular_suffix_qsort(input, indexes.as_mut(), 0, input.len() - 1, 0);
    CircularSuffixArray(indexes)
}

fn three_way_circular_suffix_qsort(input: &[u8], output_order: &mut [usize], lo: usize, hi: usize, curr_char_idx: usize) {
    if hi > lo { // recursion base case is when this condition fails
        debug_assert!(curr_char_idx < input.len(), format!("{}th char does not exist in input of len {}", curr_char_idx, input.len()));
        let curr_char_of_suffix = |suffix_idx: usize| {
            debug_assert!(suffix_idx < input.len(), format!("{}th suffix does not exist for input of len {}", curr_char_idx, input.len()));
            *input.get((suffix_idx + curr_char_idx) % input.len())
                .expect(format!("{}th suffix's {}th char should exist in input of len {}", suffix_idx, curr_char_idx, input.len()).as_ref())
        };
        let pivot_char = curr_char_of_suffix(output_order[lo]);
        let mut eq_start_idx = lo; // index of start of "equal group"
        let mut eq_finsh_idx = hi; // index of end of "equal group"
        let mut curr_str_idx = lo + 1; // index of the suffix which we're currently considering
        while curr_str_idx <= eq_finsh_idx {
            let curr_char: u8 = curr_char_of_suffix(output_order[curr_str_idx]);
            if curr_char < pivot_char {
                output_order.swap(eq_start_idx, curr_str_idx);
                eq_start_idx += 1;
                curr_str_idx += 1;
            } else if curr_char > pivot_char {
                output_order.swap(curr_str_idx, eq_finsh_idx);
                eq_finsh_idx -= 1;
            } else {
                curr_str_idx += 1;
            }
        }
        // sort less than group
        three_way_circular_suffix_qsort(input, output_order, lo, eq_start_idx - 1, curr_char_idx);
        // sort equal group (looking at 1 more character of each string)
        three_way_circular_suffix_qsort(input, output_order, eq_start_idx, eq_finsh_idx, curr_char_idx + 1);
        // sort greater than group
        three_way_circular_suffix_qsort(input, output_order, eq_finsh_idx + 1, hi, curr_char_idx);
    }
}

#[cfg(test)]
mod tests {
    use super::{create, CircularSuffixArray};

    #[test]
    fn matches_behaviour_of_example_from_spec() {
        let sample_input = "ABRACADABRA!";
        let CircularSuffixArray(result) = create(sample_input.as_ref());
        assert_eq!(result.len(), sample_input.len());
        let expected_indexes = vec![11, 10, 7, 0, 3, 5, 8, 1, 4, 6, 9, 2];
        for (i, &expected_index) in expected_indexes.iter().enumerate() {
            assert!(result[i] == expected_index, format!("Expected CSA to say index({}) is {}", i, expected_index));
        }
    }
}
