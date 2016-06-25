use ffi::*;

use std::os::raw::c_void;
use std::sync::mpsc::{self,channel,Receiver};

use message::Message;
use util::*;

static REMOVE_WATCH_MESSAGE_STR: &'static str = "gstreamer1.0-rs_remove_watch_message";

unsafe impl Sync for Bus {}
unsafe impl Send for Bus {}

pub struct Bus{
    bus: *mut GstBus
}

impl Drop for Bus{
    fn drop(&mut self){
        unsafe{
            gst_object_unref(self.bus as *mut c_void);
        }
    }
}

impl Bus{
    pub unsafe fn new(bus: *mut GstBus, owned: bool) -> Option<Bus>{
        if bus != ptr::null_mut::<GstBus>(){
            if !owned {
                gst_object_ref(bus as *mut c_void);
            }
            Some(Bus{ bus: bus })
        }else{
            None
        }
    }

    pub fn add_watch<W: Watch>(&mut self, watch: W) -> u32{
        unsafe{
            let watch: Box<Watch> = Box::new(watch);
            let watch: *mut Box<Watch> = Box::into_raw(Box::new(watch));
            gst_bus_add_watch (self.bus, Some(bus_callback), mem::transmute(watch))
        }
    }

    pub fn remove_watch(&mut self) -> bool{
        unsafe{
            let message_cstr = CString::new(REMOVE_WATCH_MESSAGE_STR).unwrap();
            let structure = gst_structure_new(message_cstr.as_ptr(), ptr::null());
            let message = gst_message_new_application(ptr::null_mut(), structure);
            gst_bus_post(self.bus, message) != 0
        }
    }

    pub fn receiver(&mut self) -> Receiver<Message>{
		let (watch,receiver) = channel();
		self.add_watch(watch);
		receiver
	}
}

extern "C" fn bus_callback(_bus: *mut GstBus, msg: *mut GstMessage, data: gpointer) -> gboolean {
    unsafe{
        let alive = {
            let mut watch: &mut Box<Watch> = mem::transmute(data);
            match Message::new(msg){
                Some(Message::Application(app_msg)) => {
                    let structure = gst_message_get_structure(app_msg);
                    let cname = gst_structure_get_name(structure);
                    if from_c_str!(cname) == REMOVE_WATCH_MESSAGE_STR{
                        false
                    }else{
                        watch.call(Message::Application(app_msg))
                    }
                }
    			Some(msg) => watch.call(msg),
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

impl ::Reference for Bus{
    fn reference(&self) -> ::Ref<Bus>{
        unsafe{
            ::Ref::from(Bus::new(self.bus, false).unwrap())
        }
    }
}
