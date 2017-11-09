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

use enum_str_derive::EnumStrSnakeCase;
use rsx_shared::types::SharedUnit;
use self_tokenize_macro::{DefaultQuote, SelfTokenize};
use self_tokenize_trait::ToCustomTokens;

pub use smallvec::SmallVec;
pub use yoga::{Align, Display, FlexDirection, FlexStyle, Justify, Overflow, PositionType, StyleUnit, Wrap};

pub type InlineBoxShadows = SmallVec<[BoxShadow; 1]>;
pub type InlineTextShadows = SmallVec<[TextShadow; 1]>;
pub type InlineFontNames = SmallVec<[FontName; 1]>;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, SelfTokenize)]
pub enum StyleDeclaration {
    Unknown,
    Theme(ThemeStyle),
    Layout(FlexStyle)
}

impl StyleDeclaration {
    pub fn into_known(self) -> Option<Self> {
        match self {
            StyleDeclaration::Unknown => None,
            StyleDeclaration::Theme(theme) => Some(StyleDeclaration::Theme(theme)),
            StyleDeclaration::Layout(layout) => Some(StyleDeclaration::Layout(layout))
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, SelfTokenize)]
pub enum ThemeStyle {
    Cursor(Cursor),
    Color(Color),
    BackgroundColor(Color),
    Opacity(u32),
    BorderTopColor(Color),
    BorderLeftColor(Color),
    BorderBottomColor(Color),
    BorderRightColor(Color),
    BorderTopStyle(BorderStyle),
    BorderBottomStyle(BorderStyle),
    BorderLeftStyle(BorderStyle),
    BorderRightStyle(BorderStyle),
    BoxShadow(InlineBoxShadows),
    TextShadow(InlineTextShadows),
    FontFamily(FontFamily),
    FontStyle(FontStyle),
    FontCaps(FontCaps),
    FontWeight(FontWeight),
    FontSize(FontSize),
    FontStretch(FontStretch),
    Visibility(Visibility)
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize, EnumStrSnakeCase, SelfTokenize)]
pub enum Cursor {
    Auto,
    None,
    Default,
    Pointer,
    ContextMenu,
    Help,
    Progress,
    Wait,
    Cell,
    Crosshair,
    Text,
    VerticalText,
    Alias,
    Copy,
    Move,
    NoDrop,
    NotAllowed,
    Grab,
    Grabbing,
    EResize,
    NResize,
    NeResize,
    NwResize,
    SResize,
    SeResize,
    SwResize,
    WResize,
    EwResize,
    NsResize,
    NeswResize,
    NwseResize,
    ColResize,
    RowResize,
    AllScroll,
    ZoomIn,
    ZoomOut
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor::Default
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize, SelfTokenize)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8
}

impl Default for Color {
    fn default() -> Self {
        Color::black()
    }
}

impl Into<[u8; 4]> for Color {
    fn into(self) -> [u8; 4] {
        [self.red, self.green, self.blue, self.alpha]
    }
}

impl Color {
    pub fn new(rgba: [u8; 4]) -> Self {
        Color {
            red: rgba[0],
            green: rgba[1],
            blue: rgba[2],
            alpha: rgba[3]
        }
    }

    pub fn transparent() -> Self {
        Color::new([0, 0, 0, 0])
    }

    pub fn black() -> Self {
        Color::new([0, 0, 0, 255])
    }

    pub fn white() -> Self {
        Color::new([255, 255, 255, 255])
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize, EnumStrSnakeCase, SelfTokenize)]
pub enum BorderStyle {
    None,
    Solid,
    Double,
    Dotted,
    Dashed,
    Hidden,
    Groove,
    Ridge,
    Inset,
    Outset
}

impl Default for BorderStyle {
    fn default() -> Self {
        BorderStyle::Solid
    }
}

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize, SelfTokenize)]
pub struct BoxShadow {
    pub color: Option<Color>,
    pub horizontal: StyleUnit,
    pub vertical: StyleUnit,
    pub blur: Option<StyleUnit>,
    pub spread: Option<StyleUnit>,
    pub inset: bool
}

impl Into<[u8; 4]> for BoxShadow {
    fn into(self) -> [u8; 4] {
        self.color.unwrap_or_default().into()
    }
}

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize, SelfTokenize)]
pub struct TextShadow {
    pub color: Option<Color>,
    pub horizontal: StyleUnit,
    pub vertical: StyleUnit,
    pub blur: Option<StyleUnit>
}

impl Into<[u8; 4]> for TextShadow {
    fn into(self) -> [u8; 4] {
        self.color.unwrap_or_default().into()
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, SelfTokenize)]
pub enum FontFamily {
    System,
    Values(InlineFontNames)
}

impl Default for FontFamily {
    fn default() -> Self {
        FontFamily::System
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, DefaultQuote)]
pub enum FontName {
    Generic(GenericFontName),
    Specific(SpecificFontName)
}

impl Default for FontName {
    fn default() -> Self {
        FontName::Generic(GenericFontName::default())
    }
}

impl AsRef<str> for FontName {
    fn as_ref(&self) -> &str {
        match self {
            &FontName::Generic(ref name) => name.as_ref(),
            &FontName::Specific(ref name) => name.as_ref()
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize, EnumStrSnakeCase, SelfTokenize)]
// See https://developer.mozilla.org/en-US/docs/Web/CSS/font-family
pub enum GenericFontName {
    Serif,
    SansSerif,
    Monospace,
    Cursive,
    Fantasy,
    SystemUI
}

impl Default for GenericFontName {
    fn default() -> Self {
        GenericFontName::SansSerif
    }
}

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize, DefaultQuote)]
pub struct SpecificFontName(pub Cow<'static, str>);

impl AsRef<str> for SpecificFontName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize, EnumStrSnakeCase, SelfTokenize)]
pub enum FontStyle {
    System,
    Normal,
    Italic,
    Oblique
}

impl Default for FontStyle {
    fn default() -> Self {
        FontStyle::Normal
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize, EnumStrSnakeCase, SelfTokenize)]
pub enum FontCaps {
    System,
    Normal,
    SmallCaps
}

impl Default for FontCaps {
    fn default() -> Self {
        FontCaps::Normal
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize, SelfTokenize)]
pub enum FontWeight {
    System,
    Normal,
    Bold,
    Bolder,
    Lighter,
    Weight(u32)
}

impl Default for FontWeight {
    fn default() -> Self {
        FontWeight::Normal
    }
}

impl TryInto<u32> for FontWeight {
    type Error = ();

    fn try_into(self) -> Result<u32, Self::Error> {
        match self {
            FontWeight::System | FontWeight::Bolder | FontWeight::Lighter => Err(()),
            FontWeight::Normal => Ok(400),
            FontWeight::Bold => Ok(700),
            FontWeight::Weight(value) => Ok(value)
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize, SelfTokenize)]
pub enum FontSize {
    System,
    Smaller,
    Larger,
    Length(StyleUnit)
}

impl Default for FontSize {
    fn default() -> Self {
        FontSize::System
    }
}

impl TryInto<SharedUnit> for FontSize {
    type Error = ();

    fn try_into(self) -> Result<SharedUnit, Self::Error> {
        match self {
            FontSize::System | FontSize::Smaller | FontSize::Larger => Err(()),
            FontSize::Length(v) => Ok(v.into())
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize, EnumStrSnakeCase, SelfTokenize)]
pub enum FontStretch {
    System,
    Normal,
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded
}

impl Default for FontStretch {
    fn default() -> Self {
        FontStretch::Normal
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize, SelfTokenize)]
pub enum Visibility {
    Hidden,
    Visible
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Visible
    }
}

impl Into<bool> for Visibility {
    fn into(self) -> bool {
        match self {
            Visibility::Visible => true,
            Visibility::Hidden => false
        }
    }
}
