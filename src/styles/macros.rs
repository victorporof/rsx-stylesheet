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

macro_rules! match_value_into_type {
    ($value_variant:path[$value_path:path] match $v:ident {
        $( $match_pattern:pat => $value_expr:expr ),*
    }) => {{
        #[allow(unreachable_patterns)]
        match $v {
            $(
                $match_pattern => {
                    $value_variant($value_path($value_expr))
                }
            )*,
            _ => {
                StyleDeclaration::Unknown
            }
        }
    }};
}

#[macro_export]
macro_rules! rgb {
    ($r: expr, $g: expr, $b: expr) => {
        Color::new([$r, $g, $b, 255])
    };
}

#[macro_export]
macro_rules! rgba {
    ($r: expr, $g: expr, $b: expr, $a: expr) => {
        Color::new([$r, $g, $b, ($a * 255.0) as u8])
    };
}

#[macro_export]
macro_rules! value {
    // Layout styles

    (align: auto) => {
        Align::Auto
    };
    (align: flex-start) => {
        Align::FlexStart
    };
    (align: center) => {
        Align::Center
    };
    (align: flex-end) => {
        Align::FlexEnd
    };
    (align: stretch) => {
        Align::Stretch
    };
    (align: baseline) => {
        Align::Baseline
    };
    (align: space-between) => {
        Align::SpaceBetween
    };
    (align: space-around) => {
        Align::SpaceAround
    };
    (display: flex) => {
        Display::Flex
    };
    (display: none) => {
        Display::None
    };
    (flex-direction: row) => {
        FlexDirection::Row
    };
    (flex-direction: row-reverse) => {
        FlexDirection::RowReverse
    };
    (flex-direction: column) => {
        FlexDirection::Column
    };
    (flex-direction: column-reverse) => {
        FlexDirection::ColumnReverse
    };
    (flex-wrap: nowrap) => {
        Wrap::NoWrap
    };
    (flex-wrap: wrap) => {
        Wrap::Wrap
    };
    (flex-wrap: wrap-reverse) => {
        Wrap::WrapReverse
    };
    (justify-content: flex-start) => {
        Justify::FlexStart
    };
    (justify-content: center) => {
        Justify::Center
    };
    (justify-content: flex-end) => {
        Justify::FlexEnd
    };
    (justify-content: space-between) => {
        Justify::SpaceBetween
    };
    (justify-content: space-around) => {
        Justify::SpaceAround
    };
    (justify-content: space-evenly) => {
        Justify::SpaceEvenly
    };
    (overflow: visible) => {
        Overflow::Visible
    };
    (overflow: hidden) => {
        Overflow::Hidden
    };
    (overflow: scroll) => {
        Overflow::Scroll
    };
    (position: relative) => {
        PositionType::Relative
    };
    (position: absolute) => {
        PositionType::Absolute
    };

    // Theme styles
    // TODO: Handle more longhands.

    (font-size: $( $tt:tt )*) => {
        FontSize::Length(value!(unit: $( $tt )*))
    };

    // Units

    (rgb($( $tt:tt )*)) => {
        rgb!($( $tt )*)
    };
    (rgba($( $tt:tt )*)) => {
        rgba!($( $tt )*)
    };
    (i32: $val:tt) => {
        value!($val as i32)
    };
    (f32: $val:tt) => {
        value!($val as f32)
    };
    (unit: auto) => {
        StyleUnit::Auto
    };
    (unit: undefined) => {
        StyleUnit::UndefinedValue
    };
    (unit: $val:tt px) => {{
        use yoga::prelude::*;
        $val.point()
    }};
    (unit: $val:tt %) => {{
        use yoga::prelude::*;
        $val.percent()
    }};

    // Catch all

    ($val:expr) => {
        $val.into()
    };
}

#[macro_export]
macro_rules! declaration {
    // Layout styles

    (align-content: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::AlignContent(value!(align: $( $value )*)))
    };
    (align-items: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::AlignItems(value!(align: $( $value )*)))
    };
    (align-self: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::AlignSelf(value!(align: $( $value )*)))
    };
    (aspect-ratio: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::AspectRatio(value!(f32: $( $value )*)))
    };
    (border-bottom-width: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::BorderBottom(value!(f32: $( $value )*)))
    };
    (border-end-width: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::BorderEnd(value!(f32: $( $value )*)))
    };
    (border-left-width: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::BorderLeft(value!(f32: $( $value )*)))
    };
    (border-right-width: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::BorderRight(value!(f32: $( $value )*)))
    };
    (border-start-width: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::BorderStart(value!(f32: $( $value )*)))
    };
    (border-top-width: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::BorderTop(value!(f32: $( $value )*)))
    };
    (border-width: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::Border(value!(f32: $( $value )*)))
    };
    (bottom: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::Bottom(value!(unit: $( $value )*)))
    };
    (display: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::Display(value!(display: $( $value )*)))
    };
    (end: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::End(value!(unit: $( $value )*)))
    };
    (flex: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::Flex(value!(f32: $( $value )*)))
    };
    (flex-basis: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::FlexBasis(value!(unit: $( $value )*)))
    };
    (flex-direction: $value:tt) => {
        StyleDeclaration::Layout(FlexStyle::FlexDirection(value!(flex-direction: $value)))
    };
    (flex-grow: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::FlexGrow(value!(f32: $( $value )*)))
    };
    (flex-shrink: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::FlexShrink(value!(f32: $( $value )*)))
    };
    (flex-wrap: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::FlexWrap(value!(flex-wrap: $( $value )*)))
    };
    (height: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::Height(value!(unit: $( $value )*)))
    };
    (justify-content: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::JustifyContent(value!(justify-content: $( $value )*)))
    };
    (left: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::Left(value!(unit: $( $value )*)))
    };
    (margin: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::Margin(value!(unit: $( $value )*)))
    };
    (margin-bottom: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::MarginBottom(value!(unit: $( $value )*)))
    };
    (margin-end: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::MarginEnd(value!(unit: $( $value )*)))
    };
    (margin-horizontal: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::MarginHorizontal(value!(unit: $( $value )*)))
    };
    (margin-left: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::MarginLeft(value!(unit: $( $value )*)))
    };
    (margin-right: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::MarginRight(value!(unit: $( $value )*)))
    };
    (margin-start: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::MarginStart(value!(unit: $( $value )*)))
    };
    (margin-top: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::MarginTop(value!(unit: $( $value )*)))
    };
    (margin-vertical: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::MarginVertical(value!(unit: $( $value )*)))
    };
    (max-height: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::MaxHeight(value!(unit: $( $value )*)))
    };
    (max-width: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::MaxWidth(value!(unit: $( $value )*)))
    };
    (min-height: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::MinHeight(value!(unit: $( $value )*)))
    };
    (min-width: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::MinWidth(value!(unit: $( $value )*)))
    };
    (overflow: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::Overflow(value!(overflow: $( $value )*)))
    };
    (padding: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::Padding(value!(unit: $( $value )*)))
    };
    (padding-bottom: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::PaddingBottom(value!(unit: $( $value )*)))
    };
    (padding-end: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::PaddingEnd(value!(unit: $( $value )*)))
    };
    (padding-horizontal: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::PaddingHorizontal(value!(unit: $( $value )*)))
    };
    (padding-left: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::PaddingLeft(value!(unit: $( $value )*)))
    };
    (padding-right: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::PaddingRight(value!(unit: $( $value )*)))
    };
    (padding-start: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::PaddingStart(value!(unit: $( $value )*)))
    };
    (padding-top: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::PaddingTop(value!(unit: $( $value )*)))
    };
    (padding-vertical: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::PaddingVertical(value!(unit: $( $value )*)))
    };
    (position: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::Position(value!(position: $( $value )*)))
    };
    (right: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::Right(value!(unit: $( $value )*)))
    };
    (start: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::Start(value!(unit: $( $value )*)))
    };
    (top: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::Top(value!(unit: $( $value )*)))
    };
    (width: $( $value:tt )*) => {
        StyleDeclaration::Layout(FlexStyle::Width(value!(unit: $( $value )*)))
    };

    // Theme styles

    (cursor: $( $value:tt )*) => {
        StyleDeclaration::Theme(ThemeStyle::Cursor(value!($( $value )*)))
    };
    (color: $( $value:tt )*) => {
        StyleDeclaration::Theme(ThemeStyle::Color(value!($( $value )*)))
    };
    (background-color: $( $value:tt )*) => {
        StyleDeclaration::Theme(ThemeStyle::BackgroundColor(value!($( $value )*)))
    };
    (opacity: $( $value:tt )*) => {
        StyleDeclaration::Theme(ThemeStyle::Opacity(value!($( $value )*)))
    };
    (border-top-color: $( $value:tt )*) => {
        StyleDeclaration::Theme(ThemeStyle::BorderTopColor(value!($( $value )*)))
    };
    (border-left-color: $( $value:tt )*) => {
        StyleDeclaration::Theme(ThemeStyle::BorderLeftColor(value!($( $value )*)))
    };
    (border-bottom-color: $( $value:tt )*) => {
        StyleDeclaration::Theme(ThemeStyle::BorderBottomColor(value!($( $value )*)))
    };
    (border-right-color: $( $value:tt )*) => {
        StyleDeclaration::Theme(ThemeStyle::BorderRightColor(value!($( $value )*)))
    };
    (border-top-style: $( $value:tt )*) => {
        StyleDeclaration::Theme(ThemeStyle::BorderTopStyle(value!($( $value )*)))
    };
    (border-bottom-style: $( $value:tt )*) => {
        StyleDeclaration::Theme(ThemeStyle::BorderBottomStyle(value!($( $value )*)))
    };
    (border-left-style: $( $value:tt )*) => {
        StyleDeclaration::Theme(ThemeStyle::BorderLeftStyle(value!($( $value )*)))
    };
    (border-right-style: $( $value:tt )*) => {
        StyleDeclaration::Theme(ThemeStyle::BorderRightStyle(value!($( $value )*)))
    };
    (box-shadow: $( $value:tt )*) => {
        StyleDeclaration::Theme(ThemeStyle::BoxShadow(value!($( $value )*)))
    };
    (text-shadow: $( $value:tt )*) => {
        StyleDeclaration::Theme(ThemeStyle::TextShadow(value!($( $value )*)))
    };
    (font-family: $( $value:tt )*) => {
        StyleDeclaration::Theme(ThemeStyle::FontFamily(value!($( $value )*)))
    };
    (font-style: $( $value:tt )*) => {
        StyleDeclaration::Theme(ThemeStyle::FontStyle(value!($( $value )*)))
    };
    (font-caps: $( $value:tt )*) => {
        StyleDeclaration::Theme(ThemeStyle::FontCaps(value!($( $value )*)))
    };
    (font-weight: $( $value:tt )*) => {
        StyleDeclaration::Theme(ThemeStyle::FontWeight(value!($( $value )*)))
    };
    (font-size: $( $value:tt )*) => {
        StyleDeclaration::Theme(ThemeStyle::FontSize(value!(font-size: $( $value )*)))
    };
    (font-stretch: $( $value:tt )*) => {
        StyleDeclaration::Theme(ThemeStyle::FontStretch(value!($( $value )*)))
    };
    (visibility: $( $value:tt )*) => {
        StyleDeclaration::Theme(ThemeStyle::Visibility(value!($( $value )*)))
    };
}

#[macro_export]
macro_rules! style {
    ( $( $name:tt$(-$suffix:tt)*: { $( $value:tt )* } );* $(;)*) => {
        StyleDeclarations(SmallVec::from_vec(vec![
            $(
                declaration!(
                    $name$(-$suffix)* : $($value)*
                ),
            )*
        ]))
    };
    ( #include("defaults.css") $(;)*) => {
        StyleDeclarations(SmallVec::from_buf([
            declaration!(flex-direction: row),
            declaration!(flex-wrap: nowrap),
            declaration!(justify-content: flex-start),
            declaration!(align-items: stretch),
            declaration!(align-content: stretch),
            declaration!(flex-grow: 0),
            declaration!(flex-shrink: 1),
            declaration!(flex-basis: auto)
        ]))
    };
    ( #include("defaults.css"); $( $name:tt$(-$suffix:tt)*: { $( $value:tt )* } );* $(;)*) => {
        StyleDeclarations(SmallVec::from_vec(vec![
            declaration!(flex-direction: row),
            declaration!(flex-wrap: nowrap),
            declaration!(justify-content: flex-start),
            declaration!(align-items: stretch),
            declaration!(align-content: stretch),
            declaration!(flex-grow: 0),
            declaration!(flex-shrink: 1),
            declaration!(flex-basis: auto),
            $(
                declaration!(
                    $name$(-$suffix)* : $($value)*
                ),
            )*
        ]))
    };
}
