use percent_encoding::{PercentEncode, AsciiSet, utf8_percent_encode};

pub fn percent_encode(src: &str) -> PercentEncode {
    lazy_static::lazy_static! {
        static ref ENCODER: AsciiSet = percent_encoding::NON_ALPHANUMERIC.remove(b'-').remove(b'_').remove(b'.').remove(b'~');
    }
    utf8_percent_encode(src, &*ENCODER)
}
