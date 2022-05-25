use std::marker::PhantomData;

#[derive(Debug)]
pub struct UniqueIdMaker<T: From<usize>> {
    next_id: usize,
    _phatnom: PhantomData<T>,
}

impl<T: From<usize>> Iterator for UniqueIdMaker<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_id == usize::MAX {
            None
        } else {
            let next_id = self.next_id;
            self.next_id += 1;
            Some(T::from(next_id))
        }
    }
}

impl<T: From<usize>> UniqueIdMaker<T> {
    pub fn starting_at(start: usize) -> Self {
        Self {
            next_id: start,
            _phatnom: PhantomData,
        }
    }

    pub fn make_new_id(&mut self) -> T {
        self.next().unwrap()
    }
}
