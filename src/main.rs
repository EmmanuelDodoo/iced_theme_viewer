use iced::{
    Background, Border, Length, Task, Theme,
    alignment::{Horizontal, Vertical},
    color,
    theme::{
        self,
        palette::{self, Extended, Pair},
    },
    time::{Duration, Instant},
    widget::{
        self, Container, Row, TextInput,
        container::{self},
    },
};
use std::sync::Arc;

#[derive(Debug)]
struct App {
    theme: Theme,
    themes: Vec<Theme>,
    custom: Option<Theme>,
    custom_input: Option<String>,
    last_change: Instant,
    pending: Option<Pending>,
}

#[derive(Debug, Clone)]
enum AppMessage {
    Select(Theme),
    ResetCustom,
    ApplyCustom,
    Tick,
    Action(String, Usage, Variant),
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Pending {
    usage: Usage,
    variant: Variant,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Variant {
    Base,
    Weak,
    Strong,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Usage {
    Primary,
    Secondary,
    Background,
    Success,
    Danger,
}

fn all_themes() -> Vec<Theme> {
    vec![
        Theme::Light,
        Theme::Dark,
        Theme::Dracula,
        Theme::Nord,
        Theme::SolarizedLight,
        Theme::SolarizedDark,
        Theme::GruvboxLight,
        Theme::GruvboxDark,
        Theme::CatppuccinLatte,
        Theme::CatppuccinFrappe,
        Theme::CatppuccinMacchiato,
        Theme::CatppuccinMocha,
        Theme::TokyoNight,
        Theme::TokyoNightStorm,
        Theme::TokyoNightLight,
        Theme::KanagawaWave,
        Theme::KanagawaDragon,
        Theme::KanagawaLotus,
        Theme::Moonfly,
        Theme::Nightfly,
        Theme::Oxocarbon,
        Theme::Ferra,
    ]
}

fn _square<'a>(text: &'a str, colors: Pair) -> Container<'a, AppMessage> {
    let size = 64.0;
    widget::container(text)
        .width(size)
        .height(size)
        .center(64.0)
        .style(move |_| {
            container::Style::default()
                .background(colors.color)
                .color(colors.text)
                .border(Border::default().rounded(10.0))
        })
}

fn my_text<'a, M>(text: &'a str) -> Container<'a, M> {
    widget::container(widget::text(text)).center_y(64.0)
}

impl App {
    pub fn new() -> Self {
        Self {
            theme: Theme::Light,
            themes: all_themes(),
            custom: None,
            custom_input: None,
            pending: None,
            last_change: Instant::now(),
        }
    }

    pub fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            AppMessage::Select(theme) => {
                self.pending = None;
                self.custom_input = None;
                self.custom = None;
                self.theme = theme;
            }
            AppMessage::ResetCustom => {
                self.pending = None;
                self.custom_input = None;
                self.custom = None;
            }
            AppMessage::Tick => {
                if self.last_change.elapsed() >= Duration::from_millis(750) {
                    return Task::done(AppMessage::ApplyCustom);
                }
            }
            AppMessage::ApplyCustom => {
                if self.pending.is_some() {
                    self.apply_custom()
                }
            }
            AppMessage::Action(input, usage, variant) => {
                self.last_change = Instant::now();

                self.custom_input = Some(input);

                self.pending = Some(Pending { usage, variant });
            }
        }

        Task::none()
    }

    fn apply_custom(&mut self) {
        let Some(input) = self.custom_input.take() else {
            return;
        };

        let Some(pair) = convert_color_str(&input) else {
            self.custom_input = Some(input);
            return;
        };

        let Some(Pending { usage, variant }) = self.pending.take() else {
            return;
        };

        let ext = *self.custom.as_ref().map_or_else(
            || self.theme.extended_palette(),
            |theme| theme.extended_palette(),
        );

        let ext = updated_extended(ext, pair, usage, variant);

        let custom =
            theme::Custom::with_fn("Custom".to_owned(), theme::Palette::DARK, move |_| ext);

        let custom = Theme::Custom(Arc::new(custom));

        self.custom = Some(custom);
    }

    fn subscription(&self) -> iced::Subscription<AppMessage> {
        iced::time::every(Duration::from_secs(1)).map(|_| AppMessage::Tick)
    }

    pub fn view(&self) -> iced::Element<'_, AppMessage> {
        let theme = self.custom.as_ref().unwrap_or(&self.theme);

        let header = widget::text("Default Themes")
            .size(32.0)
            .center()
            .width(Length::Fill);

        let theme_selector = widget::container(widget::pick_list(
            self.themes.clone(),
            Some(self.theme.clone()),
            AppMessage::Select,
        ))
        .center_x(Length::Fill);

        let spacing = 16.0;

        let labels = widget::row!(
            widget::horizontal_space().width(155),
            my_text("Base"),
            widget::horizontal_space().width(145),
            my_text("Weak"),
            widget::horizontal_space().width(135),
            my_text("Strong"),
            widget::horizontal_space().width(155),
        )
        .align_y(Vertical::Center)
        .spacing(0);

        let background_row = self
            .background(theme)
            .spacing(spacing)
            .align_y(Vertical::Center);

        let primary_row = self
            .primary(theme)
            .spacing(spacing)
            .align_y(Vertical::Center);

        let secondary_row = self
            .secondary(theme)
            .spacing(spacing)
            .align_y(Vertical::Center);

        let success_row = self
            .success(theme)
            .spacing(spacing)
            .align_y(Vertical::Center);

        let danger_row = self
            .danger(theme)
            .spacing(spacing)
            .align_y(Vertical::Center);

        let colors = widget::column!(
            background_row,
            primary_row,
            secondary_row,
            success_row,
            danger_row
        )
        .spacing(spacing);

        let content = widget::column!(labels, colors);

        let btn_message = if self.custom.is_some() {
            Some(AppMessage::ResetCustom)
        } else {
            None
        };
        let reset = widget::button("Reset Custom").on_press_maybe(btn_message);

        widget::column![
            header,
            theme_selector,
            content,
            widget::vertical_space().height(25.0),
            reset
        ]
        .align_x(Horizontal::Center)
        .spacing(spacing)
        .padding(16.0)
        .into()
    }

    fn background(&self, theme: &Theme) -> Row<'_, AppMessage> {
        let usage = Usage::Background;

        let base = {
            let variant = Variant::Base;
            let value = match self.pending {
                Some(Pending {
                    usage: pending_usage,
                    variant: pending_variant,
                }) if usage == pending_usage && variant == pending_variant => self
                    .custom_input
                    .clone()
                    .unwrap_or_else(|| theme_str(theme, usage, variant)),
                _ => theme_str(theme, usage, variant),
            };

            let content = widget::text_input("rgb or hex", &value);

            text_input(content, usage, variant)
        };

        let weak = {
            let variant = Variant::Weak;
            let value = match self.pending {
                Some(Pending {
                    usage: pending_usage,
                    variant: pending_variant,
                }) if usage == pending_usage && variant == pending_variant => self
                    .custom_input
                    .clone()
                    .unwrap_or_else(|| theme_str(theme, usage, variant)),
                _ => theme_str(theme, usage, variant),
            };

            let content = widget::text_input("rgb or hex", &value);

            text_input(content, usage, variant)
        };

        let strong = {
            let variant = Variant::Strong;
            let value = match self.pending {
                Some(Pending {
                    usage: pending_usage,
                    variant: pending_variant,
                }) if usage == pending_usage && variant == pending_variant => self
                    .custom_input
                    .clone()
                    .unwrap_or_else(|| theme_str(theme, usage, variant)),
                _ => theme_str(theme, usage, variant),
            };

            let content = widget::text_input("rgb or hex", &value);

            text_input(content, usage, variant)
        };

        widget::row!(
            my_text("Background").width(Length::Fill),
            base,
            weak,
            strong
        )
    }

    fn primary(&self, theme: &Theme) -> Row<'_, AppMessage> {
        let usage = Usage::Primary;

        let base = {
            let variant = Variant::Base;
            let value = match self.pending {
                Some(Pending {
                    usage: pending_usage,
                    variant: pending_variant,
                }) if usage == pending_usage && variant == pending_variant => self
                    .custom_input
                    .clone()
                    .unwrap_or_else(|| theme_str(theme, usage, variant)),
                _ => theme_str(theme, usage, variant),
            };

            let content = widget::text_input("rgb or hex", &value);

            text_input(content, usage, variant)
        };

        let weak = {
            let variant = Variant::Weak;
            let value = match self.pending {
                Some(Pending {
                    usage: pending_usage,
                    variant: pending_variant,
                }) if usage == pending_usage && variant == pending_variant => self
                    .custom_input
                    .clone()
                    .unwrap_or_else(|| theme_str(theme, usage, variant)),
                _ => theme_str(theme, usage, variant),
            };

            let content = widget::text_input("rgb or hex", &value);

            text_input(content, usage, variant)
        };

        let strong = {
            let variant = Variant::Strong;
            let value = match self.pending {
                Some(Pending {
                    usage: pending_usage,
                    variant: pending_variant,
                }) if usage == pending_usage && variant == pending_variant => self
                    .custom_input
                    .clone()
                    .unwrap_or_else(|| theme_str(theme, usage, variant)),
                _ => theme_str(theme, usage, variant),
            };

            let content = widget::text_input("rgb or hex", &value);

            text_input(content, usage, variant)
        };

        widget::row!(my_text("Primary").width(Length::Fill), base, weak, strong)
    }

    fn secondary(&self, theme: &Theme) -> Row<'_, AppMessage> {
        let usage = Usage::Secondary;

        let base = {
            let variant = Variant::Base;
            let value = match self.pending {
                Some(Pending {
                    usage: pending_usage,
                    variant: pending_variant,
                }) if usage == pending_usage && variant == pending_variant => self
                    .custom_input
                    .clone()
                    .unwrap_or_else(|| theme_str(theme, usage, variant)),
                _ => theme_str(theme, usage, variant),
            };

            let content = widget::text_input("rgb or hex", &value);

            text_input(content, usage, variant)
        };

        let weak = {
            let variant = Variant::Weak;
            let value = match self.pending {
                Some(Pending {
                    usage: pending_usage,
                    variant: pending_variant,
                }) if usage == pending_usage && variant == pending_variant => self
                    .custom_input
                    .clone()
                    .unwrap_or_else(|| theme_str(theme, usage, variant)),
                _ => theme_str(theme, usage, variant),
            };

            let content = widget::text_input("rgb or hex", &value);

            text_input(content, usage, variant)
        };

        let strong = {
            let variant = Variant::Strong;
            let value = match self.pending {
                Some(Pending {
                    usage: pending_usage,
                    variant: pending_variant,
                }) if usage == pending_usage && variant == pending_variant => self
                    .custom_input
                    .clone()
                    .unwrap_or_else(|| theme_str(theme, usage, variant)),
                _ => theme_str(theme, usage, variant),
            };

            let content = widget::text_input("rgb or hex", &value);

            text_input(content, usage, variant)
        };

        widget::row!(my_text("Secondary").width(Length::Fill), base, weak, strong)
    }

    fn success(&self, theme: &Theme) -> Row<'_, AppMessage> {
        let usage = Usage::Success;

        let base = {
            let variant = Variant::Base;
            let value = match self.pending {
                Some(Pending {
                    usage: pending_usage,
                    variant: pending_variant,
                }) if usage == pending_usage && variant == pending_variant => self
                    .custom_input
                    .clone()
                    .unwrap_or_else(|| theme_str(theme, usage, variant)),
                _ => theme_str(theme, usage, variant),
            };

            let content = widget::text_input("rgb or hex", &value);

            text_input(content, usage, variant)
        };

        let weak = {
            let variant = Variant::Weak;
            let value = match self.pending {
                Some(Pending {
                    usage: pending_usage,
                    variant: pending_variant,
                }) if usage == pending_usage && variant == pending_variant => self
                    .custom_input
                    .clone()
                    .unwrap_or_else(|| theme_str(theme, usage, variant)),
                _ => theme_str(theme, usage, variant),
            };

            let content = widget::text_input("rgb or hex", &value);

            text_input(content, usage, variant)
        };

        let strong = {
            let variant = Variant::Strong;
            let value = match self.pending {
                Some(Pending {
                    usage: pending_usage,
                    variant: pending_variant,
                }) if usage == pending_usage && variant == pending_variant => self
                    .custom_input
                    .clone()
                    .unwrap_or_else(|| theme_str(theme, usage, variant)),
                _ => theme_str(theme, usage, variant),
            };

            let content = widget::text_input("rgb or hex", &value);

            text_input(content, usage, variant)
        };

        widget::row!(my_text("Success").width(Length::Fill), base, weak, strong)
    }

    fn danger(&self, theme: &Theme) -> Row<'_, AppMessage> {
        let usage = Usage::Danger;

        let base = {
            let variant = Variant::Base;
            let value = match self.pending {
                Some(Pending {
                    usage: pending_usage,
                    variant: pending_variant,
                }) if usage == pending_usage && variant == pending_variant => self
                    .custom_input
                    .clone()
                    .unwrap_or_else(|| theme_str(theme, usage, variant)),
                _ => theme_str(theme, usage, variant),
            };

            let content = widget::text_input("rgb or hex", &value);

            text_input(content, usage, variant)
        };

        let weak = {
            let variant = Variant::Weak;
            let value = match self.pending {
                Some(Pending {
                    usage: pending_usage,
                    variant: pending_variant,
                }) if usage == pending_usage && variant == pending_variant => self
                    .custom_input
                    .clone()
                    .unwrap_or_else(|| theme_str(theme, usage, variant)),
                _ => theme_str(theme, usage, variant),
            };

            let content = widget::text_input("rgb or hex", &value);

            text_input(content, usage, variant)
        };

        let strong = {
            let variant = Variant::Strong;
            let value = match self.pending {
                Some(Pending {
                    usage: pending_usage,
                    variant: pending_variant,
                }) if usage == pending_usage && variant == pending_variant => self
                    .custom_input
                    .clone()
                    .unwrap_or_else(|| theme_str(theme, usage, variant)),
                _ => theme_str(theme, usage, variant),
            };

            let content = widget::text_input("rgb or hex", &value);

            text_input(content, usage, variant)
        };

        widget::row!(my_text("Danger").width(Length::Fill), base, weak, strong)
    }
}

fn main() -> iced::Result {
    iced::application("App", App::update, App::view)
        .antialiasing(true)
        .window_size((720.0, 720.0))
        .resizable(false)
        .theme(theme)
        .subscription(App::subscription)
        .run_with(|| {
            let state = App::new();
            (state, iced::Task::none())
        })
}

fn theme(app: &App) -> Theme {
    app.custom.clone().unwrap_or_else(|| app.theme.clone())
}

fn text_input(
    input: TextInput<'_, AppMessage>,
    usage: Usage,
    variant: Variant,
) -> widget::TextInput<'_, AppMessage> {
    input
        .on_input(move |input| AppMessage::Action(input, usage, variant))
        .on_submit(AppMessage::ApplyCustom)
        .width(168.0)
        .padding([10, 8])
        .style(move |theme, status| text_input_style(theme, status, usage, variant))
}

fn convert_color_str(input: &str) -> Option<Pair> {
    if input.is_empty() {
        return None;
    }

    let input = input.trim();

    let color = if input.contains(",") {
        let values = input
            .trim_start_matches("rgb(")
            .trim_end_matches(")")
            .split(",")
            .filter_map(|split| split.trim().parse::<u8>().ok())
            .collect::<Vec<u8>>();

        if values.len() != 3 {
            return None;
        }

        color!(values[0], values[1], values[2])
    } else if input.contains("#") {
        let value = u32::from_str_radix(input.trim_start_matches("#"), 16).ok()?;
        color!(value)
    } else {
        let hex = u32::from_str_radix(input.trim(), 16).ok()?;
        color!(hex)
    };

    let brightness = ((299.0 * color.r) + (587.0 * color.g) + (144.0 * color.b)) / 1000.0;

    let text = if brightness >= 128.0 {
        color!(10, 10, 10)
    } else {
        color!(235, 235, 235)
    };

    Some(Pair::new(color, text))
}

fn text_input_style(
    theme: &Theme,
    status: widget::text_input::Status,
    usage: Usage,
    variant: Variant,
) -> widget::text_input::Style {
    use widget::text_input::{Style, default};

    let default = default(theme, status);

    let pair = get_pair(theme, usage, variant);

    Style {
        background: Background::Color(pair.color),
        value: pair.text,
        placeholder: pair.text.scale_alpha(0.25),
        ..default
    }
}

fn get_pair(theme: &Theme, usage: Usage, variant: Variant) -> Pair {
    let palette = theme.extended_palette();
    match usage {
        Usage::Primary => {
            let primary = palette.primary;
            match variant {
                Variant::Base => primary.base,
                Variant::Weak => primary.weak,
                Variant::Strong => primary.strong,
            }
        }
        Usage::Secondary => {
            let secondary = palette.secondary;
            match variant {
                Variant::Base => secondary.base,
                Variant::Weak => secondary.weak,
                Variant::Strong => secondary.strong,
            }
        }
        Usage::Background => {
            let background = palette.background;
            match variant {
                Variant::Base => background.base,
                Variant::Weak => background.weak,
                Variant::Strong => background.strong,
            }
        }
        Usage::Danger => {
            let danger = palette.danger;
            match variant {
                Variant::Base => danger.base,
                Variant::Weak => danger.weak,
                Variant::Strong => danger.strong,
            }
        }
        Usage::Success => {
            let success = palette.success;
            match variant {
                Variant::Base => success.base,
                Variant::Weak => success.weak,
                Variant::Strong => success.strong,
            }
        }
    }
}

fn theme_str(theme: &Theme, usage: Usage, variant: Variant) -> String {
    let color = get_pair(theme, usage, variant).color;

    format!(
        "rgb({:.0}, {:.0}, {:.0})",
        color.r * 255.0,
        color.g * 255.0,
        color.b * 255.0
    )
}

fn updated_extended(extended: Extended, pair: Pair, usage: Usage, variant: Variant) -> Extended {
    use palette::{Background, Danger, Primary, Secondary, Success};

    match usage {
        Usage::Primary => {
            let primary = match variant {
                Variant::Base => Primary {
                    base: pair,
                    ..extended.primary
                },
                Variant::Weak => Primary {
                    weak: pair,
                    ..extended.primary
                },
                Variant::Strong => Primary {
                    strong: pair,
                    ..extended.primary
                },
            };

            Extended {
                primary,
                ..extended
            }
        }

        Usage::Secondary => {
            let secondary = match variant {
                Variant::Base => Secondary {
                    base: pair,
                    ..extended.secondary
                },
                Variant::Weak => Secondary {
                    weak: pair,
                    ..extended.secondary
                },
                Variant::Strong => Secondary {
                    strong: pair,
                    ..extended.secondary
                },
            };

            Extended {
                secondary,
                ..extended
            }
        }

        Usage::Background => {
            let background = match variant {
                Variant::Base => Background {
                    base: pair,
                    ..extended.background
                },
                Variant::Weak => Background {
                    weak: pair,
                    ..extended.background
                },
                Variant::Strong => Background {
                    strong: pair,
                    ..extended.background
                },
            };

            Extended {
                background,
                ..extended
            }
        }

        Usage::Success => {
            let success = match variant {
                Variant::Base => Success {
                    base: pair,
                    ..extended.success
                },
                Variant::Weak => Success {
                    weak: pair,
                    ..extended.success
                },
                Variant::Strong => Success {
                    strong: pair,
                    ..extended.success
                },
            };

            Extended {
                success,
                ..extended
            }
        }

        Usage::Danger => {
            let danger = match variant {
                Variant::Base => Danger {
                    base: pair,
                    ..extended.danger
                },
                Variant::Weak => Danger {
                    weak: pair,
                    ..extended.danger
                },
                Variant::Strong => Danger {
                    strong: pair,
                    ..extended.danger
                },
            };

            Extended { danger, ..extended }
        }
    }
}
