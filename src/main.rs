use iced::{
    Border, Color, Element, Length, Theme,
    overlay::menu::Catalog,
    theme::palette::Pair,
    widget::{
        self, Container,
        container::{self, rounded_box},
    },
};

#[derive(Debug, Default)]
struct App {
    theme: Theme,
    themes: Vec<Theme>,
}

#[derive(Debug, Clone)]
enum AppMessage {
    Select(Theme),
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

fn square<'a>(text: &'a str, colors: Pair) -> Container<'a, AppMessage> {
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
        }
    }
    pub fn update(&mut self, message: AppMessage) -> iced::Task<AppMessage> {
        match message {
            AppMessage::Select(theme) => self.theme = theme,
        }
        iced::Task::none()
    }

    pub fn view(&self) -> iced::Element<'_, AppMessage> {
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
        let background_row = widget::row![
            my_text("Background").width(Length::Fill),
            square("Base", self.theme.extended_palette().background.base),
            square("Weak", self.theme.extended_palette().background.weak),
            square("Strong", self.theme.extended_palette().background.strong),
        ]
        .spacing(16.0);
        let primary_row = widget::row![
            my_text("Primary").width(Length::Fill),
            square("Base", self.theme.extended_palette().primary.base),
            square("Weak", self.theme.extended_palette().primary.weak),
            square("Strong", self.theme.extended_palette().primary.strong),
        ]
        .spacing(16.0);
        let secondary_row = widget::row![
            my_text("Secondary").width(Length::Fill),
            square("Base", self.theme.extended_palette().secondary.base),
            square("Weak", self.theme.extended_palette().secondary.weak),
            square("Strong", self.theme.extended_palette().secondary.strong),
        ]
        .spacing(16.0);
        let success_row = widget::row![
            my_text("Success").width(Length::Fill),
            square("Base", self.theme.extended_palette().success.base),
            square("Weak", self.theme.extended_palette().success.weak),
            square("Strong", self.theme.extended_palette().success.strong),
        ]
        .spacing(16.0);
        let danger_row = widget::row![
            my_text("Danger").width(Length::Fill),
            square("Base", self.theme.extended_palette().danger.base),
            square("Weak", self.theme.extended_palette().danger.weak),
            square("Strong", self.theme.extended_palette().danger.strong),
        ]
        .spacing(16.0);
        widget::column![
            header,
            theme_selector,
            background_row,
            primary_row,
            secondary_row,
            success_row,
            danger_row
        ]
        .spacing(16.0)
        .padding(16.0)
        .into()
    }
}

fn main() -> iced::Result {
    iced::application("App", App::update, App::view)
        .antialiasing(true)
        .window_size((400.0, 520.0))
        .resizable(false)
        .theme(theme)
        .run_with(|| {
            let state = App::new();
            (state, iced::Task::none())
        })
}

fn theme(app: &App) -> Theme {
    app.theme.clone()
}
