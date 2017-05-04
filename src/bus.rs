use ffi::*;

use std::sync::mpsc::{self,channel,Receiver};

use message::Message;
use util::*;
use reference::Reference;
use object::Object;

static REMOVE_WATCH_MESSAGE_STR: &'static str = "gstreamer1.0-rs_remove_watch_message";

unsafe impl Sync for Bus {}
unsafe impl Send for Bus {}

pub struct Bus{
    bus: Object
}

impl Bus{
    pub unsafe fn new(bus: *mut GstBus) -> Option<Bus>{
        Object::new(bus as *mut GstObject).map(|obj| Bus{bus: obj})
    }

    pub fn add_watch<W: Watch>(&mut self, watch: W) -> u32{
        unsafe{
            let watch: Box<Watch> = Box::new(watch);
            let watch: *const Box<Watch> = Box::into_raw(Box::new(watch));
            gst_bus_add_watch (self.gst_bus_mut(), Some(bus_callback), mem::transmute(watch))
        }
    }

    pub fn remove_watch(&mut self) -> bool{
        unsafe{
            let message_cstr = CString::new(REMOVE_WATCH_MESSAGE_STR).unwrap();
            let structure = gst_structure_new(message_cstr.as_ptr(), ptr::null());
            let message = gst_message_new_application(ptr::null_mut(), structure);
            gst_bus_post(self.gst_bus_mut(), message) != 0
        }
    }

    pub fn receiver(&mut self) -> Receiver<Message>{
		let (watch,receiver) = channel();
		self.add_watch(watch);
		receiver
	}

    pub unsafe fn gst_bus(&self) -> *const GstBus{
        self.bus.gst_object() as *const GstBus
    }

    pub unsafe fn gst_bus_mut(&mut self) -> *mut GstBus{
        self.bus.gst_object_mut() as *mut GstBus
    }
}

extern "C" fn bus_callback(_bus: *mut GstBus, msg: *mut GstMessage, data: gpointer) -> gboolean {
    unsafe{
        let alive = {
            let watch: *mut Box<Watch> = mem::transmute(data);
            match Message::new(msg){
                Some(Message::Application(app_msg)) => {
                    let structure = gst_message_get_structure(app_msg);
                    let cname = gst_structure_get_name(structure);
                    if from_c_str!(cname) == REMOVE_WATCH_MESSAGE_STR{
                        Box::from_raw(watch);
                        false
                    }else{
                        (*watch).call(Message::Application(app_msg))
                    }
                },
                Some(msg) => (*watch).call(msg),
                None => true,
            }
        };
        if !alive{
            Box::from_raw(data);
        }
        if alive {1} else {0}
    }
}

pub trait Watch: Send{
    fn call(&mut self, msg: Message) -> bool;
}

impl Watch for mpsc::Sender<Message>{
	fn call(&mut self, msg: Message) -> bool{
        self.send(msg).is_ok()
	}
}

impl Reference for Bus{
    fn reference(&self) -> Bus{
        Bus{ bus: self.bus.reference() }
    }
}
