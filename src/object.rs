use glib::*;
use gobject::*;
use gst_sys::*;
use util::*;
use reference::{Reference, Ref};

use std::os::raw::{c_char};

pub struct Object{
    object: *mut GstObject,
}


impl Drop for Object{
	fn drop(&mut self){
		unsafe{
			gst_object_unref(self.object as *mut GstObject);
		}
	}
}

impl Object{

    pub unsafe fn new(object: *mut GstObject) -> Option<Object>{
        if object != ptr::null_mut(){
            Some(Object{ object: object })
        }else{
            None
        }
    }

    pub fn set_name(&mut self, name: &str) -> bool{
        let cname = CString::new(name).unwrap();
        unsafe{
            gst_object_set_name(self.object, cname.as_ptr()) != 0
        }
    }

    /// Returns the name of the object
    pub fn name(&self) -> String{
        unsafe{
            let c_str_name = gst_object_get_name(self.object);
            from_c_str!(c_str_name).to_string()
        }
    }

    pub fn flags(&self) -> u32{
        let object: &mut GstObject =
        unsafe{
            & mut* self.object
        };
        object.flags
    }

    pub fn is_flag_set(&self, flag: u32) -> bool{
        self.flags() & flag == flag
    }

    pub fn set_flag(&mut self, flag: u32){
        unsafe{
            let object: &mut GstObject = mem::transmute(self.object);
            object.flags |= flag
        }
    }

    pub fn unset_flag(&mut self, flag: u32){
        unsafe{
            let object: &mut GstObject = mem::transmute(self.object);
            object.flags |= !flag
        }
    }

    pub fn refcount(&self) -> u32{
        unsafe{
            let object: &mut GObject = mem::transmute(self.object);
            g_atomic_int_get(&object.ref_count as *const Volatile<u32> as *const i32) as u32
        }
    }

    pub fn lock<F: FnMut(&mut Object)>(&mut self, mut f: F){
        unsafe{
            let object: &mut GstObject = mem::transmute(self.object);
            g_mutex_lock(&mut object.lock);
            f(self);
            g_mutex_unlock(&mut object.lock);
        }
    }

    pub fn set_unique_name(&mut self) -> bool{
        unsafe{
            gst_object_set_name(self.object, ptr::null()) != 0
        }
    }

    pub fn set_parent(&mut self, parent: &Object) -> bool{
        unsafe{
            gst_object_set_parent(self.object, parent.object) != 0
        }
    }

    pub fn parent(&self) -> Option<Ref<Object>>{
        let parent = unsafe{ gst_object_get_parent(self.object) };
        if parent == ptr::null_mut(){
            None
        }else{
            Some(Ref::new(&Object{ object: parent }))
        }
    }

    pub fn unparent(&mut self){
        unsafe{
            gst_object_unparent(self.object);
        }
    }

    pub fn has_as_ancestor(&self, ancestor: &Object) -> bool{
        unsafe{
            gst_object_has_ancestor(self.object, ancestor.object) != 0
        }
    }

    pub fn path_string(&self) -> &str{
        unsafe{
            from_c_str!(gst_object_get_path_string(self.object))
        }
    }

    pub fn has_active_control_bindings(&self) -> bool{
        unsafe{
            gst_object_has_active_control_bindings(self.object) != 0
        }
    }

    pub fn disable_control_bindings(&mut self){
        unsafe{
            gst_object_set_control_bindings_disabled(self.object, 1)
        }
    }

    pub fn enable_control_bindings(&mut self){
        unsafe{
            gst_object_set_control_bindings_disabled(self.object, 0)
        }
    }

    pub fn disable_control_binding(&mut self, property_name: &str){
        unsafe{
            let cname = CString::new(property_name).unwrap();
            gst_object_set_control_binding_disabled(self.object, cname.as_ptr(), 1)
        }
    }

    pub fn enable_control_binding(&mut self, property_name: &str){
        unsafe{
            let cname = CString::new(property_name).unwrap();
            gst_object_set_control_binding_disabled(self.object, cname.as_ptr(), 0)
        }
    }

    pub fn set<T>(&mut self, name: &str, value: T)
    	where T: Property {
        value.set_to(name, self)
    }

    pub fn get<T>(&self, name: &str) -> T
    	where T: FromProperty {
        unsafe{
            let cname = CString::new(name).unwrap();
            let mut value = mem::uninitialized();
            g_object_get(self.gst_object() as *mut GObject, cname.as_ptr(), &mut value);
            T::from_property(value)
        }
    }

    pub unsafe fn signal_connect<T>(&mut self, signal: &str, callback: GCallback, data: &mut T)
        where Self:Sized{
        let csignal = CString::new(signal).unwrap();
        g_signal_connect_data(self.gst_object() as *mut GObject, csignal.as_ptr(), callback, mem::transmute(data), None, GConnectFlags::empty());
    }

    pub unsafe fn gst_object(&self) -> *const GstObject{
        self.object
    }

    pub unsafe fn gst_object_mut(&mut self) -> *mut GstObject{
        self.object
    }
}

impl Reference for Object{
    fn reference(&self) -> Object{
        unsafe{ gst_object_ref(self.object as *mut GstObject) };
        Object{ object: self.object }
    }
}

impl ::Transfer<GstObject> for Object{
    unsafe fn transfer(self) -> *mut GstObject{
        let object = self.object;
        mem::forget(self);
        object
    }
}

pub trait Property{
    type Target;
    fn set_to(&self, key: &str, e: &mut Object);
}

pub trait FromProperty: Property{
    fn from_property(t: <Self as Property>::Target) -> Self;
}

impl<'a> Property for &'a str{
    type Target = *const c_char;
    #[inline]
    fn set_to(&self, key: &str, e: &mut Object){
        let cname = CString::new(key).unwrap();
        let c_str = CString::new(*self).unwrap();
        unsafe{
            g_object_set(e.gst_object() as *mut GObject, cname.as_ptr(), c_str.as_ptr(), ptr::null::<c_char>());
        }
    }
}

impl<'a> FromProperty for &'a str{
    fn from_property(t: *const c_char) -> &'a str{
        unsafe{ from_c_str!(t) }
    }
}
pub trait RawProperty: Clone{
    #[inline]
    fn set_raw_to(&self, key: &str, e: &mut Object){
        let cname = CString::new(key).unwrap();
        unsafe{
            g_object_set(e.gst_object() as *mut GObject, cname.as_ptr(), self.clone(), ptr::null::<c_char>());
        }
    }
}

impl<R: RawProperty> Property for R{
    type Target = R;
    #[inline]
    fn set_to(&self, key: &str, e: &mut Object){
        self.set_raw_to(key, e);
    }
}

impl<R: RawProperty> FromProperty for R{
    fn from_property(p: <Self as Property>::Target) -> Self{
        p
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
