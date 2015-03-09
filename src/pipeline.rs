use ffi::*;
use bin::Bin;
use bin::BinT;
use bus::Bus;
use element::Element;
use element::ElementT;
use error::Error;
use error::Result;
use util::*;

unsafe impl Sync for Pipeline {}
unsafe impl Send for Pipeline {}

pub struct Pipeline{
    pub pipeline: Bin
}

impl Pipeline{
    pub fn new(name: &str) -> Option<Pipeline>{
        unsafe{
            let pipeline = gst_pipeline_new(to_c_str!(name));
            if pipeline != ptr::null_mut(){
		        g_object_ref_sink(mem::transmute(pipeline));
	            match Bin::new_from_gst_bin(pipeline as *mut GstBin){
	                Some(bin) => Some(Pipeline{ pipeline: bin }),
	                None => None
	            }
	        }else{
	            None
	        }
        }
    }
    
    pub fn new_from_str(string: &str) -> Result<Pipeline>{
        let mut error = ptr::null_mut::<GError>();
        unsafe{
            let pipeline = gst_parse_launch (to_c_str!(string), &mut error);
            if error == ptr::null_mut(){
	            g_object_ref_sink(mem::transmute(pipeline));
				match Bin::new_from_gst_bin(pipeline as *mut GstBin){
					Some(bin) => Ok(Pipeline{ pipeline: bin }),
					None => Err(Error::new(0,0,"Couldn't create bin"))
				}
			}else{
				Err(Error::new_from_g_error(error))
			}
        }
    }
    
    pub fn bus(&self) -> Option<Bus>{
        unsafe{
            Bus::new(gst_pipeline_get_bus(self.gst_pipeline()),true)
        }
    }
    
    pub fn delay(&self) -> GstClockTime{
        unsafe{
            gst_pipeline_get_delay(self.gst_pipeline())
        }
    }
    
    pub fn set_delay(&self, delay: GstClockTime){
        unsafe{
            gst_pipeline_set_delay(self.gst_pipeline(), delay);
        }
    }
    
    pub fn set<T>(&self, name: &str, value: T){
        self.pipeline.set(name,value);
    }
}

pub trait PipelineT: BinT{    
    
    fn delay(&self) -> GstClockTime;
    
    fn set_delay(&self, delay: GstClockTime);
    
    unsafe fn gst_pipeline(&self) -> *mut GstPipeline;
}

impl PipelineT for Pipeline{  
    
    fn delay(&self) -> GstClockTime{
        self.delay()
    }
    
    fn set_delay(&self, delay: GstClockTime){
        self.set_delay(delay)
    }
    
    unsafe fn gst_pipeline(&self) -> *mut GstPipeline{
        self.pipeline.gst_element() as *mut GstPipeline
    }
}

impl BinT for Pipeline{
    
    unsafe fn gst_bin(&self) -> *mut GstBin{
        self.pipeline.gst_bin()
    }
    
    fn add(&self, element: &ElementT) -> bool{
        self.pipeline.add(element)
    }
    
    fn remove(&self, element: &ElementT) -> bool{
        self.pipeline.remove(element)
    }
    
    fn get_by_name(&self, name: &str) -> Option<Element>{
        self.pipeline.get_by_name(name)
    }
    
    fn recalculate_latency(&self) -> bool{
        self.pipeline.recalculate_latency()
    }
    
    fn set_async_handling(&self, async: bool){
        self.pipeline.set_async_handling(async);
    }
    
    fn set_message_forward(&self, forward: bool){
        self.pipeline.set_message_forward(forward);
    }
}

impl ElementT for Pipeline{
    
    fn link(&mut self, dst: &mut ElementT) -> bool{
        self.pipeline.link(dst)
    }
    
    fn unlink(&mut self, dst: &mut ElementT){
        self.pipeline.unlink(dst);
    }
    
    fn bus(&self) -> Option<Bus>{
        self.pipeline.bus()
    }
    
    fn name(&self) -> String{
        self.pipeline.name()
    }
    
    fn set_name(&mut self, name: &str){
        self.pipeline.set_name(name);
    }
    
    fn set_state(&mut self, state: GstState) -> GstStateChangeReturn{
        self.pipeline.set_state(state)
    }
    
    fn get_state(&self, timeout: GstClockTime) -> (GstState, GstState, GstStateChangeReturn){
        self.pipeline.get_state(timeout)
    }
    
    fn send_event(&mut self, event: *mut GstEvent) -> bool{
        self.pipeline.send_event(event)
    }
    
    fn seek_simple(&mut self, format: GstFormat, flags: GstSeekFlags, pos: i64) -> bool{
        self.pipeline.seek_simple(format, flags, pos)
    }
    
    fn seek(&mut self, rate: f64, format: GstFormat, flags: GstSeekFlags, start_type: GstSeekType, start: i64, stop_type: GstSeekType, stop: i64) -> bool{
        self.pipeline.seek(rate, format, flags, start_type, start, stop_type, stop)
    }
    
    fn seek_async(&mut self, rate: f64, format: GstFormat, flags: GstSeekFlags, start_type: GstSeekType, start: i64, stop_type: GstSeekType, stop: i64){
        self.pipeline.seek_async(rate,format,flags,start_type,start,stop_type,stop);
    }
    
    fn query_duration(&self, format: GstFormat) -> Option<i64>{
        self.pipeline.query_duration(format)
    }
    
    fn query_position(&self, format: GstFormat) -> Option<i64>{
        self.pipeline.query_position(format)
    }
    
    fn duration_ns(&self) -> Option<i64>{
        self.pipeline.duration_ns()
    }
    
    fn duration_s(&self) -> Option<f64>{
        self.pipeline.duration_s()
    }
    
    fn position_ns(&self) -> i64{
        self.pipeline.position_ns()
    }
    
    fn position_pct(&self) -> Option<f64>{
        self.pipeline.position_pct()
    }
    
    fn position_s(&self) -> f64{
        self.pipeline.position_s()
    }
    
    fn speed(&self) -> f64{
        self.pipeline.speed()
    }
    
    fn set_position_ns(&mut self, ns: i64) -> bool{
        self.pipeline.set_position_ns(ns)
    }
    
    fn set_position_s(&mut self, s: f64) -> bool{
        self.pipeline.set_position_s(s)
    }
    
    fn set_position_pct(&mut self, pct: f64) -> bool{
        self.pipeline.set_position_pct(pct)
    }
    
    fn set_speed(&mut self, speed: f64) -> bool{
        self.pipeline.set_speed(speed)
    }
    
    fn set_position_ns_async(&mut self, ns: i64){
        self.pipeline.set_position_ns_async(ns);
    }
    
    fn set_position_s_async(&mut self, s: f64){
        self.pipeline.set_position_s_async(s);
    }
    
    fn set_position_pct_async(&mut self, pct: f64) -> bool{
        self.pipeline.set_position_pct_async(pct)
    }
    
    fn set_speed_async(&mut self, speed: f64) -> bool{
        self.pipeline.set_speed_async(speed)
    }
    
    unsafe fn gst_element(&self) -> *const GstElement{
        self.pipeline.gst_element()
    }
    
    unsafe fn gst_element_mut(&mut self) -> *mut GstElement{
        self.pipeline.gst_element_mut()
    }
    
    /*fn set<T>(&self, name: &str, value: T){
        self.pipeline.set(name,value);
    }*/
    
    fn set_null_state(&mut self){
        self.pipeline.set_null_state();
    }
    
    fn set_ready_state(&mut self){
        self.pipeline.set_ready_state();
    }
    
    fn pause(&mut self){
        self.pipeline.pause();
    }
    
    fn play(&mut self){
        self.pipeline.play();
    }
    
    fn is_paused(&self) -> bool{
        self.pipeline.is_paused()
    }
    
    fn is_playing(&self) -> bool{
        self.pipeline.is_playing()
    }
    
    fn is_null_state(&self) -> bool{
        self.pipeline.is_null_state()
    }
    
    fn is_ready_state(&self) -> bool{
        self.pipeline.is_ready_state()
    }
}
