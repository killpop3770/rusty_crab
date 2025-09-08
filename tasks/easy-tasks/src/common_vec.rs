use std::{
    alloc::{Layout, alloc, dealloc},
    usize,
};

pub struct CommonVec<T> {
    length: usize,
    capacity: usize,
    pointer: *mut T,
}

const CAPACITY_INCREASED_FACTOR: usize = 2;

impl<T> CommonVec<T> {
    pub fn new() -> CommonVec<T> {
        return CommonVec {
            length: 0,
            capacity: 0,
            pointer: std::ptr::null_mut(),
        };
    }

    pub fn new_with_capacity(capacity_size: usize) -> CommonVec<T> {
        return CommonVec {
            length: 0,
            capacity: capacity_size,
            pointer: std::ptr::null_mut(),
        };
    }

    pub fn length(&self) -> usize {
        return self.length;
    }

    pub fn capacity(&self) -> usize {
        return self.capacity;
    }

    pub fn push(&mut self, value: T) {
        if self.is_outgrown() {
            self.reallocate_memory();
        }

        unsafe {
            let end_pointer = self.pointer.add(self.length); // like 0..(length-1) + 1
            end_pointer.write(value);
        }

        self.increase_length();
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if self.check_bounds(index) {
            unsafe {
                let target_pointer = self.pointer.add(index); // 0 1 2 3 <- find target_index => 3 * size_of(T) == *mut T 
                return Some(&*target_pointer);
            }
        } else {
            return None;
        }
    }

    pub fn pop(&mut self) -> Option<&T> {
        match self.length {
            0 => None,
            other => unsafe {
                let target_pointer = self.pointer.add(other - 1); // get last element
                self.decrease_length();
                return Some(&*target_pointer);
            },
        }
    }

    fn check_bounds(&self, index: usize) -> bool {
        return index < self.length;
    }

    fn decrease_length(&mut self) {
        self.length -= 1;
    }

    fn increase_length(&mut self) {
        self.length += 1;
    }

    fn is_outgrown(&self) -> bool {
        return self.length == self.capacity;
    }

    fn reallocate_memory(&mut self) {
        let new_capacity: usize = match self.capacity {
            0 => CAPACITY_INCREASED_FACTOR,
            other => other * CAPACITY_INCREASED_FACTOR,
        };

        let new_layout = Layout::array::<T>(new_capacity).unwrap(); // concept
        let new_pointer = unsafe { alloc(new_layout) as *mut T }; // physical release

        if self.length > 0 {
            unsafe {
                std::ptr::copy(self.pointer, new_pointer, self.length);
            }
        }

        if self.capacity > 0 {
            let old_layout = Layout::array::<T>(self.capacity).unwrap();
            unsafe {
                dealloc(self.pointer as *mut u8, old_layout);
            }
        }

        self.capacity = new_capacity;
        self.pointer = new_pointer;
    }
}

impl<T> Drop for CommonVec<T> {
    fn drop(&mut self) {
        let layout = Layout::array::<T>(self.capacity).unwrap();
        unsafe {
            dealloc(self.pointer as *mut u8, layout);
        }
    }
}

#[cfg(test)]
mod test_common_vec {

    use crate::common_vec::CommonVec;

    #[test]
    fn test_new() {
        let common_vec: CommonVec<i32> = CommonVec::new();
        assert_eq!(common_vec.length(), 0);
        assert_eq!(common_vec.capacity(), 0);

        let common_vec: CommonVec<i32> = CommonVec::new_with_capacity(10);
        assert_eq!(common_vec.capacity(), 10);
    }

    #[test]
    fn test_push_and_get() {
        let target = 15;
        let mut common_vec: CommonVec<i32> = CommonVec::new();
        // common_vec.push(target);
        // unsafe {
        //     let a = common_vec.pointer.add(1) as *mut i32;
        //     println!("{:?}", a);
        //     println!("{:?}", *a);
        // }

        for i in 0..target {
            common_vec.push(i);
        }

        for i in 0..target {
            println!("{:?}", common_vec.get(i as usize));
        }

        assert_eq!(common_vec.length(), target as usize);
        assert_eq!(common_vec.get(42), None);
    }

    #[test]
    fn test_pop() {
        // kek, like killpop
        let target = 15;
        let mut common_vec: CommonVec<i32> = CommonVec::new();
        common_vec.push(target);
        assert_eq!(common_vec.length(), 1);

        let result = common_vec.pop();
        assert_eq!(result.unwrap(), &target);
        assert_eq!(common_vec.get(0), None)
    }
}
