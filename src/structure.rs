use ffi::*;
use util::*;

pub struct Structure{
    structure: *mut GstStructure,
}

impl Structure{
    pub unsafe fn new_from_gst_structure(structure: *mut GstStructure) -> Option<Structure>{
        if structure!=ptr::null_mut(){
            Some(Structure{
                structure: structure
            })
        }else{
            None
        }
    }

    pub fn name(&self) -> &str{
        unsafe{
            let cname = gst_structure_get_name(self.structure);
            from_c_str!(cname)
        }
    }
}
