use gobject::*;
use gst_sys::*;
use gst_video_sys::*;
use std::os::raw::{c_void, c_char};
use util::*;
use std::ops::{Deref, DerefMut};

use structure::Structure;
use reference::Reference;
use object::{Property, FromProperty};
use miniobject::MiniObject;
use object::Object;

#[derive(Clone)]
pub struct Caps{
	caps: MiniObject
}

impl Caps{
	pub unsafe fn new(caps: *mut GstCaps) -> Option<Caps>{
		MiniObject::new_from_gst_miniobject(caps as *mut GstMiniObject)
			.map(|miniobject| Caps{ caps: miniobject })
	}

	pub fn new_empty() -> Caps{
		unsafe{
			Caps::new(gst_caps_new_empty()).unwrap()
		}
	}

	pub fn new_empty_simple(media_type: &str) -> Caps{
		unsafe{
			let cmedia_type = CString::new(media_type).unwrap();
			Caps::new(gst_caps_new_empty_simple(cmedia_type.as_ptr())).unwrap()
		}
	}

	pub fn new_any() -> Caps{
		unsafe{
			Caps::new(gst_caps_new_any()).unwrap()
		}
	}

	pub fn is_writable(&self) -> bool{
		unsafe{
			gst_mini_object_is_writable(self.caps.gst_miniobject())!=0
		}
	}

	pub fn from_string(desc: &str) -> Option<Caps>{
		let cdesc = CString::new(desc).unwrap();
	    unsafe{
	    	Caps::new(gst_caps_from_string(mem::transmute(cdesc.as_ptr())))
	    }
	}

	pub fn to_string(&self) -> &str{
		unsafe{
			from_c_str!(gst_caps_to_string(self.gst_caps()))
		}
	}

	pub fn video_info(&self) -> Option<::VideoInfo>{
		unsafe{
			let videoinfo = mem::zeroed();
			if gst_video_info_from_caps (mem::transmute(&videoinfo), self.gst_caps()) == 1 {
				Some(videoinfo)
			}else{
			    None
			}
		}
	}

	pub unsafe fn gst_caps(&self) -> *const GstCaps{
		self.caps.gst_miniobject() as *const GstCaps
	}

	pub unsafe fn gst_caps_mut(&mut self) -> *mut GstCaps{
		self.caps.gst_miniobject_mut() as *mut GstCaps
	}

	pub fn structure(&self, index: u32) -> Option<Structure>{
		unsafe{
			let structure = gst_caps_get_structure(self.gst_caps(), index);
			Structure::new_from_gst_structure(structure)
		}
	}
}


impl ::Transfer<GstCaps> for Caps{
    unsafe fn transfer(self) ->  *mut GstCaps{
        self.caps.transfer() as *mut GstCaps
    }
}


impl Reference for Caps{
    fn reference(&self) -> Caps{
		Caps{
			caps: self.caps.reference()
		}
    }
}

impl<'a> Property for &'a Caps{
    type Target = *mut GstCaps;
    #[inline]
    fn set_to(&self, key: &str, e: &mut Object){
        let cname = CString::new(key).unwrap();
        unsafe{
            g_object_set(e.gst_object() as *mut c_void, cname.as_ptr(), self.gst_caps(), ptr::null::<c_char>());
        }
    }
}

impl Property for Caps{
    type Target = *mut GstCaps;
    #[inline]
    fn set_to(&self, key: &str, e: &mut Object){
        let cname = CString::new(key).unwrap();
        unsafe{
            g_object_set(e.gst_object() as *mut c_void, cname.as_ptr(), self.gst_caps(), ptr::null::<c_char>());
        }
    }
}

impl<'a> FromProperty for Caps{
    fn from_property(caps: *mut GstCaps) -> Caps{
        unsafe{
            Caps::new(caps).unwrap()
        }
    }
}


impl PartialEq for Caps{
    fn eq(&self, other: &Caps) -> bool{
        unsafe{
            gst_caps_is_equal(mem::transmute(self), mem::transmute(other)) != 0
        }
    }
}

impl Eq for Caps{}


impl AsRef<MiniObject> for Caps{
    fn as_ref(&self) -> &MiniObject{
        &self.caps
    }
}

impl AsMut<MiniObject> for Caps{
    fn as_mut(&mut self) -> &mut MiniObject{
        &mut self.caps
    }
}

impl From<Caps> for MiniObject{
    fn from(b: Caps) -> MiniObject{
        b.caps
    }
}

impl Deref for Caps{
    type Target = MiniObject;
    fn deref(&self) -> &MiniObject{
        &self.caps
    }
}

impl DerefMut for Caps{
    fn deref_mut(&mut self) -> &mut MiniObject{
        &mut self.caps
    }
}
