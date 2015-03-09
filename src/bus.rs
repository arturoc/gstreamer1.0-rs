use ffi::*;

use libc::c_void;
use std::ptr;
use std::mem;
use std::rc::{Rc,Weak};
use std::cell::RefCell;
use std::sync::mpsc::{self,Iter,TryRecvError,RecvError};

use message::Message;

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
    pub fn new(bus: *mut GstBus, owned: bool) -> Option<Bus>{
        if bus != ptr::null_mut::<GstBus>(){
            if !owned {
                unsafe{
                    gst_object_ref(bus as *mut c_void);
                }
            }
            Some(Bus{ bus: bus })
        }else{
            None
        }
    }
    
    pub fn add_watch(&mut self, watch: Rc<RefCell<Box<Watch>>>) -> u32{
        unsafe{
            let watch = Box::new(watch.downgrade());
            gst_bus_add_watch (self.bus, Some(mem::transmute(bus_callback)), mem::transmute(watch))
        }
    }
    
    pub fn receiver(&mut self) -> Receiver{
		let (watch,receiver) = channel();
		self.add_watch(watch);
		receiver
	}
}

extern "C" fn bus_callback(_bus: *mut GstBus, msg: *mut GstMessage, data: gpointer) -> gboolean {
    unsafe{
        let watch: &Weak<RefCell<Box<Watch>>> = mem::transmute(data);
        match watch.upgrade(){
            Some(watch) => match Message::new(msg){
				Some(msg) => if watch.borrow_mut().call(msg) {1} else {0},
				None => {1}
			},
            None => 0
        }
    }
}

pub trait Watch{
    fn call(&mut self, msg: Message) -> bool;
}


struct Sender{
    sender: mpsc::Sender<Message>,
}

impl Watch for Sender{
	fn call(&mut self, msg: Message) -> bool{
        self.sender.send(msg).is_ok()
	}
}

#[allow(dead_code)] // we need to keep the watch around
pub struct Receiver{
	receiver: mpsc::Receiver<Message>,
	watch: Rc<RefCell<Box<Watch + 'static>>>
}

impl Receiver{
    pub fn recv(&self) -> Result<Message,RecvError>{
        self.receiver.recv()
    }

    pub fn try_recv(&self) -> Result<Message,TryRecvError>{
        self.receiver.try_recv()
    }

    pub fn iter(&self) -> Iter<Message>{
        self.receiver.iter()
    }
}

pub fn channel() -> (Rc<RefCell<Box<Watch+'static>>>,Receiver){
	let (sender,receiver) = mpsc::channel();
	let watch = Rc::new(RefCell::new(Box::new(Sender{sender: sender}) as Box<Watch>));
	(watch.clone(), Receiver{receiver: receiver, watch: watch})
}
