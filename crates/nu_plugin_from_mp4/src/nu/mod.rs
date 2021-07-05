#[cfg(test)]
mod tests;

use crate::FromMp4;
use nu_errors::ShellError;
use nu_plugin::Plugin;
use nu_protocol::{CallInfo, Primitive, ReturnValue, Signature, UntaggedValue, Value};
use nu_source::Tag;

impl Plugin for FromMp4 {
    fn config(&mut self) -> Result<Signature, ShellError> {
        Ok(Signature::build("from mp4")
            .desc("Get meta-data of mp4 file")
            .filter())
    }

    fn begin_filter(&mut self, call_info: CallInfo) -> Result<Vec<ReturnValue>, ShellError> {
        self.name_tag = call_info.name_tag;
        Ok(vec![])
    }

    fn filter(&mut self, input: Value) -> Result<Vec<ReturnValue>, ShellError> {
        match input {
            Value {
                value: UntaggedValue::Primitive(Primitive::Binary(b)),
                ..
            } => {
                self.state.extend_from_slice(&b);
            }
            Value { tag, .. } => {
                return Err(ShellError::labeled_error_with_secondary(
                    "Expected binary from pipeline",
                    "requires binary input",
                    self.name_tag.clone(),
                    "value originates from here",
                    tag,
                ));
            }
        }
        Ok(vec![])
    }

    fn end_filter(&mut self) -> Result<Vec<ReturnValue>, ShellError> {
        crate::from_mp4::from_mp4(self.state.clone(), Tag::unknown())
    }
}
