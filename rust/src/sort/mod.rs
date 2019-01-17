pub mod bubble;
pub mod quick;

pub trait SortAlgorithm<T: PartialOrd + Clone> {
    fn sort(&self, data: &mut Vec<T>);
    fn get_name(&self) -> &str;
}
