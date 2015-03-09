use ffi::*;
use std::result;
use util::*;
use std::fmt::{self,Debug,Formatter};

unsafe impl Send for GError {}
unsafe impl Send for Error {}

pub struct Error{
    error: *mut GError
}

impl Debug for Error{
	fn fmt(&self, fmt: &mut Formatter) -> result::Result<(), fmt::Error>{
		fmt.write_str(format!("gst::Error: domain: {}, code: {}, message: {}",self.domain(),self.code(),self.message()).as_slice())
	}
}

impl Drop for Error{
	fn drop(&mut self){
		unsafe{
			if self.error != ptr::null_mut(){
				g_error_free(self.error);
			}
		}
	}
}

impl Error{
    pub fn new(domain: u32, code: i32, message: &str) -> Error{
		unsafe{
			Error{error: g_error_new(domain, code, to_c_str!(message))}
		}
    }
    
    pub fn new_from_g_error(err: *mut GError) -> Error{
		Error{ error: err }
	}

    pub fn message(&self) -> String{
		unsafe{
			if self.error != ptr::null_mut(){
				from_c_str!(mem::transmute((*self.error).message)).to_string()
			}else{
				"".to_string()
			}
		}
    }
    
    pub fn code(&self) -> i32{
		unsafe{ 
			if self.error !=ptr::null_mut(){
				(*self.error).code
			}else{
				0
			}
		}
	}
	
	pub fn domain(&self) -> u32{
		unsafe{
			if self.error != ptr::null_mut(){
				(*self.error).domain
			}else{
				0
			}
		}
	}
}


pub type Result<T> = result::Result<T,Error>;
