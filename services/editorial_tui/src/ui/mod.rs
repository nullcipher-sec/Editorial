pub mod components;
pub mod theme;
pub mod views;

use crate::app::{App, CurrentScreen};
use components::{render_footer, render_header, render_popup};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};
use views::*;

pub fn render_app(frame: &mut Frame, app: &App) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(10),
            Constraint::Length(2),
        ])
        .split(frame.area());

    render_header(frame, main_layout[0], app.session.as_ref(), app.config.mock_mode);

    match app.current_screen {
        CurrentScreen::Login => render_login(frame, main_layout[1], app),
        CurrentScreen::MainMenu => render_menu(frame, main_layout[1], app),
        CurrentScreen::Attention => render_attention(frame, main_layout[1], app),
        CurrentScreen::Content => render_content(frame, main_layout[1], app),
        CurrentScreen::Hr => render_hr(frame, main_layout[1], app),
        CurrentScreen::Events => render_events(frame, main_layout[1], app),
    }

    let hint = match app.current_screen {
        CurrentScreen::Login => "Login: Use [1-4] para perfiles rápidos o escriba credenciales",
        CurrentScreen::MainMenu => "Menú: [↑/↓] Navegar  [Enter] Acceder  [5/Esc] Cerrar Sesión",
        CurrentScreen::Attention => "Atención: [S] Avanzar Estado  [C] Clientes  [N] Nuevo Caso  [Esc] Volver",
        CurrentScreen::Content => "Contenido: [Enter] Leer  [N] Redactar  [R] Revisión  [P] Publicar",
        CurrentScreen::Hr => "RRHH: [V] Vacantes  [C] Candidatos  [E] Empleados  [Esc] Volver",
        CurrentScreen::Events => "Eventos: [N] Programar  [↑/↓] Navegar  [Esc] Volver",
    };
    render_footer(frame, main_layout[2], hint);

    if let Some((kind, message)) = &app.popup {
        render_popup(frame, *kind, message);
    }
}
