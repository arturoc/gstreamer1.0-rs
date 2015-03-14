use ffi::*;
use std::mem;
use std::ptr;
use std::ffi::CString;

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
	
	pub fn from_string(desc: &str) -> Option<Caps>{
	    unsafe{
	    	Caps::new(gst_caps_from_string(to_c_str!(desc)),true)
	    }
	}
	
	pub fn video_info(&self) -> Option<::VideoInfo>{
		unsafe{
			let videoinfo = ::VideoInfo::new();
			if gst_video_info_from_caps (mem::transmute(&videoinfo), mem::transmute(self.caps)) == 1 {
				Some(videoinfo)
			}else{
			    None
			}
		}
	}
	
	pub unsafe fn gst_caps(&self) -> *const GstCaps{
		self.caps
	}
	
	pub unsafe fn gst_caps_mut(&mut self) -> *mut GstCaps{
		self.caps
	}
}
