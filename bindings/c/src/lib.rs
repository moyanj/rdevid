use std::ffi::CString;
use std::os::raw::c_char;

/// 导出设备ID函数供C/C++调用
#[unsafe(no_mangle)]
pub extern "C" fn device_id() -> *mut c_char {
    let id = rdevid::device_id();
    let c_str = CString::new(id).expect("CString::new failed");
    c_str.into_raw()
}

/// 导出设备信息函数供C/C++调用
#[unsafe(no_mangle)]
pub extern "C" fn get_device_info() -> *mut c_char {
    let info = rdevid::device_info();
    let c_str = CString::new(info).expect("CString::new failed");
    c_str.into_raw()
}

/// 释放字符串内存（需由调用方调用）
#[unsafe(no_mangle)]
pub extern "C" fn free_string(ptr: *mut c_char) {
    unsafe {
        if ptr.is_null() {
            return;
        }
        let _ = CString::from_raw(ptr);
    }
}
