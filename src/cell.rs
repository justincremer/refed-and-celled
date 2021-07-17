use std::cell::UnsafeCell;

#[derive(Debug)]
pub struct Cell<T> {
    inner: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub fn new(inner: T) -> Self {
        Cell {
            inner: UnsafeCell::new(inner),
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFE: Since Unsafecell implements !Sync, we know no other thread is accessing this data
        // We are additionally returning fresh data, thus no mutable refernece conflicts can occur
        unsafe { *self.inner.get() }
    }

    pub fn set(&self, val: T) {
        // SAFE: Since Unsafecell implements !Sync, we know no other thread is accessing this data
        unsafe {
            *self.inner.get() = val;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Cell;

    #[test]
    fn works() {
        let cell: Cell<i32> = Cell::new(69);
        let inner = cell.get();
        cell.set(10);
        assert_ne!(cell.get(), inner);
    }
}
