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
use std::convert::TryInto;
use std::iter::FromIterator;
use std::iter::Iterator;
use std::ops::{Deref, DerefMut};
use std::slice;

use rsx_shared::traits::TStyleDeclarations;
use rsx_shared::types::KnownElementName;
use self_tokenize_macro::{DefaultQuote, SelfTokenize};
use self_tokenize_trait::ToCustomTokens;

use styles::util::{is_layout_style, is_theme_style};

pub use styles::longhands::*;

pub type InlineRules = SmallVec<[StyleRule; 1]>;
pub type InlineSelectors = SmallVec<[StyleSelector; 1]>;
pub type InlineDeclarations = SmallVec<[StyleDeclaration; 8]>;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, DefaultQuote)]
pub struct Stylesheet {
    pub(crate) rules: InlineRules
}

impl Stylesheet {
    pub fn push(&mut self, rule: StyleRule) {
        // TODO: dedupe declarations when already pushed with the same selector.
        self.rules.push(rule);
    }

    pub fn index_of<T>(&self, selector: T) -> Option<usize>
    where
        T: AsRef<str>
    {
        // TODO: actually handle selector matching, specificity etc.
        self.rules
            .iter()
            .position(|&StyleRule { ref selectors, .. }| selectors.iter().any(|s| s.as_ref() == selector.as_ref()))
    }

    pub fn take<T>(&mut self, selector: T) -> StyleDeclarations
    where
        T: AsRef<str>
    {
        self.index_of(selector)
            .map(|i| self.rules.swap_remove(i).declarations)
            .unwrap_or_else(|| StyleDeclarations(InlineDeclarations::default()))
    }

    pub fn get_copy<T>(&mut self, selector: T) -> Option<StyleDeclarations>
    where
        T: AsRef<str>
    {
        self.index_of(selector)
            .map(|i| self.rules[i].declarations.clone())
    }

    pub fn get_ref<T>(&self, selector: T) -> Option<&StyleDeclarations>
    where
        T: AsRef<str>
    {
        self.index_of(selector)
            .map(move |i| &self.rules[i].declarations)
    }

    pub fn get_mut<T>(&mut self, selector: T) -> Option<&mut StyleDeclarations>
    where
        T: AsRef<str>
    {
        self.index_of(selector)
            .map(move |i| &mut self.rules[i].declarations)
    }
}

#[cfg(debug_assertions)]
#[cfg(feature = "log-unused")]
impl Drop for Stylesheet {
    fn drop(&mut self) {
        self.rules
            .iter()
            .flat_map(|rule| rule.selectors.iter())
            .for_each(|selector| {
                println!("Warning: Unused selector: {:?}", selector.css_string.0);
            });
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, DefaultQuote)]
pub struct StyleSelector(pub Cow<'static, str>);

impl AsRef<str> for StyleSelector {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize, SelfTokenize)]
pub struct StyleRule {
    pub selectors: StyleSelectors,
    pub declarations: StyleDeclarations
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize, SelfTokenize)]
pub struct StyleSelectors(pub InlineSelectors);

impl Deref for StyleSelectors {
    type Target = InlineSelectors;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StyleSelectors {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromIterator<StyleSelector> for StyleSelectors {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = StyleSelector>
    {
        StyleSelectors(iter.into_iter().collect())
    }
}

impl<'a> IntoIterator for &'a StyleSelectors {
    type Item = &'a StyleSelector;
    type IntoIter = slice::Iter<'a, StyleSelector>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut StyleSelectors {
    type Item = &'a mut StyleSelector;
    type IntoIter = slice::IterMut<'a, StyleSelector>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize, SelfTokenize)]
pub struct StyleDeclarations(pub InlineDeclarations);

impl Deref for StyleDeclarations {
    type Target = InlineDeclarations;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StyleDeclarations {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromIterator<StyleDeclaration> for StyleDeclarations {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = StyleDeclaration>
    {
        StyleDeclarations(iter.into_iter().filter_map(|v| v.into_known()).collect())
    }
}

impl<'a> IntoIterator for &'a StyleDeclarations {
    type Item = &'a StyleDeclaration;
    type IntoIter = slice::Iter<'a, StyleDeclaration>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut StyleDeclarations {
    type Item = &'a mut StyleDeclaration;
    type IntoIter = slice::IterMut<'a, StyleDeclaration>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl TStyleDeclarations for StyleDeclarations {
    type LayoutStyle = FlexStyle;
    type ThemeStyle = ThemeStyle;

    fn make_user_agent_styles<T>(tag: T) -> Self
    where
        T: TryInto<KnownElementName>
    {
        match tag.try_into() {
            Ok(KnownElementName::Root) => style! {
                #include("defaults.css");
                align-items: { flex-start };
                width: { 100% };
                height: { 100% };
                flex-wrap: { wrap };
            },
            Ok(KnownElementName::Button) => style! {
                #include("defaults.css");
                align-items: { center };
                align-content: { center };
                background-color: { rgb(240, 240, 240) };
                border-width: { 2 };
                padding-left: { 8 px };
                padding-right: { 8 px };
                font-size: { 11 px };
            },
            _ => style! {
                #include("defaults.css");
            }
        }
    }

    fn for_each_layout_style<F>(&self, f: F)
    where
        F: FnMut(&Self::LayoutStyle)
    {
        self.into_iter().filter_map(is_layout_style).for_each(f);
    }

    fn for_each_theme_style<F>(&self, f: F)
    where
        F: FnMut(&Self::ThemeStyle)
    {
        self.into_iter().filter_map(is_theme_style).for_each(f);
    }
}
