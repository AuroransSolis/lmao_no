use std::mem::ManuallyDrop;
use std::fmt::{self, Debug};

pub struct SLL<T: Copy> {
    length: usize,
    start_ptr: Option<*mut ManuallyDrop<SLLElem<T>>>
}

#[derive(Copy, Clone)]
pub struct SLLElem<T: Copy> {
    data: T,
    next_ptr: Option<*mut ManuallyDrop<SLLElem<T>>>
}

impl<T: Copy> SLL<T> {
    pub fn new() -> Self {
        SLL {
            length: 0,
            start_ptr: None
        }
    }

    pub fn get_start_ptr(&self) -> Option<*mut ManuallyDrop<SLLElem<T>>> {
        self.start_ptr
    }

    pub unsafe fn set_start_ptr(&mut self,
        new: Option<*mut ManuallyDrop<SLLElem<T>>>) {
        self.start_ptr = new;
    }

    pub unsafe fn pos_inc_len(&mut self) {
        self.length += 1;
    }

    pub unsafe fn neg_inc_len(&mut self) {
        self.length -= 1;
    }

    pub unsafe fn set_len(&mut self, new: usize) {
        self.length = new;
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn ptr_to(&self, ind: usize) -> Option<*mut ManuallyDrop<SLLElem<T>>> {
        assert!(ind < self.length);
        if ind == 0 {
            self.start_ptr
        } else {
            let mut ptr = self.start_ptr;
            let mut ct = 0;
            unsafe {
                while (*(ptr.unwrap())).next_ptr.is_some() && ct < ind {
                    ptr = (*(ptr.unwrap())).next_ptr;
                    ct += 1;
                }
                ptr
            }
        }
    }

    pub fn index(&self, ind: usize) -> &T {
        assert!(ind < self.length);
        if ind == 0 {
            unsafe { &((*(self.start_ptr.unwrap())).data) }
        } else {
            let mut ptr = self.start_ptr.unwrap();
            let mut ct = 0;
            unsafe {
                while (*ptr).next_ptr.is_some() && ct < ind {
                    ptr = (*ptr).next_ptr.unwrap();
                    ct += 1;
                }
                &(*ptr).data
            }
        }
    }

    pub fn index_mut(&mut self, ind: usize) -> &mut T {
        assert!(ind < self.length);
        if ind == 0 {
            unsafe { &mut (*(self.start_ptr.unwrap())).data }
        } else {
            let mut ptr = self.start_ptr;
            let mut ct = 0;
            unsafe {
                while (*(ptr.unwrap())).next_ptr.is_some() && ct < ind {
                    ptr = (*(ptr.unwrap())).next_ptr;
                    ct += 1;
                }
                &mut (*(ptr.unwrap())).data
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            None
        } else if self.length == 1 {
            let e = unsafe {
                let foo = *(self.start_ptr.unwrap());
                ManuallyDrop::into_inner(foo)
            };
            unsafe {
                ManuallyDrop::drop(&mut *(self.start_ptr.unwrap()));
            }
            self.length -= 1;
            Some(e.data)
        } else {
            let second_to_last = self.length - 2;
            unsafe {
                (*(self.ptr_to(second_to_last).unwrap())).next_ptr = None;
            }
            let e = unsafe {
                ManuallyDrop::into_inner(
                    *(self.ptr_to(self.length - 1).unwrap())
                )
            };
            unsafe {
                ManuallyDrop::drop(
                    &mut *(self.ptr_to(self.length - 1).unwrap())
                );
            }
            self.length -= 1;
            Some(e.data)
        }
    }

    pub fn remove(&mut self, ind: usize) -> Option<T> {
        assert!(ind < self.length);
        if self.length == 0 {
            None
        } else if self.length == 1 || ind ==  self.length - 1 {
            self.pop()
        } else if ind == 0 {
            let mut old_start = unsafe { *self.start_ptr.unwrap() };
            self.start_ptr = self.ptr_to(1);
            let e = ManuallyDrop::into_inner(old_start);
            unsafe {
                ManuallyDrop::drop(&mut old_start);
            }
            self.length -= 1;
            Some(e.data)
        } else {
            unsafe {
                (*(self.ptr_to(ind - 1).unwrap())).next_ptr =
                    self.ptr_to(ind + 1);
            }
            let mut to_remove = unsafe { *(self.ptr_to(ind).unwrap()) };
            let e = ManuallyDrop::into_inner(to_remove);
            unsafe {
                ManuallyDrop::drop(&mut to_remove);
            }
            self.length -= 1;
            Some(e.data)
        }
    }
}

impl<T: Copy + Debug> Debug for SLL<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("{:?}", self.index(0));
        for i in 1..self.length {
            s = format!("{}, {:?}", s, self.index(i));
        }
        write!(f, "{}", s)
    }
}

impl<T: Copy> SLLElem<T> {
    pub fn new(data: T) -> Self {
        SLLElem {
            data,
            next_ptr: None
        }
    }

    pub fn new_with_ptr(data: T, ptr: *mut ManuallyDrop<Self>) -> Self {
        SLLElem {
            data,
            next_ptr: Some(ptr)
        }
    }

    pub fn get_data(&self) -> T {
        self.data
    }

    pub fn set_data(&mut self, data: T) {
        self.data = data;
    }

    pub fn get_next_ptr(&self) -> Option<*mut ManuallyDrop<Self>> {
        self.next_ptr
    }

    pub unsafe fn set_next_ptr(self_ptr: *mut Self,
        ptr: Option<*mut ManuallyDrop<Self>>) {
        (*self_ptr).next_ptr = ptr;
    }
}

#[macro_export]
macro_rules! push {
    ($sll:ident, $val:expr) => {
        let mut sllelem = ManuallyDrop::new(SLLElem::new($val));
        if $sll.get_start_ptr().is_none() {
            unsafe { $sll.set_start_ptr(Some(&mut sllelem)) };
        } else {
            unsafe {
                SLLElem::set_next_ptr(
                    $sll.ptr_to($sll.len() - 1).unwrap() as *mut ManuallyDrop<_>
                        as *mut _,
                    Some(&mut sllelem)
                );
            }
        }
        unsafe { $sll.pos_inc_len() };
    }
}

#[macro_export]
macro_rules! insert {
    ($sll:ident, $ind:expr, $val:expr) => {
        assert!($ind <= $sll.len());
        let mut sllelem = ManuallyDrop::new(SLLElem::new($val));
        if $ind == $sll.len() {
            if $sll.get_start_ptr().is_none() {
                unsafe { $sll.set_start_ptr(Some(&mut sllelem)) };
            } else {
                unsafe {
                    SLLElem::set_next_ptr(
                        $sll.ptr_to($sll.len() - 1).unwrap()
                            as *mut ManuallyDrop<_> as *mut _,
                        Some(&mut sllelem)
                    );
                }
            }
        } else {
            let before_ptr = $sll.ptr_to($ind - 1).unwrap();
            let after_ptr = $sll.ptr_to($ind).unwrap();
            unsafe {
                SLLElem::set_next_ptr(
                    &mut sllelem as *mut ManuallyDrop<_> as *mut _,
                    Some(after_ptr)
                );
                SLLElem::set_next_ptr(
                    before_ptr as *mut ManuallyDrop<_> as *mut _,
                    Some(&mut sllelem)
                );
            };
        }
        unsafe { $sll.pos_inc_len() };
    }
}