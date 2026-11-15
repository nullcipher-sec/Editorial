use ratatui::style::Color;

pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub background: Color,
    pub text_bright: Color,
    pub text_dim: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub border: Color,
    pub selected_bg: Color,
    pub selected_fg: Color,
}

pub const SULTANA_THEME: Theme = Theme {
    primary: Color::Rgb(15, 76, 129),
    secondary: Color::Rgb(30, 41, 59),
    accent: Color::Rgb(212, 175, 55),
    background: Color::Rgb(10, 15, 29),
    text_bright: Color::Rgb(248, 250, 252),
    text_dim: Color::Rgb(148, 163, 184),
    success: Color::Rgb(34, 197, 94),
    warning: Color::Rgb(234, 179, 8),
    danger: Color::Rgb(239, 68, 68),
    border: Color::Rgb(71, 85, 105),
    selected_bg: Color::Rgb(212, 175, 55),
    selected_fg: Color::Rgb(15, 23, 42),
};
