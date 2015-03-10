use ffi::*;
use std::mem;
use std::ptr;

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
	pub unsafe fn new(caps: *mut GstCaps, owned: bool) -> Option<Caps>{
	    if caps != ptr::null_mut(){
			if !owned {gst_mini_object_ref(caps as *mut GstMiniObject);}
			Some(Caps{caps: caps})
		}else{
		    None
		}
	}
	
	pub fn video_info(&self) -> GstVideoInfo{
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
