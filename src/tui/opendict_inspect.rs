use crate::tui::TuiContext;
use ratatui::{crossterm::event::KeyCode, prelude::*, widgets, Frame};

#[derive(Debug)]
pub(super) struct Data<'a> {
    item_codes: widgets::List<'a>,
    item_codes_selected: widgets::ListState,
    item_data: String,
}

impl<'a> Data<'a> {
    pub(super) fn new(opendict_item_codes: &Vec<u32>) -> Self {
        let item_codes = opendict_item_codes
            .iter()
            .map(|code| code.to_string())
            .collect::<widgets::List>()
            .block(widgets::Block::bordered())
            .highlight_style(Style::default().yellow());
        Self {
            item_codes,
            item_codes_selected: widgets::ListState::default(),
            item_data: String::new(),
        }
    }
}

pub(super) fn draw(frame: &mut Frame, mut area: Rect, parent_ctx: &mut TuiContext) {
    let ctx = &mut parent_ctx.opendict_inspect;

    let list_area = Rect {
        x: area.x,
        y: area.y,
        width: 9,
        height: area.height,
    };
    area = Rect {
        x: area.x + 10,
        y: area.y,
        width: area.width - 10,
        height: area.height,
    };
    frame.render_stateful_widget(&ctx.item_codes, list_area, &mut ctx.item_codes_selected);

    let paragraph =
        widgets::Paragraph::new(ctx.item_data.clone()).block(widgets::Block::bordered());
    frame.render_widget(paragraph, area);
}

pub(super) fn pressed_event(parent_ctx: &mut TuiContext, pressed: KeyCode) {
    let ctx = &mut parent_ctx.opendict_inspect;
    const REPEAT_COUNT: i32 = 5;

    match pressed {
        KeyCode::Down => ctx.item_codes_selected.select_next(),
        KeyCode::Up => ctx.item_codes_selected.select_previous(),
        KeyCode::Home => ctx.item_codes_selected.select(Some(0)),
        KeyCode::End => ctx
            .item_codes_selected
            .select(Some(parent_ctx.opendict_item_codes.len() - 1)),
        KeyCode::PageDown => {
            for _ in 0..REPEAT_COUNT {
                ctx.item_codes_selected.select_next();
            }
        }
        KeyCode::PageUp => {
            for _ in 0..REPEAT_COUNT {
                ctx.item_codes_selected.select_previous();
            }
        }
        _ => {}
    }
}
