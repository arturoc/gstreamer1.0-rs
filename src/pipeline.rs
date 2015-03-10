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

/** A GstPipeline is a special GstBin used as the toplevel container for the filter graph. The GstPipeline will manage the selection and distribution of a global GstClock as well as provide a GstBus to the application.

gst_pipeline_new() is used to create a pipeline. when you are done with the pipeline, use gst_object_unref() to free its resources including all added GstElement objects (if not otherwise referenced).

Elements are added and removed from the pipeline using the GstBin methods like gst_bin_add() and gst_bin_remove() (see GstBin).

Before changing the state of the GstPipeline (see GstElement) a GstBus can be retrieved with gst_pipeline_get_bus(). This bus can then be used to receive GstMessage from the elements in the pipeline.

By default, a GstPipeline will automatically flush the pending GstBus messages when going to the NULL state to ensure that no circular references exist when no messages are read from the GstBus. This behaviour can be changed with gst_pipeline_set_auto_flush_bus().

When the GstPipeline performs the PAUSED to PLAYING state change it will select a clock for the elements. The clock selection algorithm will by default select a clock provided by an element that is most upstream (closest to the source). For live pipelines (ones that return GST_STATE_CHANGE_NO_PREROLL from the gst_element_set_state() call) this will select the clock provided by the live source. For normal pipelines this will select a clock provided by the sinks (most likely the audio sink). If no element provides a clock, a default GstSystemClock is used.

The clock selection can be controlled with the gst_pipeline_use_clock() method, which will enforce a given clock on the pipeline. With gst_pipeline_auto_clock() the default clock selection algorithm can be restored.

A GstPipeline maintains a running time for the elements. The running time is defined as the difference between the current clock time and the base time. When the pipeline goes to READY or a flushing seek is performed on it, the running time is reset to 0. When the pipeline is set from PLAYING to PAUSED, the current clock time is sampled and used to configure the base time for the elements when the pipeline is set to PLAYING again. The effect is that the running time (as the difference between the clock time and the base time) will count how much time was spent in the PLAYING state. This default behaviour can be changed with the gst_element_set_start_time() method.*/
impl Pipeline{
    /// Create a new pipeline with the given name.
    pub fn new(name: &str) -> Option<Pipeline>{
        unsafe{
            let pipeline = gst_pipeline_new(to_c_str!(name));
            if pipeline != ptr::null_mut(){
		        gst_object_ref_sink(mem::transmute(pipeline));
	            match Bin::new_from_gst_bin(pipeline as *mut GstBin){
	                Some(bin) => Some(Pipeline{ pipeline: bin }),
	                None => None
	            }
	        }else{
	            None
	        }
        }
    }
    
    /// Creates a new pipeline using gst_parse_launch
    pub fn new_from_str(string: &str) -> Result<Pipeline>{
        let mut error = ptr::null_mut::<GError>();
        unsafe{
            let pipeline = gst_parse_launch (to_c_str!(string), &mut error);
            if error == ptr::null_mut(){
	            gst_object_ref_sink(mem::transmute(pipeline));
				match Bin::new_from_gst_bin(pipeline as *mut GstBin){
					Some(bin) => Ok(Pipeline{ pipeline: bin }),
					None => Err(Error::new(0,0,"Couldn't create bin"))
				}
			}else{
				Err(Error::new_from_g_error(error))
			}
        }
    }
    
    /// Gets the GstBus of pipeline . The bus allows applications to 
    /// receive Message packets.
    pub fn bus(&self) -> Option<Bus>{
        unsafe{
            Bus::new(gst_pipeline_get_bus(self.gst_pipeline() as *mut GstPipeline),true)
        }
    }
    
    pub fn set<T>(&self, name: &str, value: T){
        self.pipeline.set(name,value);
    }
}

pub trait PipelineT: BinT{    
    
    /// Get the configured delay (see set_delay()).
    fn delay(&self) -> GstClockTime;
    
    /// Set the expected delay needed for all elements to perform the
    /// PAUSED to PLAYING state change. delay will be added to the base
    /// time of the elements so that they wait an additional delay amount
    /// of time before starting to process buffers and cannot be 
    /// GST_CLOCK_TIME_NONE.
	///
	/// This option is used for tuning purposes and should normally not be used.
    fn set_delay(&mut self, delay: GstClockTime);
    
    /// Returns a const raw pointer to the internal GstElement
    unsafe fn gst_pipeline(&self) -> *const GstPipeline;
    
    /// Returns a mut raw pointer to the internal GstElement
    unsafe fn gst_pipeline_mut(&mut self) -> *mut GstPipeline;
}

impl PipelineT for Pipeline{
    fn delay(&self) -> GstClockTime{
        unsafe{
            gst_pipeline_get_delay(self.gst_pipeline() as *mut GstPipeline)
        }
    }
    
    fn set_delay(&mut self, delay: GstClockTime){
        unsafe{
            gst_pipeline_set_delay(self.gst_pipeline_mut(), delay);
        }
    }
    
    unsafe fn gst_pipeline(&self) -> *const GstPipeline{
        self.pipeline.gst_element() as *const GstPipeline
    }
    
    unsafe fn gst_pipeline_mut(&mut self) -> *mut GstPipeline{
        self.pipeline.gst_element_mut() as *mut GstPipeline
    }
}

impl BinT for Pipeline{
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
    
    unsafe fn gst_bin(&self) -> *const GstBin{
        self.pipeline.gst_bin()
    }
    
    unsafe fn gst_bin_mut(&mut self) -> *mut GstBin{
        self.pipeline.gst_bin_mut()
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
    
    unsafe fn send_event(&mut self, event: *mut GstEvent) -> bool{
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
