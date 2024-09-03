use crate::tui::TuiContext;
use ratatui::{crossterm::event::KeyCode, prelude::*, widgets, Frame};

#[derive(Debug)]
pub(super) struct Data<'a> {
    tab_cursor: u8,
    keyword: widgets::List<'a>,
    keyword_selected: widgets::ListState,
    opendict_select_page_list: Option<Vec<u16>>,
    page: Option<widgets::List<'a>>,
    page_selected: widgets::ListState,
    detail: Option<crate::data_collector::opendict::v1::OpendictResult>,
}

impl<'a> Data<'a> {
    pub(super) fn new(opendict_searched_word: &Vec<String>) -> Self {
        let opendict_select_word = opendict_searched_word
            .iter()
            .cloned()
            .collect::<widgets::List>()
            .block(widgets::Block::bordered())
            .highlight_style(Style::default().yellow());
        Self {
            tab_cursor: 0,
            keyword: opendict_select_word,
            keyword_selected: widgets::ListState::default(),
            opendict_select_page_list: None,
            page: None,
            page_selected: widgets::ListState::default(),
            detail: None,
        }
    }
    fn clear_page_list(&mut self) {
        self.opendict_select_page_list.take();
        self.page.take();
        self.page_selected.select(None);
    }
}

pub(super) fn draw(frame: &mut Frame, mut area: Rect, parent_ctx: &mut TuiContext) {
    let ctx = &mut parent_ctx.opendict_select;
    let list_area = Rect {
        x: area.x,
        y: area.y,
        width: 4,
        height: area.height,
    };
    area = Rect {
        x: area.x + 5,
        y: area.y,
        width: area.width - 5,
        height: area.height,
    };
    frame.render_stateful_widget(&ctx.keyword, list_area, &mut ctx.keyword_selected);

    if ctx.page.is_none() && ctx.keyword_selected.selected().is_some() {
        let selected_word = ctx.keyword_selected.selected().unwrap();
        let selected_word = parent_ctx
            .opendict_searched_word
            .get(selected_word)
            .unwrap();
        let mut pages = parent_ctx
            .opendict_searched
            .iter()
            .filter(|query| query.keyword == *selected_word)
            .map(|x| x.page)
            .collect::<Vec<_>>();
        pages.sort();
        ctx.page = Some(
            pages
                .iter()
                .map(|page| format!("{:03}페이지", page))
                .collect::<widgets::List>()
                .block(widgets::Block::bordered())
                .highlight_style(Style::default().yellow()),
        );
        ctx.opendict_select_page_list = Some(pages);
    }

    if ctx.keyword_selected.selected().is_some() {
        let list_area = Rect {
            x: area.x,
            y: area.y,
            width: 11,
            height: area.height,
        };
        area = Rect {
            x: area.x + 12,
            y: area.y,
            width: area.width - 12,
            height: area.height,
        };
        frame.render_stateful_widget(
            parent_ctx.opendict_select.page.as_ref().unwrap(),
            list_area,
            &mut parent_ctx.opendict_select.page_selected,
        );
    }

    if let Some(data) = &parent_ctx.opendict_select.detail {
        let data = format!("{:#?}", data);
        let data = widgets::Paragraph::new(data).block(widgets::Block::bordered());
        frame.render_widget(data, area);
    }
}

pub(super) fn pressed_event(parent_ctx: &mut TuiContext, pressed: KeyCode) {
    let ctx = &mut parent_ctx.opendict_select;
    match pressed {
        KeyCode::Right => {
            if ctx.tab_cursor < 2 {
                ctx.tab_cursor += 1;
            }
        }
        KeyCode::Left => {
            if ctx.tab_cursor > 0 {
                ctx.tab_cursor -= 1;
            }
        }
        KeyCode::Down => match ctx.tab_cursor {
            0 => {
                ctx.keyword_selected.select_next();
                ctx.clear_page_list();
            }
            1 => {
                ctx.page_selected.select_next();
            }
            2 => {}
            _ => unreachable!(),
        },
        KeyCode::Up => match ctx.tab_cursor {
            0 => {
                ctx.keyword_selected.select_previous();
                ctx.clear_page_list();
            }
            1 => {
                ctx.page_selected.select_previous();
            }
            2 => {}
            _ => unreachable!(),
        },
        KeyCode::PageDown => {
            let repeat_count = 5;
            match ctx.tab_cursor {
                0 => {
                    for _ in 0..repeat_count {
                        ctx.keyword_selected.select_next();
                    }
                    ctx.clear_page_list();
                }
                1 => {
                    for _ in 0..repeat_count {
                        ctx.page_selected.select_next();
                    }
                }
                2 => {}
                _ => unreachable!(),
            }
        }
        KeyCode::PageUp => {
            let repeat_count = 5;
            match ctx.tab_cursor {
                0 => {
                    for _ in 0..repeat_count {
                        ctx.keyword_selected.select_previous();
                    }
                    ctx.clear_page_list();
                }
                1 => {
                    for _ in 0..repeat_count {
                        ctx.page_selected.select_previous();
                    }
                }
                2 => {}
                _ => unreachable!(),
            }
        }
        KeyCode::Home => match ctx.tab_cursor {
            0 => {
                ctx.keyword_selected.select_first();
                ctx.clear_page_list();
            }
            1 => {
                ctx.page_selected.select_first();
            }
            2 => {}
            _ => unreachable!(),
        },
        KeyCode::End => match ctx.tab_cursor {
            0 => {
                ctx.keyword_selected.select_last();
                ctx.clear_page_list();
            }
            1 => {
                ctx.page_selected.select_last();
            }
            2 => {}
            _ => unreachable!(),
        },
        KeyCode::Enter => {
            let selected_word = ctx.keyword_selected.selected();
            let selected_page = ctx.page_selected.selected();
            if let (Some(selected_word), Some(selected_page)) = (selected_word, selected_page) {
                let selected_word = parent_ctx
                    .opendict_searched_word
                    .get(selected_word)
                    .unwrap();
                let selected_page = ctx.opendict_select_page_list.as_ref().unwrap()[selected_page];
                let query = parent_ctx
                    .opendict_searched
                    .iter()
                    .find(|query| query.keyword == *selected_word && query.page == selected_page)
                    .unwrap();
                let data = crate::prelude::get_opendict_data(query);
                ctx.detail = data;
            }
        }
        _ => {}
    }
}
