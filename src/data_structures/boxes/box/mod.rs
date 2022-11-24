use std::ptr::NonNull;

pub struct Box<T> {
    ptr: NonNull<T>,
}

impl<T> Box<T> {
    pub fn new(t: &mut T) -> Self {
        let val: NonNull<T> = if std::mem::size_of::<T>() == 0 {
            NonNull::dangling()
        } else {
            NonNull::new(t).unwrap()
        };

        Self { ptr: val }
    }

    pub fn read(&self) -> T {
        unsafe { std::ptr::read(self.ptr.as_ptr()) }
    }

    pub fn write(&self, val: T) {
        unsafe { std::ptr::write(self.ptr.as_ptr(), val) }
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structures::boxes::r#box::Box;

    #[test]
    fn new() {
        let mut val = 1;
        let b = Box::new(&mut val);
        assert_eq!(val, b.read())
    }

    #[test]
    fn mutate_locally() {
        let mut val = 1;
        let b = Box::new(&mut val);

        // modify the local variable
        val = 10;

        assert_eq!(val, b.read())
    }

    #[test]
    fn mutate_via_pointer() {
        let mut val: i32 = 1;
        let b = Box::new(&mut val);

        // modify the memory location pointed to by "b"
        let new_val = 10;
        b.write(new_val);

        // verify the values have been mutated
        assert_eq!(new_val, b.read());
        assert_eq!(val, new_val);

        // verify the values have different memory addresses
        let val_pointer = &val as *const i32;
        let new_val_pointer = &new_val as *const i32;
        assert_ne!(val_pointer, new_val_pointer);
    }

    #[test]
    fn zero_sized_type() {
        let mut val = ();
        let b = Box::new(&mut val);

        let box_value_size = std::mem::size_of_val(&b);
        let box_size = std::mem::size_of::<Box<()>>();
        assert_eq!(box_size, box_value_size);

        let value_size = std::mem::size_of_val(&b.read());
        assert_eq!(0, value_size);
    }
}
