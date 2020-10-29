use std::os::raw::*;

#[repr(C)]
#[derive(Debug)]
pub struct PilStruct {
    byte1: c_char,
    byte2: c_char,
    character1: c_char,
    character2: c_char,
    int: c_int,
    long: c_long,
    string: *const c_char,
    array: [u8; 8],
}

#[no_mangle]
pub extern "C" fn extract(c_struct: *mut PilStruct) -> i32 {
    let newstruct = PilStruct {
        byte1: 42,
        byte2: 43,
        character1: 'A' as c_char,
        character2: 'B' as c_char,
        int: 65535,
        long: 9223372036854775807,
        string: "pilrust\x00".as_ptr(),
        array: [80, 105, 99, 111, 76, 105, 115, 112],
    };
    unsafe {
        match c_struct.is_null() {
            true => return -1,
            _ => {},
        }
        println!("Received struct: {:#?}", &*c_struct);
        std::ptr::write(c_struct, newstruct)
    }
    0 // return code
}
