extern crate nature_common;

use nature_common::*;


#[no_mangle]
pub extern fn simple_convert(para: &CallOutParameter) -> ConverterReturned {
    ConverterReturned::Instances(vec![para.from.clone()])
}
