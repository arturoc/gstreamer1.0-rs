use ffi::*;
use util::*;
use std::ptr::Unique;
use error::Error;

unsafe impl Send for GstMessage {}
unsafe impl Send for GstTagList {}
unsafe impl Send for GError {}

pub type MessagePrivate = Unique<GstMessage>;

fn msg_private(msg: *mut GstMessage) -> MessagePrivate{
	unsafe{
		Unique::new(msg)
	}
}

fn gst_message_ref(msg: &mut MessagePrivate) -> *mut GstMessage{
	unsafe{
		gst_mini_object_ref(mem::transmute(msg.get_mut())) as *mut GstMessage
	}
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
    TagParsed{msg: MessagePrivate, tags: Unique<GstTagList>},
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
    pub fn new(gst_message: *const GstMessage) -> Option<Message>{
        if gst_message != ptr::null(){
            unsafe{
                let gst_message = gst_mini_object_ref(gst_message as *mut GstMiniObject) as *mut GstMessage;
                match (*gst_message)._type{
                     GST_MESSAGE_UNKNOWN => Some(Message::Unknown(msg_private(gst_message))),
                     GST_MESSAGE_EOS => Some(Message::Eos(msg_private(gst_message))),
                     GST_MESSAGE_ERROR => Some(Message::Error(msg_private(gst_message))),
                     GST_MESSAGE_WARNING => Some(Message::Warning(msg_private(gst_message))),
                     GST_MESSAGE_INFO => Some(Message::Info(msg_private(gst_message))),
                     GST_MESSAGE_TAG => Some(Message::Tag(msg_private(gst_message))),
                     GST_MESSAGE_BUFFERING => Some(Message::Buffering(msg_private(gst_message))),
                     GST_MESSAGE_STATE_CHANGED => Some(Message::StateChanged(msg_private(gst_message))),
                     GST_MESSAGE_STATE_DIRTY => Some(Message::StateDirty(msg_private(gst_message))),
                     GST_MESSAGE_STEP_DONE => Some(Message::StepDone(msg_private(gst_message))),
                     GST_MESSAGE_CLOCK_PROVIDE => Some(Message::ClockProvide(msg_private(gst_message))),
                     GST_MESSAGE_CLOCK_LOST => Some(Message::ClockLost(msg_private(gst_message))),
                     GST_MESSAGE_NEW_CLOCK => Some(Message::NewClock(msg_private(gst_message))),
                     GST_MESSAGE_STRUCTURE_CHANGE => Some(Message::StructureChange(msg_private(gst_message))),
                     GST_MESSAGE_STREAM_STATUS => Some(Message::StreamStatus(msg_private(gst_message))),
                     GST_MESSAGE_APPLICATION => Some(Message::Application(msg_private(gst_message))),
                     GST_MESSAGE_ELEMENT => Some(Message::Element(msg_private(gst_message))),
                     GST_MESSAGE_SEGMENT_START => Some(Message::SegmentStart(msg_private(gst_message))),
                     GST_MESSAGE_SEGMENT_DONE => Some(Message::SegmentDone(msg_private(gst_message))),
                     GST_MESSAGE_DURATION_CHANGED => Some(Message::DurationChanged(msg_private(gst_message))),
                     GST_MESSAGE_LATENCY => Some(Message::Latency(msg_private(gst_message))),
                     GST_MESSAGE_ASYNC_START => Some(Message::AsyncStart(msg_private(gst_message))),
                     GST_MESSAGE_ASYNC_DONE => Some(Message::AsyncDone(msg_private(gst_message))),
                     GST_MESSAGE_REQUEST_STATE => Some(Message::RequestState(msg_private(gst_message))),
                     GST_MESSAGE_STEP_START => Some(Message::StepStart(msg_private(gst_message))),
                     GST_MESSAGE_QOS => Some(Message::Qos(msg_private(gst_message))),
                     GST_MESSAGE_PROGRESS => Some(Message::Progress(msg_private(gst_message))),
                     GST_MESSAGE_TOC => Some(Message::Toc(msg_private(gst_message))),
                     GST_MESSAGE_RESET_TIME => Some(Message::ResetTime(msg_private(gst_message))),
                     GST_MESSAGE_STREAM_START => Some(Message::StreamStart(msg_private(gst_message))),
                     GST_MESSAGE_NEED_CONTEXT => Some(Message::NeedContext(msg_private(gst_message))),
                     GST_MESSAGE_HAVE_CONTEXT => Some(Message::HaveContext(msg_private(gst_message))),
                     GST_MESSAGE_EXTENDED => Some(Message::Extended(msg_private(gst_message))),
                     GST_MESSAGE_DEVICE_ADDED => Some(Message::DeviceAdded(msg_private(gst_message))),
                     GST_MESSAGE_DEVICE_REMOVED => Some(Message::DeviceRemoved(msg_private(gst_message))),
                     GST_MESSAGE_ANY => Some(Message::Any(msg_private(gst_message))),
                     _ => None
                }
            }
        }else{
            None
        }
    }
    
    pub fn new_eos(src: *mut GstObject) -> Option<Message>{
        unsafe{
            Message::new(gst_message_new_eos(src))
        }
    }
    
    pub fn new_error(src: *mut GstObject, error: *mut GError, debug: &str) -> Option<Message>{
        unsafe{
            Message::new(gst_message_new_error(src,error,to_c_str!(debug)))
        }
    }

    pub fn new_warning(src: *mut GstObject, error: *mut GError, debug: &str) -> Option<Message>{
        unsafe{
            Message::new(gst_message_new_warning(src,error,to_c_str!(debug)))
        }
    }

    pub fn new_info(src: *mut GstObject, error: *mut GError, debug: &str) -> Option<Message>{
        unsafe{
            Message::new(gst_message_new_info(src,error,to_c_str!(debug)))
        }
    }

    pub fn new_tag(src: *mut GstObject, tag_list: *mut GstTagList) -> Option<Message>{
        unsafe{
            Message::new(gst_message_new_tag(src,tag_list))
        }
    }

    pub fn new_buffering(src: *mut GstObject, pct: i32) -> Option<Message>{
        unsafe{
            Message::new(gst_message_new_buffering(src,pct))
        }
    }

    pub fn new_state_changed(src: *mut GstObject, old_state: GstState, new_state: GstState, pending: GstState) -> Option<Message>{
        unsafe{
            Message::new(gst_message_new_state_changed(src,old_state,new_state,pending))
        }
    }

    pub fn new_state_dirty(src: *mut GstObject) -> Option<Message>{
        unsafe{
            Message::new(gst_message_new_state_dirty(src))
        }
    }

    pub fn new_step_done(src: *mut GstObject, format: GstFormat,
                         amount: u64, rate: f64,
                         flush: bool, intermediate: bool,
                         duration: u64, eos: bool) -> Option<Message>{
        unsafe{
            Message::new(gst_message_new_step_done(src,format,amount,rate,flush as i32,intermediate as i32,duration,eos as i32))
        }
    }

    pub fn new_clock_provide(src: *mut GstObject, clock: *mut GstClock, ready: bool) -> Option<Message>{
        unsafe{
            Message::new(gst_message_new_clock_provide(src,clock,ready as i32))
        }
    }

    pub fn new_clock_lost(src: *mut GstObject, clock: *mut GstClock) -> Option<Message>{
        unsafe{
            Message::new(gst_message_new_clock_lost(src,clock))
        }
    }

    pub fn new_new_clock(src: *mut GstObject, clock: *mut GstClock) -> Option<Message>{
        unsafe{
            Message::new(gst_message_new_new_clock(src,clock))
        }
    }

    pub fn new_application(src: *mut GstObject, structure: *mut GstStructure) -> Option<Message>{
        unsafe{
            Message::new(gst_message_new_application(src,structure))
        }
    }

    pub fn new_element(src: *mut GstObject, structure: *mut GstStructure) -> Option<Message>{
        unsafe{
            Message::new(gst_message_new_element(src,structure))
        }
    }

    pub fn new_custom(ty: GstMessageType, src: *mut GstObject, structure: *mut GstStructure) -> Option<Message>{
        unsafe{
            Message::new(gst_message_new_custom(ty,src,structure))
        }
    }

	#[allow(unused_variables)]
    pub unsafe fn gst_message(&self) -> *const GstMessage{
        match *self{
            Message::Unknown(ref msg) => msg.get(),
            Message::Eos(ref msg) => msg.get(),
            Message::Error(ref msg) => msg.get(),
            Message::ErrorParsed{ref msg, ref error, ref debug} => msg.get(),
            Message::Warning(ref msg) => msg.get(),
            Message::WarningParsed{ref msg, ref error, ref debug} => msg.get(),
            Message::Info(ref msg) => msg.get(),
            Message::InfoParsed{ref msg, ref error, ref debug} => msg.get(),
            Message::Tag(ref msg) => msg.get(),
            Message::TagParsed{ref msg, ref tags} => msg.get(),
            Message::Buffering(ref msg) => msg.get(),
            Message::BufferingParsed{ref msg, ref pct} => msg.get(),
            Message::StateChanged(ref msg) => msg.get(),
            Message::StateChangedParsed{ref msg, ref old, ref new, ref pending} => msg.get(),
            Message::StateDirty(ref msg) => msg.get(),
            Message::StepDone(ref msg) => msg.get(),
            Message::ClockProvide(ref msg) => msg.get(),
            Message::ClockLost(ref msg) => msg.get(),
            Message::NewClock(ref msg) => msg.get(),
            Message::StructureChange(ref msg) => msg.get(),
            Message::StreamStatus(ref msg) => msg.get(),
            Message::Application(ref msg) => msg.get(),
            Message::Element(ref msg) => msg.get(),
            Message::SegmentStart(ref msg) => msg.get(),
            Message::SegmentDone(ref msg) => msg.get(),
            Message::DurationChanged(ref msg) => msg.get(),
            Message::Latency(ref msg) => msg.get(),
            Message::AsyncStart(ref msg) => msg.get(),
            Message::AsyncDone(ref msg) => msg.get(),
            Message::RequestState(ref msg) => msg.get(),
            Message::StepStart(ref msg) => msg.get(),
            Message::Qos(ref msg) => msg.get(),
            Message::Progress(ref msg) => msg.get(),
            Message::Toc(ref msg) => msg.get(),
            Message::ResetTime(ref msg) => msg.get(),
            Message::StreamStart(ref msg) => msg.get(),
            Message::NeedContext(ref msg) => msg.get(),
            Message::HaveContext(ref msg) => msg.get(),
            Message::Extended(ref msg) => msg.get(),
            Message::DeviceAdded(ref msg) => msg.get(),
            Message::DeviceRemoved(ref msg) => msg.get(),
            Message::Any(ref msg) => msg.get(),
        }
    }

	#[allow(unused_variables)]
    pub unsafe fn gst_message_mut(&mut self) -> *mut GstMessage{
        match *self{
            Message::Unknown(ref mut msg) => msg.get_mut(),
            Message::Eos(ref mut msg) => msg.get_mut(),
            Message::Error(ref mut msg) => msg.get_mut(),
            Message::ErrorParsed{ref mut msg, ref error, ref debug} => msg.get_mut(),
            Message::Warning(ref mut msg) => msg.get_mut(),
            Message::WarningParsed{ref mut msg, ref error, ref debug} => msg.get_mut(),
            Message::Info(ref mut msg) => msg.get_mut(),
            Message::InfoParsed{ref mut msg, ref error, ref debug} => msg.get_mut(),
            Message::Tag(ref mut msg) => msg.get_mut(),
            Message::TagParsed{ref mut msg, ref tags} => msg.get_mut(),
            Message::Buffering(ref mut msg) => msg.get_mut(),
            Message::BufferingParsed{ref mut msg, ref pct} => msg.get_mut(),
            Message::StateChanged(ref mut msg) => msg.get_mut(),
            Message::StateChangedParsed{ref mut msg, ref old, ref new, ref pending} => msg.get_mut(),
            Message::StateDirty(ref mut msg) => msg.get_mut(),
            Message::StepDone(ref mut msg) => msg.get_mut(),
            Message::ClockProvide(ref mut msg) => msg.get_mut(),
            Message::ClockLost(ref mut msg) => msg.get_mut(),
            Message::NewClock(ref mut msg) => msg.get_mut(),
            Message::StructureChange(ref mut msg) => msg.get_mut(),
            Message::StreamStatus(ref mut msg) => msg.get_mut(),
            Message::Application(ref mut msg) => msg.get_mut(),
            Message::Element(ref mut msg) => msg.get_mut(),
            Message::SegmentStart(ref mut msg) => msg.get_mut(),
            Message::SegmentDone(ref mut msg) => msg.get_mut(),
            Message::DurationChanged(ref mut msg) => msg.get_mut(),
            Message::Latency(ref mut msg) => msg.get_mut(),
            Message::AsyncStart(ref mut msg) => msg.get_mut(),
            Message::AsyncDone(ref mut msg) => msg.get_mut(),
            Message::RequestState(ref mut msg) => msg.get_mut(),
            Message::StepStart(ref mut msg) => msg.get_mut(),
            Message::Qos(ref mut msg) => msg.get_mut(),
            Message::Progress(ref mut msg) => msg.get_mut(),
            Message::Toc(ref mut msg) => msg.get_mut(),
            Message::ResetTime(ref mut msg) => msg.get_mut(),
            Message::StreamStart(ref mut msg) => msg.get_mut(),
            Message::NeedContext(ref mut msg) => msg.get_mut(),
            Message::HaveContext(ref mut msg) => msg.get_mut(),
            Message::Extended(ref mut msg) => msg.get_mut(),
            Message::DeviceAdded(ref mut msg) => msg.get_mut(),
            Message::DeviceRemoved(ref mut msg) => msg.get_mut(),
            Message::Any(ref mut msg) => msg.get_mut(),
        }
    }

    pub fn ty(&self) -> GstMessageType{
        unsafe{
            (*self.gst_message())._type
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
            from_c_str!(mem::transmute((*self.src()).name)).to_string()
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
			let mut ret = Message::new(gst_mini_object_copy(self.gst_message() as *mut GstMiniObject) as *const GstMessage).unwrap();
            match ret{
                Message::Error(ref mut message) => {
                    let mut error: *mut GError = ptr::null_mut();
                    let mut debug: *mut ::libc::c_char = ptr::null_mut();
                    gst_message_parse_error(message.get_mut(),&mut error,&mut debug);
                    let str_error = from_c_str!(mem::transmute(debug)).to_string();
                    g_free(mem::transmute(debug));
                    let message = gst_message_ref(message);
                    Message::ErrorParsed{msg: msg_private(message), error: Error::new_from_g_error(error), debug: str_error}
                }
                Message::Warning(ref mut message) => {
                    let mut error: *mut GError = ptr::null_mut();
                    let mut debug: *mut ::libc::c_char = ptr::null_mut();
                    gst_message_parse_warning(message.get_mut(),&mut error,&mut debug);
                    let str_error = from_c_str!(mem::transmute(debug)).to_string();
                    g_free(mem::transmute(debug));
                    let message = gst_message_ref(message);
                    Message::WarningParsed{msg: msg_private(message), error: Error::new_from_g_error(error), debug: str_error}
                }
                Message::Info(ref mut message) => {
                    let mut error: *mut GError = ptr::null_mut();
                    let mut debug: *mut ::libc::c_char = ptr::null_mut();
                    gst_message_parse_info(message.get_mut(),&mut error,&mut debug);
                    let str_error = from_c_str!(mem::transmute(debug)).to_string();
                    g_free(mem::transmute(debug));
                    let message = gst_message_ref(message);
                    Message::InfoParsed{msg: msg_private(message), error: Error::new_from_g_error(error), debug: str_error}
                }
                Message::Tag(ref mut message) => {
                    let mut tags: *mut GstTagList = ptr::null_mut();
                    gst_message_parse_tag(message.get_mut(),&mut tags);
                    let message = gst_message_ref(message);
                    Message::TagParsed{msg: msg_private(message), tags: Unique::new(tags)}
                }
                Message::Buffering(ref mut message) => {
                    let mut pct: i32 = 0;
                    let message = gst_message_ref(message);
                    gst_message_parse_buffering(message,&mut pct);
                    Message::BufferingParsed{msg: msg_private(message), pct: pct}
                }
                Message::StateChanged(ref mut message) => {
                    let mut old: GstState = GST_STATE_NULL;
                    let mut new: GstState = GST_STATE_NULL;
                    let mut pending: GstState = GST_STATE_NULL;
                    gst_message_parse_state_changed(message.get_mut(),&mut old,&mut new,&mut pending);
                    let message = gst_message_ref(message);
                    Message::StateChangedParsed{msg: msg_private(message), old: old, new: new, pending: pending}
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
                        Some($t(msg_private(gst_message)))
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
