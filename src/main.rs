use iced::{
    Background, Border, Color, Length, Task, Theme,
    alignment::Vertical,
    color,
    theme::{
        self,
        palette::{self, Extended, Pair},
    },
    time::{Duration, Instant},
    widget::{
        self, Container, Row, TextInput, column,
        container::{self},
        space,
    },
};
use std::sync::Arc;

#[derive(Debug)]
struct App {
    theme: usize,
    themes: Vec<Theme>,
    custom_count: u8,
    custom: Option<Theme>,
    custom_input: Option<String>,
    last_change: Instant,
    pending: Option<Pending>,
}

#[derive(Debug, Clone)]
enum AppMessage {
    Select(Theme),
    ResetCustom,
    SaveCustom,
    Print,
    ApplyCustom,
    Tick,
    Action {
        value: String,
        usage: Usage,
        variant: Variant,
        text: bool,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Pending {
    usage: Usage,
    variant: Variant,
    text: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Variant {
    Base,
    Weak,
    Weaker,
    Weakest,
    Strong,
    Stronger,
    Strongest,
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
    pub fn boot() -> (Self, Task<AppMessage>) {
        (Self::new(), Task::none())
    }

    pub fn new() -> Self {
        Self {
            theme: 0,
            themes: all_themes(),
            custom_count: 0,
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
                self.theme = self
                    .themes
                    .iter()
                    .enumerate()
                    .find_map(|(idx, th)| (*th == theme).then_some(idx))
                    .unwrap_or_default();
            }
            AppMessage::ResetCustom => {
                self.pending = None;
                self.custom_input = None;
                self.custom = None;
            }
            AppMessage::SaveCustom => {
                self.apply_custom();

                if let Some(custom) = self.custom.take() {
                    self.pending = None;
                    self.custom_input = None;
                    self.custom_count += 1;

                    self.theme = self.themes.len();
                    self.themes.push(custom);
                }
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
            AppMessage::Print => {
                let current = theme(&self);

                println!("{:#?}", current.extended_palette());
            }
            AppMessage::Action {
                value,
                usage,
                variant,
                text,
            } => {
                self.last_change = Instant::now();

                self.custom_input = Some(value);

                self.pending = Some(Pending {
                    usage,
                    variant,
                    text,
                });
            }
        }

        Task::none()
    }

    fn apply_custom(&mut self) {
        let Some(input) = self.custom_input.take() else {
            return;
        };

        let Some(Pending {
            usage,
            variant,
            text,
        }) = self.pending.take()
        else {
            return;
        };
        let default_ext = self.themes.get(self.theme).unwrap().extended_palette();

        let ext = *self
            .custom
            .as_ref()
            .map_or(default_ext, |theme| theme.extended_palette());

        let pair = match (usage, variant) {
            (Usage::Background, Variant::Base) => ext.background.base,
            (Usage::Background, Variant::Weak) => ext.background.weak,
            (Usage::Background, Variant::Weaker) => ext.background.weaker,
            (Usage::Background, Variant::Weakest) => ext.background.weakest,
            (Usage::Background, Variant::Strong) => ext.background.strong,
            (Usage::Background, Variant::Stronger) => ext.background.stronger,
            (Usage::Background, Variant::Strongest) => ext.background.strongest,

            (Usage::Primary, Variant::Base) => ext.primary.base,
            (Usage::Primary, Variant::Weak) => ext.primary.weak,
            (Usage::Primary, Variant::Strong) => ext.primary.strong,

            (Usage::Secondary, Variant::Base) => ext.secondary.base,
            (Usage::Secondary, Variant::Weak) => ext.secondary.weak,
            (Usage::Secondary, Variant::Strong) => ext.secondary.strong,

            (Usage::Success, Variant::Base) => ext.success.base,
            (Usage::Success, Variant::Weak) => ext.success.weak,
            (Usage::Success, Variant::Strong) => ext.success.strong,

            (Usage::Danger, Variant::Base) => ext.danger.base,
            (Usage::Danger, Variant::Weak) => ext.danger.weak,
            (Usage::Danger, Variant::Strong) => ext.danger.strong,

            _ => unreachable!(),
        };

        let Some(pair) = convert_color_str(&input, pair, text) else {
            self.custom_input = Some(input);
            return;
        };

        let ext = updated_extended(ext, pair, usage, variant);

        let custom = theme::Custom::with_fn(
            format!("Custom {}", self.custom_count),
            theme::Palette::DARK,
            move |_| ext,
        );

        let custom = Theme::Custom(Arc::new(custom));

        self.custom = Some(custom);
    }

    fn subscription(&self) -> iced::Subscription<AppMessage> {
        iced::time::every(Duration::from_secs(1)).map(|_| AppMessage::Tick)
    }

    pub fn view(&self) -> iced::Element<'_, AppMessage> {
        let theme = self
            .custom
            .as_ref()
            .unwrap_or(self.themes.get(self.theme).unwrap());

        let header = widget::text("Themes")
            .size(32.0)
            .center()
            .width(Length::Fill);

        let theme_selector = widget::container(widget::pick_list(
            self.themes.as_slice(),
            self.themes.get(self.theme),
            AppMessage::Select,
        ))
        .center_x(Length::Fill);

        let spacing = 16.0;

        let labels = widget::row!(
            space::horizontal().width(175),
            my_text("Base"),
            space::horizontal().width(150),
            my_text("Weak"),
            space::horizontal().width(140),
            my_text("Strong"),
            space::horizontal().width(135),
            my_text("Weaker"),
            space::horizontal().width(125),
            my_text("Weakest"),
            space::horizontal().width(125),
            my_text("Stronger"),
            space::horizontal().width(125),
            my_text("Strongest"),
            space::horizontal().width(150),
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
        .spacing(24.0);

        let content = widget::column!(labels, colors);

        let reset = widget::button("Reset Custom")
            .on_press_maybe(self.custom.is_some().then_some(AppMessage::ResetCustom));

        let save = widget::button("Save custom")
            .on_press_maybe(self.custom.is_some().then_some(AppMessage::SaveCustom));

        let print = widget::button("Print").on_press(AppMessage::Print);

        let actions = widget::row!(space::horizontal(), reset, save, print, space::horizontal())
            .spacing(40)
            .align_y(iced::alignment::Vertical::Center);

        widget::scrollable(
            widget::column![
                header,
                theme_selector,
                content,
                space::vertical().height(25.0),
                actions
            ]
            .spacing(spacing)
            .padding(16.0),
        )
        .into()
    }

    fn helper(
        &self,
        usage: Usage,
        variant: Variant,
        theme: &Theme,
    ) -> widget::Column<'_, AppMessage> {
        let (color, text_color) = theme_str(theme, usage, variant);

        let (color, text) = match self.pending {
            Some(Pending {
                usage: pending_usage,
                variant: pending_variant,
                text,
            }) if usage == pending_usage && variant == pending_variant => {
                if text {
                    (&color, self.custom_input.as_ref().unwrap_or(&text_color))
                } else {
                    (self.custom_input.as_ref().unwrap_or(&color), &text_color)
                }
            }
            _ => (&color, &text_color),
        };

        let color = widget::text_input("rgb or hex", &color);
        let color = text_input(color, usage, variant, false);

        let text = widget::text_input("rgb or hex", &text);
        let text = text_input(text, usage, variant, true);

        column!(color, text).spacing(16.0)
    }

    fn background(&self, theme: &Theme) -> Row<'_, AppMessage> {
        let usage = Usage::Background;

        let base = {
            let variant = Variant::Base;

            self.helper(usage, variant, theme)
        };

        let weak = {
            let variant = Variant::Weak;

            self.helper(usage, variant, theme)
        };

        let weaker = {
            let variant = Variant::Weaker;

            self.helper(usage, variant, theme)
        };

        let weakest = {
            let variant = Variant::Weakest;

            self.helper(usage, variant, theme)
        };

        let strong = {
            let variant = Variant::Strong;

            self.helper(usage, variant, theme)
        };

        let stronger = {
            let variant = Variant::Stronger;

            self.helper(usage, variant, theme)
        };

        let strongest = {
            let variant = Variant::Strongest;

            self.helper(usage, variant, theme)
        };

        widget::row!(
            my_text("Background").width(150),
            base,
            weak,
            strong,
            weaker,
            weakest,
            stronger,
            strongest,
        )
    }

    fn primary(&self, theme: &Theme) -> Row<'_, AppMessage> {
        let usage = Usage::Primary;

        let base = {
            let variant = Variant::Base;

            self.helper(usage, variant, theme)
        };

        let weak = {
            let variant = Variant::Weak;

            self.helper(usage, variant, theme)
        };

        let strong = {
            let variant = Variant::Strong;

            self.helper(usage, variant, theme)
        };

        widget::row!(my_text("Primary").width(150), base, weak, strong)
    }

    fn secondary(&self, theme: &Theme) -> Row<'_, AppMessage> {
        let usage = Usage::Secondary;

        let base = {
            let variant = Variant::Base;

            self.helper(usage, variant, theme)
        };

        let weak = {
            let variant = Variant::Weak;

            self.helper(usage, variant, theme)
        };

        let strong = {
            let variant = Variant::Strong;

            self.helper(usage, variant, theme)
        };

        widget::row!(my_text("Secondary").width(150), base, weak, strong)
    }

    fn success(&self, theme: &Theme) -> Row<'_, AppMessage> {
        let usage = Usage::Success;

        let base = {
            let variant = Variant::Base;

            self.helper(usage, variant, theme)
        };

        let weak = {
            let variant = Variant::Weak;

            self.helper(usage, variant, theme)
        };

        let strong = {
            let variant = Variant::Strong;

            self.helper(usage, variant, theme)
        };

        widget::row!(my_text("Success").width(150), base, weak, strong)
    }

    fn danger(&self, theme: &Theme) -> Row<'_, AppMessage> {
        let usage = Usage::Danger;

        let base = {
            let variant = Variant::Base;

            self.helper(usage, variant, theme)
        };

        let weak = {
            let variant = Variant::Weak;

            self.helper(usage, variant, theme)
        };

        let strong = {
            let variant = Variant::Strong;

            self.helper(usage, variant, theme)
        };

        widget::row!(my_text("Danger").width(150), base, weak, strong)
    }
}

fn main() -> iced::Result {
    iced::application(App::boot, App::update, App::view)
        .title("Theme Viewer")
        .antialiasing(true)
        .window_size((1500.0, 900.0))
        .theme(theme)
        .subscription(App::subscription)
        .run()
}

fn theme(app: &App) -> Theme {
    app.custom
        .clone()
        .unwrap_or_else(|| app.themes.get(app.theme).cloned().unwrap())
}

fn text_input(
    input: TextInput<'_, AppMessage>,
    usage: Usage,
    variant: Variant,
    text: bool,
) -> widget::TextInput<'_, AppMessage> {
    input
        .on_input(move |value| AppMessage::Action {
            value,
            usage,
            variant,
            text,
        })
        .on_submit(AppMessage::ApplyCustom)
        .width(168.0)
        .padding([10, 8])
        .style(move |theme, status| text_input_style(theme, status, usage, variant))
}

fn convert_color_str(input: &str, prev: Pair, text_color: bool) -> Option<Pair> {
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
        let [r, g, b, a] = value.to_be_bytes();
        Color::from_rgba8(r, g, b, a as f32)
    } else {
        let hex = u32::from_str_radix(input.trim(), 16).ok()?;
        let [r, g, b, a] = hex.to_be_bytes();
        Color::from_rgba8(r, g, b, a as f32)
    };

    let brightness = ((299.0 * color.r) + (587.0 * color.g) + (144.0 * color.b)) / 1000.0;

    let text = if brightness >= 128.0 {
        color!(10, 10, 10)
    } else {
        color!(235, 235, 235)
    };

    if text_color {
        Some(Pair {
            color: prev.color,
            text: color,
        })
    } else {
        Some(Pair::new(color, text))
    }
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
                _ => unreachable!(),
            }
        }
        Usage::Secondary => {
            let secondary = palette.secondary;
            match variant {
                Variant::Base => secondary.base,
                Variant::Weak => secondary.weak,
                Variant::Strong => secondary.strong,
                _ => unreachable!(),
            }
        }
        Usage::Background => {
            let background = palette.background;
            match variant {
                Variant::Base => background.base,
                Variant::Weak => background.weak,
                Variant::Weaker => background.weaker,
                Variant::Weakest => background.weakest,
                Variant::Strong => background.strong,
                Variant::Stronger => background.stronger,
                Variant::Strongest => background.strongest,
            }
        }
        Usage::Danger => {
            let danger = palette.danger;
            match variant {
                Variant::Base => danger.base,
                Variant::Weak => danger.weak,
                Variant::Strong => danger.strong,
                _ => unreachable!(),
            }
        }
        Usage::Success => {
            let success = palette.success;
            match variant {
                Variant::Base => success.base,
                Variant::Weak => success.weak,
                Variant::Strong => success.strong,
                _ => unreachable!(),
            }
        }
    }
}

fn theme_str(theme: &Theme, usage: Usage, variant: Variant) -> (String, String) {
    let pair = get_pair(theme, usage, variant);
    let color = pair.color;
    let text = pair.text;

    (
        format!(
            "rgb({:.0}, {:.0}, {:.0})",
            color.r * 255.0,
            color.g * 255.0,
            color.b * 255.0
        ),
        format!(
            "rgb({:.0}, {:.0}, {:.0})",
            text.r * 255.0,
            text.g * 255.0,
            text.b * 255.0
        ),
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
                _ => unreachable!(),
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
                _ => unreachable!(),
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
                Variant::Weaker => Background {
                    weaker: pair,
                    ..extended.background
                },
                Variant::Weakest => Background {
                    weakest: pair,
                    ..extended.background
                },
                Variant::Strong => Background {
                    strong: pair,
                    ..extended.background
                },
                Variant::Stronger => Background {
                    stronger: pair,
                    ..extended.background
                },
                Variant::Strongest => Background {
                    strongest: pair,
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
                _ => unreachable!(),
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
                _ => unreachable!(),
            };

            Extended { danger, ..extended }
        }
    }
}
