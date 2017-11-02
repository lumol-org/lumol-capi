#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum lml_status {
    LML_SUCCESS = 0,
    LML_ERROR = 1,
}

macro_rules! try {
    ($e: expr) => (
        match $e {
            Ok(val) => val,
            Err(_) => return $crate::lml_status::LML_ERROR
        }
    );
}
