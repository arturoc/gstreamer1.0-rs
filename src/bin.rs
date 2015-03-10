use ffi::*;
use element::Element;
use element::ElementT;
use bus::Bus;
use util::*;

/**
GstBin is an element that can contain other GstElement, allowing them to be managed as a group. Pads from the child elements can be ghosted to the bin, see GstGhostPad. This makes the bin look like any other elements and enables creation of higher-level abstraction elements.

A new GstBin is created with gst_bin_new(). Use a GstPipeline instead if you want to create a toplevel bin because a normal bin doesn't have a bus or handle clock distribution of its own.

After the bin has been created you will typically add elements to it with gst_bin_add(). You can remove elements with gst_bin_remove().

An element can be retrieved from a bin with gst_bin_get_by_name(), using the elements name. gst_bin_get_by_name_recurse_up() is mainly used for internal purposes and will query the parent bins when the element is not found in the current bin.

An iterator of elements in a bin can be retrieved with gst_bin_iterate_elements(). Various other iterators exist to retrieve the elements in a bin.

gst_object_unref() is used to drop your reference to the bin.

The “element-added” signal is fired whenever a new element is added to the bin. Likewise the “element-removed” signal is fired whenever an element is removed from the bin.
Notes

A GstBin internally intercepts every GstMessage posted by its children and implements the following default behaviour for each of them:

GST_MESSAGE_EOS
	

This message is only posted by sinks in the PLAYING state. If all sinks posted the EOS message, this bin will post and EOS message upwards.

GST_MESSAGE_SEGMENT_START
	

just collected and never forwarded upwards. The messages are used to decide when all elements have completed playback of their segment.

GST_MESSAGE_SEGMENT_DONE
	

Is posted by GstBin when all elements that posted a SEGMENT_START have posted a SEGMENT_DONE.

GST_MESSAGE_DURATION_CHANGED
	

Is posted by an element that detected a change in the stream duration. The default bin behaviour is to clear any cached duration values so that the next duration query will perform a full duration recalculation. The duration change is posted to the application so that it can refetch the new duration with a duration query. Note that these messages can be posted before the bin is prerolled, in which case the duration query might fail.

GST_MESSAGE_CLOCK_LOST
	

This message is posted by an element when it can no longer provide a clock. The default bin behaviour is to check if the lost clock was the one provided by the bin. If so and the bin is currently in the PLAYING state, the message is forwarded to the bin parent. This message is also generated when a clock provider is removed from the bin. If this message is received by the application, it should PAUSE the pipeline and set it back to PLAYING to force a new clock distribution.

GST_MESSAGE_CLOCK_PROVIDE
	

This message is generated when an element can provide a clock. This mostly happens when a new clock provider is added to the bin. The default behaviour of the bin is to mark the currently selected clock as dirty, which will perform a clock recalculation the next time the bin is asked to provide a clock. This message is never sent tot the application but is forwarded to the parent of the bin.

OTHERS
	

posted upwards.

A GstBin implements the following default behaviour for answering to a GstQuery:

GST_QUERY_DURATION
	

If the query has been asked before with the same format and the bin is a toplevel bin (ie. has no parent), use the cached previous value. If no previous value was cached, the query is sent to all sink elements in the bin and the MAXIMUM of all values is returned. If the bin is a toplevel bin the value is cached. If no sinks are available in the bin, the query fails.

GST_QUERY_POSITION
	

The query is sent to all sink elements in the bin and the MAXIMUM of all values is returned. If no sinks are available in the bin, the query fails.

OTHERS
	

the query is forwarded to all sink elements, the result of the first sink that answers the query successfully is returned. If no sink is in the bin, the query fails.

A GstBin will by default forward any event sent to it to all sink elements. If all the sinks return TRUE, the bin will also return TRUE, else FALSE is returned. If no sinks are in the bin, the event handler will return TRUE.*/
pub struct Bin{
    pub bin: Element
}

unsafe impl Sync for Bin {}
unsafe impl Send for Bin {}

impl Bin{
    /// Creates a new bin with the given name.
    pub fn new(name: &str) -> Option<Bin>{
        unsafe{
            let bin = gst_bin_new(to_c_str!(name));
            if bin != ptr::null_mut(){
	            gst_object_ref_sink(mem::transmute(bin));
	            Bin::new_from_gst_bin(bin as *mut GstBin)
	        }else{
	            None
	        }
        }
    }
    
    /// Creates a new bin from an already existing raw pointer to a GstBin.
    /// The passed element has to be fully referenced
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



pub trait BinT: ElementT{
    /// Adds the given element to the bin. Sets the element's parent, 
    /// and thus adds a reference.
    ///
    /// If the element's pads are linked to other pads, the pads will 
    /// be unlinked before the element is added to the bin.
    ///
    /// > When you add an element to an already-running pipeline, you 
    /// > will have to take care to set the state of the newly-added 
    /// > element to the desired state (usually PLAYING or PAUSED, same 
    /// > you set the pipeline to originally) with Element::set_state(), 
    /// > or use gst_element_sync_state_with_parent(). The bin or pipeline 
    /// > will not take care of this for you. 
    fn add(&self, element: &ElementT) -> bool;
    
    /// Remove the element from its associated bin.
    ///
    /// If the element's pads are linked to other pads, the pads will be 
    /// unlinked before the element is removed from the bin.
    fn remove(&self, element: &ElementT) -> bool;
    
    /// Get the element with the given name from this bin.
    ///
    /// Returns None if no element with the given name is found in the bin.
    fn get_by_name(&self, name: &str) -> Option<Element>;
    
    /// Query bin for the current latency using and reconfigures this latency
    /// to all the elements with a LATENCY event.
	///
	/// This method is typically called on the pipeline when a 
	/// GST_MESSAGE_LATENCY is posted on the bus.
	///
	/// This function simply emits the 'do-latency' signal so any custom 
	/// latency calculations will be performed.
    fn recalculate_latency(&self) -> bool;
    
    /// If set to true, the bin will handle asynchronous state changes. 
    /// This should be used only if the bin subclass is modifying the state
    /// of its children on its own
    fn set_async_handling(&self, async: bool);
    
    /// Forward all children messages, even those that would normally be 
    /// filtered by the bin. This can be interesting when one wants to be
    /// notified of the EOS state of individual elements, for example.
	///
	/// The messages are converted to an ELEMENT message with the bin as the
	/// source. The structure of the message is named 'GstBinForwarded' and
	/// contains a field named 'message' of type GST_TYPE_MESSAGE that
	/// contains the original forwarded message.
    fn set_message_forward(&self, forward: bool);
    
    /// Returns a const raw pointer to the internal GstElement
    unsafe fn gst_bin(&self) -> *const GstBin;
    
    /// Returns a mut raw pointer to the internal GstElement
    unsafe fn gst_bin_mut(&mut self) -> *mut GstBin;
}

impl BinT for Bin{
    fn add(&self, element: &ElementT) -> bool{
        unsafe{
            gst_bin_add(self.gst_bin() as *mut GstBin, mem::transmute(element.gst_element())) == 1
        }
    }
    
    fn remove(&self, element: &ElementT) -> bool{
        unsafe{
            gst_bin_remove(self.gst_bin() as *mut GstBin, mem::transmute(element.gst_element())) == 1
        }
    }
    
    fn get_by_name(&self, name: &str) -> Option<Element>{
        unsafe{
            let element = gst_bin_get_by_name(self.gst_bin() as *mut GstBin, to_c_str!(name));
            Element::new_from_gst_element(element)
        }
    }
    
    fn recalculate_latency(&self) -> bool{
        unsafe{
            gst_bin_recalculate_latency(self.gst_bin() as *mut GstBin) == 1
        }
    }
    
    fn set_async_handling(&self, async: bool){
        self.bin.set("async-handling", async);
    }
    
    fn set_message_forward(&self, forward: bool){
        self.bin.set("message-forward", forward);
    }
    
    unsafe fn gst_bin(&self) -> *const GstBin{
        self.bin.gst_element() as *const GstBin
    }
    
    unsafe fn gst_bin_mut(&mut self) -> *mut GstBin{
        self.bin.gst_element() as *mut GstBin
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
    
    unsafe fn send_event(&mut self, event: *mut GstEvent) -> bool{
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
