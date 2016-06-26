use ffi::*;
use caps::Caps;
use reference::Reference;
use object::Object;

use std::ptr;
use std::mem;
use std::ops::{Deref, DerefMut};

pub struct Pad{
    pad: Object
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
		Object::new(pad as *mut GstObject).map(|obj| Pad{ pad: obj })
    }

    pub fn link(&mut self, sink: &mut Pad) -> Result<(), LinkReturn>{
        unsafe{
            let ret = gst_pad_link(self.gst_pad_mut(), sink.gst_pad_mut());
            if ret == GST_PAD_LINK_OK{
                Ok(())
            }else{
                Err(mem::transmute(ret as isize))
            }
        }
    }

    pub fn is_linked(&self) -> bool{
        unsafe{
            let pad: &mut GstPad = mem::transmute(self.gst_pad());
            pad.peer != ptr::null_mut()
        }
    }

    pub fn query_caps(&self, filter: Option<Caps>) -> Option<Caps>{
        unsafe{
            let caps = gst_pad_query_caps(self.gst_pad() as *mut GstPad, filter.map(|mut caps| caps.gst_caps_mut()).unwrap_or(ptr::null_mut()));
            Caps::new(caps)
        }
    }

    pub unsafe fn gst_pad(&self) -> *const GstPad{
        self.pad.gst_object() as *const GstPad
    }

    pub unsafe fn gst_pad_mut(&mut self) -> *mut GstPad{
        self.pad.gst_object_mut() as *mut GstPad
    }
}

impl ::Transfer<GstPad> for Pad{
    unsafe fn transfer(self) -> *mut GstPad{
        self.pad.transfer() as *mut GstPad
    }
}

impl Reference for Pad{
    fn reference(&self) -> Pad{
        Pad{ pad: self.pad.reference() }
    }
}

impl AsRef<Object> for Pad{
    fn as_ref(&self) -> &Object{
        &self.pad
    }
}

impl AsMut<Object> for Pad{
    fn as_mut(&mut self) -> &mut Object{
        &mut self.pad
    }
}

impl From<Pad> for Object{
    fn from(b: Pad) -> Object{
        b.pad
    }
}

impl Deref for Pad{
    type Target = Object;
    fn deref(&self) -> &Object{
        &self.pad
    }
}

impl DerefMut for Pad{
    fn deref_mut(&mut self) -> &mut Object{
        &mut self.pad
    }
}
