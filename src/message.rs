use glib::*;
use gst_sys::*;
use util::*;
use error::Error;
use std::os::raw;
use reference::Reference;

unsafe impl Send for Message {}

pub type MessagePrivate = *mut GstMessage;


unsafe fn gst_message_ref(msg: *mut GstMessage) -> *mut GstMessage{
	gst_mini_object_ref(mem::transmute(msg)) as *mut GstMessage
}

pub enum Message{
    Unknown(MessagePrivate),
    Eos(MessagePrivate),
    Error(MessagePrivate),
    ErrorParsed{msg: MessagePrivate, error: Error, debug: String},
    Warning(MessagePrivate),
    WarningParsed{msg: MessagePrivate, error: Error, debug: String},
    Info(MessagePrivate),
    InfoParsed{msg: MessagePrivate, error: Error, debug: String},
    Tag(MessagePrivate),
    TagParsed{msg: MessagePrivate, tags: *mut GstTagList},
    Buffering(MessagePrivate),
    BufferingParsed{msg: MessagePrivate, pct: i32},
    StateChanged(MessagePrivate),
    StateChangedParsed{msg: MessagePrivate, old: GstState, new: GstState, pending: GstState},
    StateDirty(MessagePrivate),
    StepDone(MessagePrivate),
    ClockProvide(MessagePrivate),
    ClockLost(MessagePrivate),
    NewClock(MessagePrivate),
    StructureChange(MessagePrivate),
    StreamStatus(MessagePrivate),
    Application(MessagePrivate),
    Element(MessagePrivate),
    SegmentStart(MessagePrivate),
    SegmentDone(MessagePrivate),
    DurationChanged(MessagePrivate),
    Latency(MessagePrivate),
    AsyncStart(MessagePrivate),
    AsyncDone(MessagePrivate),
    RequestState(MessagePrivate),
    StepStart(MessagePrivate),
    Qos(MessagePrivate),
    Progress(MessagePrivate),
    Toc(MessagePrivate),
    ResetTime(MessagePrivate),
    StreamStart(MessagePrivate),
    NeedContext(MessagePrivate),
    HaveContext(MessagePrivate),
    Extended(MessagePrivate),
    DeviceAdded(MessagePrivate),
    DeviceRemoved(MessagePrivate),
    Any(MessagePrivate),
}

impl Drop for Message{
    fn drop(&mut self){
        unsafe{
            gst_mini_object_unref(self.gst_message() as *mut GstMiniObject);
        }
    }
}

impl Message{
    pub unsafe fn new(gst_message: *const GstMessage) -> Option<Message>{
        if gst_message != ptr::null(){
            let gst_message = gst_mini_object_ref(gst_message as *mut GstMiniObject) as *mut GstMessage;
            match (*gst_message).type_{
                 GST_MESSAGE_UNKNOWN => Some(Message::Unknown(gst_message)),
                 GST_MESSAGE_EOS => Some(Message::Eos(gst_message)),
                 GST_MESSAGE_ERROR => Some(Message::Error(gst_message)),
                 GST_MESSAGE_WARNING => Some(Message::Warning(gst_message)),
                 GST_MESSAGE_INFO => Some(Message::Info(gst_message)),
                 GST_MESSAGE_TAG => Some(Message::Tag(gst_message)),
                 GST_MESSAGE_BUFFERING => Some(Message::Buffering(gst_message)),
                 GST_MESSAGE_STATE_CHANGED => Some(Message::StateChanged(gst_message)),
                 GST_MESSAGE_STATE_DIRTY => Some(Message::StateDirty(gst_message)),
                 GST_MESSAGE_STEP_DONE => Some(Message::StepDone(gst_message)),
                 GST_MESSAGE_CLOCK_PROVIDE => Some(Message::ClockProvide(gst_message)),
                 GST_MESSAGE_CLOCK_LOST => Some(Message::ClockLost(gst_message)),
                 GST_MESSAGE_NEW_CLOCK => Some(Message::NewClock(gst_message)),
                 GST_MESSAGE_STRUCTURE_CHANGE => Some(Message::StructureChange(gst_message)),
                 GST_MESSAGE_STREAM_STATUS => Some(Message::StreamStatus(gst_message)),
                 GST_MESSAGE_APPLICATION => Some(Message::Application(gst_message)),
                 GST_MESSAGE_ELEMENT => Some(Message::Element(gst_message)),
                 GST_MESSAGE_SEGMENT_START => Some(Message::SegmentStart(gst_message)),
                 GST_MESSAGE_SEGMENT_DONE => Some(Message::SegmentDone(gst_message)),
                 GST_MESSAGE_DURATION_CHANGED => Some(Message::DurationChanged(gst_message)),
                 GST_MESSAGE_LATENCY => Some(Message::Latency(gst_message)),
                 GST_MESSAGE_ASYNC_START => Some(Message::AsyncStart(gst_message)),
                 GST_MESSAGE_ASYNC_DONE => Some(Message::AsyncDone(gst_message)),
                 GST_MESSAGE_REQUEST_STATE => Some(Message::RequestState(gst_message)),
                 GST_MESSAGE_STEP_START => Some(Message::StepStart(gst_message)),
                 GST_MESSAGE_QOS => Some(Message::Qos(gst_message)),
                 GST_MESSAGE_PROGRESS => Some(Message::Progress(gst_message)),
                 GST_MESSAGE_TOC => Some(Message::Toc(gst_message)),
                 GST_MESSAGE_RESET_TIME => Some(Message::ResetTime(gst_message)),
                 GST_MESSAGE_STREAM_START => Some(Message::StreamStart(gst_message)),
                 GST_MESSAGE_NEED_CONTEXT => Some(Message::NeedContext(gst_message)),
                 GST_MESSAGE_HAVE_CONTEXT => Some(Message::HaveContext(gst_message)),
                 GST_MESSAGE_EXTENDED => Some(Message::Extended(gst_message)),
                 GST_MESSAGE_DEVICE_ADDED => Some(Message::DeviceAdded(gst_message)),
                 GST_MESSAGE_DEVICE_REMOVED => Some(Message::DeviceRemoved(gst_message)),
                 GST_MESSAGE_ANY => Some(Message::Any(gst_message)),
                 _ => None
            }
        }else{
            None
        }
    }

    pub unsafe fn new_eos(src: *mut GstObject) -> Option<Message>{
        Message::new(gst_message_new_eos(src))
    }

    pub unsafe fn new_error(src: *mut GstObject, error: *mut GError, debug: &str) -> Option<Message>{
        let cdebug = CString::new(debug).unwrap();
        Message::new(gst_message_new_error(src,error,mem::transmute(cdebug.as_ptr())))
    }

    pub unsafe fn new_warning(src: *mut GstObject, error: *mut GError, debug: &str) -> Option<Message>{
        let cdebug = CString::new(debug).unwrap();
        Message::new(gst_message_new_warning(src,error,mem::transmute(cdebug.as_ptr())))
    }

    pub unsafe fn new_info(src: *mut GstObject, error: *mut GError, debug: &str) -> Option<Message>{
		let cdebug = CString::new(debug).unwrap();
        Message::new(gst_message_new_info(src,error,mem::transmute(cdebug.as_ptr())))
    }

    pub unsafe fn new_tag(src: *mut GstObject, tag_list: *mut GstTagList) -> Option<Message>{
        Message::new(gst_message_new_tag(src,tag_list))
    }

    pub unsafe fn new_buffering(src: *mut GstObject, pct: i32) -> Option<Message>{
        Message::new(gst_message_new_buffering(src,pct))
    }

    pub unsafe fn new_state_changed(src: *mut GstObject, old_state: GstState, new_state: GstState, pending: GstState) -> Option<Message>{
        Message::new(gst_message_new_state_changed(src,old_state,new_state,pending))
    }

    pub unsafe fn new_state_dirty(src: *mut GstObject) -> Option<Message>{
        Message::new(gst_message_new_state_dirty(src))
    }

    pub unsafe fn new_step_done(src: *mut GstObject, format: GstFormat,
                         amount: u64, rate: f64,
                         flush: bool, intermediate: bool,
                         duration: u64, eos: bool) -> Option<Message>{
        Message::new(gst_message_new_step_done(src,format,amount,rate,flush as i32,intermediate as i32,duration,eos as i32))
    }

    pub unsafe fn new_clock_provide(src: *mut GstObject, clock: *mut GstClock, ready: bool) -> Option<Message>{
        Message::new(gst_message_new_clock_provide(src,clock,ready as i32))
    }

    pub unsafe fn new_clock_lost(src: *mut GstObject, clock: *mut GstClock) -> Option<Message>{
        Message::new(gst_message_new_clock_lost(src,clock))
    }

    pub unsafe fn new_new_clock(src: *mut GstObject, clock: *mut GstClock) -> Option<Message>{
        Message::new(gst_message_new_new_clock(src,clock))
    }

    pub unsafe fn new_application(src: *mut GstObject, structure: *mut GstStructure) -> Option<Message>{
        Message::new(gst_message_new_application(src,structure))
    }

    pub unsafe fn new_element(src: *mut GstObject, structure: *mut GstStructure) -> Option<Message>{
        Message::new(gst_message_new_element(src,structure))
    }

    pub unsafe fn new_custom(ty: GstMessageType, src: *mut GstObject, structure: *mut GstStructure) -> Option<Message>{
        Message::new(gst_message_new_custom(ty,src,structure))
    }

	#[allow(unused_variables)]
    pub unsafe fn gst_message(&self) -> *const GstMessage{
        match *self{
            Message::Unknown(msg) => msg,
            Message::Eos(msg) => msg,
            Message::Error(msg) => msg,
            Message::ErrorParsed{msg, ref error, ref debug} => msg,
            Message::Warning(msg) => msg,
            Message::WarningParsed{msg, ref error, ref debug} => msg,
            Message::Info(msg) => msg,
            Message::InfoParsed{msg, ref error, ref debug} => msg,
            Message::Tag(msg) => msg,
            Message::TagParsed{msg, ref tags} => msg,
            Message::Buffering(msg) => msg,
            Message::BufferingParsed{msg, ref pct} => msg,
            Message::StateChanged(msg) => msg,
            Message::StateChangedParsed{msg, ref old, ref new, ref pending} => msg,
            Message::StateDirty(msg) => msg,
            Message::StepDone(msg) => msg,
            Message::ClockProvide(msg) => msg,
            Message::ClockLost(msg) => msg,
            Message::NewClock(msg) => msg,
            Message::StructureChange(msg) => msg,
            Message::StreamStatus(msg) => msg,
            Message::Application(msg) => msg,
            Message::Element(msg) => msg,
            Message::SegmentStart(msg) => msg,
            Message::SegmentDone(msg) => msg,
            Message::DurationChanged(msg) => msg,
            Message::Latency(msg) => msg,
            Message::AsyncStart(msg) => msg,
            Message::AsyncDone(msg) => msg,
            Message::RequestState(msg) => msg,
            Message::StepStart(msg) => msg,
            Message::Qos(msg) => msg,
            Message::Progress(msg) => msg,
            Message::Toc(msg) => msg,
            Message::ResetTime(msg) => msg,
            Message::StreamStart(msg) => msg,
            Message::NeedContext(msg) => msg,
            Message::HaveContext(msg) => msg,
            Message::Extended(msg) => msg,
            Message::DeviceAdded(msg) => msg,
            Message::DeviceRemoved(msg) => msg,
            Message::Any(msg) => msg,
        }
    }

	#[allow(unused_variables)]
    pub unsafe fn gst_message_mut(&mut self) -> *mut GstMessage{
        match *self{
            Message::Unknown(msg) => msg,
            Message::Eos(msg) => msg,
            Message::Error(msg) => msg,
            Message::ErrorParsed{msg, ref error, ref debug} => msg,
            Message::Warning(msg) => msg,
            Message::WarningParsed{msg, ref error, ref debug} => msg,
            Message::Info(msg) => msg,
            Message::InfoParsed{msg, ref error, ref debug} => msg,
            Message::Tag(msg) => msg,
            Message::TagParsed{msg, ref tags} => msg,
            Message::Buffering(msg) => msg,
            Message::BufferingParsed{msg, ref pct} => msg,
            Message::StateChanged(msg) => msg,
            Message::StateChangedParsed{msg, ref old, ref new, ref pending} => msg,
            Message::StateDirty(msg) => msg,
            Message::StepDone(msg) => msg,
            Message::ClockProvide(msg) => msg,
            Message::ClockLost(msg) => msg,
            Message::NewClock(msg) => msg,
            Message::StructureChange(msg) => msg,
            Message::StreamStatus(msg) => msg,
            Message::Application(msg) => msg,
            Message::Element(msg) => msg,
            Message::SegmentStart(msg) => msg,
            Message::SegmentDone(msg) => msg,
            Message::DurationChanged(msg) => msg,
            Message::Latency(msg) => msg,
            Message::AsyncStart(msg) => msg,
            Message::AsyncDone(msg) => msg,
            Message::RequestState(msg) => msg,
            Message::StepStart(msg) => msg,
            Message::Qos(msg) => msg,
            Message::Progress(msg) => msg,
            Message::Toc(msg) => msg,
            Message::ResetTime(msg) => msg,
            Message::StreamStart(msg) => msg,
            Message::NeedContext(msg) => msg,
            Message::HaveContext(msg) => msg,
            Message::Extended(msg) => msg,
            Message::DeviceAdded(msg) => msg,
            Message::DeviceRemoved(msg) => msg,
            Message::Any(msg) => msg,
        }
    }

    pub fn ty(&self) -> GstMessageType{
        unsafe{
            (*self.gst_message()).type_
        }
    }

    pub fn type_name(&self) -> String{
        unsafe{
            from_c_str!(gst_message_type_get_name(self.ty())).to_string()
        }
    }

    pub fn seqnum(&self) -> u32{
        unsafe{
            gst_message_get_seqnum(mem::transmute(self.gst_message()))
        }
    }

    pub fn set_seqnum(&mut self, seqnum: u32){
        unsafe{
            gst_message_set_seqnum(self.gst_message_mut(),seqnum)
        }
    }

    pub fn timestamp(&self) -> u64{
        unsafe{
            (*self.gst_message()).timestamp
        }
    }

    pub unsafe fn src(&self) -> *mut GstObject{
        (*self.gst_message()).src
    }

    pub fn src_name(&self) -> String{
        unsafe{
            from_c_str!(mem::transmute((*self.src()).name)).to_string() // ???
        }
    }

    pub unsafe fn structure(&self) -> *const GstStructure{
        gst_message_get_structure(mem::transmute(self.gst_message()))
    }

    pub fn make_writable(&self) -> Option<Message>{
        unsafe{
            Message::new(gst_mini_object_make_writable(self.gst_message() as *mut GstMiniObject) as *mut GstMessage)
        }
    }

    pub fn is_writable(&self) -> bool{
        unsafe{
            gst_mini_object_is_writable(self.gst_message() as *mut GstMiniObject) == 1
        }
    }

    pub fn parse(&self) -> Message{
        unsafe{
			let ret = Message::new(gst_mini_object_copy(self.gst_message() as *mut GstMiniObject) as *const GstMessage).unwrap();
            match ret{
                Message::Error(message) => {
                    let mut error: *mut GError = ptr::null_mut();
                    let mut debug: *mut raw::c_char = ptr::null_mut();
                    gst_message_parse_error(message,&mut error,&mut debug);
                    let str_error = from_c_str!(mem::transmute(debug)).to_string();
                    g_free(mem::transmute(debug));
                    let message = gst_message_ref(message);
                    Message::ErrorParsed{msg: message, error: Error::new_from_g_error(error), debug: str_error}
                }
                Message::Warning(message) => {
                    let mut error: *mut GError = ptr::null_mut();
                    let mut debug: *mut raw::c_char = ptr::null_mut();
                    gst_message_parse_warning(message,&mut error,&mut debug);
                    let str_error = from_c_str!(mem::transmute(debug)).to_string();
                    g_free(mem::transmute(debug));
                    let message = gst_message_ref(message);
                    Message::WarningParsed{msg: message, error: Error::new_from_g_error(error), debug: str_error}
                }
                Message::Info(message) => {
                    let mut error: *mut GError = ptr::null_mut();
                    let mut debug: *mut raw::c_char = ptr::null_mut();
                    gst_message_parse_info(message,&mut error,&mut debug);
                    let str_error = from_c_str!(mem::transmute(debug)).to_string();
                    g_free(mem::transmute(debug));
                    let message = gst_message_ref(message);
                    Message::InfoParsed{msg: message, error: Error::new_from_g_error(error), debug: str_error}
                }
                Message::Tag(message) => {
                    let mut tags: *mut GstTagList = ptr::null_mut();
                    gst_message_parse_tag(message,&mut tags);
                    let message = gst_message_ref(message);
                    Message::TagParsed{msg: message, tags: tags}
                }
                Message::Buffering(message) => {
                    let mut pct: i32 = 0;
                    let message = gst_message_ref(message);
                    gst_message_parse_buffering(message,&mut pct);
                    Message::BufferingParsed{msg: message, pct: pct}
                }
                Message::StateChanged(message) => {
                    let mut old: GstState = GST_STATE_NULL;
                    let mut new: GstState = GST_STATE_NULL;
                    let mut pending: GstState = GST_STATE_NULL;
                    gst_message_parse_state_changed(message,&mut old,&mut new,&mut pending);
                    let message = gst_message_ref(message);
                    Message::StateChangedParsed{msg: message, old: old, new: new, pending: pending}
                }
                _ => {
                    ret
                }
                /*
                Message::StateDirty(message) => message,
                Message::StepDone(message) => message,
                Message::ClockProvide(message) => message,
                Message::ClockLost(message) => message,
                Message::NewClock(message) => message,
                Message::StructureChange(message) => message,
                Message::StreamStatus(message) => message,
                Message::Application(message) => message,
                Message::Element(message) => message,
                Message::SegmentStart(message) => message,
                Message::SegmentDone(message) => message,
                Message::DurationChanged(message) => message,
                Message::Latency(message) => message,
                Message::AsyncStart(message) => message,
                Message::AsyncDone(message) => message,
                Message::RequestState(message) => message,
                Message::StepStart(message) => message,
                Message::Qos(message) => message,
                Message::Progress(message) => message,
                Message::Toc(message) => message,
                Message::ResetTime(message) => message,
                Message::StreamStart(message) => message,
                Message::NeedContext(message) => message,
                Message::HaveContext(message) => message,
                Message::Extended(message) => message,
                Message::DeviceAdded(message) => message,
                Message::DeviceRemoved(message) => message,
                Message::Any(message) => message,*/
            }
        }
    }
}


impl ::Transfer<GstMessage> for Message{
    unsafe fn transfer(mut self) ->  *mut GstMessage{
        let message = self.gst_message_mut();
		mem::forget(self);
        message
    }
}

impl Reference for Message{
    fn reference(&self) -> Message{
        unsafe{
			Message::new(self.gst_message()).unwrap()
		}
    }
}
/*pub trait MessageT{
    unsafe fn gst_message(&self) -> *mut GstMessage;

    fn class_ty() -> GstMessageType;

    fn from_gst_msg(gst_message: *mut GstMessage) -> Option<Self>;

    fn make_writable(&self) -> Option<Self>;

    fn ty(&self) -> GstMessageType{
        unsafe{
            (*self.gst_message())._type
        }
    }

    fn type_name(&self) -> String{
        unsafe{
            from_c_str!(gst_message_type_get_name(self.ty())).to_string()
        }
    }

    fn seqnum(&self) -> u32{
        unsafe{
            gst_message_get_seqnum(self.gst_message())
        }
    }

    fn set_seqnum(&mut self, seqnum: u32){
        unsafe{
            gst_message_set_seqnum(self.gst_message(),seqnum)
        }
    }

    fn timestamp(&self) -> u64{
        unsafe{
            (*self.gst_message()).timestamp
        }
    }

    unsafe fn src(&self) -> *mut GstObject{
        (*self.gst_message()).src
    }

    fn src_name(&self) -> String{
        unsafe{
            from_c_str!(mem::transmute((*self.src()).name)).to_string()
        }
    }

    unsafe fn structure(&self) -> *const GstStructure{
        gst_message_get_structure(self.gst_message())
    }

    fn is_writable(&self) -> bool{
        unsafe{
            gst_mini_object_is_writable(self.gst_message() as *mut GstMiniObject) == 1
        }
    }
}

pub struct Unknown(MessagePrivate);
pub struct Eos(MessagePrivate);
pub struct Error(MessagePrivate);
pub struct Warning(MessagePrivate);
pub struct Info(MessagePrivate);
pub struct Tag(MessagePrivate);
pub struct Buffering(MessagePrivate);
pub struct StateChanged(MessagePrivate);
pub struct StateDirty(MessagePrivate);
pub struct StepDone(MessagePrivate);
pub struct ClockProvide(MessagePrivate);
pub struct ClockLost(MessagePrivate);
pub struct NewClock(MessagePrivate);
pub struct StructureChange(MessagePrivate);
pub struct StreamStatus(MessagePrivate);
pub struct Application(MessagePrivate);
pub struct Element(MessagePrivate);
pub struct SegmentStart(MessagePrivate);
pub struct SegmentDone(MessagePrivate);
pub struct DurationChanged(MessagePrivate);
pub struct Latency(MessagePrivate);
pub struct AsyncStart(MessagePrivate);
pub struct AsyncDone(MessagePrivate);
pub struct RequestState(MessagePrivate);
pub struct StepStart(MessagePrivate);
pub struct Qos(MessagePrivate);
pub struct Progress(MessagePrivate);
pub struct Toc(MessagePrivate);
pub struct ResetTime(MessagePrivate);
pub struct StreamStart(MessagePrivate);
pub struct NeedContext(MessagePrivate);
pub struct HaveContext(MessagePrivate);
pub struct Extended(MessagePrivate);
pub struct DeviceAdded(MessagePrivate);
pub struct DeviceRemoved(MessagePrivate);
pub struct Any(MessagePrivate);


macro_rules! msg_impl(
  ($t: ident, $msg_t: expr) => (
    impl MessageT for $t {
        unsafe fn gst_message(&self) -> *mut GstMessage{
            self.0.message.0
        }

        fn class_ty() -> GstMessageType{
            $msg_t
        }

        fn from_gst_msg(gst_message: *mut GstMessage) -> Option<$t>{
            if gst_message != ptr::null_mut(){
                unsafe{
                    let gst_message = gst_mini_object_ref(gst_message as *mut GstMiniObject) as *mut GstMessage;
                    if (*gst_message)._type == $msg_t{
                        Some($t(gst_message))
                    }else{
                        None
                    }
                }
            }else{
                None
            }
        }

        fn make_writable(&self) -> Option<$t>{
            unsafe{
                MessageT::from_gst_msg(gst_mini_object_make_writable(self.gst_message() as *mut GstMiniObject) as *mut GstMessage)
            }
        }
    }
  )
);

msg_impl!(Unknown,GST_MESSAGE_UNKNOWN);
msg_impl!(Eos,GST_MESSAGE_EOS);
msg_impl!(Error,GST_MESSAGE_ERROR);
msg_impl!(Warning,GST_MESSAGE_WARNING);
msg_impl!(Info,GST_MESSAGE_INFO);
msg_impl!(Tag,GST_MESSAGE_TAG);
msg_impl!(Buffering,GST_MESSAGE_BUFFERING);
msg_impl!(StateChanged,GST_MESSAGE_STATE_CHANGED);
msg_impl!(StateDirty,GST_MESSAGE_STATE_DIRTY);
msg_impl!(StepDone,GST_MESSAGE_STEP_DONE);
msg_impl!(ClockProvide,GST_MESSAGE_CLOCK_PROVIDE);
msg_impl!(ClockLost,GST_MESSAGE_CLOCK_LOST);
msg_impl!(NewClock,GST_MESSAGE_NEW_CLOCK);
msg_impl!(StructureChange,GST_MESSAGE_STRUCTURE_CHANGE);
msg_impl!(StreamStatus,GST_MESSAGE_STREAM_STATUS);
msg_impl!(Application,GST_MESSAGE_APPLICATION);
msg_impl!(Element,GST_MESSAGE_ELEMENT);
msg_impl!(SegmentStart,GST_MESSAGE_SEGMENT_START);
msg_impl!(SegmentDone,GST_MESSAGE_SEGMENT_DONE);
msg_impl!(DurationChanged,GST_MESSAGE_DURATION_CHANGED);
msg_impl!(Latency,GST_MESSAGE_LATENCY);
msg_impl!(AsyncStart,GST_MESSAGE_ASYNC_START);
msg_impl!(AsyncDone,GST_MESSAGE_ASYNC_DONE);
msg_impl!(RequestState,GST_MESSAGE_REQUEST_STATE);
msg_impl!(StepStart,GST_MESSAGE_STEP_START);
msg_impl!(Qos,GST_MESSAGE_QOS);
msg_impl!(Progress,GST_MESSAGE_PROGRESS);
msg_impl!(Toc,GST_MESSAGE_TOC);
msg_impl!(ResetTime,GST_MESSAGE_RESET_TIME);
msg_impl!(StreamStart,GST_MESSAGE_STREAM_START);
msg_impl!(NeedContext,GST_MESSAGE_NEED_CONTEXT);
msg_impl!(HaveContext,GST_MESSAGE_HAVE_CONTEXT);
msg_impl!(Extended,GST_MESSAGE_EXTENDED);
msg_impl!(DeviceAdded,GST_MESSAGE_DEVICE_ADDED);
msg_impl!(DeviceRemoved,GST_MESSAGE_DEVICE_REMOVED);
msg_impl!(Any,GST_MESSAGE_ANY);

impl Eos{
    pub fn new(src: *mut GstObject) -> Option<Eos>{
        unsafe{
            MessageT::from_gst_msg(gst_message_new_eos(src))
        }
    }
}*/
