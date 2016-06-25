use ffi::*;
use caps::Caps;
use buffer::Buffer;
use videoframe::VideoFrame;
use std::mem;
use std::ptr;
use reference::Reference;
use miniobject::MiniObject;

unsafe impl Send for Sample {}

#[derive(Clone)]
pub struct Sample{
	sample: MiniObject
}

impl Sample{
	pub unsafe fn new(sample: *mut GstSample) -> Option<Sample>{
	    MiniObject::new_from_gst_miniobject(sample as *mut GstMiniObject)
			.map(|miniobject| Sample{ sample: miniobject })
	}

	/// Get the buffer associated with sample or None when there is no buffer.
    pub fn buffer(&self) -> Option<Buffer>{
        unsafe{
        	let buffer = gst_sample_get_buffer(mem::transmute(self.gst_sample()));
        	if buffer != ptr::null_mut(){
	            Buffer::new(gst_mini_object_ref(buffer as *mut GstMiniObject) as *mut GstBuffer)
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
	            Caps::new(gst_mini_object_ref(caps as *mut GstMiniObject) as *mut GstCaps)
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
		self.sample.gst_miniobject() as *const GstSample
	}

    pub unsafe fn gst_sample_mut(&mut self) -> *mut GstSample{
		self.sample.gst_miniobject_mut() as *mut GstSample
	}
}

impl ::Transfer<GstSample> for Sample{
    unsafe fn transfer(self) ->  *mut GstSample{
        self.sample.transfer() as *mut GstSample
    }
}

impl Reference for Sample{
    fn reference(&self) -> Sample{
        Sample{
			sample: self.sample.reference()
		}
    }
}
