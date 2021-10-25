#![feature(dropck_eyepatch)]
pub struct Boks<T> {
    p: *mut T,
}

unsafe impl<#[may_dangle] T> Drop for Boks<T> {
    fn drop(&mut self) {
        // let _: u8 = unsafe { std::ptr::read(self.p as *const u8) };

        // Safety: p was constructed from a Box in the first place, and has not been freed
        // otherwise since self still exists (otherwise, drop could not be called)
        unsafe { Box::from_raw(self.p) };
        // unsafe { std::ptr::drop_in_place(self.p) };
    }
}

impl<T> Boks<T> {
    pub fn ny(t: T) -> Self {
        Boks {
            p: Box::into_raw(Box::new(t)),
        }
    }
}

impl<T> std::ops::Deref for Boks<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // Safety: is valid since it was constructed from a valid T, and turned into a pointer
        // through Box which creates aligned pointers, and hasn't been freed, since self is alive.
        unsafe { &*self.p }
    }
}

impl<T> std::ops::DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: is valid since it was constructed from a valid T, and turned into a pointer
        // through Box which creates aligned pointers, and hasn't been freed, since self is alive.
        // Also, since we have &mut self, no other mutable reference has been given out to p.
        unsafe { &mut *self.p }
    }
}

fn main() {
    let x = 42;
    let b = Boks::ny(x);
    println!("{:?}", *b);

    let mut y = 42;
    let b = Boks::ny(&mut y);
    // let b = Box::new(&mut y);
    // println!("{:?}", *b);
    println!("{:?}", y);
    // y = 43;
    // drop(b); // read from mut y
}
