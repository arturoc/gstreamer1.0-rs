use ffi::*;
use std::ptr;
use std::marker;
use std::mem;

pub struct Iter<Item>{
    it: *mut GstIterator,
    marker: marker::PhantomData<Item>
}

impl<Item> Drop for Iter<Item>{
    fn drop(&mut self){
        unsafe{
            gst_iterator_free(self.it);
        }
    }
}

impl<Item> Iter<Item>{
    pub unsafe fn new_from_gst_iterator(it: *mut GstIterator) -> Option<Iter<Item>>{
        if it != ptr::null_mut(){
            Some(Iter{
                it: it,
                marker: marker::PhantomData,
            })
        }else{
            None
        }
    }
}

pub enum Error{
    Resync,
    Error,
    WrongType,
}

impl<I: ::FromGValue> Iterator for Iter<I>{
    type Item = Result<I,Error>;

    fn next(&mut self) -> Option<Result<I,Error>>{
        unsafe{
            let mut elem: GValue = mem::zeroed();
            match gst_iterator_next(self.it, &mut elem){
                GST_ITERATOR_DONE => None,
                GST_ITERATOR_OK => {
                    let ret = match I::from_gvalue(&elem){
                        Some(value) => Some(Ok(value)),
                        None => Some(Err(Error::WrongType))
                    };
                    g_value_unset(&mut elem);
                    ret
                },
                GST_ITERATOR_RESYNC => Some(Err(Error::Resync)),
                GST_ITERATOR_ERROR => Some(Err(Error::Error)),
                _ => None
            }
        }
    }
}
