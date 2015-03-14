use ffi::*;
use ::ElementT;
use std::mem;

pub struct AppSrc{
    appsrc: ::Element
}

unsafe impl Sync for AppSrc {}
unsafe impl Send for AppSrc {}

impl AppSrc{
    pub fn new(name: &str) -> Option<AppSrc>{
        let appsrc = ::Element::new("appsrc",name);
        match appsrc{
            Some(appsrc) => Some(AppSrc{appsrc: appsrc}),
            None => None
        }
    }
    
    pub fn new_from_element(element: ::Element) -> AppSrc{
        AppSrc{appsrc: element}
    }
    
    pub fn set_caps(&mut self, caps: &::Caps){
        unsafe{
            gst_app_src_set_caps(self.gst_appsrc_mut(), caps.gst_caps());
        }
    }
    
    pub fn caps(&self) -> Option<::Caps>{
        unsafe{
	        let gst_caps = gst_app_src_get_caps(mem::transmute(self.gst_appsrc()));
	        ::Caps::new(gst_caps,true)
	    }
    }
    
    pub fn latency(&self) -> (u64,u64){
        unsafe{
            let mut min: u64 = 0;
            let mut max: u64 = 0;
            gst_app_src_get_latency(mem::transmute(self.gst_appsrc()), &mut min, &mut max);
            (min,max)
        }
    }
    
    pub fn push_buffer(&mut self, buffer: ::Buffer) -> GstFlowReturn{
        unsafe{
            gst_app_src_push_buffer(self.gst_appsrc_mut(), buffer.transfer())
        }
    }
    
    pub fn end_of_stream(&mut self) -> GstFlowReturn{
        unsafe{
            gst_app_src_end_of_stream(self.gst_appsrc_mut())
        }
    }
    
    pub unsafe fn gst_appsrc(&self) -> *const GstAppSrc{
        self.appsrc.gst_element() as *const GstAppSrc
    }
    
    pub unsafe fn gst_appsrc_mut(&mut self) -> *mut GstAppSrc{
        self.appsrc.gst_element_mut() as *mut GstAppSrc
    }
}