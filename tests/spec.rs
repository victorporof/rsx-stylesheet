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

#![recursion_limit = "128"]

#[macro_use]
extern crate quote;
extern crate rsx_shared;
extern crate rsx_stylesheet;
extern crate self_tokenize_trait;
extern crate serde_json;
extern crate syn;

use std::iter;
use std::ops::Deref;

use rsx_shared::traits::{TComputedStyles, TInheritedStyles};
use rsx_stylesheet::servo_css_parser::parse;
use rsx_stylesheet::servo_css_parser::types::{MediaList, Origin, QuirksMode, Url};
use rsx_stylesheet::types::{
    Align,
    BorderStyle,
    BoxShadow,
    Color,
    ComputedStyles,
    Cursor,
    FlexDirection,
    FlexStyle,
    FontCaps,
    FontFamily,
    FontName,
    FontSize,
    FontStretch,
    FontStyle,
    FontWeight,
    GenericFontName,
    InlineBoxShadows,
    InlineDeclarations,
    InlineFontNames,
    InlineRules,
    InlineSelectors,
    InlineTextShadows,
    SpecificFontName,
    StyleDeclaration,
    StyleDeclarations,
    StyleRule,
    StyleSelector,
    StyleSelectors,
    StyleUnit,
    Stylesheet,
    TextShadow,
    ThemeStyle,
    Visibility
};

use FlexStyle::*;
use StyleDeclaration::*;
use ThemeStyle::*;

#[test]
fn test_from_css_1() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = "#someId > .someClass { }";
    let stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    let expected = Stylesheet::from(InlineRules::from_vec(vec![
        StyleRule {
            selectors: StyleSelectors(InlineSelectors::from_vec(vec![
                StyleSelector::from("#someId > .someClass"),
            ])),
            declarations: StyleDeclarations(InlineDeclarations::new())
        },
    ]));

    assert_eq!(stylesheet, expected);
}

#[test]
fn test_from_css_2() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = "someTag { border: 1px solid; margin: 10%; padding: 20px; }";
    let stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    let expected = Stylesheet::from(InlineRules::from_vec(vec![
        StyleRule {
            selectors: StyleSelectors(InlineSelectors::from_vec(vec![
                StyleSelector::from("someTag"),
            ])),
            declarations: StyleDeclarations(InlineDeclarations::from_vec(vec![
                Theme(BorderTopStyle(BorderStyle::Solid)),
                Layout(BorderTop(1.0.into())),
                Theme(BorderLeftStyle(BorderStyle::Solid)),
                Layout(BorderLeft(1.0.into())),
                Theme(BorderBottomStyle(BorderStyle::Solid)),
                Layout(BorderBottom(1.0.into())),
                Theme(BorderRightStyle(BorderStyle::Solid)),
                Layout(BorderRight(1.0.into())),
                Layout(MarginTop(StyleUnit::Percent(10.0.into()))),
                Layout(MarginRight(StyleUnit::Percent(10.0.into()))),
                Layout(MarginBottom(StyleUnit::Percent(10.0.into()))),
                Layout(MarginLeft(StyleUnit::Percent(10.0.into()))),
                Layout(PaddingTop(StyleUnit::Point(20.0.into()))),
                Layout(PaddingRight(StyleUnit::Point(20.0.into()))),
                Layout(PaddingBottom(StyleUnit::Point(20.0.into()))),
                Layout(PaddingLeft(StyleUnit::Point(20.0.into()))),
            ]))
        },
    ]));

    assert_eq!(stylesheet, expected);
}

#[test]
fn test_from_css_3() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = "someTag, .someClass { border: 1px solid; margin: 10%; padding: 20px; }";
    let mut stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    assert_eq!(
        stylesheet.take("bogus"),
        StyleDeclarations(InlineDeclarations::new())
    );

    assert_eq!(
        stylesheet.take("someTag").deref(),
        &InlineDeclarations::from_vec(vec![
            Theme(BorderTopStyle(BorderStyle::Solid)),
            Layout(BorderTop(1.0.into())),
            Theme(BorderLeftStyle(BorderStyle::Solid)),
            Layout(BorderLeft(1.0.into())),
            Theme(BorderBottomStyle(BorderStyle::Solid)),
            Layout(BorderBottom(1.0.into())),
            Theme(BorderRightStyle(BorderStyle::Solid)),
            Layout(BorderRight(1.0.into())),
            Layout(MarginTop(StyleUnit::Percent(10.0.into()))),
            Layout(MarginRight(StyleUnit::Percent(10.0.into()))),
            Layout(MarginBottom(StyleUnit::Percent(10.0.into()))),
            Layout(MarginLeft(StyleUnit::Percent(10.0.into()))),
            Layout(PaddingTop(StyleUnit::Point(20.0.into()))),
            Layout(PaddingRight(StyleUnit::Point(20.0.into()))),
            Layout(PaddingBottom(StyleUnit::Point(20.0.into()))),
            Layout(PaddingLeft(StyleUnit::Point(20.0.into()))),
        ])
    );

    assert_eq!(
        stylesheet.take(".someClass"),
        StyleDeclarations(InlineDeclarations::new())
    );
}

#[test]
fn test_from_css_3_copy() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = "someTag, .someClass { border: 1px solid; margin: 10%; padding: 20px; }";
    let mut stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    assert_eq!(stylesheet.get_copy("bogus"), None);

    assert_eq!(
        stylesheet.get_copy("someTag").unwrap().deref(),
        &InlineDeclarations::from_vec(vec![
            Theme(BorderTopStyle(BorderStyle::Solid)),
            Layout(BorderTop(1.0.into())),
            Theme(BorderLeftStyle(BorderStyle::Solid)),
            Layout(BorderLeft(1.0.into())),
            Theme(BorderBottomStyle(BorderStyle::Solid)),
            Layout(BorderBottom(1.0.into())),
            Theme(BorderRightStyle(BorderStyle::Solid)),
            Layout(BorderRight(1.0.into())),
            Layout(MarginTop(StyleUnit::Percent(10.0.into()))),
            Layout(MarginRight(StyleUnit::Percent(10.0.into()))),
            Layout(MarginBottom(StyleUnit::Percent(10.0.into()))),
            Layout(MarginLeft(StyleUnit::Percent(10.0.into()))),
            Layout(PaddingTop(StyleUnit::Point(20.0.into()))),
            Layout(PaddingRight(StyleUnit::Point(20.0.into()))),
            Layout(PaddingBottom(StyleUnit::Point(20.0.into()))),
            Layout(PaddingLeft(StyleUnit::Point(20.0.into()))),
        ])
    );

    assert_eq!(
        stylesheet.get_copy(".someClass").unwrap().deref(),
        &InlineDeclarations::from_vec(vec![
            Theme(BorderTopStyle(BorderStyle::Solid)),
            Layout(BorderTop(1.0.into())),
            Theme(BorderLeftStyle(BorderStyle::Solid)),
            Layout(BorderLeft(1.0.into())),
            Theme(BorderBottomStyle(BorderStyle::Solid)),
            Layout(BorderBottom(1.0.into())),
            Theme(BorderRightStyle(BorderStyle::Solid)),
            Layout(BorderRight(1.0.into())),
            Layout(MarginTop(StyleUnit::Percent(10.0.into()))),
            Layout(MarginRight(StyleUnit::Percent(10.0.into()))),
            Layout(MarginBottom(StyleUnit::Percent(10.0.into()))),
            Layout(MarginLeft(StyleUnit::Percent(10.0.into()))),
            Layout(PaddingTop(StyleUnit::Point(20.0.into()))),
            Layout(PaddingRight(StyleUnit::Point(20.0.into()))),
            Layout(PaddingBottom(StyleUnit::Point(20.0.into()))),
            Layout(PaddingLeft(StyleUnit::Point(20.0.into()))),
        ])
    );
}

#[test]
fn test_from_css_3_ref() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = "someTag, .someClass { border: 1px solid; margin: 10%; padding: 20px; }";
    let stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    assert_eq!(stylesheet.get_ref("bogus"), None);

    assert_eq!(
        stylesheet.get_ref("someTag").unwrap().deref(),
        &InlineDeclarations::from_vec(vec![
            Theme(BorderTopStyle(BorderStyle::Solid)),
            Layout(BorderTop(1.0.into())),
            Theme(BorderLeftStyle(BorderStyle::Solid)),
            Layout(BorderLeft(1.0.into())),
            Theme(BorderBottomStyle(BorderStyle::Solid)),
            Layout(BorderBottom(1.0.into())),
            Theme(BorderRightStyle(BorderStyle::Solid)),
            Layout(BorderRight(1.0.into())),
            Layout(MarginTop(StyleUnit::Percent(10.0.into()))),
            Layout(MarginRight(StyleUnit::Percent(10.0.into()))),
            Layout(MarginBottom(StyleUnit::Percent(10.0.into()))),
            Layout(MarginLeft(StyleUnit::Percent(10.0.into()))),
            Layout(PaddingTop(StyleUnit::Point(20.0.into()))),
            Layout(PaddingRight(StyleUnit::Point(20.0.into()))),
            Layout(PaddingBottom(StyleUnit::Point(20.0.into()))),
            Layout(PaddingLeft(StyleUnit::Point(20.0.into()))),
        ])
    );

    assert_eq!(
        stylesheet.get_ref(".someClass").unwrap().deref(),
        &InlineDeclarations::from_vec(vec![
            Theme(BorderTopStyle(BorderStyle::Solid)),
            Layout(BorderTop(1.0.into())),
            Theme(BorderLeftStyle(BorderStyle::Solid)),
            Layout(BorderLeft(1.0.into())),
            Theme(BorderBottomStyle(BorderStyle::Solid)),
            Layout(BorderBottom(1.0.into())),
            Theme(BorderRightStyle(BorderStyle::Solid)),
            Layout(BorderRight(1.0.into())),
            Layout(MarginTop(StyleUnit::Percent(10.0.into()))),
            Layout(MarginRight(StyleUnit::Percent(10.0.into()))),
            Layout(MarginBottom(StyleUnit::Percent(10.0.into()))),
            Layout(MarginLeft(StyleUnit::Percent(10.0.into()))),
            Layout(PaddingTop(StyleUnit::Point(20.0.into()))),
            Layout(PaddingRight(StyleUnit::Point(20.0.into()))),
            Layout(PaddingBottom(StyleUnit::Point(20.0.into()))),
            Layout(PaddingLeft(StyleUnit::Point(20.0.into()))),
        ])
    );
}

#[test]
fn test_from_css_4() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = include_str!("fixtures/test_1.css");
    let mut stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    assert_eq!(
        stylesheet.take("bogus"),
        StyleDeclarations(InlineDeclarations::new())
    );

    assert_eq!(
        stylesheet.take(".root").deref(),
        &InlineDeclarations::from_vec(vec![
            Layout(Width(StyleUnit::Point(500.0.into()))),
            Layout(Height(StyleUnit::Point(120.0.into()))),
            Layout(FlexDirection(FlexDirection::Row)),
            Layout(PaddingTop(StyleUnit::Point(20.0.into()))),
            Layout(PaddingRight(StyleUnit::Point(20.0.into()))),
            Layout(PaddingBottom(StyleUnit::Point(20.0.into()))),
            Layout(PaddingLeft(StyleUnit::Point(20.0.into()))),
        ])
    );

    assert_eq!(
        stylesheet.take(".image").deref(),
        &InlineDeclarations::from_vec(vec![
            Layout(Width(StyleUnit::Point(80.0.into()))),
            Layout(MarginRight(StyleUnit::Point(20.0.into()))),
        ])
    );

    assert_eq!(
        stylesheet.take(".text").deref(),
        &InlineDeclarations::from_vec(vec![
            Layout(Height(StyleUnit::Point(25.0.into()))),
            Layout(AlignSelf(Align::Center)),
            Layout(FlexGrow(1.0.into())),
        ])
    );
}

#[test]
fn test_from_css_5() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = include_str!("fixtures/test_2.css");
    let mut stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    assert_eq!(
        stylesheet.take("bogus"),
        StyleDeclarations(InlineDeclarations::new())
    );

    assert_eq!(
        stylesheet.take(".root").deref(),
        &InlineDeclarations::from_vec(vec![
            Theme(BackgroundColor(Color {
                red: 255,
                green: 0,
                blue: 0,
                alpha: 255
            })),
            Layout(Width(StyleUnit::Point(500.0.into()))),
            Layout(Height(StyleUnit::Point(120.0.into()))),
            Layout(FlexDirection(FlexDirection::Row)),
            Layout(PaddingTop(StyleUnit::Point(20.0.into()))),
            Layout(PaddingRight(StyleUnit::Point(20.0.into()))),
            Layout(PaddingBottom(StyleUnit::Point(20.0.into()))),
            Layout(PaddingLeft(StyleUnit::Point(20.0.into()))),
        ])
    );

    assert_eq!(
        stylesheet.take(".image").deref(),
        &InlineDeclarations::from_vec(vec![
            Theme(BackgroundColor(Color {
                red: 0,
                green: 128,
                blue: 0,
                alpha: 255
            })),
            Theme(Opacity(50)),
            Layout(Width(StyleUnit::Point(80.0.into()))),
            Layout(MarginRight(StyleUnit::Point(20.0.into()))),
        ])
    );

    assert_eq!(
        stylesheet.take(".text").deref(),
        &InlineDeclarations::from_vec(vec![
            Theme(BackgroundColor(Color {
                red: 0,
                green: 0,
                blue: 255,
                alpha: 255
            })),
            Theme(Color(Color {
                red: 255,
                green: 255,
                blue: 0,
                alpha: 255
            })),
            Layout(Height(StyleUnit::Point(25.0.into()))),
            Layout(AlignSelf(Align::Center)),
            Layout(FlexGrow(1.0.into())),
        ])
    );
}

#[test]
fn test_from_css_box_shadow_1() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = ".someClass { box-shadow: 10px 5px; }";
    let mut stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    assert_eq!(
        stylesheet.take(".someClass").deref(),
        &InlineDeclarations::from_vec(vec![
            Theme(BoxShadow(InlineBoxShadows::from_vec(vec![
                BoxShadow {
                    color: None,
                    horizontal: StyleUnit::Point(10.0.into()),
                    vertical: StyleUnit::Point(5.0.into()),
                    blur: None,
                    spread: None,
                    inset: false
                },
            ]))),
        ])
    );
}

#[test]
fn test_from_css_box_shadow_2() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = ".someClass { box-shadow: 10px 5px red; }";
    let mut stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    assert_eq!(
        stylesheet.take(".someClass").deref(),
        &InlineDeclarations::from_vec(vec![
            Theme(BoxShadow(InlineBoxShadows::from_vec(vec![
                BoxShadow {
                    color: Some(Color {
                        red: 255,
                        green: 0,
                        blue: 0,
                        alpha: 255
                    }),
                    horizontal: StyleUnit::Point(10.0.into()),
                    vertical: StyleUnit::Point(5.0.into()),
                    blur: None,
                    spread: None,
                    inset: false
                },
            ]))),
        ])
    );
}

#[test]
fn test_from_css_box_shadow_3() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = ".someClass { box-shadow: 10px 5px 5px red; }";
    let mut stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    assert_eq!(
        stylesheet.take(".someClass").deref(),
        &InlineDeclarations::from_vec(vec![
            Theme(BoxShadow(InlineBoxShadows::from_vec(vec![
                BoxShadow {
                    color: Some(Color {
                        red: 255,
                        green: 0,
                        blue: 0,
                        alpha: 255
                    }),
                    horizontal: StyleUnit::Point(10.0.into()),
                    vertical: StyleUnit::Point(5.0.into()),
                    blur: Some(StyleUnit::Point(5.0.into())),
                    spread: None,
                    inset: false
                },
            ]))),
        ])
    );
}

#[test]
fn test_from_css_box_shadow_4() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = ".someClass { box-shadow: 10px 5px 5px 1px red; }";
    let mut stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    assert_eq!(
        stylesheet.take(".someClass").deref(),
        &InlineDeclarations::from_vec(vec![
            Theme(BoxShadow(InlineBoxShadows::from_vec(vec![
                BoxShadow {
                    color: Some(Color {
                        red: 255,
                        green: 0,
                        blue: 0,
                        alpha: 255
                    }),
                    horizontal: StyleUnit::Point(10.0.into()),
                    vertical: StyleUnit::Point(5.0.into()),
                    blur: Some(StyleUnit::Point(5.0.into())),
                    spread: Some(StyleUnit::Point(1.0.into())),
                    inset: false
                },
            ]))),
        ])
    );
}

#[test]
fn test_from_css_box_shadow_5() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = ".someClass { box-shadow: inset 10px 5px 5px 1px red; }";
    let mut stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    assert_eq!(
        stylesheet.take(".someClass").deref(),
        &InlineDeclarations::from_vec(vec![
            Theme(BoxShadow(InlineBoxShadows::from_vec(vec![
                BoxShadow {
                    color: Some(Color {
                        red: 255,
                        green: 0,
                        blue: 0,
                        alpha: 255
                    }),
                    horizontal: StyleUnit::Point(10.0.into()),
                    vertical: StyleUnit::Point(5.0.into()),
                    blur: Some(StyleUnit::Point(5.0.into())),
                    spread: Some(StyleUnit::Point(1.0.into())),
                    inset: true
                },
            ]))),
        ])
    );
}

#[test]
fn test_from_css_text_shadow_1() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = ".someClass { text-shadow: 10px 5px; }";
    let mut stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    assert_eq!(
        stylesheet.take(".someClass").deref(),
        &InlineDeclarations::from_vec(vec![
            Theme(TextShadow(InlineTextShadows::from_vec(vec![
                TextShadow {
                    color: None,
                    horizontal: StyleUnit::Point(10.0.into()),
                    vertical: StyleUnit::Point(5.0.into()),
                    blur: None
                },
            ]))),
        ])
    );
}

#[test]
fn test_from_css_text_shadow_2() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = ".someClass { text-shadow: 10px 5px red; }";
    let mut stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    assert_eq!(
        stylesheet.take(".someClass").deref(),
        &InlineDeclarations::from_vec(vec![
            Theme(TextShadow(InlineTextShadows::from_vec(vec![
                TextShadow {
                    color: Some(Color {
                        red: 255,
                        green: 0,
                        blue: 0,
                        alpha: 255
                    }),
                    horizontal: StyleUnit::Point(10.0.into()),
                    vertical: StyleUnit::Point(5.0.into()),
                    blur: None
                },
            ]))),
        ])
    );
}

#[test]
fn test_from_css_text_shadow_3() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = ".someClass { text-shadow: 10px 5px 5px red; }";
    let mut stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    assert_eq!(
        stylesheet.take(".someClass").deref(),
        &InlineDeclarations::from_vec(vec![
            Theme(TextShadow(InlineTextShadows::from_vec(vec![
                TextShadow {
                    color: Some(Color {
                        red: 255,
                        green: 0,
                        blue: 0,
                        alpha: 255
                    }),
                    horizontal: StyleUnit::Point(10.0.into()),
                    vertical: StyleUnit::Point(5.0.into()),
                    blur: Some(StyleUnit::Point(5.0.into()))
                },
            ]))),
        ])
    );
}

#[test]
fn test_from_css_visibility_1() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = ".someClass { visibility: hidden; }";
    let mut stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    assert_eq!(
        stylesheet.take(".someClass").deref(),
        &InlineDeclarations::from_vec(vec![Theme(Visibility(Visibility::Hidden))])
    );
}

#[test]
fn test_from_css_visibility_2() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = ".someClass { visibility: visible; }";
    let mut stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    assert_eq!(
        stylesheet.take(".someClass").deref(),
        &InlineDeclarations::from_vec(vec![Theme(Visibility(Visibility::Visible))])
    );
}

#[test]
fn test_from_css_cursor() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = ".someClass { cursor: pointer; }";
    let mut stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    assert_eq!(
        stylesheet.take(".someClass").deref(),
        &InlineDeclarations::from_vec(vec![Theme(Cursor(Cursor::Pointer))])
    );
}

#[test]
fn test_from_css_font_family_1() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = ".someClass { font-family: sans-serif; }";
    let mut stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    assert_eq!(
        stylesheet.take(".someClass").deref(),
        &InlineDeclarations::from_vec(vec![
            Theme(FontFamily(FontFamily::Values(InlineFontNames::from_vec(
                vec![FontName::Generic(GenericFontName::SansSerif)]
            )))),
        ])
    );
}

#[test]
fn test_from_css_font_family_2() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = ".someClass { font-family: Times, \"Times New Roman\", Georgia, serif; }";
    let mut stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    assert_eq!(
        stylesheet.take(".someClass").deref(),
        &InlineDeclarations::from_vec(vec![
            Theme(FontFamily(FontFamily::Values(InlineFontNames::from_vec(
                vec![
                    FontName::Specific(SpecificFontName::from("Times")),
                    FontName::Specific(SpecificFontName::from("\"Times New Roman\"")),
                    FontName::Specific(SpecificFontName::from("Georgia")),
                    FontName::Generic(GenericFontName::Serif),
                ]
            )))),
        ])
    );
}

#[test]
fn test_from_css_font_style_1() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = ".someClass { font-style: normal }";
    let mut stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    assert_eq!(
        stylesheet.take(".someClass").deref(),
        &InlineDeclarations::from_vec(vec![Theme(FontStyle(FontStyle::Normal))])
    );
}

#[test]
fn test_from_css_font_style_2() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = ".someClass { font-style: oblique }";
    let mut stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    assert_eq!(
        stylesheet.take(".someClass").deref(),
        &InlineDeclarations::from_vec(vec![Theme(FontStyle(FontStyle::Oblique))])
    );
}

#[test]
fn test_from_css_font_caps_1() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = ".someClass { font-variant: normal }";
    let mut stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    assert_eq!(
        stylesheet.take(".someClass").deref(),
        &InlineDeclarations::from_vec(vec![Theme(FontCaps(FontCaps::Normal))])
    );
}

#[test]
fn test_from_css_font_caps_2() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = ".someClass { font-variant: small-caps }";
    let mut stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    assert_eq!(
        stylesheet.take(".someClass").deref(),
        &InlineDeclarations::from_vec(vec![Theme(FontCaps(FontCaps::SmallCaps))])
    );
}

#[test]
fn test_from_css_font_weight_1() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = ".someClass { font-weight: bold }";
    let mut stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    assert_eq!(
        stylesheet.take(".someClass").deref(),
        &InlineDeclarations::from_vec(vec![Theme(FontWeight(FontWeight::Bold))])
    );
}

#[test]
fn test_from_css_font_weight_2() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = ".someClass { font-weight: 600 }";
    let mut stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    assert_eq!(
        stylesheet.take(".someClass").deref(),
        &InlineDeclarations::from_vec(vec![Theme(FontWeight(FontWeight::Weight(600u32)))])
    );
}

#[test]
fn test_from_css_font_size_1() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = ".someClass { font-size: 10px; }";
    let mut stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    assert_eq!(
        stylesheet.take(".someClass").deref(),
        &InlineDeclarations::from_vec(vec![
            Theme(FontSize(FontSize::Length(StyleUnit::Point(10.0.into())))),
        ])
    );
}

#[test]
fn test_from_css_font_size_2() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = ".someClass { font-size: 10%; }";
    let mut stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    assert_eq!(
        stylesheet.take(".someClass").deref(),
        &InlineDeclarations::from_vec(vec![
            Theme(FontSize(FontSize::Length(StyleUnit::Percent(10.0.into())))),
        ])
    );
}

#[test]
fn test_from_css_font_stretch_1() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = ".someClass { font-stretch: normal; }";
    let mut stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    assert_eq!(
        stylesheet.take(".someClass").deref(),
        &InlineDeclarations::from_vec(vec![Theme(FontStretch(FontStretch::Normal))])
    );
}

#[test]
fn test_from_css_font_stretch_2() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = ".someClass { font-stretch: condensed; }";
    let mut stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    assert_eq!(
        stylesheet.take(".someClass").deref(),
        &InlineDeclarations::from_vec(vec![Theme(FontStretch(FontStretch::Condensed))])
    );
}

#[test]
pub fn test_tokens_1() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = include_str!("fixtures/test_1.css");
    let stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    let tokens = quote! {
        Stylesheet::from(SmallVec::from_vec(vec![
            StyleRule {
                selectors: StyleSelectors(SmallVec::from_buf([StyleSelector::from(".root"),])),
                declarations: StyleDeclarations(SmallVec::from_vec(vec![
                    StyleDeclaration::Layout(FlexStyle::Width(StyleUnit::Point(500f32.into()))),
                    StyleDeclaration::Layout(FlexStyle::Height(StyleUnit::Point(120f32.into()))),
                    StyleDeclaration::Layout(FlexStyle::FlexDirection(FlexDirection::Row)),
                    StyleDeclaration::Layout(FlexStyle::PaddingTop(StyleUnit::Point(20f32.into()))),
                    StyleDeclaration::Layout(FlexStyle::PaddingRight(StyleUnit::Point(20f32.into()))),
                    StyleDeclaration::Layout(FlexStyle::PaddingBottom(StyleUnit::Point(20f32.into()))),
                    StyleDeclaration::Layout(FlexStyle::PaddingLeft(StyleUnit::Point(20f32.into()))),
                ]))
            },
            StyleRule {
                selectors: StyleSelectors(SmallVec::from_buf([StyleSelector::from(".image"),])),
                declarations: StyleDeclarations(SmallVec::from_vec(vec![
                    StyleDeclaration::Layout(FlexStyle::Width(StyleUnit::Point(80f32.into()))),
                    StyleDeclaration::Layout(FlexStyle::MarginRight(StyleUnit::Point(20f32.into()))),
                ]))
            },
            StyleRule {
                selectors: StyleSelectors(SmallVec::from_buf([StyleSelector::from(".text"),])),
                declarations: StyleDeclarations(SmallVec::from_vec(vec![
                    StyleDeclaration::Layout(FlexStyle::Height(StyleUnit::Point(25f32.into()))),
                    StyleDeclaration::Layout(FlexStyle::AlignSelf(Align::Center)),
                    StyleDeclaration::Layout(FlexStyle::FlexGrow(1f32.into())),
                ]))
            },
        ]))
    };

    assert_eq!(
        syn::parse_expr(quote! { #stylesheet }.as_str()),
        syn::parse_expr(tokens.as_str())
    );
}

#[test]
pub fn test_tokens_2() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = include_str!("fixtures/test_2.css");
    let stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    let tokens = quote! {
        Stylesheet::from(SmallVec::from_vec(vec![
            StyleRule {
                selectors: StyleSelectors(SmallVec::from_buf([StyleSelector::from(".root"),])),
                declarations: StyleDeclarations(SmallVec::from_buf([
                    StyleDeclaration::Theme(ThemeStyle::BackgroundColor(Color {
                        red: 255u8,
                        green: 0u8,
                        blue: 0u8,
                        alpha: 255u8
                    })),
                    StyleDeclaration::Layout(FlexStyle::Width(StyleUnit::Point(500f32.into()))),
                    StyleDeclaration::Layout(FlexStyle::Height(StyleUnit::Point(120f32.into()))),
                    StyleDeclaration::Layout(FlexStyle::FlexDirection(FlexDirection::Row)),
                    StyleDeclaration::Layout(FlexStyle::PaddingTop(StyleUnit::Point(20f32.into()))),
                    StyleDeclaration::Layout(FlexStyle::PaddingRight(StyleUnit::Point(20f32.into()))),
                    StyleDeclaration::Layout(FlexStyle::PaddingBottom(StyleUnit::Point(20f32.into()))),
                    StyleDeclaration::Layout(FlexStyle::PaddingLeft(StyleUnit::Point(20f32.into()))),
                ]))
            },
            StyleRule {
                selectors: StyleSelectors(SmallVec::from_buf([StyleSelector::from(".image"),])),
                declarations: StyleDeclarations(SmallVec::from_vec(vec![
                    StyleDeclaration::Theme(ThemeStyle::BackgroundColor(Color {
                        red: 0u8,
                        green: 128u8,
                        blue: 0u8,
                        alpha: 255u8
                    })),
                    StyleDeclaration::Theme(ThemeStyle::Opacity(50u32)),
                    StyleDeclaration::Layout(FlexStyle::Width(StyleUnit::Point(80f32.into()))),
                    StyleDeclaration::Layout(FlexStyle::MarginRight(StyleUnit::Point(20f32.into()))),
                ]))
            },
            StyleRule {
                selectors: StyleSelectors(SmallVec::from_buf([StyleSelector::from(".text"),])),
                declarations: StyleDeclarations(SmallVec::from_vec(vec![
                    StyleDeclaration::Theme(ThemeStyle::BackgroundColor(Color {
                        red: 0u8,
                        green: 0u8,
                        blue: 255u8,
                        alpha: 255u8
                    })),
                    StyleDeclaration::Theme(ThemeStyle::Color(Color {
                        red: 255u8,
                        green: 255u8,
                        blue: 0u8,
                        alpha: 255u8
                    })),
                    StyleDeclaration::Layout(FlexStyle::Height(StyleUnit::Point(25f32.into()))),
                    StyleDeclaration::Layout(FlexStyle::AlignSelf(Align::Center)),
                    StyleDeclaration::Layout(FlexStyle::FlexGrow(1f32.into())),
                ]))
            },
        ]))
    };

    assert_eq!(
        syn::parse_expr(quote! { #stylesheet }.as_str()),
        syn::parse_expr(tokens.as_str())
    );
}

#[test]
pub fn test_tokens_3() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = include_str!("fixtures/test_3.css");
    let stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    let tokens = quote! {
        Stylesheet::from(SmallVec::from_vec(vec![
            StyleRule {
                selectors: StyleSelectors(SmallVec::from_buf([StyleSelector::from(".root"),])),
                declarations: StyleDeclarations(SmallVec::from_vec(vec![
                    StyleDeclaration::Theme(ThemeStyle::FontFamily(FontFamily::Values(
                        SmallVec::from_vec(vec![
                            FontName::Specific(SpecificFontName::from("Arial")),
                            FontName::Specific(SpecificFontName::from("Helvetica")),
                            FontName::Generic(GenericFontName::SansSerif),
                        ])
                    ))),
                    StyleDeclaration::Theme(ThemeStyle::BackgroundColor(Color {
                        red: 255u8,
                        green: 0u8,
                        blue: 0u8,
                        alpha: 255u8
                    })),
                    StyleDeclaration::Layout(FlexStyle::Width(StyleUnit::Point(500f32.into()))),
                    StyleDeclaration::Layout(FlexStyle::Height(StyleUnit::Point(120f32.into()))),
                    StyleDeclaration::Layout(FlexStyle::FlexDirection(FlexDirection::Row)),
                    StyleDeclaration::Layout(FlexStyle::PaddingTop(StyleUnit::Point(20f32.into()))),
                    StyleDeclaration::Layout(FlexStyle::PaddingRight(StyleUnit::Point(20f32.into()))),
                    StyleDeclaration::Layout(FlexStyle::PaddingBottom(StyleUnit::Point(20f32.into()))),
                    StyleDeclaration::Layout(FlexStyle::PaddingLeft(StyleUnit::Point(20f32.into()))),
                ]))
            },
            StyleRule {
                selectors: StyleSelectors(SmallVec::from_buf([StyleSelector::from(".image"),])),
                declarations: StyleDeclarations(SmallVec::from_vec(vec![
                    StyleDeclaration::Theme(ThemeStyle::BackgroundColor(Color {
                        red: 0u8,
                        green: 128u8,
                        blue: 0u8,
                        alpha: 255u8
                    })),
                    StyleDeclaration::Theme(ThemeStyle::Opacity(50u32)),
                    StyleDeclaration::Layout(FlexStyle::Width(StyleUnit::Point(80f32.into()))),
                    StyleDeclaration::Layout(FlexStyle::MarginRight(StyleUnit::Point(20f32.into()))),
                ]))
            },
            StyleRule {
                selectors: StyleSelectors(SmallVec::from_buf([StyleSelector::from(".text"),])),
                declarations: StyleDeclarations(SmallVec::from_vec(vec![
                    StyleDeclaration::Theme(ThemeStyle::FontFamily(FontFamily::Values(
                        SmallVec::from_vec(vec![
                            FontName::Specific(SpecificFontName::from("\"Times New Roman\"")),
                            FontName::Specific(SpecificFontName::from("Times")),
                            FontName::Generic(GenericFontName::Serif),
                        ])
                    ))),
                    StyleDeclaration::Theme(ThemeStyle::BackgroundColor(Color {
                        red: 0u8,
                        green: 0u8,
                        blue: 255u8,
                        alpha: 255u8
                    })),
                    StyleDeclaration::Theme(ThemeStyle::Color(Color {
                        red: 255u8,
                        green: 255u8,
                        blue: 0u8,
                        alpha: 255u8
                    })),
                    StyleDeclaration::Layout(FlexStyle::Height(StyleUnit::Point(25f32.into()))),
                    StyleDeclaration::Layout(FlexStyle::AlignSelf(Align::Center)),
                    StyleDeclaration::Layout(FlexStyle::FlexGrow(1f32.into())),
                ]))
            },
        ]))
    };

    assert_eq!(
        syn::parse_expr(quote! { #stylesheet }.as_str()),
        syn::parse_expr(tokens.as_str())
    );
}

#[test]
pub fn test_serialize_1() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = include_str!("fixtures/test_1.css");
    let stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();
    let stringified = serde_json::to_string_pretty(stylesheet.get_ref(".root").unwrap()).unwrap();

    let expected = r#"
[
  {
    "Layout": {
      "Width": {
        "Point": 500.0
      }
    }
  },
  {
    "Layout": {
      "Height": {
        "Point": 120.0
      }
    }
  },
  {
    "Layout": {
      "FlexDirection": "Row"
    }
  },
  {
    "Layout": {
      "PaddingTop": {
        "Point": 20.0
      }
    }
  },
  {
    "Layout": {
      "PaddingRight": {
        "Point": 20.0
      }
    }
  },
  {
    "Layout": {
      "PaddingBottom": {
        "Point": 20.0
      }
    }
  },
  {
    "Layout": {
      "PaddingLeft": {
        "Point": 20.0
      }
    }
  }
]"#;

    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&stringified).unwrap(),
        serde_json::from_str::<serde_json::Value>(expected).unwrap()
    );
}

#[test]
pub fn test_serialize_2() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = include_str!("fixtures/test_2.css");
    let stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();
    let stringified = serde_json::to_string_pretty(stylesheet.get_ref(".root").unwrap()).unwrap();

    let expected = r#"
[
  {
    "Theme": {
      "BackgroundColor": {
        "red": 255,
        "green": 0,
        "blue": 0,
        "alpha": 255
      }
    }
  },
  {
    "Layout": {
      "Width": {
        "Point": 500.0
      }
    }
  },
  {
    "Layout": {
      "Height": {
        "Point": 120.0
      }
    }
  },
  {
    "Layout": {
      "FlexDirection": "Row"
    }
  },
  {
    "Layout": {
      "PaddingTop": {
        "Point": 20.0
      }
    }
  },
  {
    "Layout": {
      "PaddingRight": {
        "Point": 20.0
      }
    }
  },
  {
    "Layout": {
      "PaddingBottom": {
        "Point": 20.0
      }
    }
  },
  {
    "Layout": {
      "PaddingLeft": {
        "Point": 20.0
      }
    }
  }
]"#;

    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&stringified).unwrap(),
        serde_json::from_str::<serde_json::Value>(expected).unwrap()
    );
}

#[test]
pub fn test_serialize_3() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = include_str!("fixtures/test_3.css");
    let stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();
    let stringified = serde_json::to_string_pretty(stylesheet.get_ref(".root").unwrap()).unwrap();

    let expected = r#"
[
  {
    "Theme": {
      "FontFamily": {
        "Values": [
          {
            "Specific": "Arial"
          },
          {
            "Specific": "Helvetica"
          },
          {
            "Generic": "SansSerif"
          }
        ]
      }
    }
  },
  {
    "Theme": {
      "BackgroundColor": {
        "red": 255,
        "green": 0,
        "blue": 0,
        "alpha": 255
      }
    }
  },
  {
    "Layout": {
      "Width": {
        "Point": 500.0
      }
    }
  },
  {
    "Layout": {
      "Height": {
        "Point": 120.0
      }
    }
  },
  {
    "Layout": {
      "FlexDirection": "Row"
    }
  },
  {
    "Layout": {
      "PaddingTop": {
        "Point": 20.0
      }
    }
  },
  {
    "Layout": {
      "PaddingRight": {
        "Point": 20.0
      }
    }
  },
  {
    "Layout": {
      "PaddingBottom": {
        "Point": 20.0
      }
    }
  },
  {
    "Layout": {
      "PaddingLeft": {
        "Point": 20.0
      }
    }
  }
]"#;

    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&stringified).unwrap(),
        serde_json::from_str::<serde_json::Value>(expected).unwrap()
    );
}

#[test]
fn test_computed_styles_1() {
    let computed = ComputedStyles::default();

    assert_eq!(computed.cursor(), Cursor::Default);
    assert_eq!(computed.color().red, 0);
    assert_eq!(computed.color().green, 0);
    assert_eq!(computed.color().blue, 0);
    assert_eq!(computed.color().alpha, 255);
    assert_eq!(computed.text_shadows_copy(), vec![]);
    assert_eq!(computed.font_names_copy(), vec![]);
    assert_eq!(computed.font_caps(), FontCaps::Normal);
    assert_eq!(computed.font_weight(), FontWeight::Normal);
    assert_eq!(computed.font_size(), FontSize::System);
    assert_eq!(computed.font_stretch(), FontStretch::Normal);
    assert_eq!(computed.visibility(), Visibility::Visible);
}

#[test]
fn test_computed_styles_2() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = include_str!("fixtures/test_1.css");
    let stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    let mut computed = ComputedStyles::default();

    computed.apply_rules(iter::once(stylesheet.get_ref(".root").unwrap()));
    assert_eq!(computed.cursor(), Cursor::Default);
    assert_eq!(computed.color().red, 0);
    assert_eq!(computed.color().green, 0);
    assert_eq!(computed.color().blue, 0);
    assert_eq!(computed.color().alpha, 255);
    assert_eq!(computed.text_shadows_copy(), vec![]);
    assert_eq!(computed.font_names_copy(), vec![]);
    assert_eq!(computed.font_caps(), FontCaps::Normal);
    assert_eq!(computed.font_weight(), FontWeight::Normal);
    assert_eq!(computed.font_size(), FontSize::System);
    assert_eq!(computed.font_stretch(), FontStretch::Normal);
    assert_eq!(computed.visibility(), Visibility::Visible);
}

#[test]
fn test_computed_styles_3() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = include_str!("fixtures/test_2.css");
    let stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    let mut computed = ComputedStyles::default();

    computed.apply_rules(iter::once(stylesheet.get_ref(".root").unwrap()));
    assert_eq!(computed.cursor(), Cursor::Default);
    assert_eq!(computed.color().red, 0);
    assert_eq!(computed.color().green, 0);
    assert_eq!(computed.color().blue, 0);
    assert_eq!(computed.color().alpha, 255);
    assert_eq!(computed.text_shadows_copy(), vec![]);
    assert_eq!(computed.font_names_copy(), vec![]);
    assert_eq!(computed.font_caps(), FontCaps::Normal);
    assert_eq!(computed.font_weight(), FontWeight::Normal);
    assert_eq!(computed.font_size(), FontSize::System);
    assert_eq!(computed.font_stretch(), FontStretch::Normal);
    assert_eq!(computed.visibility(), Visibility::Visible);
}

#[test]
fn test_computed_styles_4() {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    let css = include_str!("fixtures/test_3.css");
    let stylesheet: Stylesheet = parse(css, url, origin, qm, media).into();

    let mut computed = ComputedStyles::default();

    computed.apply_rules(iter::once(stylesheet.get_ref(".root").unwrap()));
    assert_eq!(computed.cursor(), Cursor::Default);
    assert_eq!(computed.color().red, 0);
    assert_eq!(computed.color().green, 0);
    assert_eq!(computed.color().blue, 0);
    assert_eq!(computed.color().alpha, 255);
    assert_eq!(computed.text_shadows_copy(), vec![]);
    assert_eq!(
        computed.font_names_copy(),
        vec![
            FontName::Specific(SpecificFontName::from("Arial")),
            FontName::Specific(SpecificFontName::from("Helvetica")),
            FontName::Generic(GenericFontName::SansSerif),
        ]
    );
    assert_eq!(computed.font_caps(), FontCaps::Normal);
    assert_eq!(computed.font_weight(), FontWeight::Normal);
    assert_eq!(computed.font_size(), FontSize::System);
    assert_eq!(computed.font_stretch(), FontStretch::Normal);
    assert_eq!(computed.visibility(), Visibility::Visible);

    computed.apply_rules(iter::once(stylesheet.get_ref(".image").unwrap()));
    assert_eq!(computed.cursor(), Cursor::Default);
    assert_eq!(computed.color().red, 0);
    assert_eq!(computed.color().green, 0);
    assert_eq!(computed.color().blue, 0);
    assert_eq!(computed.color().alpha, 255);
    assert_eq!(computed.text_shadows_copy(), vec![]);
    assert_eq!(
        computed.font_names_copy(),
        vec![
            FontName::Specific(SpecificFontName::from("Arial")),
            FontName::Specific(SpecificFontName::from("Helvetica")),
            FontName::Generic(GenericFontName::SansSerif),
        ]
    );
    assert_eq!(computed.font_caps(), FontCaps::Normal);
    assert_eq!(computed.font_weight(), FontWeight::Normal);
    assert_eq!(computed.font_size(), FontSize::System);
    assert_eq!(computed.font_stretch(), FontStretch::Normal);
    assert_eq!(computed.visibility(), Visibility::Visible);

    computed.apply_rules(iter::once(stylesheet.get_ref(".text").unwrap()));
    assert_eq!(computed.cursor(), Cursor::Default);
    assert_eq!(computed.color().red, 255);
    assert_eq!(computed.color().green, 255);
    assert_eq!(computed.color().blue, 0);
    assert_eq!(computed.color().alpha, 255);
    assert_eq!(computed.text_shadows_copy(), vec![]);
    assert_eq!(
        computed.font_names_copy(),
        vec![
            FontName::Specific(SpecificFontName::from("\"Times New Roman\"")),
            FontName::Specific(SpecificFontName::from("Times")),
            FontName::Generic(GenericFontName::Serif),
        ]
    );
    assert_eq!(computed.font_caps(), FontCaps::Normal);
    assert_eq!(computed.font_weight(), FontWeight::Normal);
    assert_eq!(computed.font_size(), FontSize::System);
    assert_eq!(computed.font_stretch(), FontStretch::Normal);
    assert_eq!(computed.visibility(), Visibility::Visible);
}
