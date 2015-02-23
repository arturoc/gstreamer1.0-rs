use ffi::*;
use caps::Caps;
use buffer::Buffer;
use std::ptr::Unique;
use std::mem;

pub struct Sample{
	sample: Unique<GstSample>,
	owned: bool
}

impl Drop for Sample{
	fn drop(&mut self){
		unsafe{
			if self.owned{
				gst_mini_object_unref(self.gst_sample() as *mut GstMiniObject);
			}
		}
	}
}

impl Sample{
	pub fn new(sample: *mut GstSample, owned: bool) -> Sample{
		unsafe{
			Sample{sample: Unique::new(sample), owned: owned}
		}
	}

    pub fn buffer(&self) -> Buffer{
        unsafe{
            Buffer::new(gst_sample_get_buffer(mem::transmute(self.gst_sample())))
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
		self.sample.get()
	}
    
    pub unsafe fn gst_sample_mut(&mut self) -> *mut GstSample{
		self.sample.get_mut()
	}
}
