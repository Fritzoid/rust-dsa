use crate::algorithm::SortingAlgorithm;

pub struct QuickSort;
impl SortingAlgorithm for QuickSort {
    type Input = Vec<i32>;
    type Output = Vec<i32>;

    fn run(&self, mut input: Vec<i32>) -> Vec<i32> {
        input
    }
}