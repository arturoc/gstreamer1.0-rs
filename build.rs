#![feature(env)]

#[cfg(target_os="macos")]
fn build_flags(){
	println!("cargo:rustc-flags= -L framework=/Library/Frameworks");
}


#[cfg(target_os="linux")]
fn build_flags(){
}


#[cfg(target_os="windows")]
use std::env;
fn build_flags(){
	if let Ok(gst_root) = env::var("GSTREAMER_1_0_ROOT_X86_64"){
		println!("cargo:rustc-flags= -L native={}lib",gst_root);
	}else{
		println!("error: GSTREAMER_1_0_ROOT_X86 var not present, probably gstreamer is not installed");
	}
}

fn main(){
	build_flags();
}
