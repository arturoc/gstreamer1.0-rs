pub use std::ffi::CString;
pub use std::ffi::CStr;
pub use std::str;
pub use std::mem;
pub use std::ptr;

/// Converts nanoseconds to seconds
pub fn ns_to_s(ns: u64) -> f64{
    (ns as f64) / 1000000000.0
}

/// Converts seconds to nanoseconds
pub fn s_to_ns(s: f64) -> u64{
    (s * 1000000000.0) as u64
}

macro_rules! from_c_str{
	($c_string: expr) => (
		str::from_utf8(CStr::from_ptr($c_string).to_bytes()).unwrap();
	);
}
