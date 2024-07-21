use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use objc::runtime::Object;

use objc::Encode;
use objc::rc::StrongPtr;
use objc::runtime;

#[no_mangle]
pub extern "C" fn rust_hello(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };
    CString::new("from-lib.rs->HeLLo ".to_owned() + recipient)
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn rust_hello_free(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}

#[macro_use]
extern crate objc;
use objc::runtime::Class;

#[no_mangle]
pub extern "C" fn rust_objc() {
    // Get a class
    let cls = class!(NSObject);
    println!("NSObject size: {}", cls.instance_size());

    // Inspect its ivars
    println!("NSObject ivars:");
    for ivar in cls.instance_variables().iter() {
        println!("{}", ivar.name());
    }

    // Allocate an instance
    let obj = unsafe {
        let obj: *mut Object = msg_send![cls, alloc];
        let obj: *mut Object = msg_send![obj, init];
        StrongPtr::new(obj)
    };
    println!("NSObject address: {:p}", obj);

    // Access an ivar of the object
    let isa: *const Class = unsafe {
        *(**obj).get_ivar("isa")
    };
    println!("NSObject isa: {:?}", isa);

    // Inspect a method of the class
    let hash_sel = sel!(hash);
    let hash_method = cls.instance_method(hash_sel).unwrap();
    let hash_return = hash_method.return_type();
    println!("-[NSObject hash] return type: {:?}", hash_return);
    assert!(*hash_return == usize::ENCODING);
}
