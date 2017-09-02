use gst_sys::*;
use reference::Reference;
use miniobject::MiniObject;

use std::mem;
use std::fmt::{Debug, Formatter, Error};
use std::ops::{Deref, DerefMut};
use std::os::raw::c_uint;

#[derive(Clone)]
pub struct Buffer{
    buffer: MiniObject
}

macro_rules! gst_buffer_flag {
    ($getter:ident, $setter:ident, $flag:path) => (
        pub fn $getter(&self) -> bool {
            let flags = unsafe { (*self.gst_buffer()).mini_object.flags };
            GstBufferFlags::from_bits_truncate(flags).contains($flag)
        }

        pub fn $setter(&mut self, value: bool) {
            if !value {
                /* Clear */
                unsafe {
                    (*self.gst_buffer_mut()).mini_object.flags &= !$flag.bits();
                }
            } else {
                /* Set */
                unsafe {
                    (*self.gst_buffer_mut()).mini_object.flags |= $flag.bits();
                }
            }
        }
    )
}

impl Buffer{
    pub unsafe fn new(buffer: *mut GstBuffer) -> Option<Buffer>{
		MiniObject::new_from_gst_miniobject(buffer as *mut GstMiniObject)
            .map(|miniobject| Buffer{ buffer: miniobject })
    }

    pub fn map_read<'a,F:FnMut(&GstMapInfo)->U,U>(&'a self, mut f: F ) -> Result<U,()>{
        unsafe{
	        let mut mapinfo = mem::zeroed();
	        if gst_buffer_map(self.gst_buffer() as *mut GstBuffer, &mut mapinfo, GST_MAP_READ) != 0{
	        	let ret = f(&mapinfo);
        		gst_buffer_unmap(self.gst_buffer() as *mut GstBuffer, &mut mapinfo);
        		Ok(ret)
        	}else{
        	    Err(())
        	}
	    }
    }

    pub fn map_write<'a,F:FnMut(&mut GstMapInfo)->U,U>(&'a mut self, mut f: F ) -> Result<U,()>{
        unsafe{
	        let mut mapinfo = mem::zeroed();
	        if gst_buffer_map(self.gst_buffer_mut(), &mut mapinfo, GST_MAP_WRITE) != 0{
	        	let ret = f(&mut mapinfo);
        		gst_buffer_unmap(self.gst_buffer_mut(), &mut mapinfo);
        		Ok(ret)
        	}else{
        	    Err(())
        	}
	    }
    }

    pub fn map<'a,F:FnMut(&mut GstMapInfo)->U,U>(&'a mut self, flags: ::GstMapFlags, mut f: F ) -> Result<U,()>{
        unsafe{
	        let mut mapinfo = mem::zeroed();
	        if gst_buffer_map(self.gst_buffer_mut(), &mut mapinfo, flags) != 0{
	        	let ret = f(&mut mapinfo);
        		gst_buffer_unmap(self.gst_buffer_mut(), &mut mapinfo);
        		Ok(ret)
        	}else{
        	    Err(())
        	}
	    }
    }

    pub fn size(&self) -> u64{
        unsafe{ gst_buffer_get_size(self.gst_buffer() as *mut GstBuffer) as u64 }
    }

	pub fn len<T>(&self) -> u64{
		(self.size() / mem::size_of::<T>() as u64) as u64
	}

    pub unsafe fn gst_buffer(&self) -> *const GstBuffer{
        self.buffer.gst_miniobject() as *const GstBuffer
    }

    pub unsafe fn gst_buffer_mut(&mut self) -> *mut GstBuffer{
        self.buffer.gst_miniobject_mut() as *mut GstBuffer
    }

    pub fn flags(&self) -> c_uint {
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
        self.buffer.transfer() as *mut GstBuffer
    }
}

impl Reference for Buffer{
    fn reference(&self) -> Buffer{
        Buffer{ buffer: self.buffer.reference() }
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



impl AsRef<MiniObject> for Buffer{
    fn as_ref(&self) -> &MiniObject{
        &self.buffer
    }
}

impl AsMut<MiniObject> for Buffer{
    fn as_mut(&mut self) -> &mut MiniObject{
        &mut self.buffer
    }
}

impl From<Buffer> for MiniObject{
    fn from(b: Buffer) -> MiniObject{
        b.buffer
    }
}

impl Deref for Buffer{
    type Target = MiniObject;
    fn deref(&self) -> &MiniObject{
        &self.buffer
    }
}

impl DerefMut for Buffer{
    fn deref_mut(&mut self) -> &mut MiniObject{
        &mut self.buffer
    }
}
