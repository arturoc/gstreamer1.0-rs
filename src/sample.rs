use ffi::*;
use caps::Caps;
use buffer::Buffer;
use std::mem;

unsafe impl Send for Sample {}

pub struct Sample{
	sample: *mut GstSample
}

impl Drop for Sample{
	fn drop(&mut self){
		unsafe{
			gst_mini_object_unref(self.gst_sample() as *mut GstMiniObject);
		}
	}
}

impl Sample{
	pub fn new(sample: *mut GstSample, owned: bool) -> Sample{
	    if !owned{
	        unsafe{
	        	gst_mini_object_ref(sample as *mut GstMiniObject);
	        }
	    }
		Sample{sample: sample}
	}

    pub fn buffer(&self) -> Buffer{
        unsafe{
        	let buffer = gst_sample_get_buffer(mem::transmute(self.gst_sample()));
        	gst_mini_object_ref(buffer as *mut GstMiniObject);
            Buffer::new(buffer,true)
        }
    }
	
    pub fn caps(&self) -> Caps{
		unsafe{
			let caps = gst_sample_get_caps(mem::transmute(self.gst_sample()));
            Caps::new(caps,false)
		}
	}
    
    pub fn segment(&self) -> GstSegment{
        unsafe{
            (*gst_sample_get_segment(mem::transmute(self.gst_sample())))
        }
    }
    
    pub unsafe fn gst_sample(&self) -> *const GstSample{
		self.sample
	}
    
    pub unsafe fn gst_sample_mut(&mut self) -> *mut GstSample{
		self.sample
	}
}
