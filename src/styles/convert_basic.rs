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

use std::borrow::Cow;

use styles::types::{InlineRules, SpecificFontName, StyleSelector, Stylesheet};

impl From<InlineRules> for Stylesheet {
    fn from(rules: InlineRules) -> Self {
        Stylesheet { rules }
    }
}

impl From<&'static str> for StyleSelector {
    fn from(string: &'static str) -> Self {
        StyleSelector(Cow::from(string))
    }
}

impl From<String> for StyleSelector {
    fn from(string: String) -> Self {
        StyleSelector(Cow::from(string))
    }
}

impl From<&'static str> for SpecificFontName {
    fn from(string: &'static str) -> Self {
        SpecificFontName(Cow::from(string))
    }
}

impl From<String> for SpecificFontName {
    fn from(string: String) -> Self {
        SpecificFontName(Cow::from(string))
    }
}
