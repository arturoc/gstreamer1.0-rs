use ffi::*;

use pipeline::Pipeline;
use pipeline::PipelineT;
use element::Element;
use element::ElementT;
use std::ffi::CString;
use ::Transfer;

unsafe impl Sync for PlayBin {}
unsafe impl Send for PlayBin {}

pub struct PlayBin{
    playbin: Pipeline
}

impl PlayBin{
    pub fn new(name: &str) -> Option<PlayBin>{
        let pipeline = Element::new("playbin",name);
        match pipeline{
            Some(p) => {
                match unsafe{ Pipeline::new_from_gst_pipeline( p.transfer() as *mut GstPipeline) }{
                    Some(p) => Some(PlayBin{ playbin: p }),
                    None => None
                }
            }
            None => None
        }
    }

    pub fn set_audio_sink(&self, audio_sink: &ElementT){
        self.set("audio-sink", unsafe{ audio_sink.gst_element() });
    }

    /*pub fn frame(&self) -> GBuffer{
        GBuffer::new(playbin.get<GstBuffer*>("frame"))
    }*/

    pub fn set_subtitle_font_desc(&self, font: &str){
        let cfont = CString::new(font).unwrap();
        self.set("subtitle-font-desc", cfont);
    }

    pub fn set_video_sink(&self, video_sink: &ElementT){
        self.set("video-sink", unsafe{ video_sink.gst_element() });
    }

    pub fn set_vis_plugin(&self, vis_plugin: &ElementT){
        self.set("vis-plugin", vis_plugin);
    }

    pub fn set_volume(&self, volume: f64){
        self.set("volume", volume);
    }

    pub fn set_connection_speed(&self, connection_speed: u64){
        self.set("connection-speed",connection_speed);
    }

    pub fn set_av_offset(&self, av_offset: i64){
        self.set("av-offset", av_offset);
    }

    pub fn set_buffer_duration(&self, buffer_duration: i64){
        self.set("buffer-duration",buffer_duration);
    }

    pub fn set_current_audio(&self, current_audio: i32){
        self.set("current-audio",current_audio);
    }

    pub fn set_current_text(&self, current_text: i32){
        self.set("current-text", current_text);
    }

    /*pub fn set_flags(&self, flags: GstPlayFlags){
        self.set("flags", flags);
    }*/

    pub fn mute(&self){
        self.set("mute", 1 as gboolean);
    }

    pub fn unmute(&self){
        self.set("mute", 0 as gboolean);
    }

    pub fn set_ring_buffer_max_size(&self, ring_buffer_max_size: u64){
        self.set("ring-buffer-max-size", ring_buffer_max_size);
    }

    pub fn set_source(&self, source: &ElementT){
        self.set("source", unsafe{ source.gst_element() });
    }

    pub fn set_subtitle_encoding(&self, encoding: &str){
        let cencoding = CString::new(encoding).unwrap();
        self.set("subtitle-encoding", cencoding);
    }

    pub fn set_suburi(&self, suburi: &str){
        let csuburi = CString::new(suburi).unwrap();
        self.set("suburi", csuburi);
    }

    pub fn set_text_sink(&self, textsink: &ElementT){
        self.set("text-sink", unsafe{ textsink.gst_element() });
    }

    pub fn set_uri(&self, uri: &str){
        let curi = CString::new(uri).unwrap();
        self.set("uri", curi);
    }

    pub fn set_force_aspect_ratio(&self, force_aspect_ratio: bool){
        self.set("force-aspect-ratio", force_aspect_ratio as gboolean);
    }

    pub fn set_audio_stream_combiner(&self, audio_stream_combiner: &ElementT){
        self.set("audio-stream-combiner", unsafe{ audio_stream_combiner.gst_element() });
    }

    pub fn set_video_stream_combiner(&self, video_stream_combiner: &ElementT){
        self.set("vide-stream-combiner", unsafe{ video_stream_combiner.gst_element() });
    }

    pub fn set_flags(&self, flags: i32){
        self.set("flags", flags);
    }
}

impl PipelineT for PlayBin{
    fn as_pipeline(&self) -> &Pipeline{
        &self.playbin
    }

    fn as_pipeline_mut(&mut self) -> &mut Pipeline{
        &mut self.playbin
    }
}

impl ::Transfer for PlayBin{
    unsafe fn transfer(self) -> *mut GstElement{
        self.playbin.transfer()
    }
}
