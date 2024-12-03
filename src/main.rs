mod algorithm;
mod quicksort;
mod mergesort;

use algorithm::SortingAlgorithm;
use quicksort::QuickSort;
use mergesort::MergeSort;

fn main() {
    let sorting_algorithms: Vec<Box<dyn SortingAlgorithm<Input = Vec<i32>, Output = Vec<i32>>>> = vec![Box::new(QuickSort), Box::new(MergeSort)];
    sorting_algorithms.iter().for_each(|sorting_algorithm| {
        let input = vec![3, 2, 1];
        let output = sorting_algorithm.run(input);
        println!("{:?}", output);
    });
}
