use ffi::*;

use pipeline::Pipeline;
use pipeline::PipelineT;
use element::Element;
use element::ElementT;
use bin::Bin;
use bin::BinT;
use bus::Bus;
use std::ffi::CString;
use std::ptr;

pub struct PlayBin{
    playbin: Pipeline
}

impl PlayBin{
    pub fn new(name: &str) -> Option<PlayBin>{
        let pipeline = Element::new("playbin",name);
        match pipeline{
            Some(p) => Some(PlayBin{ playbin: Pipeline{ pipeline: Bin{ bin: p} } }),
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
        self.set("subtitle-font-desc",to_c_str!(font));
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
        self.set("subtitle-encoding", to_c_str!(encoding));
    }
    
    pub fn set_suburi(&self, suburi: &str){
        self.set("suburi", to_c_str!(suburi));
    }
    
    pub fn set_text_sink(&self, textsink: &ElementT){
        self.set("text-sink", unsafe{ textsink.gst_element() });
    }
    
    pub fn set_uri(&self, uri: &str){
        self.set("uri", to_c_str!(uri));
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
    
    pub fn set<T>(&self, name: &str, value: T){
        self.playbin.set(name,value);
    }
}

impl PipelineT for PlayBin{  
    
    fn delay(&self) -> GstClockTime{
        self.playbin.delay()
    }
    
    fn set_delay(&self, delay: GstClockTime){
        self.playbin.set_delay(delay);
    }
    
    unsafe fn gst_pipeline(&self) -> *mut GstPipeline{
        self.playbin.gst_pipeline()
    }
}

impl BinT for PlayBin{
    
    unsafe fn gst_bin(&self) -> *mut GstBin{
        self.playbin.gst_bin()
    }
    
    fn add(&self, element: &ElementT) -> bool{
        self.playbin.add(element)
    }
    
    fn remove(&self, element: &ElementT) -> bool{
        self.playbin.remove(element)
    }
    
    fn get_by_name(&self, name: &str) -> Option<Element>{
        self.playbin.get_by_name(name)
    }
    
    fn recalculate_latency(&self) -> bool{
        self.playbin.recalculate_latency()
    }
    
    fn set_async_handling(&self, async: bool){
        self.playbin.set_async_handling(async);
    }
    
    fn set_message_forward(&self, forward: bool){
        self.playbin.set_message_forward(forward);
    }
}

impl ElementT for PlayBin{
    
    fn link(&mut self, dst: &mut ElementT) -> bool{
        self.playbin.link(dst)
    }
    
    fn unlink(&mut self, dst: &mut ElementT){
        self.playbin.unlink(dst);
    }
    
    fn bus(&self) -> Option<Bus>{
        self.playbin.bus()
    }
    
    fn name(&self) -> String{
        self.playbin.name()
    }
    
    fn set_name(&mut self, name: &str){
        self.playbin.set_name(name);
    }
    
    fn set_state(&mut self, state: GstState) -> GstStateChangeReturn{
        self.playbin.set_state(state)
    }
    
    fn get_state(&self, timeout: GstClockTime) -> (GstState, GstState, GstStateChangeReturn){
        self.playbin.get_state(timeout)
    }
    
    fn send_event(&mut self, event: *mut GstEvent) -> bool{
        self.playbin.send_event(event)
    }
    
    fn seek_simple(&mut self, format: GstFormat, flags: GstSeekFlags, pos: i64) -> bool{
        self.playbin.seek_simple(format, flags, pos)
    }
    
    fn seek(&mut self, rate: f64, format: GstFormat, flags: GstSeekFlags, start_type: GstSeekType, start: i64, stop_type: GstSeekType, stop: i64) -> bool{
        self.playbin.seek(rate, format, flags, start_type, start, stop_type, stop)
    }
    
    fn seek_async(&mut self, rate: f64, format: GstFormat, flags: GstSeekFlags, start_type: GstSeekType, start: i64, stop_type: GstSeekType, stop: i64){
        self.playbin.seek_async(rate,format,flags,start_type,start,stop_type,stop);
    }
    
    fn query_duration(&self, format: GstFormat) -> Option<i64>{
        self.playbin.query_duration(format)
    }
    
    fn query_position(&self, format: GstFormat) -> Option<i64>{
        self.playbin.query_position(format)
    }
    
    fn duration_ns(&self) -> Option<i64>{
        self.playbin.duration_ns()
    }
    
    fn duration_s(&self) -> Option<f64>{
        self.playbin.duration_s()
    }
    
    fn position_ns(&self) -> i64{
        self.playbin.position_ns()
    }
    
    fn position_pct(&self) -> Option<f64>{
        self.playbin.position_pct()
    }
    
    fn position_s(&self) -> f64{
        self.playbin.position_s()
    }
    
    fn speed(&self) -> f64{
        self.playbin.speed()
    }
    
    fn set_position_ns(&mut self, ns: i64) -> bool{
        self.playbin.set_position_ns(ns)
    }
    
    fn set_position_s(&mut self, s: f64) -> bool{
        self.playbin.set_position_s(s)
    }
    
    fn set_position_pct(&mut self, pct: f64) -> bool{
        self.playbin.set_position_pct(pct)
    }
    
    fn set_speed(&mut self, speed: f64) -> bool{
        self.playbin.set_speed(speed)
    }
    
    fn set_position_ns_async(&mut self, ns: i64){
        self.playbin.set_position_ns_async(ns);
    }
    
    fn set_position_s_async(&mut self, s: f64){
        self.playbin.set_position_s_async(s);
    }
    
    fn set_position_pct_async(&mut self, pct: f64) -> bool{
        self.playbin.set_position_pct_async(pct)
    }
    
    fn set_speed_async(&mut self, speed: f64) -> bool{
        self.playbin.set_speed_async(speed)
    }
    
    unsafe fn gst_element(&self) -> *const GstElement{
        self.playbin.gst_element()
    }
    
    unsafe fn gst_element_mut(&mut self) -> *mut GstElement{
        self.playbin.gst_element_mut()
    }
    
    /*fn set<T>(&self, name: &str, value: T){
        self.playbin.set(name,value);
    }*/
    
    fn set_null_state(&mut self){
        self.playbin.set_null_state();
    }
    
    fn set_ready_state(&mut self){
        self.playbin.set_ready_state();
    }
    
    fn pause(&mut self){
        self.playbin.pause();
    }
    
    fn play(&mut self){
        self.playbin.play();
    }
    
    fn is_paused(&self) -> bool{
        self.playbin.is_paused()
    }
    
    fn is_playing(&self) -> bool{
        self.playbin.is_playing()
    }
    
    fn is_null_state(&self) -> bool{
        self.playbin.is_null_state()
    }
    
    fn is_ready_state(&self) -> bool{
        self.playbin.is_ready_state()
    }
}
