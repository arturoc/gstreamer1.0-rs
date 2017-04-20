use gst_sys::*;
use std::mem;
use std::ptr;

use reference::Reference;

pub struct MiniObject{
    miniobject: *mut GstMiniObject,
}

impl Drop for MiniObject{
	fn drop(&mut self){
        unsafe{
			gst_mini_object_unref(self.miniobject);
		}
	}
}

impl MiniObject{
    pub unsafe fn new_from_gst_miniobject(miniobject: *mut GstMiniObject) -> Option<MiniObject>{
        if miniobject != ptr::null_mut(){
            Some(MiniObject{
                miniobject: miniobject
            })
        }else{
            None
        }
    }

    pub fn lock<F:FnMut(&mut MiniObject)>(&mut self, flags: &[GstLockFlags], mut f: F) -> bool{
        let flags = flags.iter().fold(0,|ret, flag| ret | *flag as u32);
        unsafe{
            if gst_mini_object_lock(self.miniobject, flags) != 0{
                f(self);
                gst_mini_object_unlock(self.miniobject, flags);
                true
            }else{
                false
            }
        }
    }

    pub fn make_writable(self) -> MiniObject{
        unsafe{
            MiniObject{
                miniobject: gst_mini_object_make_writable(self.miniobject)
            }
        }
    }

    pub unsafe fn gst_miniobject(&self) -> *const GstMiniObject{
        self.miniobject as *const GstMiniObject
    }

    pub unsafe fn gst_miniobject_mut(&mut self) -> *mut GstMiniObject{
        self.miniobject
    }
}

impl Clone for MiniObject{
	fn clone(&self) -> MiniObject{
		unsafe{
			MiniObject{ miniobject: gst_mini_object_copy(self.miniobject as *mut GstMiniObject) }
		}
	}
}

impl Reference for MiniObject{
    fn reference(&self) -> MiniObject{
        unsafe{
			MiniObject{ miniobject: gst_mini_object_ref(self.miniobject) }
		}
    }
}

impl ::Transfer<GstMiniObject> for MiniObject{
    unsafe fn transfer(self) ->  *mut GstMiniObject{
        let miniobject = self.miniobject;
		mem::forget(self);
        miniobject
    }
}
