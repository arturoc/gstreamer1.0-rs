use ffi::*;
use util::*;
use ::Caps;

pub type VideoInfo = GstVideoInfo;

impl VideoInfo{
    #[inline]
    pub fn format_info(&self) -> &GstVideoFormatInfo{
        unsafe{ &(*self.finfo) }
    }

    #[inline]
    pub fn format(&self) -> GstVideoFormat{
        self.format_info().format
    }

    #[inline]
    pub fn format_name(&self) -> String{
        unsafe{ from_c_str!(self.format_info().name).to_string() }
    }

    #[inline]
    pub fn is_yuv(&self) -> bool{
        self.format_info().flags & GST_VIDEO_FORMAT_FLAG_YUV == GST_VIDEO_FORMAT_FLAG_YUV
    }

    #[inline]
    pub fn is_rgb(&self) -> bool{
        self.format_info().flags & GST_VIDEO_FORMAT_FLAG_RGB == GST_VIDEO_FORMAT_FLAG_RGB
    }

    #[inline]
    pub fn is_gray(&self) -> bool{
        self.format_info().flags & GST_VIDEO_FORMAT_FLAG_GRAY == GST_VIDEO_FORMAT_FLAG_GRAY
    }

    #[inline]
    pub fn has_alpha(&self) -> bool{
        self.format_info().flags & GST_VIDEO_FORMAT_FLAG_ALPHA == GST_VIDEO_FORMAT_FLAG_ALPHA
    }

    #[inline]
    pub fn interlace_mode(&self) -> GstVideoInterlaceMode{
        self.interlace_mode
    }

    #[inline]
    pub fn is_interlaced(&self) -> bool{
        self.interlace_mode != GST_VIDEO_INTERLACE_MODE_PROGRESSIVE
    }

    #[inline]
    pub fn flags(&self) -> GstVideoFlags{
        self.flags
    }

    #[inline]
    pub fn width(&self) -> i32{
        self.width
    }

    #[inline]
    pub fn height(&self) -> i32{
        self.height
    }

    #[inline]
    pub fn size(&self) -> u64{
        self.size as u64
    }

    #[inline]
    pub fn views(&self) -> i32{
        self.views
    }

    #[inline]
    pub fn par_n(&self) -> i32{
        self.par_n
    }

    #[inline]
    pub fn par_d(&self) -> i32{
        self.par_d
    }

    #[inline]
    pub fn fps_n(&self) -> i32{
        self.fps_n
    }

    #[inline]
    pub fn fps_d(&self) -> i32{
        self.fps_d
    }

    #[inline]
    pub fn n_planes(&self) -> u32{
        self.format_info().n_planes
    }

    #[inline]
    pub fn plane_stride(&self, p: usize) -> i32{
        self.stride[p]
    }

    #[inline]
    pub fn plane_offset(&self, p: usize) -> u64{
        self.offset[p] as u64
    }

    pub fn to_caps(&self) -> Option<::Caps>{
        unsafe{ Caps::new(gst_video_info_to_caps(mem::transmute(self))) }
    }
}

impl PartialEq for VideoInfo{
    fn eq(&self, other: &VideoInfo) -> bool{
        unsafe{
            gst_video_info_is_equal(mem::transmute(self), mem::transmute(other)) != 0
        }
    }
}

impl Eq for VideoInfo{}
