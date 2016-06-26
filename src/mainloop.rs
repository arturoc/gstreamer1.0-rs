use ffi::*;
use std::ptr;
use std::thread;
use std::mem;
use std::cell::RefCell;
use std::os::raw::c_void;

unsafe impl Sync for MainLoop {}
unsafe impl Send for MainLoop {}

pub struct MainLoop{
	gst_loop: *mut GMainLoop,
	running: bool
}

impl Drop for MainLoop{
	fn drop(&mut self){
		unsafe{
			if self.running {
				self.quit();
			}
			g_main_loop_unref(mem::transmute(self.gst_loop));
		}
	}
}

impl MainLoop{
	pub fn new() -> MainLoop{
		unsafe{
			MainLoop{ gst_loop: g_main_loop_new(mem::transmute(ptr::null::<c_void>()), 0), running: false }
		}
	}

	pub fn spawn(&mut self){
		if !self.running {
			self.running = true;
			let gst_loop: usize = unsafe{ mem::transmute(self.gst_loop) };
			thread::spawn( move|| {
				unsafe{
					g_main_loop_run ( mem::transmute(gst_loop) );
				}
				/*loop{
					g_main_context_iteration(gst_loop,1);
				}*/
			});
		}
	}

	pub fn run(&mut self){
		unsafe{
			if !self.running {
				self.running = true;
				let gst_loop = self.gst_loop.clone();
				g_main_loop_run ( mem::transmute(gst_loop) );
			}
		}
	}

	pub fn quit(&mut self){
		unsafe{
			if self.running{
				self.running = false;
				g_main_loop_quit(mem::transmute(self.gst_loop));
			}
		}
	}
}

thread_local!(static LOOP: RefCell<MainLoop> = RefCell::new(MainLoop::new()));

pub fn spawn(){
	LOOP.with(|mainloop| mainloop.borrow_mut().spawn());
}

pub fn run(){
	LOOP.with(|mainloop| mainloop.borrow_mut().run());
}

pub fn quit(){
	LOOP.with(|mainloop| mainloop.borrow_mut().quit());
}
