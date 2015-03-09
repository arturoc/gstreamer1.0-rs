use ffi::*;
use bus::Bus;
use util::*;

use libc::c_void;
use std::thread;

unsafe impl Sync for GstElement {}
unsafe impl Send for GstElement {}
unsafe impl Sync for Element {}
unsafe impl Send for Element {}

pub struct Element{
    element: *mut GstElement,
    speed: f64,
    last_pos_ns: i64
}

impl Drop for Element{
	fn drop(&mut self){
		self.set_state(GST_STATE_NULL);
		self.get_state(-1);
		unsafe{
			gst_object_unref(self.gst_element() as *mut c_void);
		}
	}
}

impl Element{
    pub fn new(element_name: &str, name: &str) -> Option<Element>{
        unsafe{
            let element = gst_element_factory_make(to_c_str!(element_name), to_c_str!(name));
            if element != ptr::null_mut::<GstElement>(){
                Some( Element{element: element, speed: 1.0, last_pos_ns: 0} )
            }else{
				println!("Erroro creating {} return {:?}",element_name, element);
                None
            }
        }
    }

    pub fn factory_make(element: &str, name: &str) -> Option<Element>{
		Element::new(element,name)
	}
    
    pub fn new_from_gst_element(element: *mut GstElement) -> Option<Element>{
		if element != ptr::null_mut::<GstElement>(){
			Some( Element{element: element, speed: 1.0, last_pos_ns: 0} )
		}else{
			None
		}
    }
    
    pub fn set<T>(&self, name: &str, value: T){
        unsafe{
            g_object_set(self.gst_element() as *mut  c_void, to_c_str!(name), value, ptr::null::<gchar>());
        }
    }
    
}

pub trait ElementT{
    
    fn link(&mut self, dst: &mut ElementT) -> bool;
    
    fn unlink(&mut self, dst: &mut ElementT);
    
    fn bus(&self) -> Option<Bus>;
    
    fn name(&self) -> String;
    
    fn set_name(&mut self, name: &str);
    
    fn set_state(&mut self, state: GstState) -> GstStateChangeReturn;
    
    fn get_state(&self, timeout: GstClockTime) -> (GstState, GstState, GstStateChangeReturn);
    
    fn send_event(&mut self, event: *mut GstEvent) -> bool;
    
    fn seek_simple(&mut self, format: GstFormat, flags: GstSeekFlags, pos: i64) -> bool;
    
    fn seek(&mut self, rate: f64, format: GstFormat, flags: GstSeekFlags, start_type: GstSeekType, start: i64, stop_type: GstSeekType, stop: i64) -> bool;
    
    fn seek_async(&mut self, rate: f64, format: GstFormat, flags: GstSeekFlags, start_type: GstSeekType, start: i64, stop_type: GstSeekType, stop: i64);
    
    fn query_duration(&self, format: GstFormat) -> Option<i64>;
    
    fn query_position(&self, format: GstFormat) -> Option<i64>;
    
    fn duration_ns(&self) -> Option<i64>;
    
    fn duration_s(&self) -> Option<f64>;
    
    fn position_ns(&self) -> i64;
    
    fn position_pct(&self) -> Option<f64>;
    
    fn position_s(&self) -> f64;
    
    fn speed(&self) -> f64;
    
    fn set_position_ns(&mut self, ns: i64) -> bool;
    
    fn set_position_s(&mut self, s: f64) -> bool;
    
    fn set_position_pct(&mut self, pct: f64) -> bool;
    
    fn set_speed(&mut self, speed: f64) -> bool;
    
    fn set_position_ns_async(&mut self, ns: i64);
    
    fn set_position_s_async(&mut self, s: f64);
    
    fn set_position_pct_async(&mut self, pct: f64) -> bool;
    
    fn set_speed_async(&mut self, speed: f64) -> bool;
    
    unsafe fn gst_element(&self) -> *const GstElement;
    
    unsafe fn gst_element_mut(&mut self) -> *mut GstElement;
    
    // fn set<T>(&self, name: &str, value: T);
    
    fn set_null_state(&mut self);
    
    fn set_ready_state(&mut self);
    
    fn pause(&mut self);
    
    fn play(&mut self);
    
    fn is_paused(&self) -> bool;
    
    fn is_playing(&self) -> bool;
    
    fn is_null_state(&self) -> bool;
    
    fn is_ready_state(&self) -> bool;
}

impl ElementT for Element{
    
    fn link(&mut self, dst: &mut ElementT) -> bool{
        unsafe{
            gst_element_link(self.gst_element_mut(), dst.gst_element_mut()) == 1
        }
    }
    
    fn unlink(&mut self, dst: &mut ElementT){
        unsafe{
            gst_element_unlink(self.gst_element_mut(), dst.gst_element_mut());
        }
    }
    
    fn bus(&self) -> Option<Bus>{
        unsafe{
            Bus::new(gst_element_get_bus(mem::transmute(self.gst_element())),true)
        }
    }
    
    fn name(&self) -> String{
        unsafe{
            let c_str_name = gst_object_get_name(self.gst_element() as *mut GstObject);
            from_c_str!(c_str_name).to_string()
        }
    }
    
    fn set_name(&mut self, name: &str){
        unsafe{
            gst_object_set_name(self.gst_element() as *mut GstObject, to_c_str!(name));
        }
    }
    
    fn set_state(&mut self, state: GstState) -> GstStateChangeReturn{
        unsafe{
            gst_element_set_state(self.gst_element_mut(), state)
        }
    }
    
    fn get_state(&self, timeout: GstClockTime) -> (GstState, GstState, GstStateChangeReturn){
        let mut state: GstState = GST_STATE_NULL;
        let mut pending: GstState = GST_STATE_NULL;
        unsafe{
            let ret = gst_element_get_state(mem::transmute(self.gst_element()), &mut state, &mut pending, timeout);
            (state, pending, ret)
        }
    }
    
    fn send_event(&mut self, event: *mut GstEvent) -> bool{
        unsafe{
            gst_element_send_event(self.gst_element_mut(), event) == 1
        }
    }
    
    fn seek_simple(&mut self, format: GstFormat, flags: GstSeekFlags, pos: i64) -> bool{
        unsafe{
            gst_element_seek_simple(self.gst_element_mut(), format, flags, pos) == 1
        }
    }
    
    fn seek(&mut self, rate: f64, format: GstFormat, flags: GstSeekFlags, start_type: GstSeekType, start: i64, stop_type: GstSeekType, stop: i64) -> bool{
        unsafe{
            gst_element_seek(self.gst_element_mut(), rate, format, flags, start_type, start, stop_type, stop) == 1
        }
    }
    
    fn seek_async(&mut self, rate: f64, format: GstFormat, flags: GstSeekFlags, start_type: GstSeekType, start: i64, stop_type: GstSeekType, stop: i64){
        unsafe{
            let element: u64 = mem::transmute(self.element);
			gst_object_ref(mem::transmute(element));
            thread::spawn(move||{
                let mut state: GstState = GST_STATE_NULL;
                let mut pending: GstState = GST_STATE_NULL;
                gst_element_get_state(mem::transmute(element), &mut state, &mut pending, s_to_ns(1.0));
                gst_element_seek(mem::transmute(element), rate, format, flags, start_type, start, stop_type, stop);
				gst_object_unref(mem::transmute(element));
            });
        }
    }
    
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
    
    fn duration_ns(&self) -> Option<i64>{
        self.query_duration(GST_FORMAT_TIME)
    }
    
    fn duration_s(&self) -> Option<f64>{
        let duration_ns = self.duration_ns();
        match duration_ns{
            Some(t) => Some(ns_to_s(t as u64)),
            None => None
        }
    }
    
    fn position_ns(&self) -> i64{
        match self.query_position(GST_FORMAT_TIME){
            Some(t) => t,
            None => self.last_pos_ns
        }
    }
    
    fn position_pct(&self) -> Option<f64>{
        let pos = self.position_ns();
        let dur = self.duration_ns();
        if dur.is_some(){
            Some( pos as f64 / dur.unwrap() as f64 )
        }else{
            None
        }
    }
    
    fn position_s(&self) -> f64{
        ns_to_s(self.position_ns() as u64)
    }
    
    fn speed(&self) -> f64{
        self.speed
    }
    
    fn set_position_ns(&mut self, ns: i64) -> bool{
        let format = GST_FORMAT_TIME;
	    let flags = GST_SEEK_FLAG_FLUSH; // | GST_SEEK_FLAG_ACCURATE | 
	    let speed = self.speed;
        let ret = if speed > 0.0 {
			self.seek(speed, format,
					flags,
					GST_SEEK_TYPE_SET,
					ns,
					GST_SEEK_TYPE_SET,
					-1)
		} else {
			self.seek(speed, format,
					flags,
					GST_SEEK_TYPE_SET,
					0,
					GST_SEEK_TYPE_SET,
					ns)
		};
        if ret { 
            self.last_pos_ns = ns;
        }
        
        ret
    }
    
    fn set_position_s(&mut self, s: f64) -> bool{
        self.set_position_ns(s_to_ns(s) as i64)
    }
    
    fn set_position_pct(&mut self, pct: f64) -> bool{
        let dur = self.duration_ns();
        match dur{
            Some(t) =>  self.set_position_ns((t as f64 * pct) as i64),
            None => false
        }
    }
    
    fn set_speed(&mut self, speed: f64) -> bool{
        let format = GST_FORMAT_TIME;
	    let flags = GST_SEEK_FLAG_SKIP | GST_SEEK_FLAG_ACCURATE | GST_SEEK_FLAG_FLUSH;
        if speed==0.0 {
            self.speed = speed;
            return self.set_state(GST_STATE_PAUSED) != GST_STATE_CHANGE_FAILURE;
        }
        
        let pos_opt = self.query_position(GST_FORMAT_TIME);
        if pos_opt.is_none(){
            return false;
        }
        
        let pos = pos_opt.unwrap();

        let ret = if speed > 0.0 {
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
                };
                
        if ret{
            self.speed = speed;
        }
        
        ret
            
    }
    
    fn set_position_ns_async(&mut self, ns: i64){
        let format = GST_FORMAT_TIME;
	    let flags = GST_SEEK_FLAG_ACCURATE | GST_SEEK_FLAG_FLUSH;
	    let speed = self.speed;
        if speed > 0.0 {
            self.seek_async(speed, format,
                    flags,
                    GST_SEEK_TYPE_SET,
                    ns,
                    GST_SEEK_TYPE_SET,
                    -1);
        } else {
            self.seek_async(speed, format,
                    flags,
                    GST_SEEK_TYPE_SET,
                    0,
                    GST_SEEK_TYPE_SET,
                    ns);
        }
        self.last_pos_ns = ns;
    }
    
    fn set_position_s_async(&mut self, s: f64){
        self.set_position_ns_async(s_to_ns(s) as i64);
    }
    
    fn set_position_pct_async(&mut self, pct: f64) -> bool{
        let dur = self.duration_ns();
        match dur{
            Some(t) =>  {self.set_position_ns_async((t as f64 * pct) as i64); true},
            None => false
        }
    }
    
    fn set_speed_async(&mut self, speed: f64) -> bool{
        let format = GST_FORMAT_TIME;
	    let flags = GST_SEEK_FLAG_SKIP | GST_SEEK_FLAG_ACCURATE | GST_SEEK_FLAG_FLUSH;
        self.speed = speed;
        if speed==0.0 {
            return self.set_state(GST_STATE_PAUSED) != GST_STATE_CHANGE_FAILURE;
        }
        
        let pos_opt = self.query_position(GST_FORMAT_TIME);
        if pos_opt.is_none(){
            return false;
        }
        
        let pos = pos_opt.unwrap();

        if speed > 0.0 {
            self.seek_async(speed, format,
                    flags,
                    GST_SEEK_TYPE_SET,
                    pos,
                    GST_SEEK_TYPE_SET,
                    -1);
            true
        } else {
            self.seek_async(speed, format,
                    flags,
                    GST_SEEK_TYPE_SET,
                    0,
                    GST_SEEK_TYPE_SET,
                    pos);
            true
        }
    }
    
    unsafe fn gst_element(&self) -> *const GstElement{
        self.element
    }
    
    unsafe fn gst_element_mut(&mut self) -> *mut GstElement{
        mem::transmute(self.element)
    }
    
    /*fn set<T>(&self, name: &str, value: T){
        unsafe{
            g_object_set(self.gst_element() as *mut  c_void, name.to_c_str().as_ptr(), value, ptr::null::<gchar>());
        }
    }*/
    
    fn set_null_state(&mut self){
        self.set_state(GST_STATE_NULL);
    }
    
    fn set_ready_state(&mut self){
        self.set_state(GST_STATE_READY);
    }
    
    fn pause(&mut self){
        self.set_state(GST_STATE_PAUSED);
    }
    
    fn play(&mut self){
        self.set_state(GST_STATE_PLAYING);
    }
    
    fn is_paused(&self) -> bool{
        if let (GST_STATE_PAUSED, _pending, GST_STATE_CHANGE_SUCCESS) = self.get_state(-1){
			true
		}else{
			false
		}
    }
    
    fn is_playing(&self) -> bool{
        if let (GST_STATE_PLAYING, _pending, GST_STATE_CHANGE_SUCCESS) = self.get_state(-1){
			true
		}else{
			false
		}
    }
    
    fn is_null_state(&self) -> bool{
        if let (GST_STATE_NULL, _pending, GST_STATE_CHANGE_SUCCESS) = self.get_state(-1){
			true
		}else{
			false
		}
    }
    
    fn is_ready_state(&self) -> bool{
        if let (GST_STATE_READY, _pending, GST_STATE_CHANGE_SUCCESS) = self.get_state(-1){
			true
		}else{
			false
		}
    }
}
