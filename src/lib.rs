#![crate_type = "lib"]
#![crate_name = "gst"]

pub use self::appsink::AppSink;
pub use self::appsrc::AppSrc;
pub use self::sample::Sample;
pub use self::caps::Caps;
pub use self::buffer::Buffer;
pub use self::mapinfo::MapInfo;
pub use self::mapinfo::Map;
pub use self::element::Element;
pub use self::bus::Bus;
pub use self::bin::Bin;
pub use self::pipeline::Pipeline;
pub use self::playbin::PlayBin;
pub use self::message::Message;
pub use self::mainloop::MainLoop;
pub use self::error::Error;
pub use self::error::Result;
pub use self::videoframe::VideoFrame;
pub use self::videoframe::VideoPlane;
pub use self::videoframe::VideoComponent;
pub use self::videoinfo::VideoInfo;
pub use self::buffer_pool::BufferPool;
pub use self::pad::Pad;
pub use self::structure::Structure;
pub use self::iterator::Iter;

use self::reference::Reference;

pub use ffi::*;
use std::ptr;
use std::mem;
use std::ffi::CString;
use std::str;
use std::ffi::CStr;
use std::ops::{Deref, DerefMut};
use std::convert::{From, AsRef, AsMut};

#[macro_use] mod util;
pub mod ffi;

/// Easy way for applications to extract samples from a pipeline.
pub mod appsink;

/// Easy way for applications to inject buffers into a pipeline.
pub mod appsrc;
mod sample;
mod caps;
mod buffer;
mod element;
pub mod bus;
mod bin;
mod pipeline;
mod playbin;
mod message;
pub mod mainloop;
mod error;
mod videoframe;
mod videoinfo;
mod mapinfo;
mod buffer_pool;
mod pad;
mod structure;
mod iterator;
mod reference;
#[cfg(target_os="linux")]
mod link_linux;
#[cfg(target_os="macos")]
mod link_osx;
#[cfg(target_os="windows")]
mod link_windows;

pub fn init(){
    unsafe{
        gst_init(ptr::null::<i32>() as *mut i32, ptr::null_mut::<i8>() as *mut *mut *mut i8);
    }
}

pub fn filename_to_uri(filename: &str) -> Result<String>{
    let cfilename = CString::new(filename).unwrap();
    unsafe{
        if gst_uri_is_valid(cfilename.as_ptr())==1{
            return Ok(filename.to_string())
        }
        let err: *mut GError = ptr::null_mut();
        let c_uri = gst_filename_to_uri(cfilename.as_ptr(), mem::transmute(&err));
        if err != ptr::null_mut(){
            Err(Error::new(0, 0, from_c_str!(mem::transmute((*err).message))))
        }else{
            let uri = from_c_str!(mem::transmute(c_uri)).to_string();
            g_free(mem::transmute(c_uri));
            Ok(uri)
        }
    }
}

pub fn uri_get_protocol(uri: &str) -> Result<String>{
    let curi = CString::new(uri).unwrap();
    unsafe{
        if gst_uri_is_valid(curi.as_ptr())==1{
            Ok(from_c_str!(mem::transmute(gst_uri_get_protocol(curi.as_ptr()))).to_string())
        }else{
            Err(Error::new(0,0,"not a valid URI"))
        }
    }
}

pub trait Transfer<PtrType=GstElement>{
    /// Consumes the current object and transfers ownership of the raw pointer
    /// Used to transfer ownership to ffi functions, should be used when an ffi
    /// function expects full transfer of an object to avoid the original object
    /// to be unreferenced in the process
    unsafe fn transfer(self) -> *mut PtrType;
}

pub trait FromGValue{
    fn from_gvalue(value: &GValue) -> Option<Self> where Self:Sized;
}


pub struct Ref<T>{
    value: T
}

impl<T:Reference> Ref<T>{
    pub fn new(t: &T) -> Ref<T>{
        Ref{ value: t.reference() }
    }
}

impl<T> Deref for Ref<T>{
    type Target = T;
    fn deref(&self) -> &T{
        &self.value
    }
}

impl<T> DerefMut for Ref<T>{
    fn deref_mut(&mut self) -> &mut T{
        &mut self.value
    }
}

impl<T:Reference> From<T> for Ref<T>{
    fn from(t: T) -> Ref<T>{
        Ref{ value: t }
    }
}

impl<T> AsRef<T> for Ref<T>{
    fn as_ref(&self) -> &T{
        &self.value
    }
}

impl<T> AsMut<T> for Ref<T>{
    fn as_mut(&mut self) -> &mut T{
        &mut self.value
    }
}
