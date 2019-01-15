pub trait SortAlgorithm<T: PartialOrd + Clone> {
    fn sort(&self, data: &mut Vec<T>);
    fn get_name(&self) -> &str;
}

pub struct BubbleSort;


impl BubbleSort {
    pub fn new() -> BubbleSort {
        BubbleSort {}
    }
}

impl<T: PartialOrd + Clone> SortAlgorithm<T> for BubbleSort {
    fn sort(&self, data: &mut Vec<T>) {
        let mut change = true;
        while change {
            change = false;

            for n in 1..data.len() {
                if data[n] < data[n - 1] {
                    let tmp = data[n].clone();
                    data[n] = data[n - 1].clone();
                    data[n - 1] = tmp;
                    change = true;
                }
            }
        }
    }

    fn get_name(&self) -> &str {
        "Bubble sort"
    }
}
