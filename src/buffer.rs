use ffi::*;

use std::mem;
use std::ptr;
use std::fmt::{Debug, Formatter, Error};

pub struct Buffer{
    buffer: *mut GstBuffer
}

impl Drop for Buffer{
    fn drop(&mut self){
        unsafe{
       		gst_mini_object_unref(self.buffer as *mut GstMiniObject);
        }
    }
}

macro_rules! gst_buffer_flag {
    ($getter:ident, $setter:ident, $flag:path) => (
        pub fn $getter(&self) -> bool {
            let flags = unsafe { (*self.gst_buffer()).mini_object.flags };
            flags & $flag != 0
        }

        pub fn $setter(&mut self, value: bool) {
            if !value {
                /* Clear */
                unsafe {
                    (*self.gst_buffer_mut()).mini_object.flags &= !$flag;
                }
            } else {
                /* Set */
                unsafe {
                    (*self.gst_buffer_mut()).mini_object.flags |= $flag;
                }
            }
        }
    )
}

impl Buffer{
    pub unsafe fn new(buffer: *mut GstBuffer, owned: bool) -> Option<Buffer>{
		if buffer != ptr::null_mut(){
	    	if !owned{
        		gst_mini_object_ref(buffer as *mut GstMiniObject);
        	}
	        let buff = Buffer{ buffer: buffer };
	        Some(buff)
	    }else{
	        None
	    }
    }

    pub fn map_read<'a,F:FnMut(&::MapInfo)->U,U>(&'a self, mut f: F ) -> Result<U,()>{
        unsafe{
	        let mut mapinfo = ::MapInfo::new();
	        if gst_buffer_map(self.buffer, &mut mapinfo, GST_MAP_READ) != 0{
	        	let ret = f(&mapinfo);
        		gst_buffer_unmap(self.buffer, &mut mapinfo);
        		Ok(ret)
        	}else{
        	    Err(())
        	}
	    }
    }

    pub fn map_write<'a,F:FnMut(&mut ::MapInfo)->U,U>(&'a mut self, mut f: F ) -> Result<U,()>{
        unsafe{
	        let mut mapinfo = ::MapInfo::new();
	        if gst_buffer_map(self.buffer, &mut mapinfo, GST_MAP_WRITE) != 0{
	        	let ret = f(&mut mapinfo);
        		gst_buffer_unmap(self.buffer, &mut mapinfo);
        		Ok(ret)
        	}else{
        	    Err(())
        	}
	    }
    }

    pub fn map<'a,F:FnMut(&mut ::MapInfo)->U,U>(&'a mut self, flags: ::Map, mut f: F ) -> Result<U,()>{
        unsafe{
	        let mut mapinfo = ::MapInfo::new();
	        if gst_buffer_map(self.buffer, &mut mapinfo, flags as u32) != 0{
	        	let ret = f(&mut mapinfo);
        		gst_buffer_unmap(self.buffer, &mut mapinfo);
        		Ok(ret)
        	}else{
        	    Err(())
        	}
	    }
    }

    pub fn size(&self) -> u64{
        unsafe{ gst_buffer_get_size(self.buffer) }
    }

	pub fn len<T>(&self) -> usize{
		(self.size() / mem::size_of::<T>() as u64)  as usize
	}

    pub fn gst_buffer(&self) -> *const GstBuffer{
        self.buffer
    }

    pub fn gst_buffer_mut(&mut self) -> *mut GstBuffer{
        self.buffer
    }

    pub fn flags(&self) -> guint {
        unsafe { (*self.gst_buffer()).mini_object.flags }
    }

    gst_buffer_flag!(is_live, set_live, GST_BUFFER_FLAG_LIVE);
    gst_buffer_flag!(is_decode_only, set_decode_only, GST_BUFFER_FLAG_DECODE_ONLY);
    gst_buffer_flag!(is_discont, set_discont, GST_BUFFER_FLAG_DISCONT);
    gst_buffer_flag!(is_resync, set_resync, GST_BUFFER_FLAG_RESYNC);
    gst_buffer_flag!(is_corrupted, set_corrupted, GST_BUFFER_FLAG_CORRUPTED);
    gst_buffer_flag!(is_marker, set_marker, GST_BUFFER_FLAG_MARKER);
    gst_buffer_flag!(is_header, set_header, GST_BUFFER_FLAG_HEADER);
    gst_buffer_flag!(is_gap, set_gap, GST_BUFFER_FLAG_GAP);
    gst_buffer_flag!(is_droppable, set_droppable, GST_BUFFER_FLAG_DROPPABLE);
    gst_buffer_flag!(is_delta_unit, set_delta_unit, GST_BUFFER_FLAG_DELTA_UNIT);
    gst_buffer_flag!(is_tag_memory, set_tag_memory, GST_BUFFER_FLAG_TAG_MEMORY);
}

impl ::Transfer<GstBuffer> for Buffer{
    unsafe fn transfer(self) ->  *mut GstBuffer{
        let buffer = self.buffer;
		mem::forget(self);
        buffer
    }
}

impl ::Reference for Buffer{
    fn reference(&self) -> Buffer{
        unsafe{
            Buffer::new(self.buffer, false).unwrap()
        }
    }
}

macro_rules! fmt_buffer_flag {
    ($buffer:ident, $fmt:ident, $getter:ident) => (
        if $buffer.$getter() {
            try!($fmt.write_fmt(format_args!(", {}", stringify!($getter))));
        }
    )
}

impl Debug for Buffer {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        try!(fmt.write_fmt(format_args!("GstBuffer<{} bytes", self.size())));
        fmt_buffer_flag!(self, fmt, is_live);
        fmt_buffer_flag!(self, fmt, is_decode_only);
        fmt_buffer_flag!(self, fmt, is_discont);
        fmt_buffer_flag!(self, fmt, is_resync);
        fmt_buffer_flag!(self, fmt, is_corrupted);
        fmt_buffer_flag!(self, fmt, is_marker);
        fmt_buffer_flag!(self, fmt, is_header);
        fmt_buffer_flag!(self, fmt, is_gap);
        fmt_buffer_flag!(self, fmt, is_droppable);
        fmt_buffer_flag!(self, fmt, is_delta_unit);
        fmt_buffer_flag!(self, fmt, is_tag_memory);
        try!(fmt.write_str(">"));
        Ok(())
    }
}
