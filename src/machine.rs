use crate::extra;
use std::fs;

/// Read product version from `/sys/class/dmi/id/product_version`
pub fn product_version() -> String {
    let name = fs::read_to_string("/sys/class/dmi/id/product_version");
    let ret = match name {
        Ok(ret) => ret,
        Err(_e) => return String::new(),
    };
    extra::pop_newline(ret)
}

/// Read system vendor from `/sys/class/dmi/id/sys_vendor`
pub fn sys_vendor() -> String {
    let name = fs::read_to_string("/sys/class/dmi/id/sys_vendor");
    let ret = match name {
        Ok(ret) => ret,
        Err(_e) => return String::new(),
    };
    extra::pop_newline(ret)
}

/// Read product family from `/sys/class/dmi/id/sys_vendor`
pub fn product_family() -> String {
    let name = fs::read_to_string("/sys/class/dmi/id/product_family");
    let ret = match name {
        Ok(ret) => ret,
        Err(_e) => return String::new(),
    };
    extra::pop_newline(ret)
}

