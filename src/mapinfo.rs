use gst_sys::*;
use std::mem;
use std::slice::{self,from_raw_parts,from_raw_parts_mut};

pub struct MapInfo{
    pub mapinfo: GstMapInfo
}

impl MapInfo{
    #[inline]
    pub fn size(&self) -> usize{
        self.mapinfo.size
    }

    #[inline]
    pub fn data<'a,T>(&self) -> &'a [T]{
        unsafe{ from_raw_parts( mem::transmute(self.mapinfo.data), self.len::<T>() ) }
    }

    #[inline]
    pub fn data_mut<'a,T>(&mut self) -> &'a mut [T]{
        unsafe{ from_raw_parts_mut( mem::transmute(self.mapinfo.data), self.len::<T>() ) }
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
		(self.size() / mem::size_of::<T>())
	}
}
