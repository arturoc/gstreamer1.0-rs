use ffi::*;
use caps::Caps;
use buffer::Buffer;
use videoframe::VideoFrame;
use std::mem;
use std::ptr;
use reference::Reference;

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
	pub unsafe fn new(sample: *mut GstSample, owned: bool) -> Option<Sample>{
	    if sample!=ptr::null_mut(){
		    if !owned{
		        gst_mini_object_ref(sample as *mut GstMiniObject);
		    }
			Some(Sample{sample: sample})
		}else{
		    None
		}
	}

	/// Get the buffer associated with sample or None when there is no buffer.
    pub fn buffer(&self) -> Option<Buffer>{
        unsafe{
        	let buffer = gst_sample_get_buffer(mem::transmute(self.gst_sample()));
        	if buffer != ptr::null_mut(){
	        	gst_mini_object_ref(buffer as *mut GstMiniObject);
	            Buffer::new(buffer,true)
	        }else{
	            None
	        }
        }
    }

	/// Get the caps associated with sample or None when there's no caps
    pub fn caps(&self) -> Option<Caps>{
		unsafe{
			let caps = gst_sample_get_caps(mem::transmute(self.gst_sample()));
        	if caps != ptr::null_mut(){
	            Caps::new(caps,false)
	        }else{
	            None
	        }
		}
	}

    /// Get the segment associated with sample
    pub fn segment(&self) -> GstSegment{
        unsafe{
            (*gst_sample_get_segment(mem::transmute(self.gst_sample())))
        }
    }

    /// Get a video frame from this sample if it contains one
    pub fn video_frame(&self) -> Option<VideoFrame>{
        let buffer = match self.buffer(){
            Some(buffer) => buffer,
            None => return None
        };

        let vi = match self.caps(){
            Some(caps) => match caps.video_info(){
                Some(vi) => vi,
                None => return None
            },
            None => return None
        };

        unsafe{ VideoFrame::new(vi, buffer) }
    }

    pub unsafe fn gst_sample(&self) -> *const GstSample{
		self.sample
	}

    pub unsafe fn gst_sample_mut(&mut self) -> *mut GstSample{
		self.sample
	}
}

impl ::Transfer<GstSample> for Sample{
    unsafe fn transfer(self) ->  *mut GstSample{
        let sample = self.sample;
		mem::forget(self);
        sample
    }
}

impl Reference for Sample{
    fn reference(&self) -> Sample{
        unsafe{
			Sample::new(self.sample, false).unwrap()
		}
    }
}
