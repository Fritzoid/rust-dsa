pub trait SortingAlgorithm {
    type Input;
    type Output;

    fn run(&self, input: Self::Input) -> Self::Output;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ArraySorter;
    impl SortingAlgorithm for ArraySorter {
        type Input = [i32; 3];
        type Output = [i32; 3];

        fn run(&self, input: [i32; 3]) -> [i32; 3] {
            let mut output = input;
            output.sort();
            output
        }
    }

    struct StringSorter;
    impl SortingAlgorithm for StringSorter {
        type Input = String;
        type Output = String;

        fn run(&self, input: String) -> String {
            let mut output = input.chars().collect::<Vec<char>>();
            output.sort();
            output.into_iter().collect()
        }
    }

    struct VectorSorter;
    impl SortingAlgorithm for VectorSorter {
        type Input = Vec<i32>;
        type Output = Vec<i32>;

        fn run(&self, input: Vec<i32>) -> Vec<i32> {
            let mut output = input;
            output.sort();
            output
        }
    }

    #[test]
    fn test_array_sorter() {
        let sorter = ArraySorter;
        let input = [3, 2, 1];
        let output = sorter.run(input);
        assert_eq!(output, [1, 2, 3]);
    }

    #[test]
    fn test_string_sorter() {
        let sorter = StringSorter;
        let input = "cba".to_string();
        let output = sorter.run(input);
        assert_eq!(output, "abc");
    }

    #[test]
    fn test_vector_sorter() {
        let sorter = VectorSorter;
        let input = vec![3, 2, 1];
        let output = sorter.run(input);
        assert_eq!(output, vec![1, 2, 3]);
    }

}