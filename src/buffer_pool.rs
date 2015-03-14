use ffi::*;
use std::ptr;
use std::mem;

pub struct BufferPool{
    pool: *mut GstBufferPool
}

unsafe impl Sync for BufferPool {}
unsafe impl Send for BufferPool {}

impl BufferPool{
    pub fn new() -> Option<BufferPool>{
        unsafe{ 
	        let pool = gst_buffer_pool_new();
	        if pool!=ptr::null_mut(){
	        	Some(BufferPool{pool: pool})
	        }else{
	            None
	        }
        }
    }
    
    pub fn set_params(&self, caps: &::Caps, size: u32, min_buffers: u32, max_buffers: u32){
        unsafe{
	        let config = gst_buffer_pool_get_config(self.pool);
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
            gst_buffer_pool_set_config(self.pool, config);
		}
    }
    
    pub fn acquire_buffer(&self) -> Option<::Buffer>{
        /*let mut params = GstBufferPoolAcquireParams{ 
            format: GST_FORMAT_DEFAULT,
            start: 0,
            stop: 0,
            flags: GST_BUFFER_POOL_ACQUIRE_FLAG_NONE,
            _gst_reserved: [ptr::null_mut();4u]
        };*/
        let mut buffer: *mut GstBuffer = ptr::null_mut();
        unsafe{
        	let ret = gst_buffer_pool_acquire_buffer(self.pool, &mut buffer, ptr::null_mut());
	        if buffer!=ptr::null_mut() && ret==GST_FLOW_OK{
	            ::Buffer::new(buffer,true)
	        }else{
	            None
	        }
	    }
    }
    
    pub fn active(&self) -> bool{
        unsafe{
            gst_buffer_pool_is_active(self.pool) != 0
        }
    }
    
    pub fn set_active(&self, active: bool) -> Result<(),()>{
        unsafe{
        	if gst_buffer_pool_set_active(self.pool, if active{1} else {0}) != 0{
        	    Ok(())
        	}else{
        	    Err(())
        	}
        }
    }
}