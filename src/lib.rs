extern crate nature_common;

use nature_common::*;


#[no_mangle]
pub extern fn rtn_none(_para: &CallOutParameter) -> ConverterReturned {
    ConverterReturned::None
}

#[no_mangle]
pub extern fn rtn_one(_para: &CallOutParameter) -> ConverterReturned {
    let mut instance = Instance::default();
    instance.data.content = "one".to_string();
    ConverterReturned::Instances(vec![instance])
}

#[no_mangle]
pub extern fn rtn_tow(_para: &CallOutParameter) -> ConverterReturned {
    let mut one = Instance::default();
    one.data.content = "one".to_string();
    let mut two = Instance::default();
    two.data.content = "two".to_string();
    ConverterReturned::Instances(vec![one, two])
}

#[no_mangle]
pub extern fn rtn_logical_error(_para: &CallOutParameter) -> ConverterReturned {
    ConverterReturned::LogicalError("logical".to_string())
}

#[no_mangle]
pub extern fn rtn_environment_error(_para: &CallOutParameter) -> ConverterReturned {
    ConverterReturned::EnvError
}

