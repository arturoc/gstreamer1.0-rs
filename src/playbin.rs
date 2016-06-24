use ffi::*;

use pipeline::Pipeline;
use pipeline::PipelineT;
use element::Element;
use element::ElementT;
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

    pub fn set_audio_sink(&mut self, audio_sink: &ElementT){
        self.set("audio-sink", audio_sink);
    }

    /*pub fn frame(&self) -> GBuffer{
        GBuffer::new(playbin.get<GstBuffer*>("frame"))
    }*/

    pub fn set_subtitle_font_desc(&mut self, font: &str){
        self.set("subtitle-font-desc", font);
    }

    pub fn set_video_sink(&mut self, video_sink: &ElementT){
        self.set("video-sink", video_sink);
    }

    pub fn set_vis_plugin(&mut self, vis_plugin: &ElementT){
        self.set("vis-plugin", vis_plugin);
    }

    pub fn set_volume(&mut self, volume: f64){
        self.set("volume", volume);
    }

    pub fn set_connection_speed(&mut self, connection_speed: u64){
        self.set("connection-speed",connection_speed);
    }

    pub fn set_av_offset(&mut self, av_offset: i64){
        self.set("av-offset", av_offset);
    }

    pub fn set_buffer_duration(&mut self, buffer_duration: i64){
        self.set("buffer-duration",buffer_duration);
    }

    pub fn set_current_audio(&mut self, current_audio: i32){
        self.set("current-audio",current_audio);
    }

    pub fn set_current_text(&mut self, current_text: i32){
        self.set("current-text", current_text);
    }

    /*pub fn set_flags(&self, flags: GstPlayFlags){
        self.set("flags", flags);
    }*/

    pub fn mute(&mut self){
        self.set("mute", 1 as gboolean);
    }

    pub fn unmute(&mut self){
        self.set("mute", 0 as gboolean);
    }

    pub fn set_ring_buffer_max_size(&mut self, ring_buffer_max_size: u64){
        self.set("ring-buffer-max-size", ring_buffer_max_size);
    }

    pub fn set_source(&mut self, source: &ElementT){
        self.set("source", source);
    }

    pub fn set_subtitle_encoding(&mut self, encoding: &str){
        self.set("subtitle-encoding", encoding);
    }

    pub fn set_suburi(&mut self, suburi: &str){
        self.set("suburi", suburi);
    }

    pub fn set_text_sink(&mut self, textsink: &ElementT){
        self.set("text-sink", textsink);
    }

    pub fn set_uri(&mut self, uri: &str){
        self.set("uri", uri);
    }

    pub fn set_force_aspect_ratio(&mut self, force_aspect_ratio: bool){
        self.set("force-aspect-ratio", force_aspect_ratio as gboolean);
    }

    pub fn set_audio_stream_combiner(&mut self, audio_stream_combiner: &ElementT){
        self.set("audio-stream-combiner", audio_stream_combiner);
    }

    pub fn set_video_stream_combiner(&mut self, video_stream_combiner: &ElementT){
        self.set("vide-stream-combiner", video_stream_combiner);
    }

    pub fn set_flags(&mut self, flags: i32){
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

impl ::Reference for PlayBin{
    fn reference(&self) -> PlayBin{
        PlayBin{ playbin: self.playbin.reference() }
    }
}
