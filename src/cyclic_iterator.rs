pub struct CyclicIterator<'a, T> {
    data: &'a [T],
    index: usize,
}

impl<'a, T> CyclicIterator<'a, T> {
    pub fn new(data: &'a [T]) -> CyclicIterator<'a, T> {
        CyclicIterator { data, index: 0 }
    }
}

impl<'a, T> Iterator for CyclicIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            return None;
        }
        let result = Some(&self.data[self.index]);
        self.index = (self.index + 1) % self.data.len();
        result
    }
}
