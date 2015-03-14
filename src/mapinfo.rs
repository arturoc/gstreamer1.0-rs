use ffi::*;
use std::mem;
use std::ptr;
use libc::c_void;
use std::slice::{self,from_raw_parts,from_raw_parts_mut};

pub type MapInfo = GstMapInfo;

impl MapInfo{
    pub unsafe fn new() -> MapInfo{
	    MapInfo{ memory: ptr::null_mut::<GstMemory>(),
	                        flags: 0,
	                        data: ptr::null_mut::<u8>(),
	                        size: 0,
	                        maxsize: 0,
	                        user_data: [mem::transmute(ptr::null::<c_void>());4],
	                        _gst_reserved: [mem::transmute(ptr::null::<c_void>());4] }
	}
    
    #[inline]
    pub fn size(&self) -> u64{
        self.size
    }

    #[inline]
    pub fn data<'a,T>(&self) -> &'a [T]{
        unsafe{ from_raw_parts( mem::transmute(self.data), self.len::<T>() ) }
    }

    #[inline]
    pub fn data_mut<'a,T>(&mut self) -> &'a mut [T]{
        unsafe{ from_raw_parts_mut( mem::transmute(self.data), self.len::<T>() ) }
    }
    
    #[inline]
    pub fn iter<'a,T>(&'a self) -> slice::Iter<'a,T>{
		self.data::<T>().iter()
	}
    
    #[inline]
    pub fn iter_mut<'a,T>(&'a mut self) -> slice::IterMut<'a,T>{
		self.data_mut::<T>().iter_mut()
	}
	
    #[inline]
	pub fn len<T>(&self) -> usize{
		(self.size() / mem::size_of::<T>() as u64)  as usize
	}
}

#[derive(Copy)]
pub enum Map{
	Read = GST_MAP_READ as isize,
	Write = GST_MAP_WRITE as isize
}
