use ffi::*;

use std::mem;
use std::ops::{Deref, DerefMut};

use sample::Sample;
use element::Element;
use caps::Caps;

unsafe impl Sync for AppSinkPoll {}
unsafe impl Send for AppSinkPoll {}

pub struct AppSinkPoll{
    appsink: Element,
}

impl AppSinkPoll{
    pub fn new(name: &str) -> Option<AppSinkPoll>{
        let appsink = Element::new("appsink",name);
        match appsink{
            Some(a) => {
                Some(AppSinkPoll{ appsink: a })
            },

            None => None
        }
    }

    pub fn new_from_element(element: Element) -> AppSinkPoll{
        AppSinkPoll{ appsink: element }
    }

	pub fn pull_sample(&mut self) -> Option<Sample>{
		unsafe{
			Sample::new(gst_app_sink_pull_sample(self.gst_appsink_mut()))
		}
	}

	pub fn pull_preroll(&mut self) -> Option<Sample>{
		unsafe{
			Sample::new(gst_app_sink_pull_preroll(self.gst_appsink_mut()))
		}
	}

    pub unsafe fn gst_appsink(&self) -> *const GstAppSink{
        self.appsink.gst_element() as *const GstAppSink
    }

    pub unsafe fn gst_appsink_mut(&mut self) -> *mut GstAppSink{
        self.appsink.gst_element() as *mut GstAppSink
    }

    pub fn set_caps(&mut self, caps: Caps){
		unsafe{
			gst_app_sink_set_caps(self.gst_appsink_mut(), caps.gst_caps() as *const GstCaps);
		}
	}

	pub fn get_caps(&self) -> Option<Caps>{
		unsafe{
			let caps = gst_app_sink_get_caps(mem::transmute(self.gst_appsink()));
			Caps::new(caps)
		}
	}

	pub fn is_eos(&self) -> bool{
		unsafe{
			gst_app_sink_is_eos(mem::transmute(self.gst_appsink())) == 1
		}
	}

	pub fn set_emit_signals(&mut self, emit: bool){
		unsafe{
			gst_app_sink_set_emit_signals(self.gst_appsink_mut(), emit as gboolean);
		}
	}

    pub fn get_emit_signals(&self) -> bool{
		unsafe{
			gst_app_sink_get_emit_signals(mem::transmute(self.gst_appsink())) == 1
		}
	}

	pub fn set_max_buffers(&mut self, max_buffers: u32){
		unsafe{
			gst_app_sink_set_max_buffers(self.gst_appsink_mut(), max_buffers);
		}
	}

	pub fn max_buffers(&self) -> u32{
		unsafe{
			gst_app_sink_get_max_buffers(mem::transmute(self.gst_appsink()))
		}
	}

	pub fn set_drop(&mut self, drop: bool){
		unsafe{
			gst_app_sink_set_drop(self.gst_appsink_mut(), drop as gboolean);
		}
	}

	pub fn get_drop(&self) -> bool{
		unsafe{
			gst_app_sink_get_drop(mem::transmute(self.gst_appsink())) == 1
		}
	}
}


impl AsRef<Element> for AppSinkPoll{
    fn as_ref(&self) -> &Element{
        &self.appsink
    }
}

impl AsMut<Element> for AppSinkPoll{
    fn as_mut(&mut self) -> &mut Element{
        &mut self.appsink
    }
}

impl From<AppSinkPoll> for Element{
	fn from(a: AppSinkPoll) -> Element{
		a.appsink
	}
}

impl Deref for AppSinkPoll{
	type Target = Element;
    fn deref(&self) -> &Element{
        &self.appsink
    }
}

impl DerefMut for AppSinkPoll{
    fn deref_mut(&mut self) -> &mut Element{
        &mut self.appsink
    }
}

impl ::Transfer for AppSinkPoll{
    unsafe fn transfer(self) -> *mut GstElement{
        self.appsink.transfer()
    }
}
