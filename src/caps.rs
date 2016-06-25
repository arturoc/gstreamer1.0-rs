use ffi::*;
use std::os::raw::c_void;
use util::*;

use structure::Structure;
use reference::Reference;
use element::{Element, Property, FromProperty};

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
		let cdesc = CString::new(desc).unwrap();
	    unsafe{
	    	Caps::new(gst_caps_from_string(mem::transmute(cdesc.as_ptr())),true)
	    }
	}

	pub fn to_string(&self) -> &str{
		unsafe{
			from_c_str!(gst_caps_to_string(self.caps))
		}
	}

	pub fn video_info(&self) -> Option<::VideoInfo>{
		unsafe{
			let videoinfo = mem::zeroed();
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

	pub fn structure(&self, index: u32) -> Option<Structure>{
		unsafe{
			let structure = gst_caps_get_structure(self.caps, index);
			Structure::new_from_gst_structure(structure)
		}
	}
}


impl ::Transfer<GstCaps> for Caps{
    unsafe fn transfer(self) ->  *mut GstCaps{
        let caps = self.caps;
		mem::forget(self);
        caps
    }
}


impl Reference for Caps{
    fn reference(&self) -> Caps{
        unsafe{
			Caps::new(self.caps, false).unwrap()
		}
    }
}

impl<'a> Property for &'a Caps{
    type Target = *mut GstCaps;
    #[inline]
    fn set_to(&self, key: &str, e: &mut Element){
        let cname = CString::new(key).unwrap();
        unsafe{
            g_object_set(e.gst_element() as *mut c_void, cname.as_ptr(), self.gst_caps(), ptr::null::<gchar>());
        }
    }
}

impl Property for Caps{
    type Target = *mut GstCaps;
    #[inline]
    fn set_to(&self, key: &str, e: &mut Element){
        let cname = CString::new(key).unwrap();
        unsafe{
            g_object_set(e.gst_element() as *mut c_void, cname.as_ptr(), self.gst_caps(), ptr::null::<gchar>());
        }
    }
}

impl<'a> FromProperty for Caps{
    fn from_property(caps: *mut GstCaps) -> Caps{
        unsafe{
            Caps::new(caps, true).unwrap()
        }
    }
}
