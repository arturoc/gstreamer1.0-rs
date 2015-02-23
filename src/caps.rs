use ffi::*;
use std::mem;

pub struct Caps{
	caps: *mut GstCaps
}

impl Drop for Caps{
	fn drop(&mut self){
        unsafe{
			gst_mini_object_unref(self.caps as *mut GstMiniObject);
		}
	}
}

impl Caps{
	pub fn new(caps: *mut GstCaps, owned: bool) -> Caps{
		unsafe{
			if !owned {gst_mini_object_ref(caps as *mut GstMiniObject);}
		}
		Caps{caps: caps}
	}
	
	pub fn video_info(&self) -> Struct__GstVideoInfo{
		unsafe{
			let videoinfo = ::video_info_new();
			gst_video_info_from_caps (mem::transmute(&videoinfo), mem::transmute(self.caps));
			videoinfo
		}
	}
	
	pub fn gst_caps(&self) -> *mut GstCaps{
		self.caps
	}
}
