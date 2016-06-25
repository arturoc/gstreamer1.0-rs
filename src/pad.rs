use ffi::*;
use std::ptr;
use std::mem;
use caps::Caps;
use std::os::raw::c_void;

pub struct Pad{
    pad: *mut GstPad
}

impl Drop for Pad{
    fn drop(&mut self){
		unsafe{
			gst_object_unref(self.pad as *mut c_void);
		}
    }
}

#[derive(Debug)]
#[repr(isize)]
pub enum LinkReturn{
    WrongHierarchy = GST_PAD_LINK_WRONG_HIERARCHY as isize,
    WasLinked = GST_PAD_LINK_WAS_LINKED as isize,
    WrongDirection = GST_PAD_LINK_WRONG_DIRECTION as isize,
    NoFormat = GST_PAD_LINK_NOFORMAT as isize,
    NoSched = GST_PAD_LINK_NOSCHED as isize,
    Refused = GST_PAD_LINK_REFUSED as isize,
}

impl Pad{
    pub unsafe fn new(pad: *mut GstPad) -> Option<Pad>{
		if pad != ptr::null_mut(){
			Some( Pad{pad: pad} )
		}else{
			None
		}
    }

    pub fn link(&mut self, sink: &mut Pad) -> Result<(), LinkReturn>{
        unsafe{
            let ret = gst_pad_link(self.pad, sink.pad);
            if ret == GST_PAD_LINK_OK{
                Ok(())
            }else{
                Err(mem::transmute(ret as isize))
            }
        }
    }

    pub fn is_linked(&self) -> bool{
        unsafe{
            let pad: &mut GstPad = mem::transmute(self.pad);
            pad.peer != ptr::null_mut()
        }
    }

    pub fn query_caps(&self, filter: Option<Caps>) -> Option<Caps>{
        unsafe{
            let caps = gst_pad_query_caps(self.pad, filter.map(|mut caps| caps.gst_caps_mut()).unwrap_or(ptr::null_mut()));
            Caps::new(caps, true)
        }
    }
}

impl ::Reference for Pad{
    fn reference(&self) -> ::Ref<Pad>{
        unsafe{
            gst_object_ref(self.pad as *mut c_void);
			::Ref::from(Pad::new(self.pad).unwrap())
		}
    }
}
