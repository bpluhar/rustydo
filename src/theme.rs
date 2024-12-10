use cursive::theme::{BaseColor, BorderStyle, Color, PaletteColor, Theme};

pub fn set_theme() -> Theme {
    let mut theme = Theme::default();
    theme.shadow = false;
    theme.borders = BorderStyle::Simple;

    let border_color = Color::Light(BaseColor::Red);

    theme.palette[PaletteColor::Primary] = border_color;
    theme.palette[PaletteColor::Secondary] = border_color;

    theme.palette[PaletteColor::Background] = Color::TerminalDefault;
    theme.palette[PaletteColor::Tertiary] = Color::Light(BaseColor::Red);
    theme.palette[PaletteColor::TitlePrimary] = Color::Light(BaseColor::Red);
    theme.palette[PaletteColor::TitleSecondary] = Color::Dark(BaseColor::Magenta);
    theme.palette[PaletteColor::Highlight] = Color::Dark(BaseColor::Red);
    theme.palette[PaletteColor::HighlightText] = Color::Light(BaseColor::Black);
    theme.palette[PaletteColor::View] = Color::TerminalDefault;

    theme
}
