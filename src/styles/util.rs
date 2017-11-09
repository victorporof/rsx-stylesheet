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

use types::{FlexStyle, StyleDeclaration, ThemeStyle};

pub fn is_layout_style(declaration: &StyleDeclaration) -> Option<&FlexStyle> {
    match declaration {
        &StyleDeclaration::Layout(ref v) => Some(v),
        _ => None
    }
}

pub fn is_theme_style(declaration: &StyleDeclaration) -> Option<&ThemeStyle> {
    match declaration {
        &StyleDeclaration::Theme(ref v) => Some(v),
        _ => None
    }
}
