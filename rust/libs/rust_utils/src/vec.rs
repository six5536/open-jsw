pub trait SafeVecUpdate<T> {
    fn safe_update(&mut self, index: usize, value: T) -> bool;
}

impl<T> SafeVecUpdate<T> for Vec<T> {
    fn safe_update(&mut self, index: usize, value: T) -> bool {
        if let Some(elem) = self.get_mut(index) {
            *elem = value;
            true
        } else {
            false
        }
    }
}
