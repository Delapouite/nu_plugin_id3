mod parse;

use nu_errors::ShellError;
use nu_plugin::{serve_plugin, Plugin};
use nu_protocol::{
    CallInfo, Primitive, ReturnSuccess, ReturnValue, Signature, TaggedDictBuilder, UntaggedValue, Value,
};

use id3::Tag as Id3Tag;
use parse::Id3Tag as MyTag;

struct Id3;

impl Id3 {
    fn new() -> Id3 {
        Id3
    }

    fn id3(&mut self, value: Value) -> Result<Value, ShellError> {
        let row = parse_id3_tag(&value);
    }
}

impl Plugin for Id3 {
    fn config(&mut self) -> Result<Signature, ShellError> {
        Ok(Signature::build("id3").desc("Display Id3 tag information for mp3 files").filter())
    }

    fn begin_filter(&mut self, _: CallInfo) -> Result<Vec<ReturnValue>, ShellError> {
        Ok(vec![])
    }

    fn filter(&mut self, input: Value) -> Result<Vec<ReturnValue>, ShellError> {
        Ok(vec![ReturnSuccess::value(self.id3(input)?)])
    }
}

fn main() {
    serve_plugin(&mut Id3::new());
}
