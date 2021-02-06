#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Status<T> {
    pub state: T,
}