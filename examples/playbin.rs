extern crate gst;

use gst::ElementT;
use std::env;

fn main(){
    gst::init();
    let args: Vec<String> = env::args().collect();
    let uri = if args.len() == 2 {
        gst::filename_to_uri(args[1].as_slice()).unwrap()
    }else{
        panic!("Usage: playbin file_path");
    };
    let mut playbin = gst::PlayBin::new("video_player").expect("Couldn't create playbin");
    playbin.set_uri(uri.as_slice());
    let mut mainloop = gst::MainLoop::new();
    let mut bus = playbin.bus().expect("Couldn't get pipeline bus");
    let bus_receiver = bus.receiver();
    mainloop.spawn();
    playbin.play();
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
