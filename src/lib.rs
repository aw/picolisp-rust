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
        string: "pilrust\x00".as_ptr() as *const _,
        array: [80, 105, 99, 111, 76, 105, 115, 112],
    };
    if let true = c_struct.is_null() { return -1 };
    unsafe {
        let r_struct = &*c_struct;
        println!("Received struct: {:#?}", &r_struct);

        // tests
        assert_eq!(r_struct.byte1, 32);
        assert_eq!(r_struct.byte2, 33);
        assert_eq!(r_struct.character1, 67);
        assert_eq!(r_struct.character2, 68);
        assert_eq!(r_struct.int, -1);
        assert_eq!(r_struct.long, 1);
        assert_eq!(r_struct.array, [1,2,3,4,5,6,7,8]);

        std::ptr::write(c_struct, newstruct);
    }
    0 // return code
}
