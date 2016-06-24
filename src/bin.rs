use ffi::*;
use element::Element;
use element::ElementT;
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
    bin: Element
}

unsafe impl Sync for Bin {}
unsafe impl Send for Bin {}

impl Bin{
    /// Creates a new bin with the given name.
    pub fn new(name: &str) -> Option<Bin>{
        unsafe{
            let cname = CString::new(name).unwrap();
            let name = if name != "" {
                cname.as_ptr()
            } else {
                ptr::null()
            };
            let bin = gst_bin_new(name);
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
}


pub trait BinT: ElementT{
    fn as_bin(&self) -> &Bin;
    fn as_bin_mut(&mut self) -> &mut Bin;

    fn to_bin(&self) -> Bin{
        Bin{bin: self.to_element()}
    }

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
    fn add<E:ElementT>(&mut self, element: E) -> bool{
        unsafe{
            gst_bin_add(self.gst_bin_mut(), element.transfer() as *mut GstElement) == 1
        }
    }

    fn add_and_link<E:ElementT>(&mut self, mut src: E, mut sink: E) -> bool{
        self.add(src.to_element()) &&
        self.add(sink.to_element()) &&
        src.link(&mut sink)
    }

    /// Remove the element from its associated bin.
    ///
    /// If the element's pads are linked to other pads, the pads will be
    /// unlinked before the element is removed from the bin.
    fn remove(&mut self, element: &ElementT) -> bool{
        unsafe{
            gst_bin_remove(self.gst_bin_mut(), mem::transmute(element.gst_element())) == 1
        }
    }

    /// Get the element with the given name from this bin.
    ///
    /// Returns None if no element with the given name is found in the bin.
    fn get_by_name(&self, name: &str) -> Option<Element>{
        let cname = CString::new(name).unwrap();
        unsafe{
            let element = gst_bin_get_by_name(self.gst_bin() as *mut GstBin, cname.as_ptr());
            Element::new_from_gst_element(element)
        }
    }

    /// Query bin for the current latency using and reconfigures this latency
    /// to all the elements with a LATENCY event.
	///
	/// This method is typically called on the pipeline when a
	/// GST_MESSAGE_LATENCY is posted on the bus.
	///
	/// This function simply emits the 'do-latency' signal so any custom
	/// latency calculations will be performed.
    fn recalculate_latency(&mut self) -> bool{
        unsafe{
            gst_bin_recalculate_latency(self.gst_bin() as *mut GstBin) == 1
        }
    }

    /// If set to true, the bin will handle asynchronous state changes.
    /// This should be used only if the bin subclass is modifying the state
    /// of its children on its own
    fn set_async_handling(&mut self, async: bool){
        self.as_bin_mut().set("async-handling", async);
    }

    /// Forward all children messages, even those that would normally be
    /// filtered by the bin. This can be interesting when one wants to be
    /// notified of the EOS state of individual elements, for example.
	///
	/// The messages are converted to an ELEMENT message with the bin as the
	/// source. The structure of the message is named 'GstBinForwarded' and
	/// contains a field named 'message' of type GST_TYPE_MESSAGE that
	/// contains the original forwarded message.
    fn set_message_forward(&mut self, forward: bool){
        self.as_bin_mut().set("message-forward", forward);
    }

    /// Returns a const raw pointer to the internal GstElement
    unsafe fn gst_bin(&self) -> *const GstBin{
        self.as_bin().gst_bin()
    }

    /// Returns a mut raw pointer to the internal GstElement
    unsafe fn gst_bin_mut(&mut self) -> *mut GstBin{
        self.as_bin_mut().gst_bin_mut()
    }
}

impl BinT for Bin{
    fn as_bin(&self) -> &Bin{
        self
    }

    fn as_bin_mut(&mut self) -> &mut Bin{
        self
    }

    unsafe fn gst_bin(&self) -> *const GstBin{
        self.bin.gst_element() as *const GstBin
    }

    unsafe fn gst_bin_mut(&mut self) -> *mut GstBin{
        self.bin.gst_element() as *mut GstBin
    }
}

impl<B:BinT> ElementT for B{
    fn as_element(&self) -> &Element{
        &self.as_bin().bin
    }

    fn as_element_mut(&mut self) -> &mut Element{
        &mut self.as_bin_mut().bin
    }
}

impl ::Transfer for Bin{
    unsafe fn transfer(self) -> *mut GstElement{
        self.bin.transfer()
    }
}
