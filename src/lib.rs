extern crate nature_common;

use nature_common::*;

#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes)]
pub extern fn rtn_none(_para: &ConverterParameter) -> ConverterReturned {
    ConverterReturned::None
}

#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes)]
pub extern fn rtn_logical_error(_para: &ConverterParameter) -> ConverterReturned {
    ConverterReturned::LogicalError("logical".to_string())
}

#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes)]
pub extern fn rtn_one(_para: &ConverterParameter) -> ConverterReturned {
    let mut instance = Instance::default();
    instance.data.content = "one".to_string();
    ConverterReturned::Instances(vec![instance])
}

#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes)]
pub extern fn rtn_tow(_para: &ConverterParameter) -> ConverterReturned {
    let mut one = Instance::default();
    one.data.content = "one".to_string();
    let mut two = Instance::default();
    two.data.content = "two".to_string();
    ConverterReturned::Instances(vec![one, two])
}

#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes)]
pub extern fn rtn_environment_error(_para: &ConverterParameter) -> ConverterReturned {
    ConverterReturned::EnvError("aforethought".to_string())
}

