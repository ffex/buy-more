use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListDirection, ListItem, Paragraph, Wrap},
};

use crate::app::{App, Screen};

pub fn render_app(frame: &mut Frame, app: &App) {
    // Initialize the layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    print_header(frame, chunks[0]);
    match app.current_screen {
        Screen::Main => {
            print_main(frame, app, chunks[1]);
        }
        Screen::Cart => {
            print_cart(frame, app, chunks[1]);
        }
        Screen::Payment => {
            print_popup_confirm(frame, chunks[1]);
        }
    }
    print_footer(frame, app, chunks[2]);
}
pub fn print_header(frame: &mut Frame, area: Rect) {
    // Title
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled("RustBuy", Style::default().fg(Color::Blue)))
        .block(title_block);

    frame.render_widget(title, area);
}
pub fn print_main(frame: &mut Frame, app: &App, area: Rect) {
    // Center Screen
    let chunks_center = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(area);

    let items: Vec<ListItem> = app
        .available_products
        .iter()
        .enumerate()
        .map(|(i, product)| {
            let style = if i == app.selected_product_index {
                Style::default().fg(Color::White).bg(Color::Red)
            } else {
                Style::default()
            };
            ListItem::new(format!(
                "{}{}",
                match product.in_cart {
                    true => "(*)",
                    false => "",
                },
                &product.name
            ))
            .style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::bordered().title("List"))
        .style(Style::new().white())
        .highlight_style(Style::new().italic())
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    let selected_product_detail = vec![
        Line::from(
            app.available_products[app.selected_product_index]
                .name
                .clone(),
        ),
        Line::default(),
        Line::from(
            app.available_products[app.selected_product_index]
                .description
                .clone(),
        ),
        Line::default(),
        Line::from(format!(
            "Price: {}",
            app.available_products[app.selected_product_index].price
        )),
    ];
    let product_detail_block = Paragraph::new(selected_product_detail)
        .block(Block::bordered().title("Product Detail"))
        .style(Style::default());

    frame.render_widget(list, chunks_center[0]);
    frame.render_widget(product_detail_block, chunks_center[1]);
}

pub fn print_cart(frame: &mut Frame, app: &App, area: Rect) {
    // Center Screen
    let chunks_center = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
        .split(area);

    let items: Vec<ListItem> = app
        .order
        .products
        .iter()
        .map(|product| {
            let style = Style::default();
            ListItem::new(product.name.to_string()).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::bordered().title("List"))
        .style(Style::new().white())
        .highlight_style(Style::new().italic())
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    let checkout_detail = vec![
        Line::from(format!("Taxable: {:.2} $", app.order.taxable)),
        Line::from(format!("VAT: {:.2} $", app.order.vat_calculated)),
        Line::from(format!(
            "Discount: {:.2} $ ({}%)",
            app.order.discount_calculated, app.order.discount_perc
        )),
        Line::from(format!("Delivery cost: {:.2} $", app.order.delivery_cost)),
        Line::from(format!("Total: {:.2} $", app.order.totals)),
    ];
    let product_detail_block = Paragraph::new(checkout_detail)
        .block(Block::bordered().title("Checkout"))
        .style(Style::default());

    frame.render_widget(product_detail_block, chunks_center[0]);
    frame.render_widget(list, chunks_center[1]);
}
pub fn print_footer(frame: &mut Frame, app: &App, area: Rect) {
    let current_navigation_text = vec![
        Span::styled(
            match app.current_screen {
                Screen::Main => "Main",
                Screen::Cart => "Cart",
                Screen::Payment => "Payment",
            },
            Style::default().fg(Color::Green),
        ),
        Span::styled(" | ", Style::default().fg(Color::White)),
    ];

    let current_nav_key_hint_text = match app.current_screen {
        Screen::Main => Span::styled(
            "(q) Quit | (a) Add | (r) Remove | (c) Checkout",
            Style::default().fg(Color::Blue),
        ),
        Screen::Cart => Span::styled(
            "(q) Quit | (m) Main | (p) Payment",
            Style::default().fg(Color::Blue),
        ),
        Screen::Payment => Span::styled(
            "(q) Quit | (m) Main | (p) Payment",
            Style::default().fg(Color::Blue),
        ),
    };

    let current_nav = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_nav_key_hint =
        Paragraph::new(current_nav_key_hint_text).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    frame.render_widget(current_nav, footer_chunks[0]);
    frame.render_widget(current_nav_key_hint, footer_chunks[1]);
}
pub fn print_popup_confirm(frame: &mut Frame, area: Rect) {
    let popup_area = centered_rect(40, 50, area);

    //Clear the area
    frame.render_widget(Clear, popup_area);

    let popup_block = Block::default()
        .borders(Borders::ALL)
        .title("Confirm")
        .style(Style::default().bg(Color::Blue));

    let exit_text = Text::styled(
        "Do you want confirm the order? Y/n",
        Style::default().fg(Color::Red),
    );

    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_block)
        .wrap(Wrap { trim: false });

    frame.render_widget(exit_paragraph, popup_area);
}
fn centered_rect(perc_x: u16, perc_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - perc_y) / 2),
            Constraint::Percentage(perc_y),
            Constraint::Percentage((100 - perc_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - perc_x) / 2),
            Constraint::Percentage(perc_x),
            Constraint::Percentage((100 - perc_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
