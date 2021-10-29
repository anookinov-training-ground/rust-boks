#![feature(dropck_eyepatch)]

use std::fmt::Debug;
use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct Boks<T> {
    // p: *mut T,
    p: NonNull<T>,
    _t: PhantomData<T>,
}

struct Deserializer<T> {
    _t: PhantomData<fn() -> T>, // covariance but no drop check
}

struct EmptyIterator<T> {
    _t: PhantomData<fn() -> T>, // covariance but no drop check
}

impl<T> Iterator for EmptyIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

// Deserializer<Oisann<&mut i32>>

// Cannot treat Boks<&'static str> as Boks<&'some_shorter_lifetime str>
// even though &'static str as &'some_shorter_lifetime str
// and can treat Box<&'static str> as Box<&'some_shorter_lifetime str>

unsafe impl<#[may_dangle] T> Drop for Boks<T> {
    fn drop(&mut self) {
        // let _: u8 = unsafe { std::ptr::read(self.p as *const u8) };

        // Safety: p was constructed from a Box in the first place, and has not been freed
        // otherwise since self still exists (otherwise, drop could not be called)
        unsafe { Box::from_raw(self.p.as_mut()) };
        // unsafe { std::ptr::drop_in_place(self.p) };
    }
}

impl<T> Boks<T> {
    pub fn ny(t: T) -> Self {
        Boks {
            // Safety: Box never creates a null pointer
            p: unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(t))) },
            _t: PhantomData,
        }
    }
}

impl<T> std::ops::Deref for Boks<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // Safety: is valid since it was constructed from a valid T, and turned into a pointer
        // through Box which creates aligned pointers, and hasn't been freed, since self is alive.
        unsafe { &*self.p.as_ref() }
    }
}

impl<T> std::ops::DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: is valid since it was constructed from a valid T, and turned into a pointer
        // through Box which creates aligned pointers, and hasn't been freed, since self is alive.
        // Also, since we have &mut self, no other mutable reference has been given out to p.
        unsafe { &mut *self.p.as_mut() }
    }
}

struct Oisann<T: Debug>(T);

unsafe impl<#[may_dangle] T: Debug> Drop for Oisann<T> {
    fn drop(&mut self) {
        // println!("{:?}", self.0);
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

    let mut z = 42;
    let b = Boks::ny(Oisann(&mut z));
    println!("{:?}", z);

    let s = String::from("hei");
    let mut box1 = Box::new(&*s);
    let box2: Box<&'static str> = Box::new("heisann");
    box1 = box2;

    let s = String::from("hei");
    let mut boks1 = Boks::ny(&*s);
    let boks2: Boks<&'static str> = Boks::ny("heisann");
    boks1 = boks2;
}
