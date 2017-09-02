extern crate gst;
extern crate gstreamer_sys;

use std::env;
use std::os::raw::c_void;
use std::mem;

extern "C" fn signal_callback(_src: *mut gst::Element, pad: *mut gstreamer_sys::GstPad, sinkpad: &mut gst::Pad){
    unsafe{
        if !sinkpad.is_linked(){
            let mut decodebin_pad = gst::Pad::new(pad).unwrap();
            let caps = decodebin_pad.query_caps(None).unwrap();
            let structure = caps.structure(0).unwrap();
            if structure.name().starts_with("video") {
                decodebin_pad.link(sinkpad).unwrap();
            }
        }
    }
}

fn main(){
    gst::init();
    let args: Vec<_> = env::args().collect();
    let uri: &str = if args.len() == 2 {
        args[1].as_ref()
    }else{
        panic!("Usage: pipeline file_path");
    };

    let mut pipeline = gst::Pipeline::new("video_player").expect("Couldn't create playbin");
    let mut filesrc = gst::Element::new("filesrc", "").unwrap();
    filesrc.set("location", uri);
    let mut decodebin = gst::Element::new("decodebin", "").unwrap();
    let mut sink = gst::Element::new("glimagesink", "").unwrap();
    let mut sink_pad = sink.static_pad("sink").unwrap();
    unsafe{
        decodebin.signal_connect("pad-added", mem::transmute(signal_callback as *mut c_void), &mut sink_pad);
    }
    if !pipeline.add_and_link(filesrc, decodebin){
        panic!("couldn't link filesrc and decodebin");
    }
    pipeline.add(sink);
    let mut mainloop = gst::MainLoop::new();
    let mut bus = pipeline.bus().expect("Couldn't get pipeline bus");
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
