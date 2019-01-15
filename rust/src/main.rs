use rand::distributions::{Distribution, Uniform};
use rand::distributions::uniform::SampleUniform;

mod sort;
use self::sort::SortAlgorithm;

fn create_array<T: SampleUniform>(size: usize, min: T, max: T) -> Vec<T> {
    let mut result: Vec<T> = Vec::with_capacity(size);

    let between = Uniform::new(min, max);
    let mut rng = rand::thread_rng();

    for _ in 0..size {
        result.push(between.sample(&mut rng));
    }

    result
}


fn test_for_sorting<T: PartialOrd>(data: &Vec<T>) -> bool {
    for n in 1..data.len() {
        if data[n] < data[n - 1] {
            return false
        }
    }

    return true
}


fn run_sort<T: PartialOrd + Clone>(mut data: &mut Vec<T>,
                                   algorithm: &SortAlgorithm<T>) {
    println!("Algorithm: {}", algorithm.get_name());

    algorithm.sort(&mut data);

    if test_for_sorting(&data) {
        println!("    Sorted");
    }
    else {
        println!("    Unsorted");
    }
}


fn main() {
    let size = 10000;
    let data = create_array(size, 0, 1000);
    println!("Data size: {}", size);

    let bubble_sort = sort::BubbleSort::new();
    run_sort(&mut data.clone(), &bubble_sort);
}