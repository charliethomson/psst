use std::fs::File;

use crate::ui::theme::{
    grid, BACKGROUND_DARK, BACKGROUND_LIGHT, BLUE_100, BLUE_200, BORDER_DARK, BORDER_LIGHT,
    BUTTON_DARK, BUTTON_LIGHT, CURSOR_COLOR, FOREGROUND_DARK, FOREGROUND_LIGHT, GREY_000, GREY_100,
    GREY_200, GREY_300, GREY_400, GREY_500, GREY_600, GREY_700, ICON_COLOR, LINK_ACTIVE_COLOR,
    LINK_COLD_COLOR, LINK_HOT_COLOR, MENU_BUTTON_BG_ACTIVE, MENU_BUTTON_BG_INACTIVE,
    MENU_BUTTON_FG_ACTIVE, MENU_BUTTON_FG_INACTIVE, PLACEHOLDER_COLOR, PRIMARY_DARK, PRIMARY_LIGHT,
    RED, SCROLLBAR_BORDER_COLOR, SCROLLBAR_COLOR, SELECTED_TEXT_BACKGROUND_COLOR,
    SELECTION_TEXT_COLOR, TEXT_COLOR, TEXT_SIZE_SMALL, UI_FONT_MEDIUM, UI_FONT_MONO,
    WINDOW_BACKGROUND_COLOR,
};
use druid::{
    theme::{
        BASIC_WIDGET_HEIGHT, BORDERED_WIDGET_HEIGHT, BUTTON_BORDER_RADIUS, BUTTON_BORDER_WIDTH,
        PROGRESS_BAR_RADIUS, SCROLLBAR_EDGE_WIDTH, SCROLLBAR_FADE_DELAY, SCROLLBAR_MAX_OPACITY,
        SCROLLBAR_PAD, SCROLLBAR_RADIUS, SCROLLBAR_WIDTH, TEXTBOX_BORDER_RADIUS,
        TEXTBOX_BORDER_WIDTH, TEXTBOX_INSETS, TEXT_SIZE_LARGE, TEXT_SIZE_NORMAL, UI_FONT,
        WIDE_WIDGET_WIDTH, WIDGET_CONTROL_COMPONENT_PADDING, WIDGET_PADDING_HORIZONTAL,
        WIDGET_PADDING_VERTICAL,
    },
    Color, Env, FontDescriptor, FontFamily, FontWeight, Insets,
};
use serde::Deserialize;

macro_rules! define_theme_definition {
    (indirect_mappings: [$($indirect_mappings:tt),+], direct_mappings: [$($direct_mappings:tt),+]) => {

        #[derive(Deserialize)]
        pub struct ThemeDefinition {
            $(
                $indirect_mappings: String,
            )+
            $(
                $direct_mappings: String,
            )+
        }
        impl ThemeDefinition {
            fn get_referenced_color<S: ToString>(&self, key: &S) -> Color {
                match key.to_string().as_str() {
                    $(
                        stringify!($indirect_mappings) => self.get_color(&self.$indirect_mappings),
                     )+
                    _ => panic!(),
                }
            }

            fn apply_direct_mappings(&self, env: &mut Env) {
                $(
                    env.set($direct_mappings, self.get_color(&self.$direct_mappings));
                )+
            }
        }
    };
}
define_theme_definition!(
    indirect_mappings: [GREY_000, GREY_100, GREY_200, GREY_300, GREY_400,
    GREY_500, GREY_600, GREY_700, BLUE_100, BLUE_200, RED, LINK_HOT_COLOR,
    LINK_ACTIVE_COLOR, LINK_COLD_COLOR],
    direct_mappings:[WINDOW_BACKGROUND_COLOR, TEXT_COLOR, ICON_COLOR,
    PLACEHOLDER_COLOR, PRIMARY_LIGHT, PRIMARY_DARK, BACKGROUND_LIGHT,
    BACKGROUND_DARK, FOREGROUND_LIGHT, FOREGROUND_DARK, BUTTON_LIGHT,
    BUTTON_DARK, BORDER_LIGHT, BORDER_DARK, SELECTION_TEXT_COLOR,
    SELECTED_TEXT_BACKGROUND_COLOR, CURSOR_COLOR, SCROLLBAR_COLOR,
    SCROLLBAR_BORDER_COLOR, MENU_BUTTON_BG_ACTIVE, MENU_BUTTON_BG_INACTIVE,
    MENU_BUTTON_FG_ACTIVE, MENU_BUTTON_FG_INACTIVE]
);

impl ThemeDefinition {
    fn get_color(&self, colstr: &String) -> Color {
        match colstr.split("$").nth(1) {
            Some(referenced_key) => self.get_referenced_color(&referenced_key),
            None => Color::from_hex_str(&colstr.replace("0x", "#")).unwrap(),
        }
    }
    pub fn load_theme(theme_name: String) -> Self {
        serde_json::from_reader::<File, Self>(
            File::open(format!("/home/c/.config/Psst/{theme_name}.json")).unwrap(),
        )
        .unwrap()
    }
    pub fn apply_theme(&self, env: &mut Env) {
        self.apply_direct_mappings(env);
        env.set(PROGRESS_BAR_RADIUS, 4.0);
        env.set(BUTTON_BORDER_RADIUS, 4.0);
        env.set(BUTTON_BORDER_WIDTH, 1.0);

        env.set(
            UI_FONT,
            FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(13.0),
        );
        env.set(
            UI_FONT_MEDIUM,
            FontDescriptor::new(FontFamily::SYSTEM_UI)
                .with_size(13.0)
                .with_weight(FontWeight::MEDIUM),
        );
        env.set(
            UI_FONT_MONO,
            FontDescriptor::new(FontFamily::MONOSPACE).with_size(13.0),
        );
        env.set(TEXT_SIZE_SMALL, 11.0);
        env.set(TEXT_SIZE_NORMAL, 13.0);
        env.set(TEXT_SIZE_LARGE, 16.0);

        env.set(BASIC_WIDGET_HEIGHT, 16.0);
        env.set(WIDE_WIDGET_WIDTH, grid(12.0));
        env.set(BORDERED_WIDGET_HEIGHT, grid(4.0));

        env.set(TEXTBOX_BORDER_RADIUS, 4.0);
        env.set(TEXTBOX_BORDER_WIDTH, 1.0);
        env.set(TEXTBOX_INSETS, Insets::uniform_xy(grid(1.2), grid(1.0)));

        env.set(SCROLLBAR_MAX_OPACITY, 0.8);
        env.set(SCROLLBAR_FADE_DELAY, 1500u64);
        env.set(SCROLLBAR_WIDTH, 6.0);
        env.set(SCROLLBAR_PAD, 2.0);
        env.set(SCROLLBAR_RADIUS, 5.0);
        env.set(SCROLLBAR_EDGE_WIDTH, 1.0);

        env.set(WIDGET_PADDING_VERTICAL, grid(0.5));
        env.set(WIDGET_PADDING_HORIZONTAL, grid(1.0));
        env.set(WIDGET_CONTROL_COMPONENT_PADDING, grid(1.0));
    }
}
