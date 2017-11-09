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

use std::convert::TryInto;

use rsx_shared::traits::{TComputedStyles, TInheritedStyles};
use rsx_shared::types::KnownElementName;

use styles::types::{
    BorderStyle,
    BoxShadow,
    Color,
    Cursor,
    FlexStyle,
    FontCaps,
    FontFamily,
    FontName,
    FontSize,
    FontStretch,
    FontStyle,
    FontWeight,
    InlineBoxShadows,
    InlineFontNames,
    InlineTextShadows,
    StyleDeclaration,
    StyleDeclarations,
    TextShadow,
    ThemeStyle,
    Visibility
};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct ComputedStyles {
    // Non-inherited layout styles
    pub border_bottom_width: u32,
    pub border_left_width: u32,
    pub border_right_width: u32,
    pub border_top_width: u32,

    // Non-inherited theme styles
    pub background_color: Color,
    pub opacity: u32,
    pub border_bottom_color: Color,
    pub border_bottom_style: BorderStyle,
    pub border_left_color: Color,
    pub border_left_style: BorderStyle,
    pub border_right_color: Color,
    pub border_right_style: BorderStyle,
    pub border_top_color: Color,
    pub border_top_style: BorderStyle,
    pub box_shadows: InlineBoxShadows,

    // Inherited theme styles
    // https://www.w3.org/TR/CSS21/propidx.html
    // https://developer.mozilla.org/en-US/docs/Web/CSS/Reference
    pub cursor: Cursor,
    pub color: Color,
    pub text_shadows: InlineTextShadows,
    pub font_names: InlineFontNames,
    pub font_style: FontStyle,
    pub font_caps: FontCaps,
    pub font_weight: FontWeight,
    pub font_size: FontSize,
    pub font_stretch: FontStretch,
    pub visibility: Visibility
}

impl TComputedStyles for ComputedStyles {
    type BackgroundColor = Color;
    type Opacity = u32;
    type BorderSize = u32;
    type BorderColor = Color;
    type BorderStyle = BorderStyle;
    type BoxShadow = BoxShadow;

    fn make_initial_computed_styles<T>(_: T) -> Self
    where
        T: TryInto<KnownElementName>
    {
        ComputedStyles {
            background_color: Color::transparent(),
            border_bottom_color: Color::transparent(),
            border_left_color: Color::transparent(),
            border_right_color: Color::transparent(),
            border_top_color: Color::transparent(),
            ..Default::default()
        }
    }

    fn reset_custom_styles<T>(&mut self, tag: T)
    where
        T: TryInto<KnownElementName>
    {
        *self = ComputedStyles::make_initial_computed_styles(tag)
    }

    fn apply_styles(&mut self, styles: &Self::Styles) {
        use self::FlexStyle::*;
        use self::FontFamily::*;
        use self::StyleDeclaration::*;
        use self::ThemeStyle::*;

        styles.iter().for_each(|style| match style {
            // Non-inherited layout styles
            &Layout(BorderBottom(border_bottom_width)) => self.border_bottom_width = border_bottom_width.into_inner() as u32,
            &Layout(BorderLeft(border_left_width)) => self.border_left_width = border_left_width.into_inner() as u32,
            &Layout(BorderRight(border_right_width)) => self.border_right_width = border_right_width.into_inner() as u32,
            &Layout(BorderTop(border_top_width)) => self.border_top_width = border_top_width.into_inner() as u32,

            // Non-inherited theme styles
            &Theme(BackgroundColor(background_color)) => self.background_color = background_color,
            &Theme(Opacity(opacity)) => self.opacity = opacity,
            &Theme(BorderBottomColor(border_bottom_color)) => self.border_bottom_color = border_bottom_color,
            &Theme(BorderBottomStyle(border_bottom_style)) => self.border_bottom_style = border_bottom_style,
            &Theme(BorderLeftColor(border_left_color)) => self.border_left_color = border_left_color,
            &Theme(BorderLeftStyle(border_left_style)) => self.border_left_style = border_left_style,
            &Theme(BorderRightColor(border_right_color)) => self.border_right_color = border_right_color,
            &Theme(BorderRightStyle(border_right_style)) => self.border_right_style = border_right_style,
            &Theme(BorderTopColor(border_top_color)) => self.border_top_color = border_top_color,
            &Theme(BorderTopStyle(border_top_style)) => self.border_top_style = border_top_style,
            &Theme(BoxShadow(ref box_shadows)) => self.box_shadows = box_shadows.clone(),

            // Inherited theme styles
            &Theme(Cursor(cursor)) => self.cursor = cursor,
            &Theme(Color(color)) => self.color = color,
            &Theme(TextShadow(ref text_shadows)) => self.text_shadows = text_shadows.clone(),
            &Theme(FontFamily(Values(ref font_names))) => self.font_names = font_names.clone(),
            &Theme(FontStyle(font_style)) => self.font_style = font_style,
            &Theme(FontCaps(font_caps)) => self.font_caps = font_caps,
            &Theme(FontWeight(font_weight)) => self.font_weight = font_weight,
            &Theme(FontSize(font_size)) => self.font_size = font_size,
            &Theme(FontStretch(font_stretch)) => self.font_stretch = font_stretch,
            &Theme(Visibility(visibility)) => self.visibility = visibility,
            _ => {}
        })
    }

    fn background_color(&self) -> Self::BackgroundColor {
        self.background_color
    }

    fn opacity(&self) -> Self::Opacity {
        self.opacity
    }

    fn border_bottom_width(&self) -> Self::BorderSize {
        self.border_bottom_width
    }

    fn border_bottom_color(&self) -> Self::BorderColor {
        self.border_bottom_color
    }

    fn border_bottom_style(&self) -> Self::BorderStyle {
        self.border_bottom_style
    }

    fn border_left_width(&self) -> Self::BorderSize {
        self.border_left_width
    }

    fn border_left_color(&self) -> Self::BorderColor {
        self.border_left_color
    }

    fn border_left_style(&self) -> Self::BorderStyle {
        self.border_left_style
    }

    fn border_right_width(&self) -> Self::BorderSize {
        self.border_right_width
    }

    fn border_right_color(&self) -> Self::BorderColor {
        self.border_right_color
    }

    fn border_right_style(&self) -> Self::BorderStyle {
        self.border_right_style
    }

    fn border_top_width(&self) -> Self::BorderSize {
        self.border_top_width
    }

    fn border_top_color(&self) -> Self::BorderColor {
        self.border_top_color
    }

    fn border_top_style(&self) -> Self::BorderStyle {
        self.border_top_style
    }

    fn box_shadows_copy(&self) -> Vec<Self::BoxShadow> {
        self.box_shadows.to_vec()
    }
}

impl TInheritedStyles for ComputedStyles {
    type Styles = StyleDeclarations;
    type Cursor = Cursor;
    type Color = Color;
    type TextShadow = TextShadow;
    type FontName = FontName;
    type FontStyle = FontStyle;
    type FontCaps = FontCaps;
    type FontWeight = FontWeight;
    type FontSize = FontSize;
    type FontStretch = FontStretch;
    type Visibility = Visibility;

    fn inherit_styles(&mut self, other: &Self) {
        self.cursor = other.cursor;
        self.color = other.color;
        self.text_shadows = other.text_shadows.clone();
        self.font_names = other.font_names.clone();
        self.font_style = other.font_style;
        self.font_caps = other.font_caps;
        self.font_weight = other.font_weight;
        self.font_size = other.font_size;
        self.font_stretch = other.font_stretch;
        self.visibility = other.visibility;
    }

    fn cursor(&self) -> Self::Cursor {
        self.cursor
    }

    fn color(&self) -> Self::Color {
        self.color
    }

    fn text_shadows_copy(&self) -> Vec<Self::TextShadow> {
        self.text_shadows.to_vec()
    }

    fn font_names_copy(&self) -> Vec<Self::FontName> {
        self.font_names.to_vec()
    }

    fn font_style(&self) -> Self::FontStyle {
        self.font_style
    }

    fn font_caps(&self) -> Self::FontCaps {
        self.font_caps
    }

    fn font_weight(&self) -> Self::FontWeight {
        self.font_weight
    }

    fn font_size(&self) -> Self::FontSize {
        self.font_size
    }

    fn font_stretch(&self) -> Self::FontStretch {
        self.font_stretch
    }

    fn visibility(&self) -> Self::Visibility {
        self.visibility
    }

    fn find_font<F, O>(&self, predicate: F) -> Option<O>
    where
        F: FnMut(&Self::FontName) -> Option<O>
    {
        self.font_names.iter().filter_map(predicate).next()
    }
}
