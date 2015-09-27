extern crate rustc_serialize;

use rustc_serialize::{Decoder, Decodable};

use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::slice;

#[derive(Debug, Copy, Clone)]
pub struct _0;
#[derive(Debug, Copy, Clone)]
pub struct _1;

/// Nonnegative type-level integer, e.g., `((_1,_0),_1) = 0b101 = 5`.
pub trait Nat {
    fn reify() -> u64;
}
impl Nat for _0 { fn reify() -> u64 { 0 } }
impl Nat for _1 { fn reify() -> u64 { 1 } }
impl<N: Nat> Nat for (N, _0) {
    fn reify() -> u64 { N::reify() << 1 }
}
impl<N: Nat> Nat for (N, _1) {
    fn reify() -> u64 { (N::reify() << 1) | 1 }
}

/// Trait making GenericArray work
pub unsafe trait ArrayLength<T> : Nat {
	/// Associated type representing the array type for the number
	type ArrayType;
}

unsafe impl<T> ArrayLength<T> for _0 {
	type ArrayType = ();
}
unsafe impl<T> ArrayLength<T> for _1 {
	type ArrayType = T;
}

#[allow(dead_code)]
#[repr(C)]
pub struct GenericArrayImplEven<T, U> {
	parent1: U,
	parent2: U,
	_marker: PhantomData<T>
}

#[allow(dead_code)]
#[repr(C)]
pub struct GenericArrayImplOdd<T, U> {
	parent1: U,
	parent2: U,
	data: T
}

unsafe impl<T, N: ArrayLength<T>> ArrayLength<T> for (N, _0) {
	type ArrayType = GenericArrayImplEven<T, N::ArrayType>;
}

unsafe impl<T, N: ArrayLength<T>> ArrayLength<T> for (N, _1) {
	type ArrayType = GenericArrayImplOdd<T, N::ArrayType>;
}

#[allow(dead_code)]
#[derive(RustcEncodable)]
pub struct GenericArray<T, U: ArrayLength<T>> {
	data: U::ArrayType
}

impl<T, N> Deref for GenericArray<T, N> where N: ArrayLength<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe {
            slice::from_raw_parts(self as *const Self as *const T, N::reify() as usize)
        }
    }
}

impl<T, N> DerefMut for GenericArray<T, N> where N: ArrayLength<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut Self as *mut T, N::reify() as usize)
        }
    }
}

impl<T, N> GenericArray<T, N> where N: ArrayLength<T> {

	pub unsafe fn new() -> GenericArray<T, N> {
		mem::zeroed()
	}
	
	pub fn fill<F>(f: F) -> GenericArray<T, N> where F: Fn(usize) -> T {
        let mut res: GenericArray<T, N> = unsafe { mem::zeroed() };
        for i in 0..N::reify() as usize {
            let uninit = mem::replace(&mut res[i], f(i));
            mem::forget(uninit);
        }
        res
    }

}

impl<T, N> Decodable for GenericArray<T, N> where N: ArrayLength<T>, T: Decodable {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        let mut res: GenericArray<T, N> = unsafe { mem::zeroed() };
        for i in 0..N::reify() as usize {
            let item = try!( T::decode(d) );
            let uninit = mem::replace(&mut res[i], item);
            mem::forget(uninit);
        }
        Ok(res)
    }
}

#[cfg(test)]
mod test {
	use super::{_0, _1, GenericArray};

	type P97 = ((((((_1, _1), _0), _0), _0), _0), _1);

	#[test]
	fn test() {
		let mut list97 = [0; 97];
		for i in 0..97 {
			list97[i] = i as i32;
		}
	    let l : GenericArray<i32, P97> = GenericArray::new_list(&list97);
	    assert_eq!(l[0], 0);
	    assert_eq!(l[1], 1);
	    assert_eq!(l[32], 32);
	    assert_eq!(l[56], 56);
	}	
}
