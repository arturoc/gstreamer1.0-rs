#[cfg(target_os="macos")]
fn build_flags(){
	println!("cargo:rustc-flags= -L /opt/local/lib");
}


#[cfg(target_os="linux")]
fn build_flags(){
}


#[cfg(target_os="windows")]
extern crate libc;
#[cfg(target_os="windows")]
use std::env;
#[cfg(target_os="windows")]
use std::mem;
#[cfg(target_os="windows")]
fn build_flags(){
	let key = if cfg!(target_pointer_width = "32") {
		"GSTREAMER_1_0_ROOT_X86"
	}else{
		"GSTREAMER_1_0_ROOT_X86_64"
	};
	if let Ok(gst_root) = env::var(key){
		println!("cargo:rustc-flags= -L native={}lib",gst_root);
	}else{
		println!("error: GSTREAMER_1_0_ROOT_X86 var not present, probably gstreamer is not installed");
	}
}

fn main(){
	build_flags();
}
