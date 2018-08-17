extern crate nature_common;

use nature_common::*;

struct SimpleConverter;

impl ConverterTrait for SimpleConverter {
    fn convert(para: CallOutParameter) -> ConverterReturned {
        ConverterReturned::Instances(vec![Instance::default()])
    }
}