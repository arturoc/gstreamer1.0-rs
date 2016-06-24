extern crate gst;

use gst::ElementT;
use std::env;

fn main(){
    gst::init();
    let pipeline_str = env::args().collect::<Vec<String>>()[1..].join(" ");
    let mut pipeline = gst::Pipeline::new_from_str(pipeline_str.as_ref()).unwrap();
	let mut mainloop = gst::MainLoop::new();
	let mut bus = pipeline.bus().expect("Couldn't get bus from pipeline");
	let bus_receiver = bus.receiver();
	mainloop.spawn();
	pipeline.play();
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
