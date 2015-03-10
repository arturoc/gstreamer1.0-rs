use ffi::*;

use std::ptr;
use std::mem;
use libc::c_void;
use std::sync::mpsc::{Sender,Receiver,TryRecvError,RecvError,SendError,channel};


use sample::Sample;
use element::Element;
use caps::Caps;
use element::ElementT;
use bus::Bus;

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

impl Drop for AppSink{
	fn drop(&mut self){
	}
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
                                eos: Some(mem::transmute(on_eos_from_source)),
                                new_preroll: Some(mem::transmute(on_new_preroll_from_source)),
                                new_sample: Some(mem::transmute(on_new_sample_from_source)),
                                _gst_reserved: [mem::transmute(ptr::null::<c_void>());4]
                    };
                    gst_app_sink_set_callbacks(a.gst_element() as *mut GstAppSink, &mut gst_callbacks, mem::transmute(&*sender), mem::transmute(ptr::null::<c_void>()));
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
                        eos: Some(mem::transmute(on_eos_from_source)),
                        new_preroll: Some(mem::transmute(on_new_preroll_from_source)),
                        new_sample: Some(mem::transmute(on_new_sample_from_source)),
                        _gst_reserved: [mem::transmute(ptr::null::<c_void>());4]
            };
            gst_app_sink_set_callbacks(element.gst_element() as *mut GstAppSink, &mut gst_callbacks, mem::transmute(&*sender), mem::transmute(ptr::null::<c_void>()));
        }
        AppSink{ appsink: element, samples_receiver: receiver, samples_sender: sender }
    }
    
    pub fn recv(&self) -> Result<Message,RecvError>{
        self.samples_receiver.recv()
    }
    
    pub fn try_recv(&self) -> Result<Message,TryRecvError>{
        self.samples_receiver.try_recv()
    }
    
    pub unsafe fn gst_appsink(&self) -> *mut GstAppSink{
        self.appsink.gst_element() as *mut GstAppSink
    }    
    
    pub fn set_caps(&self, caps: Caps){
		unsafe{
			gst_app_sink_set_caps(self.gst_appsink(), caps.gst_caps() as *const GstCaps);
		}
	}
	
	pub fn get_caps(&self) -> Option<Caps>{
		unsafe{
			let caps = gst_app_sink_get_caps(self.gst_appsink());
			Caps::new(caps, true)
		}
	}
	
	pub fn is_eos(&self) -> bool{
		unsafe{
			gst_app_sink_is_eos(self.gst_appsink()) == 1
		}
	}
	
	pub fn set_emit_signals(&self, emit: bool){
		unsafe{
			gst_app_sink_set_emit_signals(self.gst_appsink(), emit as gboolean);
		}
	}
    
    pub fn get_emit_signals(&self) -> bool{
		unsafe{
			gst_app_sink_get_emit_signals(self.gst_appsink()) == 1
		}
	}
	
	pub fn set_max_buffers(&self, max_buffers: u32){
		unsafe{
			gst_app_sink_set_max_buffers(self.gst_appsink(), max_buffers);
		}
	}
	
	pub fn max_buffers(&self) -> u32{
		unsafe{
			gst_app_sink_get_max_buffers(self.gst_appsink())
		}
	}
	
	pub fn set_drop(&self, drop: bool){
		unsafe{
			gst_app_sink_set_drop(self.gst_appsink(), drop as gboolean);
		}
	}
	
	pub fn get_drop(&self) -> bool{
		unsafe{
			gst_app_sink_get_drop(self.gst_appsink()) == 1
		}
	}
	
	pub fn set<T>(&self, name: &str, value: T){
        self.appsink.set(name,value);
    }
}

extern "C" fn on_new_sample_from_source (elt: *mut GstAppSink, data: gpointer ) -> GstFlowReturn{
    unsafe{
		let sender = data as *mut Sender<Message>;
        let sample = gst_app_sink_pull_sample (elt);
        match Sample::new(sample,true){
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
        match Sample::new(sample,true){
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


impl ElementT for AppSink{
    
    fn link(&mut self, dst: &mut ElementT) -> bool{
        self.appsink.link(dst)
    }
    
    fn unlink(&mut self, dst: &mut ElementT){
        self.appsink.unlink(dst);
    }
    
    fn bus(&self) -> Option<Bus>{
        self.appsink.bus()
    }
    
    fn name(&self) -> String{
        self.appsink.name()
    }
    
    fn set_name(&mut self, name: &str){
        self.appsink.set_name(name);
    }
    
    fn set_state(&mut self, state: GstState) -> GstStateChangeReturn{
        self.appsink.set_state(state)
    }
    
    fn get_state(&self, timeout: GstClockTime) -> (GstState, GstState, GstStateChangeReturn){
        self.appsink.get_state(timeout)
    }
    
    unsafe fn send_event(&mut self, event: *mut GstEvent) -> bool{
        self.appsink.send_event(event)
    }
    
    fn seek_simple(&mut self, format: GstFormat, flags: GstSeekFlags, pos: i64) -> bool{
        self.appsink.seek_simple(format, flags, pos)
    }
    
    fn seek(&mut self, rate: f64, format: GstFormat, flags: GstSeekFlags, start_type: GstSeekType, start: i64, stop_type: GstSeekType, stop: i64) -> bool{
        self.appsink.seek(rate, format, flags, start_type, start, stop_type, stop)
    }
    
    fn seek_async(&mut self, rate: f64, format: GstFormat, flags: GstSeekFlags, start_type: GstSeekType, start: i64, stop_type: GstSeekType, stop: i64){
        self.appsink.seek_async(rate,format,flags,start_type,start,stop_type,stop);
    }
    
    fn query_duration(&self, format: GstFormat) -> Option<i64>{
        self.appsink.query_duration(format)
    }
    
    fn query_position(&self, format: GstFormat) -> Option<i64>{
        self.appsink.query_position(format)
    }
    
    fn duration_ns(&self) -> Option<i64>{
        self.appsink.duration_ns()
    }
    
    fn duration_s(&self) -> Option<f64>{
        self.appsink.duration_s()
    }
    
    fn position_ns(&self) -> i64{
        self.appsink.position_ns()
    }
    
    fn position_pct(&self) -> Option<f64>{
        self.appsink.position_pct()
    }
    
    fn position_s(&self) -> f64{
        self.appsink.position_s()
    }
    
    fn speed(&self) -> f64{
        self.appsink.speed()
    }
    
    fn set_position_ns(&mut self, ns: i64) -> bool{
        self.appsink.set_position_ns(ns)
    }
    
    fn set_position_s(&mut self, s: f64) -> bool{
        self.appsink.set_position_s(s)
    }
    
    fn set_position_pct(&mut self, pct: f64) -> bool{
        self.appsink.set_position_pct(pct)
    }
    
    fn set_speed(&mut self, speed: f64) -> bool{
        self.appsink.set_speed(speed)
    }
    
    fn set_position_ns_async(&mut self, ns: i64){
        self.appsink.set_position_ns_async(ns);
    }
    
    fn set_position_s_async(&mut self, s: f64){
        self.appsink.set_position_s_async(s);
    }
    
    fn set_position_pct_async(&mut self, pct: f64) -> bool{
        self.appsink.set_position_pct_async(pct)
    }
    
    fn set_speed_async(&mut self, speed: f64) -> bool{
        self.appsink.set_speed_async(speed)
    }
    
    unsafe fn gst_element(&self) -> *const GstElement{
        self.appsink.gst_element()
    }
    
    unsafe fn gst_element_mut(&mut self) -> *mut GstElement{
        self.appsink.gst_element_mut()
    }
    
    /*fn set<T>(&self, name: &str, value: T){
        self.appsink.set(name,value);
    }*/
    
    fn set_null_state(&mut self){
        self.appsink.set_null_state();
    }
    
    fn set_ready_state(&mut self){
        self.appsink.set_ready_state();
    }
    
    fn pause(&mut self){
        self.appsink.pause();
    }
    
    fn play(&mut self){
        self.appsink.play();
    }
    
    fn is_paused(&self) -> bool{
        self.appsink.is_paused()
    }
    
    fn is_playing(&self) -> bool{
        self.appsink.is_playing()
    }
    
    fn is_null_state(&self) -> bool{
        self.appsink.is_null_state()
    }
    
    fn is_ready_state(&self) -> bool{
        self.appsink.is_ready_state()
    }
}
