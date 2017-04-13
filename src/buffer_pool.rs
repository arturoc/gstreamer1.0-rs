use gst_sys::*;
use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::ops::{Deref, DerefMut};
use ::Buffer;
use ::Object;
use reference::Reference;

pub struct BufferPool{
    pool: Object
}

unsafe impl Sync for BufferPool {}
unsafe impl Send for BufferPool {}

impl BufferPool{
    pub fn new() -> Option<BufferPool>{
        unsafe{
	        let pool = gst_buffer_pool_new();
	        if pool!=ptr::null_mut(){
                gst_object_ref_sink(pool as *mut c_void);
	        	Some(BufferPool{ pool: Object::new(pool as *mut GstObject).unwrap() })
	        }else{
	            None
	        }
        }
    }

    pub fn set_params(&mut self, caps: &::Caps, size: u32, min_buffers: u32, max_buffers: u32){
        unsafe{
	        let config = gst_buffer_pool_get_config(self.gst_bufferpool_mut());
	        /*let mut current_caps = gst_caps_new_empty();
	        let mut curret_size = 0;
	        let mut current_min_buffers = 0;
	        let mut current_max_buffers = 0;
	        gst_buffer_pool_config_get_params(config, &mut current_caps, &mut curret_size, &mut current_min_buffers, &mut current_max_buffers);
			gst_mini_object_unref(current_caps as *mut GstMiniObject);*/

			gst_buffer_pool_config_set_params(config, mem::transmute(caps.gst_caps()), size, min_buffers, max_buffers);
            /*let mut params = GstAllocationParams {
			    flags: GST_MEMORY_FLAG_PHYSICALLY_CONTIGUOUS,
			    align: 0,
			    prefix: 0,
			    padding: 0,
			    _gst_reserved: [ptr::null_mut(); 4u]
			};
            gst_allocation_params_init(&mut params);
            params.flags = GST_MEMORY_FLAG_PHYSICALLY_CONTIGUOUS;
            gst_buffer_pool_config_set_allocator(config,ptr::null_mut(),&params);*/
            gst_buffer_pool_set_config(self.gst_bufferpool_mut(), config);
		}
    }

    pub fn acquire_buffer(&mut self) -> Option<Buffer>{
        /*let mut params = GstBufferPoolAcquireParams{
            format: GST_FORMAT_DEFAULT,
            start: 0,
            stop: 0,
            flags: GST_BUFFER_POOL_ACQUIRE_FLAG_NONE,
            _gst_reserved: [ptr::null_mut();4u]
        };*/
        let mut buffer: *mut GstBuffer = ptr::null_mut();
        unsafe{
        	let ret = gst_buffer_pool_acquire_buffer(self.gst_bufferpool_mut(), &mut buffer, ptr::null_mut());
	        if buffer!=ptr::null_mut() && ret==GST_FLOW_OK{
	            Buffer::new(buffer)
	        }else{
	            None
	        }
	    }
    }

    pub fn active(&self) -> bool{
        unsafe{
            gst_buffer_pool_is_active(self.gst_bufferpool() as *mut GstBufferPool) != 0
        }
    }

    pub fn set_active(&mut self, active: bool) -> Result<(),()>{
        unsafe{
        	if gst_buffer_pool_set_active(self.gst_bufferpool_mut(), if active{1} else {0}) != 0{
        	    Ok(())
        	}else{
        	    Err(())
        	}
        }
    }

    pub unsafe fn gst_bufferpool(&self) -> *const GstBufferPool{
        self.pool.gst_object() as *const GstBufferPool
    }

    pub unsafe fn gst_bufferpool_mut(&mut self) -> *mut GstBufferPool{
        self.pool.gst_object_mut() as *mut GstBufferPool
    }
}


impl ::Transfer<GstBufferPool> for BufferPool{
    unsafe fn transfer(self) -> *mut GstBufferPool{
        self.pool.transfer() as *mut GstBufferPool
    }
}

impl Reference for BufferPool{
    fn reference(&self) -> BufferPool{
        BufferPool{ pool: self.pool.reference() }
    }
}

impl AsRef<Object> for BufferPool{
    fn as_ref(&self) -> &Object{
        &self.pool
    }
}

impl AsMut<Object> for BufferPool{
    fn as_mut(&mut self) -> &mut Object{
        &mut self.pool
    }
}

impl From<BufferPool> for Object{
    fn from(b: BufferPool) -> Object{
        b.pool
    }
}

impl Deref for BufferPool{
    type Target = Object;
    fn deref(&self) -> &Object{
        &self.pool
    }
}

impl DerefMut for BufferPool{
    fn deref_mut(&mut self) -> &mut Object{
        &mut self.pool
    }
}
