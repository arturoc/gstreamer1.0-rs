extern crate gst;

use gst::ElementT;
use gst::BinT;
use std::env;
use std::thread;
use std::sync::{Condvar,Mutex};
use std::time::Duration;

fn main(){
    gst::init();
    match gst::Pipeline::new_from_str(format!("appsrc caps=\"video/x-raw,format=RGB,width=640,height=480,framerate=1/60\" name=appsrc0 ! videoconvert ! autovideosink").as_slice()){
		Ok(mut pipeline) => {
			let mut mainloop = gst::MainLoop::new();
			if let Some(mut bus) = pipeline.bus(){
				let bus_receiver = bus.receiver();
				if let Some(appsrc) = pipeline.get_by_name("appsrc0"){
					let mut appsrc = gst::AppSrc::new_from_element(appsrc);
					let bufferpool = gst::BufferPool::new().unwrap();
					let appsrc_caps = appsrc.caps().unwrap();
					bufferpool.set_params(&appsrc_caps,640*480*3,2,0);
					if bufferpool.set_active(true).is_err(){
					    panic!("Couldn't activate buffer pool");
					}
					mainloop.spawn();
					pipeline.play();

					thread::spawn(move||{
					    let condvar = Condvar::new();
					    let mutex = Mutex::new(());
					    let mut gray = 0;
						loop {
						    if let Some(mut buffer) = bufferpool.acquire_buffer(){
							    buffer.map_write(|mut mapping|{
							        for c in mapping.iter_mut::<u8>(){
							            *c = gray;
							        }
							    }).ok();
							    gray += 1;
							    gray %= 255;
								appsrc.push_buffer(buffer);
								let guard = mutex.lock().unwrap();
								condvar.wait_timeout(guard,Duration::milliseconds((1./60. * 1000.) as i64)).ok();
							}else{
							    println!("Couldn't get buffer, sending EOS and finishing thread");
							    appsrc.end_of_stream();
							    break;
							}
						}
					});
					
					for message in bus_receiver.iter(){
						match message.parse(){
							gst::Message::StateChangedParsed{ref msg, ref old, ref new, ref pending} => {
								println!("element `{}` changed from {:?} to {:?}", message.src_name(), old, new);
							}
							gst::Message::ErrorParsed{ref msg, ref error, ref debug} => {
								println!("error msg from element `{}`: {}, quitting", message.src_name(), error.message());
								break;
							}
							gst::Message::Eos(ref msg) => {
								println!("eos received quiting");
								break;
							}
							_ => {
								println!("msg of type `{}` from element `{}`", message.type_name(), message.src_name());
							}
						}
					}
					mainloop.quit();
				}
			}else{
				panic!("Couldn't get bus from pipeline");	
			}
		}
		
		Err(err) => panic!("Error: {} while creating pipline",err.message())
	}
}
