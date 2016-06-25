extern crate gst;

use std::thread;
use std::sync::{Condvar,Mutex};
use std::time::Duration;

fn main(){
    gst::init();
    let pipeline_str = "appsrc caps=\"video/x-raw,format=RGB,width=640,height=480,framerate=1/60\" name=appsrc0 ! videoconvert ! autovideosink";
    let mut pipeline = gst::Pipeline::new_from_str(pipeline_str).unwrap();
	let mut mainloop = gst::MainLoop::new();
	let mut bus = pipeline.bus().expect("Couldn't get bus from pipeline");
	let bus_receiver = bus.receiver();
	let appsrc = pipeline.get_by_name("appsrc0").expect("Couldn't get appsrc from pipeline");
	let mut appsrc = gst::AppSrc::new_from_element(appsrc);
	let bufferpool = gst::BufferPool::new().unwrap();
	let appsrc_caps = appsrc.caps().unwrap();
	bufferpool.set_params(&appsrc_caps,640*480*3,0,0);
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
				condvar.wait_timeout(guard, Duration::from_millis((1000./60.) as u64)).ok();
			}else{
			    println!("Couldn't get buffer, sending EOS and finishing thread");
			    appsrc.end_of_stream();
			    break;
			}
		}
	});

	for message in bus_receiver.iter(){
		match message.parse(){
			gst::Message::StateChangedParsed{ref old, ref new, ..} => {
				println!("element `{}` changed from {:?} to {:?}", message.src_name(), old, new);
			}
            gst::Message::ErrorParsed{ref error, ref debug, ..} => {
				println!("error msg from element `{}`: {}, {}. Quitting", message.src_name(), error.message(), debug);
                break;
            }
			gst::Message::Eos(_) => {
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
