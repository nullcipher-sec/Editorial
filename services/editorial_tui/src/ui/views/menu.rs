use crate::app::App;
use crate::domain::AreaCode;
use crate::ui::theme::SULTANA_THEME;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn render_menu(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(12),
            Constraint::Length(3),
        ])
        .split(area);

    let welcome = if let Some(s) = &app.session {
        format!("Bienvenido, {} | Rol: {}", s.user.display_name, s.user.area.display_name())
    } else {
        "Sesión no iniciada".to_string()
    };

    let title = Paragraph::new(vec![
        Line::from(Span::styled(
            "MENÚ PRINCIPAL - CONTROL DE ACCESO BASADO EN ROLES (RBAC)",
            Style::default().fg(SULTANA_THEME.accent).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(welcome, Style::default().fg(SULTANA_THEME.text_dim))),
    ])
    .alignment(Alignment::Center);
    frame.render_widget(title, chunks[0]);

    let center_col = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(chunks[1])[1];

    let user_area = app.session.as_ref().map(|s| s.user.area);

    let options = [
        ("1. Área de Atención al Cliente", AreaCode::Attention, "Gestión de clientes y casos de soporte (VLAN 210)"),
        ("2. Manejo de Contenido y Editorial", AreaCode::Production, "Redacción y ciclo editorial de artículos (VLAN 220)"),
        ("3. Módulo de Reclutamiento y Selección", AreaCode::Recruitment, "Consulta de vacantes y postulantes (VLAN 230)"),
        ("4. Módulo de Recursos Humanos", AreaCode::Hr, "Gestión de nómina y vacantes de personal (VLAN 240)"),
        ("5. Gestión de Eventos Literarios", AreaCode::Marketing, "Organización de ferias y eventos (VLAN 250)"),
        ("6. Cerrar Sesión (Logout)", AreaCode::Attention, "Destruir token en memoria y volver a pantalla de login"),
    ];

    let mut list_items = Vec::new();
    for (idx, (label, required_area, desc)) in options.iter().enumerate() {
        let is_selected = idx == app.menu_index;
        let is_logout = idx == 5;

        let has_permission = if is_logout {
            true
        } else if let Some(area) = user_area {
            area == *required_area
        } else {
            false
        };

        let badge = if is_logout {
            Span::styled(" [SESIÓN] ", Style::default().fg(SULTANA_THEME.warning))
        } else if has_permission {
            Span::styled(" [PERMITIDO] ", Style::default().fg(SULTANA_THEME.success).add_modifier(Modifier::BOLD))
        } else {
            Span::styled(" [BLOQUEADO 403] ", Style::default().fg(SULTANA_THEME.danger))
        };

        let prefix = if is_selected { " > " } else { "   " };

        let item_style = if is_selected {
            Style::default().fg(SULTANA_THEME.selected_fg).bg(SULTANA_THEME.selected_bg).add_modifier(Modifier::BOLD)
        } else if has_permission || is_logout {
            Style::default().fg(SULTANA_THEME.text_bright)
        } else {
            Style::default().fg(SULTANA_THEME.text_dim)
        };

        let line = Line::from(vec![
            Span::styled(prefix, item_style),
            Span::styled(*label, item_style),
            badge,
            Span::styled(format!(" - {}", desc), Style::default().fg(SULTANA_THEME.text_dim)),
        ]);

        list_items.push(ListItem::new(line));
    }

    let menu_block = Block::default()
        .title(" Seleccione un módulo con [↑/↓] y presione [Enter] ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(SULTANA_THEME.primary));

    let menu_list = List::new(list_items).block(menu_block);
    frame.render_widget(menu_list, center_col);

    let tip = Paragraph::new(Line::from(vec![
        Span::styled("Regla de Seguridad: ", Style::default().fg(SULTANA_THEME.accent)),
        Span::styled(
            "Las terminales están segmentadas. Intentar acceder a un módulo fuera de su área disparará un rechazo 403 Forbidden.",
            Style::default().fg(SULTANA_THEME.text_dim),
        ),
    ]))
    .alignment(Alignment::Center);
    frame.render_widget(tip, chunks[2]);
}
