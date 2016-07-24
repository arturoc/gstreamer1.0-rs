extern crate gst;

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
    let mut appsink = gst::AppSinkPoll::new("video_sink").expect("Couldn't create appsink");
    playbin.set_uri(uri.as_ref());
    playbin.set_video_sink(&appsink);
    appsink.set("sync", false);
    let mut mainloop = gst::MainLoop::new();
    mainloop.spawn();
    playbin.play();
	loop {
        let sample = if playbin.is_paused(){
            println!("polling preroll");
            appsink.pull_preroll()
        }else if playbin.is_playing(){
            println!("polling sample");
            appsink.pull_sample()
        }else{
            None
        };
		if let Some(sample) = sample{
            let videoframe = sample.video_frame().unwrap();
            println!("Received sample with w: {}, h: {}", videoframe.width(), videoframe.height());
        }else{ //Eos
            println!("Got no sample when polling. EOS");
            break;
        }
    }
    playbin.set_null_state();
    playbin.get_state(gst::ffi::GST_CLOCK_TIME_NONE);
	mainloop.quit();
}
