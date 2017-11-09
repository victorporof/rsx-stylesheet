/*
Copyright 2016 Mozilla
Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
*/

#![cfg_attr(feature = "cargo-clippy", allow(match_ref_pats))]
#![feature(proc_macro)]
#![feature(slice_patterns)]
#![feature(try_from)]

extern crate enum_str_derive;
#[macro_use]
extern crate quote;
extern crate rsx_shared;
extern crate self_tokenize_macro;
extern crate self_tokenize_trait;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate smallvec;

#[cfg(feature = "css-parse")]
pub extern crate servo_css_parser;
pub extern crate yoga;

#[macro_use]
mod styles;
mod computed_styles;

pub mod types {
    pub use computed_styles::types::*;
    pub use styles::types::*;
}
