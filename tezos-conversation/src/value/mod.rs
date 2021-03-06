// Copyright (c) SimpleStaking and Tezedge Contributors
// SPDX-License-Identifier: MIT

mod chunked_data;
mod fields;
mod message;
mod named;

use std::ops::Range;

pub trait HasBodyRange {
    fn body(&self) -> Range<usize>;
    fn set_continuation(&self);
}

pub use self::fields::{TezosEncoded, Named};
pub use self::chunked_data::{DecodingError, ChunkedData};
pub use self::message::show;
