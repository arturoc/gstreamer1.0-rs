use buffer::Buffer;
use ffi::*;
use std::mem;
use std::slice::from_raw_parts;
use std::intrinsics::offset;
use std::ptr;

pub struct VideoFrame{
    vf: GstVideoFrame,
    buffer: Buffer
}

pub struct VideoPlane<'a>{
	vf: &'a GstVideoFrame,
	p: usize
}

pub struct VideoComponent<'a>{
	vf: &'a GstVideoFrame,
	c: usize
}

macro_rules! GST_VIDEO_FRAME_COMP_WIDTH(
	($video_frame:expr,$c:expr) => ( -((-$video_frame.info.width) >> ((*$video_frame.info.finfo).w_sub[$c]) as usize) ) ;
);

macro_rules! GST_VIDEO_FRAME_COMP_HEIGHT(
	($video_frame:expr,$c:expr) => ( -((-$video_frame.info.height) >> ((*$video_frame.info.finfo).h_sub[$c]) as usize) );
);

macro_rules! GST_VIDEO_FRAME_COMP_OFFSET(
	($video_frame:expr,$c:expr) => ((($video_frame.info.offset)[(*$video_frame.info.finfo).plane[($c)] as usize]) + (*$video_frame.info.finfo).poffset[($c)] as u64)
);

macro_rules! GST_VIDEO_FRAME_COMP_STRIDE(
	($video_frame:expr,$c:expr) => (($video_frame.info.stride)[(*$video_frame.info.finfo).plane[($c)] as usize])
);

macro_rules! GST_VIDEO_FRAME_COMP_DATA(
	($video_frame:expr,$c:expr) => {
	    offset(($video_frame.data[(*$video_frame.info.finfo).plane[$c] as usize] as *mut u8), (*$video_frame.info.finfo).poffset[$c] as isize)
	}
);


impl<'a> VideoPlane<'a>{
    pub fn stride(&self) -> i32{
        self.info().stride[self.p]
    }
    
    pub fn offset(&self) -> u64{
        self.info().offset[self.p]
    }
    
    pub fn width(&self) -> i32{
        unsafe{ GST_VIDEO_FRAME_COMP_WIDTH!(self.vf,self.p) }
    }
    
    pub fn height(&self) -> i32{
        unsafe{ GST_VIDEO_FRAME_COMP_HEIGHT!(self.vf,self.p) }
    }
    
    pub fn size(&self) -> usize{
        (self.stride()*self.height()) as usize
    }
    
    pub fn len<T>(&self) -> usize{
        self.size()/mem::size_of::<T>()
    }
    
    pub fn depth(&self) -> u32{
        self.format_info().depth[self.p]
    }
    
    pub fn data<T:'a>(&self) -> &'a[T]{
        unsafe{
	        from_raw_parts( mem::transmute(self.vf.data[self.p]), self.len::<T>())
	    }
    }
    
    fn info(&self) -> &::VideoInfo{
        &self.vf.info
    }
    
    fn format_info(&self) -> &GstVideoFormatInfo{
        unsafe{ &(*self.vf.info.finfo) }
    }
}

impl<'a> VideoComponent<'a>{
    pub fn stride(&self) -> i32{
        unsafe{  GST_VIDEO_FRAME_COMP_STRIDE!(self.vf,self.c) }
    }
    
    pub fn offset(&self) -> u64{
        unsafe{ GST_VIDEO_FRAME_COMP_OFFSET!(self.vf,self.c) }
    }
    
    pub fn width(&self) -> i32{
        unsafe{ GST_VIDEO_FRAME_COMP_WIDTH!(self.vf,self.c) }
    }
    
    pub fn height(&self) -> i32{
        unsafe{ GST_VIDEO_FRAME_COMP_HEIGHT!(self.vf,self.c) }
    }
    
    pub fn size(&self) -> usize{
        (self.stride()*self.height()) as usize
    }
    
    pub fn len<T>(&self) -> usize{
        self.size()/mem::size_of::<T>()
    }
    
    pub fn depth(&self) -> u32{
        self.format_info().depth[self.c]
    }
    
    pub fn data<T:'a>(&self) -> &'a[T]{
        unsafe{
            let data = GST_VIDEO_FRAME_COMP_DATA!(self.vf,self.c);
	        from_raw_parts( mem::transmute(data), self.len::<T>())
	    }
    }
    
    fn format_info(&self) -> &GstVideoFormatInfo{
        unsafe{ &(*self.vf.info.finfo) }
    }
}

impl Drop for VideoFrame{
    fn drop(&mut self){
        unsafe{ gst_video_frame_unmap(&mut self.vf) };
    }
}

impl VideoFrame{
    pub unsafe fn new(mut vi: GstVideoInfo, mut buffer: Buffer) -> Option<VideoFrame>{
        let mut gstframe = video_frame_new();
        if gst_video_frame_map(&mut gstframe, &mut vi, buffer.gst_buffer_mut(), GST_MAP_READ) != 0{
            Some(VideoFrame{ vf: gstframe, buffer: buffer })
        }else{
        	None
        }
    }
    
    #[inline]
    pub fn info(&self) -> &::VideoInfo{
        &self.vf.info
    }
    
    #[inline]
    pub fn flags(&self) -> &GstVideoFlags{
        &self.vf.flags
    }
    
    #[inline]
    pub fn buffer(&self) -> &Buffer{
        &self.buffer
    }
    
    #[inline]
    pub fn format_info(&self) -> &GstVideoFormatInfo{
        unsafe{ &(*self.vf.info.finfo) }
    }
    
    #[inline]
    pub fn format(&self) -> &GstVideoFormat{
        &self.format_info().format
    }
    
    #[inline]
    pub fn width(&self) -> i32{
        self.info().width
    }
    
    #[inline]
    pub fn height(&self) -> i32{
        self.info().height
    }
    
    #[inline]
    pub fn size(&self) -> u64{
        self.info().size
    }
	
	#[inline]
	pub fn len<T>(&self) -> usize{
		(self.size() / mem::size_of::<T>() as u64)  as usize
	}
    
    #[inline]
    pub fn is_interlaced(&self) -> bool{
        self.flags() & GST_VIDEO_FRAME_FLAG_INTERLACED == GST_VIDEO_FRAME_FLAG_INTERLACED
    }
    
    #[inline]
    pub fn is_tff(&self) -> bool{
        self.flags() & GST_VIDEO_FRAME_FLAG_TFF == GST_VIDEO_FRAME_FLAG_TFF
    }
    
    #[inline]
    pub fn is_rff(&self) -> bool{
        self.flags() & GST_VIDEO_FRAME_FLAG_RFF == GST_VIDEO_FRAME_FLAG_RFF
    }
    
    #[inline]
    pub fn is_onefield(&self) -> bool{
        self.flags() & GST_VIDEO_FRAME_FLAG_ONEFIELD == GST_VIDEO_FRAME_FLAG_ONEFIELD
    }
    
    #[inline]
    pub fn n_planes(&self) -> u32{
        self.format_info().n_planes
    }
    
    #[inline]
    pub fn plane<'a>(&'a self, p: u32) -> Option<VideoPlane<'a>>{
        if p < self.n_planes(){
	        Some(VideoPlane{
	            vf: &self.vf,
	            p: p as usize
	    	})
	    }else{
	        None
	    }
    }

	#[inline]
	pub fn n_components(&self) -> u32{
	    self.format_info().n_components
	}
    
    #[inline]
    pub fn component<'a>(&'a self, c: u32) -> Option<VideoComponent<'a>>{
        if c < self.n_components(){
	        Some(VideoComponent{
	            vf: &self.vf,
	            c: c as usize
	    	})
	    }else{
	        None
	    }
    }
}


fn map_info_new() -> GstMapInfo{
    GstMapInfo{ memory: ptr::null_mut(),
                        flags: 0,
                        data: ptr::null_mut(),
                        size: 0,
                        maxsize: 0,
                        user_data: [ptr::null_mut();4],
                        _gst_reserved: [ptr::null_mut();4] }
}

fn video_frame_new() -> GstVideoFrame{
	GstVideoFrame{
		  info: ::VideoInfo::new(),
		  flags: GST_VIDEO_FRAME_FLAG_NONE,
		  buffer: ptr::null_mut(),
		  meta: ptr::null_mut(),
		  id: 0,
		  data: [ptr::null_mut();4],
		  map: [map_info_new();4],
		  _gst_reserved: [ptr::null_mut();4],
	}
}