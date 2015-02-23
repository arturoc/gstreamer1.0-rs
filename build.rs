#[cfg(target_os="macos")]
fn build_flags(){
	println!("cargo:rustc-flags= -L framework=/Library/Frameworks");
}


#[cfg(target_os="linux")]
fn build_flags(){
}

fn main(){
	build_flags();
}
