use crate::app::App;
use crate::ui::theme::SULTANA_THEME;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn render_login(frame: &mut Frame, area: Rect, app: &App) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(area);

    let banner = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(
                "SISTEMA DE CONTROL EDITORIAL - ACCESO SEGURO",
                Style::default()
                    .fg(SULTANA_THEME.accent)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Autenticación centralizada por credenciales de área",
                Style::default().fg(SULTANA_THEME.text_dim),
            ),
        ]),
    ])
    .alignment(Alignment::Center);
    frame.render_widget(banner, main_layout[0]);

    let center_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(main_layout[1]);

    let box_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Length(10),
        ])
        .split(center_layout[1]);

    let input_block = Block::default()
        .title(" Iniciar Sesión con Credenciales ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(SULTANA_THEME.primary));

    let username_cursor = if app.active_field == 0 { "_" } else { "" };
    let password_masked = "*".repeat(app.input_2.len());
    let password_cursor = if app.active_field == 1 { "_" } else { "" };

    let login_form = vec![
        Line::from(vec![
            Span::styled("  Usuario LDAP:   ", Style::default().fg(SULTANA_THEME.text_dim)),
            Span::styled(
                format!("{}{}", app.input_1, username_cursor),
                if app.active_field == 0 {
                    Style::default().fg(SULTANA_THEME.accent).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(SULTANA_THEME.text_bright)
                },
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Contraseña:     ", Style::default().fg(SULTANA_THEME.text_dim)),
            Span::styled(
                format!("{}{}", password_masked, password_cursor),
                if app.active_field == 1 {
                    Style::default().fg(SULTANA_THEME.accent).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(SULTANA_THEME.text_bright)
                },
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [Tab] ", Style::default().fg(SULTANA_THEME.accent)),
            Span::styled("Cambiar campo   ", Style::default().fg(SULTANA_THEME.text_dim)),
            Span::styled("[Enter] ", Style::default().fg(SULTANA_THEME.accent)),
            Span::styled("Ingresar con estas credenciales", Style::default().fg(SULTANA_THEME.text_bright)),
        ]),
    ];

    let form_paragraph = Paragraph::new(login_form).block(input_block);
    frame.render_widget(form_paragraph, box_chunks[0]);

    let quick_block = Block::default()
        .title(" O seleccionar perfil de prueba rápido (Use [1] al [4]) ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(SULTANA_THEME.border));

    let quick_items = vec![
        ListItem::new(Line::from(vec![
            Span::styled(" [1] ", Style::default().fg(SULTANA_THEME.accent).add_modifier(Modifier::BOLD)),
            Span::styled("atencion.agente", Style::default().fg(SULTANA_THEME.text_bright).add_modifier(Modifier::BOLD)),
            Span::styled(" -> Área de Atención al Cliente (VLAN 210)", Style::default().fg(SULTANA_THEME.text_dim)),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled(" [2] ", Style::default().fg(SULTANA_THEME.accent).add_modifier(Modifier::BOLD)),
            Span::styled("prod.editor", Style::default().fg(SULTANA_THEME.text_bright).add_modifier(Modifier::BOLD)),
            Span::styled(" -> Área de Producción Editorial / Contenido (VLAN 220)", Style::default().fg(SULTANA_THEME.text_dim)),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled(" [3] ", Style::default().fg(SULTANA_THEME.accent).add_modifier(Modifier::BOLD)),
            Span::styled("rrhh.gerente", Style::default().fg(SULTANA_THEME.text_bright).add_modifier(Modifier::BOLD)),
            Span::styled(" -> Área de Recursos Humanos (VLAN 240)", Style::default().fg(SULTANA_THEME.text_dim)),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled(" [4] ", Style::default().fg(SULTANA_THEME.accent).add_modifier(Modifier::BOLD)),
            Span::styled("recluta.especialista", Style::default().fg(SULTANA_THEME.text_bright).add_modifier(Modifier::BOLD)),
            Span::styled(" -> Área de Reclutamiento (VLAN 230)", Style::default().fg(SULTANA_THEME.text_dim)),
        ])),
    ];

    let quick_list = List::new(quick_items).block(quick_block);
    frame.render_widget(quick_list, box_chunks[1]);

    let security_note = Paragraph::new(Line::from(vec![
        Span::styled("Seguridad Defensiva: ", Style::default().fg(SULTANA_THEME.accent)),
        Span::styled(
            "Las credenciales y tokens se gestionan estrictamente en memoria volátil (Zeroize RAM).",
            Style::default().fg(Color::DarkGray),
        ),
    ]))
    .alignment(Alignment::Center);
    frame.render_widget(security_note, main_layout[2]);
}
