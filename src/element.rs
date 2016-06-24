use ffi::*;
use bus::Bus;
use util::*;
use pad::Pad;

use std::os::raw::c_void;

unsafe impl Sync for GstElement {}
unsafe impl Send for GstElement {}
unsafe impl Sync for Element {}
unsafe impl Send for Element {}

pub struct Element{
    element: *mut GstElement
}

impl Drop for Element{
	fn drop(&mut self){
		unsafe{
			gst_object_unref(self.element as *mut c_void);
		}
	}
}

impl Element {
    /// Use a factory `factory_name` to create an element with name `element_name`.
    pub fn new(factory_name: &str, element_name: &str) -> Option<Element> {
        let cname = CString::new(element_name).unwrap();
        let element_cname = CString::new(factory_name).unwrap();
        unsafe{
            let element_name = if element_name != "" {
                cname.as_ptr()
            } else {
                ptr::null()
            };
            let element = gst_element_factory_make(element_cname.as_ptr(), element_name);
            if element != ptr::null_mut::<GstElement>() {
                gst_object_ref_sink(mem::transmute(element));
                Some( Element{element: element} )
            } else {
				println!("Error creating {} return {:?}", factory_name, element);
                None
            }
        }
    }

    pub fn factory_make(element: &str, name: &str) -> Option<Element>{
		Element::new(element,name)
	}

    pub unsafe fn new_from_gst_element(element: *mut GstElement) -> Option<Element>{
		if element != ptr::null_mut::<GstElement>(){
			Some( Element{element: element} )
		}else{
			None
		}
    }

    /// Link several elements in succession.
    ///
    /// Calling `Element::link_many(&mut[&mut A, &mut B, &mut C, ...])` will attempt to link
    /// `A->B`, then `B->C`, ... . In case of error, this function returns immediately, without
    /// attempting to unlink.
    ///
    /// See `ElementT::link` for more details about linking.
    ///
    /// Make sure you have added your elements to a bin or pipeline with
    /// `Bin::add()`` **before** trying to link them.
    ///
    /// returns `true` if all elements could be linked, `false` otherwise.
    pub fn link_many(items: &mut [&mut ElementT]) -> bool {
        let mut latest : Option<&mut ElementT> = None;
        for mut item in items {
            let item : &mut ElementT = *item;
            if let Some(prev) = latest {
                if !prev.link(item) {
                    return false;
                }
            }
            latest = Some(item)
        }
        true
    }
}

/// http://gstreamer.freedesktop.org/data/doc/gstreamer/head/gstreamer/html/GstElement.html
pub trait ElementT: ::Transfer{
    fn as_element(&self) -> &Element;

    fn as_element_mut(&mut self) -> &mut Element;

    fn to_element(&self) -> Element{
		let element = Element{element: self.as_element().element};
		unsafe{ gst_object_ref(element.element as *mut c_void); }
		element
    }

    /// Links this element to dest .
    /// The link must be from source to destination; the other direction
    /// will not be tried. The function looks for existing pads that aren't
    /// linked yet. It will request new pads if necessary. Such pads need
    /// to be released manually when unlinking.
    /// If multiple links are possible, only one is established.
	///
	/// Make sure you have added your elements to a bin or pipeline with
	/// Bin::add() **before** trying to link them.
	///
	/// returns true if the elements could be linked, false otherwise.
    fn link(&mut self, dst: &mut ElementT) -> bool{
        unsafe{
            gst_element_link(self.gst_element_mut(), dst.gst_element_mut()) == 1
        }
    }

    /// Unlinks all source pads of the this element with all sink pads
    /// of the sink element to which they are linked.
	///
	/// If the link has been made using Element::link(), it could have
	/// created a requestpad, which has to be released using
	/// gst_element_release_request_pad().
    fn unlink(&mut self, dst: &mut ElementT){
        unsafe{
            gst_element_unlink(self.gst_element_mut(), dst.gst_element_mut());
        }
    }

	/// Returns the bus of the element. Note that only a Pipeline
	/// will provide a bus for the application.
    fn bus(&self) -> Option<Bus>{
        unsafe{
            Bus::new(gst_element_get_bus(mem::transmute(self.gst_element())),true)
        }
    }

    /// Returns the name of the element
    fn name(&self) -> String{
        unsafe{
            let c_str_name = gst_object_get_name(self.gst_element() as *mut GstObject);
            from_c_str!(c_str_name).to_string()
        }
    }

    /// Sets the name of the element
    fn set_name(&mut self, name: &str){
        let cname = CString::new(name).unwrap();
        unsafe{
            gst_object_set_name(self.gst_element() as *mut GstObject, cname.as_ptr());
        }
    }

    /// Sets the state of the element. This function will try to
    /// set the requested state by going through all the intermediary
    /// states and calling the class's state change function for each.
	///
	/// This function can return GST_STATE_CHANGE_ASYNC, in which case
	/// the element will perform the remainder of the state change
	/// asynchronously in another thread. An application can use
	/// get_state() to wait for the completion of the state
	/// change or it can wait for a GST_MESSAGE_ASYNC_DONE or
	/// GST_MESSAGE_STATE_CHANGED on the bus.
	///
	/// State changes to GST_STATE_READY or GST_STATE_NULL
	/// never return GST_STATE_CHANGE_ASYNC.
    fn set_state(&mut self, state: GstState) -> GstStateChangeReturn{
        unsafe{
            gst_element_set_state(self.gst_element_mut(), state)
        }
    }

    /// Gets the state of the element.
	///
	/// For elements that performed an ASYNC state change, as reported
	/// by set_state(), this function will block up to the specified
	/// timeout value for the state change to complete. If the element
	/// completes the state change or goes into an error, this function
	/// returns immediately with a return value of GST_STATE_CHANGE_SUCCESS
	/// or GST_STATE_CHANGE_FAILURE respectively.
	///
	/// For elements that did not return GST_STATE_CHANGE_ASYNC, this function
	/// returns the current and pending state immediately.
	///
	/// This function returns GST_STATE_CHANGE_NO_PREROLL if the element
	/// successfully changed its state but is not able to provide data yet.
	/// This mostly happens for live sources that only produce data in
	/// GST_STATE_PLAYING. While the state change return is equivalent to
	/// GST_STATE_CHANGE_SUCCESS, it is returned to the application to signal
	/// that some sink elements might not be able to complete their state change
	/// because an element is not producing data to complete the preroll.
	/// When setting the element to playing, the preroll will complete and
	/// playback will start.
	/// Returns
	///
	/// GST_STATE_CHANGE_SUCCESS if the element has no more pending state and
	/// the last state change succeeded, GST_STATE_CHANGE_ASYNC if the element
	/// is still performing a state change or GST_STATE_CHANGE_FAILURE if
	/// the last state change failed.
    fn get_state(&self, timeout: GstClockTime) -> (GstState, GstState, GstStateChangeReturn){
        let mut state: GstState = GST_STATE_NULL;
        let mut pending: GstState = GST_STATE_NULL;
        unsafe{
            let ret = gst_element_get_state(mem::transmute(self.gst_element()), &mut state, &mut pending, timeout);
            (state, pending, ret)
        }
    }

    /// Sends an event to an element. If the element doesn't implement an event
    /// handler, the event will be pushed on a random linked sink pad for
    /// downstream events or a random linked source pad for upstream events.
	///
	/// This function takes ownership of the provided event so you should
	/// gst_event_ref() it if you want to reuse the event after this call.
    unsafe fn send_event(&mut self, event: *mut GstEvent) -> bool{
        gst_element_send_event(self.gst_element_mut(), event) == 1
    }

    /// Simple API to perform a seek on the given element, meaning it just
    /// seeks to the given position relative to the start of the stream.
    /// For more complex operations like segment seeks (e.g. for looping)
    /// or changing the playback rate or seeking relative to the last
    /// configured playback segment you should use gst_element_seek().
	///
	/// In a completely prerolled PAUSED or PLAYING pipeline, seeking is
	/// always guaranteed to return TRUE on a seekable media type or FALSE
	/// when the media type is certainly not seekable (such as a live stream).
	///
	/// Some elements allow for seeking in the READY state, in this case
	/// they will store the seek event and execute it when they are put to
	/// PAUSED. If the element supports seek in READY, it will always return
	/// true when it receives the event in the READY state.
    fn seek_simple(&mut self, format: GstFormat, flags: GstSeekFlags, pos: i64) -> bool{
        unsafe{
            gst_element_seek_simple(self.gst_element_mut(), format, flags, pos) == 1
        }
    }

    /// Sends a seek event to an element. See [gst_event_new_seek()](http://gstreamer.freedesktop.org/data/doc/gstreamer/head/gstreamer/html/GstEvent.html#gst-event-new-seek)
    /// for the details of the parameters. The seek event is sent to the
    /// element using send_event().
    fn seek(&mut self, rate: f64, format: GstFormat, flags: GstSeekFlags, start_type: GstSeekType, start: i64, stop_type: GstSeekType, stop: i64) -> bool{
        unsafe{
            gst_element_seek(self.gst_element_mut(), rate, format, flags, start_type, start, stop_type, stop) == 1
        }
    }

    /// Queries an element (usually top-level pipeline or playbin element)
    /// for the total stream duration in nanoseconds. This query will only
    /// work once the pipeline is prerolled (i.e. reached PAUSED or PLAYING
    /// state). The application will receive an ASYNC_DONE message on the
    /// pipeline bus when that is the case.
	///
	/// If the duration changes for some reason, you will get a
	/// DURATION_CHANGED message on the pipeline bus, in which case you should
	/// re-query the duration using this function.
    fn query_duration(&self, format: GstFormat) -> Option<i64>{
        unsafe{
            let mut duration = 0;
            if gst_element_query_duration(mem::transmute(self.gst_element()), format, &mut duration) == 1{
                Some(duration)
            }else{
                None
            }
        }
    }

    /// Queries an element (usually top-level pipeline or playbin element)
    /// for the stream position in nanoseconds. This will be a value between 0
    /// and the stream duration (if the stream duration is known). This query
    /// will usually only work once the pipeline is prerolled (i.e. reached
    /// PAUSED or PLAYING state). The application will receive an ASYNC_DONE
    /// message on the pipeline bus when that is the case.
    fn query_position(&self, format: GstFormat) -> Option<i64>{
        unsafe{
            let mut pos = 0;
            if gst_element_query_position(mem::transmute(self.gst_element()), format, &mut pos) == 1{
                Some(pos)
            }else{
                None
            }
        }
    }

    /// Shortcut for query_duration with format == TIME
    fn duration_ns(&self) -> Option<i64>{
        self.query_duration(GST_FORMAT_TIME)
    }

    /// Shortcut for query_duration with format == TIME and conversion to
    /// seconds
    fn duration_s(&self) -> Option<f64>{
        let duration_ns = self.duration_ns();
        match duration_ns{
            Some(t) => Some(ns_to_s(t as u64)),
            None => None
        }
    }

    /// Shortcut for query_position with format == TIME
    fn position_ns(&self) -> Option<i64>{
        self.query_position(GST_FORMAT_TIME)
    }

    /// Shortcut for query_position with format == TIME and conversion to
    /// pct as 0..1
    fn position_pct(&self) -> Option<f64>{
        let pos = self.position_ns();
        let dur = self.duration_ns();
        if dur.is_some() && pos.is_some(){
            Some( pos.unwrap() as f64 / dur.unwrap() as f64 )
        }else{
            None
        }
    }

    /// Shortcut for query_position with format == TIME and conversion to
    /// seconds
    fn position_s(&self) -> Option<f64>{
        if let Some(pos_ns) = self.position_ns(){
        	Some(ns_to_s(pos_ns as u64))
        }else{
            None
        }
    }

    /// Shortcut for seek to a ceratin position in ns
    fn set_position_ns(&mut self, ns: i64) -> bool{
        let format = GST_FORMAT_TIME;
	    let flags = GST_SEEK_FLAG_FLUSH; // | GST_SEEK_FLAG_ACCURATE |
		self.seek_simple(format, flags,	ns)
    }

    /// Shortcut for seek to a ceratin position in secs
    fn set_position_s(&mut self, s: f64) -> bool{
        self.set_position_ns(s_to_ns(s) as i64)
    }

    /// Shortcut for seek to a ceratin position in pcs as 0..1
    fn set_position_pct(&mut self, pct: f64) -> bool{
        let dur = self.duration_ns();
        match dur{
            Some(t) =>  self.set_position_ns((t as f64 * pct) as i64),
            None => false
        }
    }

    /// Shortcut for seek to the current position but change in playback
    /// rate
    fn set_speed(&mut self, speed: f64) -> bool{
        let format = GST_FORMAT_TIME;
	    let flags = GST_SEEK_FLAG_SKIP | GST_SEEK_FLAG_ACCURATE | GST_SEEK_FLAG_FLUSH;
        if speed==0.0 {
            return self.set_state(GST_STATE_PAUSED) != GST_STATE_CHANGE_FAILURE;
        }

        let pos_opt = self.query_position(GST_FORMAT_TIME);
        if pos_opt.is_none(){
            return false;
        }

        let pos = pos_opt.unwrap();

        if speed > 0.0 {
            self.seek(speed, format,
                    flags,
                    GST_SEEK_TYPE_SET,
                    pos,
                    GST_SEEK_TYPE_SET,
                    -1)
        } else {
            self.seek(speed, format,
                    flags,
                    GST_SEEK_TYPE_SET,
                    0,
                    GST_SEEK_TYPE_SET,
                    pos)
        }
    }

    /// shortcut to set_state with state == NULL
    fn set_null_state(&mut self) -> GstStateChangeReturn{
        self.set_state(GST_STATE_NULL)
    }

    /// shortcut to set_state with state == READY
    fn set_ready_state(&mut self) -> GstStateChangeReturn{
        self.set_state(GST_STATE_READY)
    }

    /// shortcut to set_state with state == PAUSED
    fn pause(&mut self) -> GstStateChangeReturn{
        self.set_state(GST_STATE_PAUSED)
    }

    /// shortcut to set_state with state == PLAYING
    fn play(&mut self) -> GstStateChangeReturn{
        self.set_state(GST_STATE_PLAYING)
    }

    /// shortcut to query the state and returns state == PAUSED
    fn is_paused(&self) -> bool{
        if let (GST_STATE_PAUSED, _pending, GST_STATE_CHANGE_SUCCESS) = self.get_state(GST_CLOCK_TIME_NONE){
			true
		}else{
			false
		}
    }

    /// shortcut to query the state and returns state == PLAYING
    fn is_playing(&self) -> bool{
        if let (GST_STATE_PLAYING, _pending, GST_STATE_CHANGE_SUCCESS) = self.get_state(GST_CLOCK_TIME_NONE){
			true
		}else{
			false
		}
    }

    /// shortcut to query the state and returns state == NULL
    fn is_null_state(&self) -> bool{
        if let (GST_STATE_NULL, _pending, GST_STATE_CHANGE_SUCCESS) = self.get_state(GST_CLOCK_TIME_NONE){
			true
		}else{
			false
		}
    }

    /// shortcut to query the state and returns state == READY
    fn is_ready_state(&self) -> bool{
        if let (GST_STATE_READY, _pending, GST_STATE_CHANGE_SUCCESS) = self.get_state(GST_CLOCK_TIME_NONE){
			true
		}else{
			false
		}
    }

    // Retrieves a pad from element by name.
    // This version only retrieves already-existing (i.e. 'static') pads.
    fn static_pad(&mut self, name: &str) -> Option<Pad>{
        let cname = CString::new(name).unwrap();
        unsafe{
            let pad = gst_element_get_static_pad(self.gst_element_mut(), cname.as_ptr());
            Pad::new(pad)
        }
    }

    /// Returns a const raw pointer to the internal GstElement
    unsafe fn gst_element(&self) -> *const GstElement{
        self.as_element().gst_element()
    }

    /// Returns a mutable raw pointer to the internal GstElement
    unsafe fn gst_element_mut(&mut self) -> *mut GstElement{
        self.as_element_mut().gst_element_mut()
    }

    fn set<T>(&mut self, name: &str, value: T)
    	where Self:Sized,
            T: Property {
        value.set_to(name, self)
    }

    unsafe fn signal_connect<T>(&mut self, signal: &str, callback: GCallback, data: &mut T)
        where Self:Sized{
        let csignal = CString::new(signal).unwrap();
        g_signal_connect_data(self.gst_element() as *mut c_void, csignal.as_ptr(), callback, mem::transmute(data), None, 0);
    }
}

impl ElementT for Element{
    fn as_element(&self) -> &Element{
        self
    }

    fn as_element_mut(&mut self) -> &mut Element{
        self
    }

    unsafe fn gst_element(&self) -> *const GstElement{
        self.element
    }

    unsafe fn gst_element_mut(&mut self) -> *mut GstElement{
        self.element
    }
}

impl ::Transfer for Element{
    unsafe fn transfer(self) -> *mut GstElement{
        let element = self.element;
        mem::forget(self);
        element
    }
}

impl ::Reference for Element{
    fn reference(&self) -> Element{
        self.to_element()
    }
}

impl ::FromGValue for Element{
    fn from_gvalue(value: &GValue) -> Option<Element>{
        unsafe{
            if g_type_check_value_holds(mem::transmute(value), gst_element_get_type()) != 0{
                let ptr = g_value_get_object(value);
                gst_object_ref(ptr);
                Element::new_from_gst_element(ptr as *mut GstElement)
            }else{
                None
            }
        }
    }
}

pub trait Property{
    fn set_to(&self, key: &str, e: &mut ElementT);
}

impl<'a> Property for &'a str{
    #[inline]
    fn set_to(&self, key: &str, e: &mut ElementT){
        let cname = CString::new(key).unwrap();
        let c_str = CString::new(*self).unwrap();
        unsafe{
            g_object_set(e.gst_element() as *mut  c_void, cname.as_ptr(), c_str.as_ptr(), ptr::null::<gchar>());
        }
    }
}

impl<'a> Property for &'a ElementT{
    #[inline]
    fn set_to(&self, key: &str, e: &mut ElementT){
        let cname = CString::new(key).unwrap();
        unsafe{
            g_object_set(e.gst_element() as *mut  c_void, cname.as_ptr(), self.gst_element(), ptr::null::<gchar>());
        }
    }
}

pub trait RawProperty: Clone{
    #[inline]
    fn set_raw_to(&self, key: &str, e: &mut ElementT){
        let cname = CString::new(key).unwrap();
        unsafe{
            g_object_set(e.gst_element() as *mut  c_void, cname.as_ptr(), self.clone(), ptr::null::<gchar>());
        }
    }
}

impl<R: RawProperty> Property for R{
    #[inline]
    fn set_to(&self, key: &str, e: &mut ElementT){
        self.set_raw_to(key, e);
    }
}

impl RawProperty for i8{}
impl RawProperty for u8{}
impl RawProperty for i16{}
impl RawProperty for u16{}
impl RawProperty for i32{}
impl RawProperty for u32{}
impl RawProperty for i64{}
impl RawProperty for u64{}
impl RawProperty for f32{}
impl RawProperty for f64{}
impl RawProperty for bool{}
