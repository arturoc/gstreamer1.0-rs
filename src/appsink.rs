use glib::*;
use gst_sys::*;
use gst_app_sys::*;
use std::ptr;
use std::mem;
use std::sync::mpsc::{Sender,Receiver,TryRecvError,RecvError,SendError,channel};
use std::ops::{Deref, DerefMut};

use sample::Sample;
use element::Element;
use caps::Caps;

pub enum Message{
	NewSample(Sample),
	NewPreroll(Sample),
	Eos
}

impl Message{
    pub fn is_eos(&self) -> bool{
        match self{
            &Message::Eos => true,
            _   => false
        }
    }

    pub fn is_preroll(&self) -> bool{
        match self{
            &Message::NewPreroll(..) => true,
            _              => false
        }
    }

    pub fn is_sample(&self) -> bool{
        match self{
            &Message::NewSample(..)  => true,
            _              => false
        }
    }
}

unsafe impl Sync for AppSink {}
unsafe impl Send for AppSink {}

#[allow(dead_code)] // we need to keep the samples_sender around
pub struct AppSink{
    appsink: Element,
    samples_receiver: Receiver<Message>,
    samples_sender: Box<Sender<Message>>
}

impl AppSink{
    pub fn new(name: &str) -> Option<AppSink>{
        let (sender,receiver) = channel();
        let sender = Box::new(sender);
        let appsink = Element::new("appsink",name);
        unsafe{
            match appsink{
                Some(a) => {
                    let mut gst_callbacks = GstAppSinkCallbacks{
                                eos: Some(on_eos_from_source),
                                new_preroll: Some(on_new_preroll_from_source),
                                new_sample: Some(on_new_sample_from_source),
                                _gst_reserved: [ptr::null_mut(); 4]
                    };
                    gst_app_sink_set_callbacks(a.gst_element() as *mut GstAppSink, &mut gst_callbacks, mem::transmute(&*sender), None);
                    Some(AppSink{ appsink: a, samples_receiver: receiver, samples_sender: sender })
                },

                None => None
            }
        }
    }

    pub fn new_from_element(element: Element) -> AppSink{
        let (sender,receiver) = channel();
        let sender = Box::new(sender);
        unsafe{
            let mut gst_callbacks = GstAppSinkCallbacks{
                        eos: Some(on_eos_from_source),
                        new_preroll: Some(on_new_preroll_from_source),
                        new_sample: Some(on_new_sample_from_source),
                        _gst_reserved: [ptr::null_mut(); 4]
            };
            gst_app_sink_set_callbacks(element.gst_element() as *mut GstAppSink, &mut gst_callbacks, mem::transmute(&*sender), None);
        }
        AppSink{ appsink: element, samples_receiver: receiver, samples_sender: sender }
    }

    pub fn recv(&self) -> Result<Message,RecvError>{
        self.samples_receiver.recv()
    }

    pub fn try_recv(&self) -> Result<Message,TryRecvError>{
        self.samples_receiver.try_recv()
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

extern "C" fn on_new_sample_from_source (elt: *mut GstAppSink, data: gpointer ) -> GstFlowReturn{
    unsafe{
		let sender = data as *mut Sender<Message>;
        let sample = gst_app_sink_pull_sample (elt);
        match Sample::new(sample){
            Some(sample) => {
		        match (*sender).send(Message::NewSample(sample)){
					Ok(()) => GST_FLOW_OK,
					Err(SendError(_msg)) => GST_FLOW_EOS
				}
		    }
            None => GST_FLOW_EOS
        }
    }
}

extern "C" fn on_new_preroll_from_source (elt: *mut GstAppSink, data: gpointer) -> GstFlowReturn{
    unsafe{
		let sender = data as *mut Sender<Message>;
        let sample = gst_app_sink_pull_preroll (elt);
        match Sample::new(sample){
            Some(sample) => {
		        match (*sender).send(Message::NewPreroll(sample)){
					Ok(()) => GST_FLOW_OK,
					Err(SendError(_msg)) => GST_FLOW_EOS
				}
		    }
            None => GST_FLOW_EOS
        }
    }
}

extern "C" fn on_eos_from_source (_elt: *mut GstAppSink, data: gpointer){
    unsafe{
		let sender = data as *mut Sender<Message>;
        (*sender).send(Message::Eos).unwrap();
    }
}


impl AsRef<Element> for AppSink{
    fn as_ref(&self) -> &Element{
        &self.appsink
    }
}

impl AsMut<Element> for AppSink{
    fn as_mut(&mut self) -> &mut Element{
        &mut self.appsink
    }
}

impl From<AppSink> for Element{
	fn from(a: AppSink) -> Element{
		a.appsink
	}
}

impl Deref for AppSink{
	type Target = Element;
    fn deref(&self) -> &Element{
        &self.appsink
    }
}

impl DerefMut for AppSink{
    fn deref_mut(&mut self) -> &mut Element{
        &mut self.appsink
    }
}

impl ::Transfer for AppSink{
    unsafe fn transfer(self) -> *mut GstElement{
        self.appsink.transfer()
    }
}
