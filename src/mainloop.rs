use ffi::*;
use std::sync::Arc;
use std::ptr::{self,Unique};
use std::thread;
use std::mem;
use libc::c_void;


pub struct MainLoop{
	gst_loop: Arc<Unique<GMainLoop>>
}

impl Drop for MainLoop{
	fn drop(&mut self){
		unsafe{
			g_main_loop_unref(mem::transmute(self.gst_loop.get()));
		}
	}
}

impl MainLoop{
	pub fn new() -> MainLoop{
		unsafe{
			MainLoop{ gst_loop: Arc::new(Unique::new(g_main_loop_new(mem::transmute(ptr::null::<c_void>()), 0))) }
		}
	}
	
	pub fn spawn(&mut self){
		let gst_loop = self.gst_loop.clone();
		thread::spawn( move|| { 
			unsafe{
				g_main_loop_run ( mem::transmute(gst_loop.get()) );
			}
			/*loop{ 
				g_main_context_iteration(gst_loop,1);
			}*/
		});
	}
	
	pub fn run(&mut self){
		unsafe{
			let gst_loop = self.gst_loop.clone();
			g_main_loop_run ( mem::transmute(gst_loop.get()) );
		}
	}
	
	pub fn quit(&mut self){
		unsafe{
			g_main_loop_quit(mem::transmute(self.gst_loop.get()));
		}
	}
}
