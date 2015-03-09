/* automatically generated by rust-bindgen */
#![crate_type = "lib"]
#![crate_name = "gst"]
#![feature(int_uint,libc,core,alloc,unique)]


#[macro_use] extern crate log;
extern crate libc;

pub use self::appsink::AppSink;
pub use self::sample::Sample;
pub use self::caps::Caps;
pub use self::buffer::Buffer;
pub use self::element::Element;
pub use self::element::ElementT;
pub use self::bus::Bus;
pub use self::bin::Bin;
pub use self::bin::BinT;
pub use self::pipeline::Pipeline;
pub use self::pipeline::PipelineT;
pub use self::playbin::PlayBin;
pub use self::message::Message;
pub use self::mainloop::MainLoop;
pub use self::error::Error;
pub use self::error::Result;

pub use ffi::*;
use std::ptr;
use libc::c_void;
use std::mem;
use std::ffi::CString;
use std::str;
use std::ffi::CStr;

#[macro_use] mod util;
pub mod ffi;
pub mod appsink;
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
	unsafe{
		if gst_uri_is_valid(to_c_str!(filename))==1{
			return Ok(filename.to_string())
		}
		let err: *mut GError = ptr::null_mut();
		let c_uri = gst_filename_to_uri(to_c_str!(filename),mem::transmute(&err));
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
	unsafe{
		if gst_uri_is_valid(to_c_str!(uri))==1{
			Ok(from_c_str!(mem::transmute(gst_uri_get_protocol(to_c_str!(uri)))).to_string())
		}else{
			Err(Error::new(0,0,"not a valid URI"))
		}
	}
}

pub fn video_info_new() -> Struct__GstVideoInfo{
    let colorimetry = GstVideoColorimetry {
                            range: 0,
                            matrix: 0,
                            transfer: 0,
                            primaries: 0,
                      };
                      
    let videoinfo = Struct__GstVideoInfo{
        finfo: ptr::null::<GstVideoFormatInfo>(),
        interlace_mode: 0,
        flags: 0,
        width: 0,
        height: 0,
        size: 0,
        views: 0,
        chroma_site: 0,
        colorimetry: colorimetry,
        par_n: 0,
        par_d: 0,
        fps_n: 0,
        fps_d: 0,
        offset: [0;4],
        stride: [0;4],
        _gst_reserved: [ptr::null_mut::<c_void>();4],
    };
    
    videoinfo
}

fn map_info_new() -> GstMapInfo{
    GstMapInfo{ memory: ptr::null_mut::<GstMemory>(),
                        flags: 0,
                        data: ptr::null_mut::<u8>(),
                        size: 0,
                        maxsize: 0,
                        user_data: [ptr::null_mut::<c_void>();4],
                        _gst_reserved: [ptr::null_mut::<c_void>();4] }
}

pub fn video_frame_new() -> Struct__GstVideoFrame{
	Struct__GstVideoFrame{
		  info: video_info_new(),
		  flags: GST_VIDEO_FRAME_FLAG_NONE,
		  buffer: ptr::null_mut::<GstBuffer>(),
		  meta: ptr::null_mut::<c_void>(),
		  id: 0,
		  data: [ptr::null_mut::<c_void>();4],
		  map: [map_info_new();4],
		  _gst_reserved: [ptr::null_mut::<c_void>();4],
	}
}

