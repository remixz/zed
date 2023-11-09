// This file was generated by the `theme_importer`.
// Be careful when modifying it by hand.

use gpui::rgba;

#[allow(unused)]
use crate::{
    Appearance, StatusColorsRefinement, ThemeColorsRefinement, UserFontStyle, UserFontWeight,
    UserHighlightStyle, UserSyntaxTheme, UserTheme, UserThemeFamily, UserThemeStylesRefinement,
};

pub fn synthwave_84() -> UserThemeFamily {
    UserThemeFamily {
        name: "Synthwave 84".into(),
        author: "Robb Owen (robb0wen)".into(),
        themes: vec![UserTheme {
            name: "Synthwave 84".into(),
            appearance: Appearance::Dark,
            styles: UserThemeStylesRefinement {
                colors: ThemeColorsRefinement {
                    border_focused: Some(rgba(0x1f212bff).into()),
                    background: Some(rgba(0x252334ff).into()),
                    element_background: Some(rgba(0x614d85ff).into()),
                    element_hover: Some(rgba(0x37294d99).into()),
                    element_selected: Some(rgba(0xffffff20).into()),
                    drop_target_background: Some(rgba(0x34294f66).into()),
                    ghost_element_hover: Some(rgba(0x37294d99).into()),
                    text: Some(rgba(0xffffffff).into()),
                    tab_inactive_background: Some(rgba(0x252334ff).into()),
                    editor_background: Some(rgba(0x252334ff).into()),
                    editor_gutter_background: Some(rgba(0x252334ff).into()),
                    editor_line_number: Some(rgba(0xffffff73).into()),
                    terminal_ansi_bright_red: Some(rgba(0xfe444fff).into()),
                    terminal_ansi_bright_green: Some(rgba(0x71f1b7ff).into()),
                    terminal_ansi_bright_yellow: Some(rgba(0xfede5cff).into()),
                    terminal_ansi_bright_blue: Some(rgba(0x02edf9ff).into()),
                    terminal_ansi_bright_magenta: Some(rgba(0xff7ddaff).into()),
                    terminal_ansi_bright_cyan: Some(rgba(0x02edf9ff).into()),
                    terminal_ansi_red: Some(rgba(0xfe444fff).into()),
                    terminal_ansi_green: Some(rgba(0x71f1b7ff).into()),
                    terminal_ansi_yellow: Some(rgba(0xf3e70fff).into()),
                    terminal_ansi_blue: Some(rgba(0x02edf9ff).into()),
                    terminal_ansi_magenta: Some(rgba(0xff7ddaff).into()),
                    terminal_ansi_cyan: Some(rgba(0x02edf9ff).into()),
                    ..Default::default()
                },
                status: StatusColorsRefinement {
                    deleted: Some(rgba(0xfe444fff).into()),
                    error: Some(rgba(0xfe444fff).into()),
                    warning: Some(rgba(0x71f1b7bb).into()),
                    ..Default::default()
                },
                syntax: Some(UserSyntaxTheme {
                    highlights: vec![
                        (
                            "attribute".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfede5cff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "boolean".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf97d71ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "comment".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x848bbdff).into()),
                                font_style: Some(UserFontStyle::Italic),
                                ..Default::default()
                            },
                        ),
                        (
                            "function".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x35f9f5ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "keyword".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfede5cff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "label".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfe444fff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "link_text".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xd50c50ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "link_uri".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xd50c50ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "number".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf97d71ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "operator".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfede5cff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "property".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xff7ddaff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x35f9f5ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "tag".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x71f1b7ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "title".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfe444fff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "type".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfe444fff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "variable".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xff7ddaff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "variable.special".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfe444fff).into()),
                                font_weight: Some(UserFontWeight(700.0)),
                                ..Default::default()
                            },
                        ),
                    ],
                }),
            },
        }],
    }
}
