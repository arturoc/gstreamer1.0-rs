use ffi::*;
use element::Element;
use element::ElementT;
use bus::Bus;
use util::*;

unsafe impl Sync for Bin {}
unsafe impl Send for Bin {}

pub struct Bin{
    pub bin: Element
}

impl Bin{
    pub fn new(name: &str) -> Option<Bin>{
        unsafe{
            let bin = gst_bin_new(to_c_str!(name));
            if bin != ptr::null_mut(){
	            g_object_ref_sink(mem::transmute(bin));
	            Bin::new_from_gst_bin(bin as *mut GstBin)
	        }else{
	            None
	        }
        }
    }
    
    pub unsafe fn new_from_gst_bin(element: *mut GstBin) -> Option<Bin>{
        match Element::new_from_gst_element(element as *mut GstElement){
            Some(element) => Some( Bin{ bin: element } ),
            None => None
        }
    }
    
    pub fn set<T>(&self, name: &str, value: T){
        self.bin.set(name,value);
    }
}

impl BinT for Bin{
    unsafe fn gst_bin(&self) -> *mut GstBin{
        self.bin.gst_element() as *mut GstBin
    }
    
    fn add(&self, element: &ElementT) -> bool{
        unsafe{
            gst_bin_add(self.gst_bin(), mem::transmute(element.gst_element())) == 1
        }
    }
    
    fn remove(&self, element: &ElementT) -> bool{
        unsafe{
            gst_bin_remove(self.gst_bin(), mem::transmute(element.gst_element())) == 1
        }
    }
    
    fn get_by_name(&self, name: &str) -> Option<Element>{
        unsafe{
            let element = gst_bin_get_by_name(self.gst_bin(), to_c_str!(name));
            Element::new_from_gst_element(element)
        }
    }
    
    fn recalculate_latency(&self) -> bool{
        unsafe{
            gst_bin_recalculate_latency(self.gst_bin()) == 1
        }
    }
    
    fn set_async_handling(&self, async: bool){
        self.bin.set("async-handling", async);
    }
    
    fn set_message_forward(&self, forward: bool){
        self.bin.set("message-forward", forward);
    }
}

impl ElementT for Bin{
    
    fn link(&mut self, dst: &mut ElementT) -> bool{
        self.bin.link(dst)
    }
    
    fn unlink(&mut self, dst: &mut ElementT){
        self.bin.unlink(dst);
    }
    
    fn bus(&self) -> Option<Bus>{
        self.bin.bus()
    }
    
    fn name(&self) -> String{
        self.bin.name()
    }
    
    fn set_name(&mut self, name: &str){
        self.bin.set_name(name);
    }
    
    fn set_state(&mut self, state: GstState) -> GstStateChangeReturn{
        self.bin.set_state(state)
    }
    
    fn get_state(&self, timeout: GstClockTime) -> (GstState, GstState, GstStateChangeReturn){
        self.bin.get_state(timeout)
    }
    
    fn send_event(&mut self, event: *mut GstEvent) -> bool{
        self.bin.send_event(event)
    }
    
    fn seek_simple(&mut self, format: GstFormat, flags: GstSeekFlags, pos: i64) -> bool{
        self.bin.seek_simple(format, flags, pos)
    }
    
    fn seek(&mut self, rate: f64, format: GstFormat, flags: GstSeekFlags, start_type: GstSeekType, start: i64, stop_type: GstSeekType, stop: i64) -> bool{
        self.bin.seek(rate, format, flags, start_type, start, stop_type, stop)
    }
    
    fn seek_async(&mut self, rate: f64, format: GstFormat, flags: GstSeekFlags, start_type: GstSeekType, start: i64, stop_type: GstSeekType, stop: i64){
        self.bin.seek_async(rate,format,flags,start_type,start,stop_type,stop);
    }
    
    fn query_duration(&self, format: GstFormat) -> Option<i64>{
        self.bin.query_duration(format)
    }
    
    fn query_position(&self, format: GstFormat) -> Option<i64>{
        self.bin.query_position(format)
    }
    
    fn duration_ns(&self) -> Option<i64>{
        self.bin.duration_ns()
    }
    
    fn duration_s(&self) -> Option<f64>{
        self.bin.duration_s()
    }
    
    fn position_ns(&self) -> i64{
        self.bin.position_ns()
    }
    
    fn position_pct(&self) -> Option<f64>{
        self.bin.position_pct()
    }
    
    fn position_s(&self) -> f64{
        self.bin.position_s()
    }
    
    fn speed(&self) -> f64{
        self.bin.speed()
    }
    
    fn set_position_ns(&mut self, ns: i64) -> bool{
        self.bin.set_position_ns(ns)
    }
    
    fn set_position_s(&mut self, s: f64) -> bool{
        self.bin.set_position_s(s)
    }
    
    fn set_position_pct(&mut self, pct: f64) -> bool{
        self.bin.set_position_pct(pct)
    }
    
    fn set_speed(&mut self, speed: f64) -> bool{
        self.bin.set_speed(speed)
    }
    
    fn set_position_ns_async(&mut self, ns: i64){
        self.bin.set_position_ns_async(ns);
    }
    
    fn set_position_s_async(&mut self, s: f64){
        self.bin.set_position_s_async(s);
    }
    
    fn set_position_pct_async(&mut self, pct: f64) -> bool{
        self.bin.set_position_pct_async(pct)
    }
    
    fn set_speed_async(&mut self, speed: f64) -> bool{
        self.bin.set_speed_async(speed)
    }
    
    unsafe fn gst_element(&self) -> *const GstElement{
        self.bin.gst_element()
    }
    
    unsafe fn gst_element_mut(&mut self) -> *mut GstElement{
        self.bin.gst_element_mut()
    }
    
    /*fn set<T>(&self, name: &str, value: T){
        self.bin.set(name,value);
    }*/
    
    fn set_null_state(&mut self){
        self.bin.set_null_state();
    }
    
    fn set_ready_state(&mut self){
        self.bin.set_ready_state();
    }
    
    fn pause(&mut self){
        self.bin.pause();
    }
    
    fn play(&mut self){
        self.bin.play();
    }
    
    fn is_paused(&self) -> bool{
        self.bin.is_paused()
    }
    
    fn is_playing(&self) -> bool{
        self.bin.is_playing()
    }
    
    fn is_null_state(&self) -> bool{
        self.bin.is_null_state()
    }
    
    fn is_ready_state(&self) -> bool{
        self.bin.is_ready_state()
    }
}


pub trait BinT: ElementT{
    
    unsafe fn gst_bin(&self) -> *mut GstBin;
    
    fn add(&self, element: &ElementT) -> bool;
    
    fn remove(&self, element: &ElementT) -> bool;
    
    fn get_by_name(&self, name: &str) -> Option<Element>;
    
    fn recalculate_latency(&self) -> bool;
    
    fn set_async_handling(&self, async: bool);
    
    fn set_message_forward(&self, forward: bool);
}
