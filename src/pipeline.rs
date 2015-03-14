use ffi::*;
use bin::{Bin,BinT};
use bus::Bus;
use element::ElementT;
use error::Error;
use error::Result;
use util::*;


/** A GstPipeline is a special GstBin used as the toplevel container for the filter graph. The GstPipeline will manage the selection and distribution of a global GstClock as well as provide a GstBus to the application.

gst_pipeline_new() is used to create a pipeline. when you are done with the pipeline, use gst_object_unref() to free its resources including all added GstElement objects (if not otherwise referenced).

Elements are added and removed from the pipeline using the GstBin methods like gst_bin_add() and gst_bin_remove() (see GstBin).

Before changing the state of the GstPipeline (see GstElement) a GstBus can be retrieved with gst_pipeline_get_bus(). This bus can then be used to receive GstMessage from the elements in the pipeline.

By default, a GstPipeline will automatically flush the pending GstBus messages when going to the NULL state to ensure that no circular references exist when no messages are read from the GstBus. This behaviour can be changed with gst_pipeline_set_auto_flush_bus().

When the GstPipeline performs the PAUSED to PLAYING state change it will select a clock for the elements. The clock selection algorithm will by default select a clock provided by an element that is most upstream (closest to the source). For live pipelines (ones that return GST_STATE_CHANGE_NO_PREROLL from the gst_element_set_state() call) this will select the clock provided by the live source. For normal pipelines this will select a clock provided by the sinks (most likely the audio sink). If no element provides a clock, a default GstSystemClock is used.

The clock selection can be controlled with the gst_pipeline_use_clock() method, which will enforce a given clock on the pipeline. With gst_pipeline_auto_clock() the default clock selection algorithm can be restored.

A GstPipeline maintains a running time for the elements. The running time is defined as the difference between the current clock time and the base time. When the pipeline goes to READY or a flushing seek is performed on it, the running time is reset to 0. When the pipeline is set from PLAYING to PAUSED, the current clock time is sampled and used to configure the base time for the elements when the pipeline is set to PLAYING again. The effect is that the running time (as the difference between the clock time and the base time) will count how much time was spent in the PLAYING state. This default behaviour can be changed with the gst_element_set_start_time() method.*/
pub struct Pipeline{
    pipeline: Bin
}

unsafe impl Sync for Pipeline {}
unsafe impl Send for Pipeline {}

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
    
    pub unsafe fn new_from_gst_pipeline(pipeline: *mut GstPipeline) -> Option<Pipeline>{
        match Bin::new_from_gst_bin(pipeline as *mut GstBin){
            Some(pipeline) => Some( Pipeline{ pipeline: pipeline } ),
            None => None
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
    fn as_pipeline(&self) -> &Pipeline;
    fn as_pipeline_mut(&mut self) -> &mut Pipeline;
    
    fn to_pipeline(&self) -> Pipeline{
        Pipeline{ pipeline: self.to_bin() }
    }
    
    /// Get the configured delay (see set_delay()).
    fn delay(&self) -> GstClockTime{
        self.as_pipeline().delay()
    }
    
    /// Set the expected delay needed for all elements to perform the
    /// PAUSED to PLAYING state change. delay will be added to the base
    /// time of the elements so that they wait an additional delay amount
    /// of time before starting to process buffers and cannot be 
    /// GST_CLOCK_TIME_NONE.
	///
	/// This option is used for tuning purposes and should normally not be used.
    fn set_delay(&mut self, delay: GstClockTime){
        self.as_pipeline_mut().set_delay(delay)
    }
    
    /// Returns a const raw pointer to the internal GstElement
    unsafe fn gst_pipeline(&self) -> *const GstPipeline{
        self.as_pipeline().gst_pipeline()
    }
    
    /// Returns a mut raw pointer to the internal GstElement
    unsafe fn gst_pipeline_mut(&mut self) -> *mut GstPipeline{
        self.as_pipeline_mut().gst_pipeline_mut()
    }
}

impl PipelineT for Pipeline{
    fn as_pipeline(&self) -> &Pipeline{
        self
    }
    
    fn as_pipeline_mut(&mut self) -> &mut Pipeline{
        self
    }
    
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

impl<P:PipelineT> BinT for P{
    fn as_bin(&self) -> &Bin{
        &self.as_pipeline().pipeline
    }
    
    fn as_bin_mut(&mut self) -> &mut Bin{
        &mut self.as_pipeline_mut().pipeline
    }
}

impl ::Transfer for Pipeline{
    unsafe fn transfer(self) -> *mut GstElement{
        self.pipeline.transfer()
    }
}
