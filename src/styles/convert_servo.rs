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

#![cfg_attr(feature = "cargo-clippy", allow(cyclomatic_complexity))]

use std::iter::FromIterator;

use servo_css_parser::cssparser::ToCss as cssparser_ToCss;
use servo_css_parser::selectors::parser::{Selector, SelectorList};
use servo_css_parser::style::properties::{longhands, PropertyDeclaration};
use servo_css_parser::style::properties::declaration_block::PropertyDeclarationBlock;
use servo_css_parser::style::servo::selector_parser::SelectorImpl;
use servo_css_parser::style::stylesheets::CssRule;
use servo_css_parser::style::values::{computed, generics, specified, RGBA};
use servo_css_parser::style_traits;
use servo_css_parser::style_traits::values::ToCss as style_traits_ToCss;
use servo_css_parser::types::ServoStylesheet;

use types::{
    self,
    FlexStyle,
    SmallVec,
    StyleDeclaration,
    StyleDeclarations,
    StyleRule,
    StyleSelector,
    StyleSelectors,
    Stylesheet,
    ThemeStyle
};

impl From<ServoStylesheet> for Stylesheet {
    fn from(stylesheet: ServoStylesheet) -> Self {
        let guard = stylesheet.shared_lock.read();
        let rules = &stylesheet.contents.rules.read_with(&guard).0;
        Stylesheet::from(SmallVec::from_vec(
            rules
                .iter()
                .filter_map(|rule| {
                    match rule {
                        &CssRule::Style(ref style) => {
                            let rule = style.read_with(&guard);
                            let selectors = &rule.selectors;
                            let block = rule.block.read_with(&guard);
                            Some((selectors, block).into())
                        }
                        &CssRule::Namespace(..)
                        | &CssRule::Import(..)
                        | &CssRule::Media(..)
                        | &CssRule::FontFace(..)
                        | &CssRule::FontFeatureValues(..)
                        | &CssRule::CounterStyle(..)
                        | &CssRule::Viewport(..)
                        | &CssRule::Keyframes(..)
                        | &CssRule::Supports(..)
                        | &CssRule::Page(..)
                        | &CssRule::Document(..) => {
                            // Not supported yet.
                            None
                        }
                    }
                })
                .collect()
        ))
    }
}

impl<'a> From<(&'a SelectorList<SelectorImpl>, &'a PropertyDeclarationBlock)> for StyleRule {
    fn from((selectors, block): (&SelectorList<SelectorImpl>, &PropertyDeclarationBlock)) -> Self {
        StyleRule {
            selectors: StyleSelectors::from_iter(selectors.0.iter().map(|v| v.into())),
            declarations: StyleDeclarations::from_iter(block.declarations().iter().map(|v| v.into()))
        }
    }
}

impl<'a> From<&'a Selector<SelectorImpl>> for StyleSelector {
    fn from(selector: &Selector<SelectorImpl>) -> Self {
        StyleSelector::from(selector.to_css_string())
    }
}

impl<'a> From<&'a PropertyDeclaration> for StyleDeclaration {
    fn from(declaration: &PropertyDeclaration) -> Self {
        match declaration {
            // Theme rules
            &PropertyDeclaration::BackgroundColor(ref value) => {
                use self::specified::Color::Numeric;
                match_value_into_type!(StyleDeclaration::Theme[ThemeStyle::BackgroundColor] match value {
                    &Numeric { parsed: RGBA { red, green, blue, alpha }, .. } => types::Color::new([red, green, blue, alpha])
                })
            }
            &PropertyDeclaration::BorderTopColor(ref value) => {
                use self::specified::Color::Numeric;
                match_value_into_type!(StyleDeclaration::Theme[ThemeStyle::BorderTopColor] match value {
                    &Numeric { parsed: RGBA { red, green, blue, alpha }, .. } => types::Color::new([red, green, blue, alpha])
                })
            }
            &PropertyDeclaration::BorderLeftColor(ref value) => {
                use self::specified::Color::Numeric;
                match_value_into_type!(StyleDeclaration::Theme[ThemeStyle::BorderLeftColor] match value {
                    &Numeric { parsed: RGBA { red, green, blue, alpha }, .. } => types::Color::new([red, green, blue, alpha])
                })
            }
            &PropertyDeclaration::BorderBottomColor(ref value) => {
                use self::specified::Color::Numeric;
                match_value_into_type!(StyleDeclaration::Theme[ThemeStyle::BorderBottomColor] match value {
                    &Numeric { parsed: RGBA { red, green, blue, alpha }, .. } => types::Color::new([red, green, blue, alpha])
                })
            }
            &PropertyDeclaration::BorderRightColor(ref value) => {
                use self::specified::Color::Numeric;
                match_value_into_type!(StyleDeclaration::Theme[ThemeStyle::BorderRightColor] match value {
                    &Numeric { parsed: RGBA { red, green, blue, alpha }, .. } => types::Color::new([red, green, blue, alpha])
                })
            }
            &PropertyDeclaration::BorderTopStyle(ref value) => {
                use self::specified::BorderStyle::{Dashed, Dotted, Double, Groove, Hidden, Inset, None, Outset, Ridge, Solid};
                match_value_into_type!(StyleDeclaration::Theme[ThemeStyle::BorderTopStyle] match value {
                    &None => types::BorderStyle::None,
                    &Solid => types::BorderStyle::Solid,
                    &Double => types::BorderStyle::Double,
                    &Dotted => types::BorderStyle::Dotted,
                    &Dashed => types::BorderStyle::Dashed,
                    &Hidden => types::BorderStyle::Hidden,
                    &Groove => types::BorderStyle::Groove,
                    &Ridge => types::BorderStyle::Ridge,
                    &Inset => types::BorderStyle::Inset,
                    &Outset => types::BorderStyle::Outset
                })
            }
            &PropertyDeclaration::BorderLeftStyle(ref value) => {
                use self::specified::BorderStyle::{Dashed, Dotted, Double, Groove, Hidden, Inset, None, Outset, Ridge, Solid};
                match_value_into_type!(StyleDeclaration::Theme[ThemeStyle::BorderLeftStyle] match value {
                    &None => types::BorderStyle::None,
                    &Solid => types::BorderStyle::Solid,
                    &Double => types::BorderStyle::Double,
                    &Dotted => types::BorderStyle::Dotted,
                    &Dashed => types::BorderStyle::Dashed,
                    &Hidden => types::BorderStyle::Hidden,
                    &Groove => types::BorderStyle::Groove,
                    &Ridge => types::BorderStyle::Ridge,
                    &Inset => types::BorderStyle::Inset,
                    &Outset => types::BorderStyle::Outset
                })
            }
            &PropertyDeclaration::BorderBottomStyle(ref value) => {
                use self::specified::BorderStyle::{Dashed, Dotted, Double, Groove, Hidden, Inset, None, Outset, Ridge, Solid};
                match_value_into_type!(StyleDeclaration::Theme[ThemeStyle::BorderBottomStyle] match value {
                    &None => types::BorderStyle::None,
                    &Solid => types::BorderStyle::Solid,
                    &Double => types::BorderStyle::Double,
                    &Dotted => types::BorderStyle::Dotted,
                    &Dashed => types::BorderStyle::Dashed,
                    &Hidden => types::BorderStyle::Hidden,
                    &Groove => types::BorderStyle::Groove,
                    &Ridge => types::BorderStyle::Ridge,
                    &Inset => types::BorderStyle::Inset,
                    &Outset => types::BorderStyle::Outset
                })
            }
            &PropertyDeclaration::BorderRightStyle(ref value) => {
                use self::specified::BorderStyle::{Dashed, Dotted, Double, Groove, Hidden, Inset, None, Outset, Ridge, Solid};
                match_value_into_type!(StyleDeclaration::Theme[ThemeStyle::BorderRightStyle] match value {
                    &None => types::BorderStyle::None,
                    &Solid => types::BorderStyle::Solid,
                    &Double => types::BorderStyle::Double,
                    &Dotted => types::BorderStyle::Dotted,
                    &Dashed => types::BorderStyle::Dashed,
                    &Hidden => types::BorderStyle::Hidden,
                    &Groove => types::BorderStyle::Groove,
                    &Ridge => types::BorderStyle::Ridge,
                    &Inset => types::BorderStyle::Inset,
                    &Outset => types::BorderStyle::Outset
                })
            }
            &PropertyDeclaration::Cursor(ref value) => {
                use self::longhands::cursor::computed_value::Keyword::{Auto, Cursor};
                use self::style_traits::cursor::Cursor::*;
                match_value_into_type!(StyleDeclaration::Theme[ThemeStyle::Cursor] match value {
                    &Auto => types::Cursor::Auto,
                    &Cursor(None) => types::Cursor::None,
                    &Cursor(Default) => types::Cursor::Default,
                    &Cursor(Pointer) => types::Cursor::Pointer,
                    &Cursor(ContextMenu) => types::Cursor::ContextMenu,
                    &Cursor(Help) => types::Cursor::Help,
                    &Cursor(Progress) => types::Cursor::Progress,
                    &Cursor(Wait) => types::Cursor::Wait,
                    &Cursor(Cell) => types::Cursor::Cell,
                    &Cursor(Crosshair) => types::Cursor::Crosshair,
                    &Cursor(Text) => types::Cursor::Text,
                    &Cursor(VerticalText) => types::Cursor::VerticalText,
                    &Cursor(Alias) => types::Cursor::Alias,
                    &Cursor(Copy) => types::Cursor::Copy,
                    &Cursor(Move) => types::Cursor::Move,
                    &Cursor(NoDrop) => types::Cursor::NoDrop,
                    &Cursor(NotAllowed) => types::Cursor::NotAllowed,
                    &Cursor(Grab) => types::Cursor::Grab,
                    &Cursor(Grabbing) => types::Cursor::Grabbing,
                    &Cursor(EResize) => types::Cursor::EResize,
                    &Cursor(NResize) => types::Cursor::NResize,
                    &Cursor(NeResize) => types::Cursor::NeResize,
                    &Cursor(NwResize) => types::Cursor::NwResize,
                    &Cursor(SResize) => types::Cursor::SResize,
                    &Cursor(SeResize) => types::Cursor::SeResize,
                    &Cursor(SwResize) => types::Cursor::SwResize,
                    &Cursor(WResize) => types::Cursor::WResize,
                    &Cursor(EwResize) => types::Cursor::EwResize,
                    &Cursor(NsResize) => types::Cursor::NsResize,
                    &Cursor(NeswResize) => types::Cursor::NeswResize,
                    &Cursor(NwseResize) => types::Cursor::NwseResize,
                    &Cursor(ColResize) => types::Cursor::ColResize,
                    &Cursor(RowResize) => types::Cursor::RowResize,
                    &Cursor(AllScroll) => types::Cursor::AllScroll,
                    &Cursor(ZoomIn) => types::Cursor::ZoomIn,
                    &Cursor(ZoomOut) => types::Cursor::ZoomOut
                })
            }
            &PropertyDeclaration::Color(ref value) => {
                use self::specified::Color::Numeric;
                use self::specified::ColorPropertyValue;
                match_value_into_type!(StyleDeclaration::Theme[ThemeStyle::Color] match value {
                    &ColorPropertyValue(Numeric { parsed: RGBA { red, green, blue, alpha }, .. }) => types::Color::new([red, green, blue, alpha])
                })
            }
            &PropertyDeclaration::Opacity(ref value) => {
                use self::specified::Opacity;
                match_value_into_type!(StyleDeclaration::Theme[ThemeStyle::Opacity] match value {
                    value @ &Opacity { .. } => (value.to_css_string().parse::<f32>().unwrap() * 100.0) as u32
                })
            }
            &PropertyDeclaration::BoxShadow(ref value) => {
                use self::generics::effects::{BoxShadow, SimpleShadow};
                use self::longhands::box_shadow::SpecifiedValue;
                use self::specified::color::Color::Numeric;
                use self::specified::color::RGBAColor;
                match_value_into_type!(StyleDeclaration::Theme[ThemeStyle::BoxShadow] match value {
                    &SpecifiedValue(ref vec) => vec.iter().map(|&BoxShadow { base: SimpleShadow { ref color, ref horizontal, ref vertical, ref blur }, ref spread, inset }| {
                        types::BoxShadow {
                            color: color.as_ref().and_then(|&RGBAColor(ref c)| match c {
                                &Numeric { parsed: RGBA { red, green, blue, alpha }, .. } => Some(types::Color::new([red, green, blue, alpha])),
                                _ => None
                            }),
                            horizontal: match horizontal {
                                &specified::Length::NoCalc(specified::NoCalcLength::Absolute(specified::AbsoluteLength::Px(px))) => types::StyleUnit::Point(px.into()),
                                _ => types::StyleUnit::UndefinedValue,
                            },
                            vertical: match vertical {
                                &specified::Length::NoCalc(specified::NoCalcLength::Absolute(specified::AbsoluteLength::Px(px))) => types::StyleUnit::Point(px.into()),
                                _ => types::StyleUnit::UndefinedValue,
                            },
                            blur: blur.as_ref().and_then(|&generics::NonNegative(ref length)| match length {
                                &specified::Length::NoCalc(specified::NoCalcLength::Absolute(specified::AbsoluteLength::Px(px))) => Some(types::StyleUnit::Point(px.into())),
                                _ => None,
                            }),
                            spread: spread.as_ref().and_then(|spread| match spread {
                                &specified::Length::NoCalc(specified::NoCalcLength::Absolute(specified::AbsoluteLength::Px(px))) => Some(types::StyleUnit::Point(px.into())),
                                _ => None,
                            }),
                            inset
                        }
                    }).collect()
                })
            }
            &PropertyDeclaration::TextShadow(ref value) => {
                use self::generics::effects::SimpleShadow;
                use self::longhands::text_shadow::SpecifiedValue;
                use self::specified::color::Color::Numeric;
                use self::specified::color::RGBAColor;
                match_value_into_type!(StyleDeclaration::Theme[ThemeStyle::TextShadow] match value {
                    &SpecifiedValue(ref vec) => vec.iter().map(|&SimpleShadow { ref color, ref horizontal, ref vertical, ref blur }| {
                        types::TextShadow {
                            color: color.as_ref().and_then(|&RGBAColor(ref c)| match c {
                                &Numeric { parsed: RGBA { red, green, blue, alpha }, .. } => Some(types::Color::new([red, green, blue, alpha])),
                                _ => None
                            }),
                            horizontal: match horizontal {
                                &specified::Length::NoCalc(specified::NoCalcLength::Absolute(specified::AbsoluteLength::Px(px))) => types::StyleUnit::Point(px.into()),
                                _ => types::StyleUnit::UndefinedValue,
                            },
                            vertical: match vertical {
                                &specified::Length::NoCalc(specified::NoCalcLength::Absolute(specified::AbsoluteLength::Px(px))) => types::StyleUnit::Point(px.into()),
                                _ => types::StyleUnit::UndefinedValue,
                            },
                            blur: blur.as_ref().and_then(|&generics::NonNegative(ref length)| match length {
                                &specified::Length::NoCalc(specified::NoCalcLength::Absolute(specified::AbsoluteLength::Px(px))) => Some(types::StyleUnit::Point(px.into())),
                                _ => None,
                            })
                        }
                    }).collect()
                })
            }
            &PropertyDeclaration::FontFamily(ref value) => {
                use self::longhands::font_family::SpecifiedValue::*;
                match_value_into_type!(StyleDeclaration::Theme[ThemeStyle::FontFamily] match value {
                    &System(..) => types::FontFamily::System,
                    &Values(ref vec) => types::FontFamily::Values(vec.iter().map(|v| {
                        match v.to_css_string() {
                            ref s if s == "serif" => types::FontName::Generic(types::GenericFontName::Serif),
                            ref s if s == "sans-serif" => types::FontName::Generic(types::GenericFontName::SansSerif),
                            ref s if s == "monospace" => types::FontName::Generic(types::GenericFontName::Monospace),
                            ref s if s == "cursive" => types::FontName::Generic(types::GenericFontName::Cursive),
                            ref s if s == "fantasy" => types::FontName::Generic(types::GenericFontName::Fantasy),
                            ref s if s == "system-ui" => types::FontName::Generic(types::GenericFontName::SystemUI),
                            s => types::FontName::Specific(types::SpecificFontName::from(s))
                        }
                    }).collect())
                })
            }
            &PropertyDeclaration::FontStyle(ref value) => {
                use self::longhands::font_style::SpecifiedValue::{Keyword, System};
                use self::longhands::font_style::computed_value::T::{Italic, Normal, Oblique};
                match_value_into_type!(StyleDeclaration::Theme[ThemeStyle::FontStyle] match value {
                    &System(..) => types::FontStyle::System,
                    &Keyword(Normal) => types::FontStyle::Normal,
                    &Keyword(Italic) => types::FontStyle::Italic,
                    &Keyword(Oblique) => types::FontStyle::Oblique
                })
            }
            &PropertyDeclaration::FontVariantCaps(ref value) => {
                use self::longhands::font_variant_caps::SpecifiedValue::{Keyword, System};
                use self::longhands::font_variant_caps::computed_value::T::{Normal, SmallCaps};
                match_value_into_type!(StyleDeclaration::Theme[ThemeStyle::FontCaps] match value {
                    &System(..) => types::FontCaps::System,
                    &Keyword(Normal) => types::FontCaps::Normal,
                    &Keyword(SmallCaps) => types::FontCaps::SmallCaps
                })
            }
            &PropertyDeclaration::FontWeight(ref value) => {
                use self::computed::font::FontWeight;
                use self::specified::FontWeight::{Bold, Bolder, Lighter, Normal, System, Weight};
                match_value_into_type!(StyleDeclaration::Theme[ThemeStyle::FontWeight] match value {
                    &System(..) => types::FontWeight::System,
                    &Normal => types::FontWeight::Normal,
                    &Bold => types::FontWeight::Bold,
                    &Bolder => types::FontWeight::Bolder,
                    &Lighter => types::FontWeight::Lighter,
                    &Weight(FontWeight(v)) => types::FontWeight::Weight(u32::from(v))
                })
            }
            &PropertyDeclaration::FontSize(ref value) => {
                use self::computed::Percentage;
                use self::specified::{AbsoluteLength, LengthOrPercentage, NoCalcLength};
                use self::specified::FontSize::{Larger, Length, Smaller, System};
                match_value_into_type!(StyleDeclaration::Theme[ThemeStyle::FontSize] match value {
                    &System(..) => types::FontSize::System,
                    &Smaller => types::FontSize::Smaller,
                    &Larger => types::FontSize::Larger,
                    &Length(LengthOrPercentage::Length(NoCalcLength::Absolute(AbsoluteLength::Px(px)))) => types::FontSize::Length(types::StyleUnit::Point(px.into())),
                    &Length(LengthOrPercentage::Percentage(Percentage(pc))) => types::FontSize::Length(types::StyleUnit::Percent((100.0 * pc).into()))
                })
            }
            &PropertyDeclaration::FontStretch(ref value) => {
                use self::longhands::font_stretch::SpecifiedValue::{Keyword, System};
                use self::longhands::font_stretch::computed_value::T::{
                    Condensed,
                    Expanded,
                    ExtraCondensed,
                    ExtraExpanded,
                    Normal,
                    SemiCondensed,
                    SemiExpanded,
                    UltraCondensed,
                    UltraExpanded
                };
                match_value_into_type!(StyleDeclaration::Theme[ThemeStyle::FontStretch] match value {
                    &System(..) => types::FontStretch::System,
                    &Keyword(Normal) => types::FontStretch::Normal,
                    &Keyword(UltraCondensed) => types::FontStretch::UltraCondensed,
                    &Keyword(ExtraCondensed) => types::FontStretch::ExtraCondensed,
                    &Keyword(Condensed) => types::FontStretch::Condensed,
                    &Keyword(SemiCondensed) => types::FontStretch::SemiCondensed,
                    &Keyword(SemiExpanded) => types::FontStretch::SemiExpanded,
                    &Keyword(Expanded) => types::FontStretch::Expanded,
                    &Keyword(ExtraExpanded) => types::FontStretch::ExtraExpanded,
                    &Keyword(UltraExpanded) => types::FontStretch::UltraExpanded
                })
            }
            &PropertyDeclaration::Visibility(ref value) => {
                use self::longhands::visibility::computed_value::T::{Hidden, Visible};
                match_value_into_type!(StyleDeclaration::Theme[ThemeStyle::Visibility] match value {
                    &Hidden => types::Visibility::Hidden,
                    &Visible => types::Visibility::Visible
                })
            }
            // Layout rules
            &PropertyDeclaration::AlignContent(ref value) => {
                use self::longhands::align_content::computed_value::T::{Center, FlexEnd, FlexStart, SpaceAround, SpaceBetween, Stretch};
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::AlignContent] match value {
                    &Stretch => types::Align::Stretch,
                    &FlexStart => types::Align::FlexStart,
                    &FlexEnd => types::Align::FlexEnd,
                    &Center => types::Align::Center,
                    &SpaceBetween => types::Align::SpaceBetween,
                    &SpaceAround => types::Align::SpaceAround
                })
            }
            &PropertyDeclaration::AlignItems(ref value) => {
                use self::longhands::align_items::computed_value::T::{Baseline, Center, FlexEnd, FlexStart, Stretch};
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::AlignItems] match value {
                    &Stretch => types::Align::Stretch,
                    &FlexStart => types::Align::FlexStart,
                    &FlexEnd => types::Align::FlexEnd,
                    &Center => types::Align::Center,
                    &Baseline => types::Align::Baseline
                })
            }
            &PropertyDeclaration::AlignSelf(ref value) => {
                use self::longhands::align_self::computed_value::T::{Auto, Baseline, Center, FlexEnd, FlexStart, Stretch};
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::AlignSelf] match value {
                    &Auto => types::Align::Auto,
                    &Stretch => types::Align::Stretch,
                    &FlexStart => types::Align::FlexStart,
                    &FlexEnd => types::Align::FlexEnd,
                    &Center => types::Align::Center,
                    &Baseline => types::Align::Baseline
                })
            }
            &PropertyDeclaration::BorderBottomWidth(ref value) => {
                use self::specified::{AbsoluteLength, Length, NoCalcLength};
                use self::specified::BorderSideWidth::{Length as Value, Medium, Thick, Thin};
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::BorderBottom] match value {
                    &Thin => 1f32.into(),
                    &Medium => 2f32.into(),
                    &Thick => 3f32.into(),
                    &Value(Length::NoCalc(NoCalcLength::Absolute(AbsoluteLength::Px(px)))) => px.into()
                })
            }
            &PropertyDeclaration::BorderLeftWidth(ref value) => {
                use self::specified::{AbsoluteLength, Length, NoCalcLength};
                use self::specified::BorderSideWidth::{Length as Value, Medium, Thick, Thin};
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::BorderLeft] match value {
                    &Thin => 1f32.into(),
                    &Medium => 2f32.into(),
                    &Thick => 3f32.into(),
                    &Value(Length::NoCalc(NoCalcLength::Absolute(AbsoluteLength::Px(px)))) => px.into()
                })
            }
            &PropertyDeclaration::BorderRightWidth(ref value) => {
                use self::specified::{AbsoluteLength, Length, NoCalcLength};
                use self::specified::BorderSideWidth::{Length as Value, Medium, Thick, Thin};
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::BorderRight] match value {
                    &Thin => 1f32.into(),
                    &Medium => 2f32.into(),
                    &Thick => 3f32.into(),
                    &Value(Length::NoCalc(NoCalcLength::Absolute(AbsoluteLength::Px(px)))) => px.into()
                })
            }
            &PropertyDeclaration::BorderTopWidth(ref value) => {
                use self::specified::{AbsoluteLength, Length, NoCalcLength};
                use self::specified::BorderSideWidth::{Length as Value, Medium, Thick, Thin};
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::BorderTop] match value {
                    &Thin => 1f32.into(),
                    &Medium => 2f32.into(),
                    &Thick => 3f32.into(),
                    &Value(Length::NoCalc(NoCalcLength::Absolute(AbsoluteLength::Px(px)))) => px.into()
                })
            }
            &PropertyDeclaration::Bottom(ref value) => {
                use self::computed::Percentage as Pc;
                use self::specified::AbsoluteLength::Px;
                use self::specified::LengthOrPercentageOrAuto::{Auto, Length, Percentage};
                use self::specified::NoCalcLength::Absolute;
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::Bottom] match value {
                    &Auto => types::StyleUnit::Auto,
                    &Percentage(Pc(pc)) => types::StyleUnit::Percent((100.0 * pc).into()),
                    &Length(Absolute(Px(px))) => types::StyleUnit::Point(px.into())
                })
            }
            &PropertyDeclaration::Left(ref value) => {
                use self::computed::Percentage as Pc;
                use self::specified::AbsoluteLength::Px;
                use self::specified::LengthOrPercentageOrAuto::{Auto, Length, Percentage};
                use self::specified::NoCalcLength::Absolute;
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::Left] match value {
                    &Auto => types::StyleUnit::Auto,
                    &Percentage(Pc(pc)) => types::StyleUnit::Percent((100.0 * pc).into()),
                    &Length(Absolute(Px(px))) => types::StyleUnit::Point(px.into())
                })
            }
            &PropertyDeclaration::Right(ref value) => {
                use self::computed::Percentage as Pc;
                use self::specified::AbsoluteLength::Px;
                use self::specified::LengthOrPercentageOrAuto::{Auto, Length, Percentage};
                use self::specified::NoCalcLength::Absolute;
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::Right] match value {
                    &Auto => types::StyleUnit::Auto,
                    &Percentage(Pc(pc)) => types::StyleUnit::Percent((100.0 * pc).into()),
                    &Length(Absolute(Px(px))) => types::StyleUnit::Point(px.into())
                })
            }
            &PropertyDeclaration::Top(ref value) => {
                use self::computed::Percentage as Pc;
                use self::specified::AbsoluteLength::Px;
                use self::specified::LengthOrPercentageOrAuto::{Auto, Length, Percentage};
                use self::specified::NoCalcLength::Absolute;
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::Top] match value {
                    &Auto => types::StyleUnit::Auto,
                    &Percentage(Pc(pc)) => types::StyleUnit::Percent((100.0 * pc).into()),
                    &Length(Absolute(Px(px))) => types::StyleUnit::Point(px.into())
                })
            }
            &PropertyDeclaration::Display(ref value) => {
                use self::longhands::display::computed_value::T::{Flex, None};
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::Display] match value {
                    &None => types::Display::None,
                    &Flex => types::Display::Flex
                })
            }
            &PropertyDeclaration::FlexBasis(ref value) => {
                use self::generics::flex::FlexBasis;
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::FlexBasis] match value {
                    &FlexBasis::Auto => types::StyleUnit::Auto,
                    &FlexBasis::Length(specified::LengthOrPercentage::Length(specified::NoCalcLength::Absolute(specified::AbsoluteLength::Px(px)))) => types::StyleUnit::Point(px.into())
                })
            }
            &PropertyDeclaration::FlexDirection(ref value) => {
                use self::longhands::flex_direction::computed_value::T::{Column, ColumnReverse, Row, RowReverse};
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::FlexDirection] match value {
                    &Row => types::FlexDirection::Row,
                    &RowReverse => types::FlexDirection::RowReverse,
                    &Column => types::FlexDirection::Column,
                    &ColumnReverse => types::FlexDirection::ColumnReverse
                })
            }
            &PropertyDeclaration::FlexGrow(ref value) => {
                use self::generics::NonNegative;
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::FlexGrow] match value {
                    &NonNegative(ref n) => n.get().into()
                })
            }
            &PropertyDeclaration::FlexShrink(ref value) => {
                use self::generics::NonNegative;
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::FlexShrink] match value {
                    &NonNegative(ref n) => n.get().into()
                })
            }
            &PropertyDeclaration::FlexWrap(ref value) => {
                use self::longhands::flex_wrap::computed_value::T::{Nowrap, Wrap, WrapReverse};
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::FlexWrap] match value {
                    &Nowrap => types::Wrap::NoWrap,
                    &Wrap => types::Wrap::Wrap,
                    &WrapReverse => types::Wrap::WrapReverse
                })
            }
            &PropertyDeclaration::Height(ref value) => {
                use self::computed::Percentage as Pc;
                use self::specified::AbsoluteLength::Px;
                use self::specified::LengthOrPercentageOrAuto::{Auto, Length, Percentage};
                use self::specified::NoCalcLength::Absolute;
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::Height] match value {
                    &Auto => types::StyleUnit::Auto,
                    &Percentage(Pc(pc)) => types::StyleUnit::Percent((100.0 * pc).into()),
                    &Length(Absolute(Px(px))) => types::StyleUnit::Point(px.into())
                })
            }
            &PropertyDeclaration::Width(ref value) => {
                use self::computed::Percentage as Pc;
                use self::specified::AbsoluteLength::Px;
                use self::specified::LengthOrPercentageOrAuto::{Auto, Length, Percentage};
                use self::specified::NoCalcLength::Absolute;
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::Width] match value {
                    &Auto => types::StyleUnit::Auto,
                    &Percentage(Pc(pc)) => types::StyleUnit::Percent((100.0 * pc).into()),
                    &Length(Absolute(Px(px))) => types::StyleUnit::Point(px.into())
                })
            }
            &PropertyDeclaration::MaxHeight(ref value) => {
                use self::computed::Percentage as Pc;
                use self::specified::AbsoluteLength::Px;
                use self::specified::LengthOrPercentageOrNone::{Length, None, Percentage};
                use self::specified::NoCalcLength::Absolute;
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::MaxHeight] match value {
                    &None => types::StyleUnit::UndefinedValue,
                    &Percentage(Pc(pc)) => types::StyleUnit::Percent((100.0 * pc).into()),
                    &Length(Absolute(Px(px))) => types::StyleUnit::Point(px.into())
                })
            }
            &PropertyDeclaration::MaxWidth(ref value) => {
                use self::computed::Percentage as Pc;
                use self::specified::AbsoluteLength::Px;
                use self::specified::LengthOrPercentageOrNone::{Length, None, Percentage};
                use self::specified::NoCalcLength::Absolute;
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::MaxWidth] match value {
                    &None => types::StyleUnit::UndefinedValue,
                    &Percentage(Pc(pc)) => types::StyleUnit::Percent((100.0 * pc).into()),
                    &Length(Absolute(Px(px))) => types::StyleUnit::Point(px.into())
                })
            }
            &PropertyDeclaration::MinHeight(ref value) => {
                use self::computed::Percentage as Pc;
                use self::specified::AbsoluteLength::Px;
                use self::specified::LengthOrPercentage::{Length, Percentage};
                use self::specified::NoCalcLength::Absolute;
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::MinHeight] match value {
                    &Percentage(Pc(pc)) => types::StyleUnit::Percent((100.0 * pc).into()),
                    &Length(Absolute(Px(px))) => types::StyleUnit::Point(px.into())
                })
            }
            &PropertyDeclaration::MinWidth(ref value) => {
                use self::computed::Percentage as Pc;
                use self::specified::AbsoluteLength::Px;
                use self::specified::LengthOrPercentage::{Length, Percentage};
                use self::specified::NoCalcLength::Absolute;
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::MinWidth] match value {
                    &Percentage(Pc(pc)) => types::StyleUnit::Percent((100.0 * pc).into()),
                    &Length(Absolute(Px(px))) => types::StyleUnit::Point(px.into())
                })
            }
            &PropertyDeclaration::OverflowX(ref value) => {
                use self::longhands::overflow_x::computed_value::T::{Auto, Hidden, Scroll, Visible};
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::Overflow] match value {
                    &Visible => types::Overflow::Visible,
                    &Hidden => types::Overflow::Hidden,
                    &Scroll => types::Overflow::Scroll,
                    &Auto => types::Overflow::Visible
                })
            }
            &PropertyDeclaration::OverflowY(ref value) => {
                use self::longhands::overflow_x::computed_value::T::{Auto, Hidden, Scroll, Visible};
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::Overflow] match value {
                    &Visible => types::Overflow::Visible,
                    &Hidden => types::Overflow::Hidden,
                    &Scroll => types::Overflow::Scroll,
                    &Auto => types::Overflow::Visible
                })
            }
            &PropertyDeclaration::JustifyContent(ref value) => {
                use self::longhands::justify_content::computed_value::T::{Center, FlexEnd, FlexStart, SpaceAround, SpaceBetween};
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::JustifyContent] match value {
                    &FlexStart => types::Justify::FlexStart,
                    &FlexEnd => types::Justify::FlexEnd,
                    &Center => types::Justify::Center,
                    &SpaceBetween => types::Justify::SpaceBetween,
                    &SpaceAround => types::Justify::SpaceAround
                })
            }
            &PropertyDeclaration::MarginBottom(ref value) => {
                use self::computed::Percentage as Pc;
                use self::specified::AbsoluteLength::Px;
                use self::specified::LengthOrPercentageOrAuto::{Auto, Length, Percentage};
                use self::specified::NoCalcLength::Absolute;
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::MarginBottom] match value {
                    &Auto => types::StyleUnit::Auto,
                    &Percentage(Pc(pc)) => types::StyleUnit::Percent((100.0 * pc).into()),
                    &Length(Absolute(Px(px))) => types::StyleUnit::Point(px.into())
                })
            }
            &PropertyDeclaration::MarginLeft(ref value) => {
                use self::computed::Percentage as Pc;
                use self::specified::AbsoluteLength::Px;
                use self::specified::LengthOrPercentageOrAuto::{Auto, Length, Percentage};
                use self::specified::NoCalcLength::Absolute;
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::MarginLeft] match value {
                    &Auto => types::StyleUnit::Auto,
                    &Percentage(Pc(pc)) => types::StyleUnit::Percent((100.0 * pc).into()),
                    &Length(Absolute(Px(px))) => types::StyleUnit::Point(px.into())
                })
            }
            &PropertyDeclaration::MarginRight(ref value) => {
                use self::computed::Percentage as Pc;
                use self::specified::AbsoluteLength::Px;
                use self::specified::LengthOrPercentageOrAuto::{Auto, Length, Percentage};
                use self::specified::NoCalcLength::Absolute;
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::MarginRight] match value {
                    &Auto => types::StyleUnit::Auto,
                    &Percentage(Pc(pc)) => types::StyleUnit::Percent((100.0 * pc).into()),
                    &Length(Absolute(Px(px))) => types::StyleUnit::Point(px.into())
                })
            }
            &PropertyDeclaration::MarginTop(ref value) => {
                use self::computed::Percentage as Pc;
                use self::specified::AbsoluteLength::Px;
                use self::specified::LengthOrPercentageOrAuto::{Auto, Length, Percentage};
                use self::specified::NoCalcLength::Absolute;
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::MarginTop] match value {
                    &Auto => types::StyleUnit::Auto,
                    &Percentage(Pc(pc)) => types::StyleUnit::Percent((100.0 * pc).into()),
                    &Length(Absolute(Px(px))) => types::StyleUnit::Point(px.into())
                })
            }
            &PropertyDeclaration::PaddingBottom(ref value) => {
                use self::computed::Percentage as Pc;
                use self::generics::NonNegative;
                use self::specified::AbsoluteLength::Px;
                use self::specified::LengthOrPercentage::{Length, Percentage};
                use self::specified::NoCalcLength::Absolute;
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::PaddingBottom] match value {
                    &NonNegative(Percentage(Pc(pc))) => types::StyleUnit::Percent((100.0 * pc).into()),
                    &NonNegative(Length(Absolute(Px(px)))) => types::StyleUnit::Point(px.into())
                })
            }
            &PropertyDeclaration::PaddingLeft(ref value) => {
                use self::computed::Percentage as Pc;
                use self::generics::NonNegative;
                use self::specified::AbsoluteLength::Px;
                use self::specified::LengthOrPercentage::{Length, Percentage};
                use self::specified::NoCalcLength::Absolute;
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::PaddingLeft] match value {
                    &NonNegative(Percentage(Pc(pc))) => types::StyleUnit::Percent((100.0 * pc).into()),
                    &NonNegative(Length(Absolute(Px(px)))) => types::StyleUnit::Point(px.into())
                })
            }
            &PropertyDeclaration::PaddingRight(ref value) => {
                use self::computed::Percentage as Pc;
                use self::generics::NonNegative;
                use self::specified::AbsoluteLength::Px;
                use self::specified::LengthOrPercentage::{Length, Percentage};
                use self::specified::NoCalcLength::Absolute;
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::PaddingRight] match value {
                    &NonNegative(Percentage(Pc(pc))) => types::StyleUnit::Percent((100.0 * pc).into()),
                    &NonNegative(Length(Absolute(Px(px)))) => types::StyleUnit::Point(px.into())
                })
            }
            &PropertyDeclaration::PaddingTop(ref value) => {
                use self::computed::Percentage as Pc;
                use self::generics::NonNegative;
                use self::specified::AbsoluteLength::Px;
                use self::specified::LengthOrPercentage::{Length, Percentage};
                use self::specified::NoCalcLength::Absolute;
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::PaddingTop] match value {
                    &NonNegative(Percentage(Pc(pc))) => types::StyleUnit::Percent((100.0 * pc).into()),
                    &NonNegative(Length(Absolute(Px(px)))) => types::StyleUnit::Point(px.into())
                })
            }
            &PropertyDeclaration::Position(ref value) => {
                use self::longhands::position::computed_value::T::{Absolute, Relative};
                match_value_into_type!(StyleDeclaration::Layout[FlexStyle::Position] match value {
                    &Absolute => types::PositionType::Absolute,
                    &Relative => types::PositionType::Relative
                })
            }
            _ => StyleDeclaration::Unknown
        }
    }
}
