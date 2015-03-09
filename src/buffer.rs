use ffi::*;

use std::mem;
use std::ptr;
use std::slice::from_raw_parts;
use libc::c_void;
use std::slice;

pub struct Buffer{
    buffer: *mut GstBuffer,
    mapinfo: GstMapInfo
}

impl Drop for Buffer{
    fn drop(&mut self){
        unsafe{
            gst_buffer_unmap(self.buffer,mem::transmute(&self.mapinfo));
        	gst_mini_object_unref(self.buffer as *mut GstMiniObject);
        }
    }
}

impl Buffer{
    pub unsafe fn new(buffer: *mut GstBuffer, owned: bool) -> Buffer{
		//TODO: check buffer is valid and return Result or Option
    	if !owned{
    	    gst_mini_object_ref(buffer as *mut GstMiniObject);
    	}
        let rin_buffer = Buffer{ buffer: buffer, mapinfo: gst_map_info_new()};
        gst_buffer_map(rin_buffer.buffer, mem::transmute(&rin_buffer.mapinfo), GST_MAP_READ);
        rin_buffer
    }

    pub fn size(&self) -> u64{
        self.mapinfo.size
    }

    pub fn data<'a,T>(&'a self) -> &'a [T]{
        unsafe{ from_raw_parts( mem::transmute(self.mapinfo.data), self.len::<T>() ) }
    }
    
    pub fn iter<'a,T>(&'a self) -> slice::Iter<'a,T>{
		self.data::<T>().iter()
	}
	
	pub fn len<T>(&self) -> usize{
		(self.size() / mem::size_of::<T>() as u64)  as usize
	}
    
    pub fn gst_buffer(&self) -> *mut GstBuffer{
        self.buffer
    }
}

unsafe fn gst_map_info_new() -> GstMapInfo{
    GstMapInfo{ memory: ptr::null_mut::<GstMemory>(),
                        flags: 0,
                        data: ptr::null_mut::<u8>(),
                        size: 0,
                        maxsize: 0,
                        user_data: [mem::transmute(ptr::null::<c_void>());4],
                        _gst_reserved: [mem::transmute(ptr::null::<c_void>());4] }
}
