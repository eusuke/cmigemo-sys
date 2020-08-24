#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::redundant_static_lifetimes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;
    use std::ffi::CString;

    #[test]
    fn it_works() {
        unsafe {
            let dic_path = CString::new("test").unwrap();
            let m = migemo_open(dic_path.as_ptr());

            let query_string = "test";
            let query_cstring = CString::new(query_string).unwrap();
            let ret = migemo_query(m, query_cstring.as_bytes_with_nul().as_ptr());

            let ret_string = CStr::from_ptr(ret as *const i8).to_str().unwrap();
            assert_eq!(ret_string, query_string);

            migemo_release(m, ret);
        }
    }
}
