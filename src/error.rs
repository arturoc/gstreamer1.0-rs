use ffi::*;
use std::result;
use util::*;
use std::ptr::Unique;
use std::fmt::{self,Debug,Formatter};

pub struct Error{
    error: Unique<GError>
}

impl Debug for Error{
	fn fmt(&self, fmt: &mut Formatter) -> result::Result<(), fmt::Error>{
		fmt.write_str(format!("gst::Error: domain: {}, code: {}, message: {}",self.domain(),self.code(),self.message()).as_slice())
	}
}

impl Drop for Error{
	fn drop(&mut self){
		unsafe{
			if self.error.get() as *const GError != ptr::null(){
				g_error_free(self.error.get_mut());
			}
		}
	}
}

impl Error{
    pub fn new(domain: u32, code: i32, message: &str) -> Error{
		unsafe{
			Error{error: Unique::new(g_error_new(domain, code, to_c_str!(message)))}
		}
    }
    
    pub fn new_from_g_error(err: *mut GError) -> Error{
		unsafe{
			Error{ error: Unique::new(err) }
		}	
	}

    pub fn message(&self) -> String{
		unsafe{
			if self.error.get() as *const GError != ptr::null(){
				from_c_str!(mem::transmute(self.error.get().message)).to_string()
			}else{
				"".to_string()
			}
		}
    }
    
    pub fn code(&self) -> i32{
		unsafe{ 
			if self.error.get() as *const GError != ptr::null(){
				self.error.get().code
			}else{
				0
			}
		}
	}
	
	pub fn domain(&self) -> u32{
		unsafe{
			if self.error.get() as *const GError != ptr::null(){
				self.error.get().domain
			}else{
				0
			}
		}
	}
}


pub type Result<T> = result::Result<T,Error>;
