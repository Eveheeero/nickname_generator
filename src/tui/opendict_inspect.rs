use crate::tui::TuiContext;
use ratatui::{crossterm::event::KeyCode, prelude::*, widgets, Frame};

#[derive(Debug)]
pub(super) struct Data<'a> {
    item_codes: widgets::List<'a>,
    item_codes_selected: widgets::ListState,
    item_codes_inputted: String,
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
            item_codes_inputted: String::new(),
            item_data: String::new(),
        }
    }
    fn selected_with_arrow(&mut self, opendict_item_codes: &Vec<u32>) {
        let Some(selected) = self.item_codes_selected.selected() else {
            return;
        };
        self.item_codes_inputted = opendict_item_codes[selected].to_string();
    }
    fn selected_with_num(&mut self, opendict_item_codes: &Vec<u32>) {
        let Ok(data): Result<u32, _> = self.item_codes_inputted.parse() else {
            return;
        };
        let selected = opendict_item_codes.iter().position(|&x| x == data);
        self.item_codes_selected.select(selected);
    }
    fn select_if_can(&mut self) {
        self.item_data.clear();
        let Ok(code) = self.item_codes_inputted.parse::<u32>() else {
            return;
        };
        let Some(item) = crate::prelude::get_opendict_item(code) else {
            return;
        };
        self.item_data = serde_json::to_string_pretty(&item).unwrap();
    }
}

pub(super) fn draw(frame: &mut Frame, mut area: Rect, parent_ctx: &mut TuiContext) {
    let ctx = &mut parent_ctx.opendict_inspect;

    let widget_area = Rect {
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
    frame.render_stateful_widget(&ctx.item_codes, widget_area, &mut ctx.item_codes_selected);

    let widget_area = Rect {
        x: area.x,
        y: area.y,
        width: area.width,
        height: area.height - 1,
    };
    area = Rect {
        x: area.x,
        y: area.y + area.height - 1,
        width: area.width,
        height: 1,
    };
    let paragraph =
        widgets::Paragraph::new(ctx.item_data.clone()).block(widgets::Block::bordered());
    frame.render_widget(paragraph, widget_area);

    frame.render_widget(
        widgets::Paragraph::new(ctx.item_codes_inputted.clone()),
        area,
    );
}

pub(super) fn pressed_event(parent_ctx: &mut TuiContext, pressed: KeyCode) {
    let ctx = &mut parent_ctx.opendict_inspect;
    let opendict_item_codes = &parent_ctx.opendict_item_codes;
    const REPEAT_COUNT: i32 = 5;

    match pressed {
        KeyCode::Down => {
            ctx.item_codes_selected.select_next();
            ctx.selected_with_arrow(opendict_item_codes);
            ctx.select_if_can();
        }
        KeyCode::Up => {
            ctx.item_codes_selected.select_previous();
            ctx.selected_with_arrow(opendict_item_codes);
            ctx.select_if_can();
        }
        KeyCode::Home => {
            ctx.item_codes_selected.select(Some(0));
            ctx.selected_with_arrow(opendict_item_codes);
            ctx.select_if_can();
        }
        KeyCode::End => {
            ctx.item_codes_selected
                .select(Some(parent_ctx.opendict_item_codes.len() - 1));
            ctx.selected_with_arrow(opendict_item_codes);
            ctx.select_if_can();
        }
        KeyCode::PageDown => {
            for _ in 0..REPEAT_COUNT {
                ctx.item_codes_selected.select_next();
            }
            ctx.selected_with_arrow(opendict_item_codes);
            ctx.select_if_can();
        }
        KeyCode::PageUp => {
            for _ in 0..REPEAT_COUNT {
                ctx.item_codes_selected.select_previous();
            }
            ctx.selected_with_arrow(opendict_item_codes);
            ctx.select_if_can();
        }
        KeyCode::Char(c) => {
            let is_num = c.is_ascii_digit();
            if is_num {
                ctx.item_codes_inputted.push(c);
                ctx.selected_with_num(opendict_item_codes);
                ctx.select_if_can();
            }
        }
        KeyCode::Backspace => {
            ctx.item_codes_inputted.pop();
            ctx.selected_with_num(opendict_item_codes);
            ctx.select_if_can();
        }
        _ => {}
    }
}
