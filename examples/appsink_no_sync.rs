extern crate gst;
extern crate gstreamer_sys;

use std::env;

fn main(){
    gst::init();
    let args: Vec<String> = env::args().collect();
    let uri = if args.len() == 2 {
        gst::filename_to_uri(args[1].as_ref()).unwrap()
    }else{
        panic!("Usage: playbin file_path");
    };
    let mut playbin = gst::PlayBin::new("video_player").expect("Couldn't create playbin");
    let mut appsink = gst::AppSink::new("video_sink").expect("Couldn't create appsink");
    playbin.set_uri(uri.as_ref());
    playbin.set_video_sink(&appsink);
    appsink.set("sync", false);
	let mut bus = playbin.bus().expect("Couldn't get bus from pipeline");
	let bus_receiver = bus.receiver();
    let mut mainloop = gst::MainLoop::new();
    mainloop.spawn();
    playbin.play();
	loop {
        let mut exit = false;
        while let Ok(msg) = bus_receiver.try_recv(){
            match msg.parse(){
                gst::Message::StateChangedParsed{ref old, ref new, ..} => {
                    println!("element `{}` changed from {:?} to {:?}", msg.src_name(), old, new);
                }
                gst::Message::ErrorParsed{ref error, ref debug, ..} => {
                    println!("error msg from element `{}`: {}, {}. Quitting", msg.src_name(), error.message(), debug);
                    exit = true;
                    break;
                }
                _ => {
                    println!("msg of type `{}` from element `{}`", msg.type_name(), msg.src_name());
                }
            }
        }
        if exit { break; }

		match appsink.recv(){
            Ok(gst::appsink::Message::NewSample(sample)) | Ok(gst::appsink::Message::NewPreroll(sample)) => {
                let videoframe = sample.video_frame().unwrap();
                println!("Received sample with w: {}, h: {}", videoframe.width(), videoframe.height());
            }
            Ok(gst::appsink::Message::Eos) => {
                println!("Got no sample when polling. EOS");
                break;
            }
            Err(_) => {
                println!("Error, exiting");
                break;
            }
        }
    }

    playbin.set_null_state();
    playbin.get_state(gstreamer_sys::GST_CLOCK_TIME_NONE);
	mainloop.quit();
}
