use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    prelude::*,
    widgets,
};
use std::collections::HashSet;

#[derive(Debug)]
struct TuiContext<'a> {
    tab_selected: usize,
    opendict_searched: Vec<crate::data_collector::opendict::OpendictQuery>,
    opendict_searched_word: Vec<String>,
    opendict_select_cursor: u8,
    opendict_select_word: widgets::List<'a>,
    opendict_select_word_selected: widgets::ListState,
    opendict_select_page_list: Option<Vec<u16>>,
    opendict_select_page: Option<widgets::List<'a>>,
    opendict_select_page_selected: widgets::ListState,
    opendict_select_data: Option<crate::data_collector::opendict::v1::OpendictResult>,
}

pub(super) fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    let mut ctx = TuiContext::default();
    let ctx = &mut ctx;
    loop {
        terminal.draw(|frame| {
            frame.render_widget(
                widgets::Tabs::new(vec![
                    "닉네임 생성",
                    "사전 데이터 조회",
                    "사전 데이터 검색",
                    "사전 데이터 크롤링",
                ])
                .block(widgets::Block::bordered())
                .style(Style::default().white())
                .highlight_style(Style::default().yellow())
                .select(ctx.tab_selected)
                .divider(symbols::DOT),
                frame.area(),
            );
            let area = frame.area();
            let area = Rect {
                x: area.x + 2,
                y: area.y + 2,
                width: area.width - 4,
                height: area.height - 3,
            };
            match ctx.tab_selected {
                0 => {}
                1 => {}
                2 => opendict_select(frame, area, ctx),
                3 => {}
                _ => unreachable!(),
            }
        })?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Tab => {
                        ctx.increase_tab();
                    }
                    KeyCode::BackTab => {
                        ctx.decrease_tab();
                    }
                    _ => {}
                }
            }
            match ctx.tab_selected {
                0 => {}
                1 => {}
                2 => match key.code {
                    KeyCode::Right => {
                        if ctx.opendict_select_cursor < 2 {
                            ctx.opendict_select_cursor += 1;
                        }
                    }
                    KeyCode::Left => {
                        if ctx.opendict_select_cursor > 0 {
                            ctx.opendict_select_cursor -= 1;
                        }
                    }
                    KeyCode::Down => match ctx.opendict_select_cursor {
                        0 => {
                            ctx.opendict_select_word_selected.select_next();
                            ctx.clear_page_list();
                        }
                        1 => {
                            ctx.opendict_select_page_selected.select_next();
                        }
                        2 => {}
                        _ => unreachable!(),
                    },
                    KeyCode::Up => match ctx.opendict_select_cursor {
                        0 => {
                            ctx.opendict_select_word_selected.select_previous();
                            ctx.clear_page_list();
                        }
                        1 => {
                            ctx.opendict_select_page_selected.select_previous();
                        }
                        2 => {}
                        _ => unreachable!(),
                    },
                    KeyCode::PageDown => {
                        let repeat_count = 5;
                        match ctx.opendict_select_cursor {
                            0 => {
                                for _ in 0..repeat_count {
                                    ctx.opendict_select_word_selected.select_next();
                                }
                                ctx.clear_page_list();
                            }
                            1 => {
                                for _ in 0..repeat_count {
                                    ctx.opendict_select_page_selected.select_next();
                                }
                            }
                            2 => {}
                            _ => unreachable!(),
                        }
                    }
                    KeyCode::PageUp => {
                        let repeat_count = 5;
                        match ctx.opendict_select_cursor {
                            0 => {
                                for _ in 0..repeat_count {
                                    ctx.opendict_select_word_selected.select_previous();
                                }
                                ctx.clear_page_list();
                            }
                            1 => {
                                for _ in 0..repeat_count {
                                    ctx.opendict_select_page_selected.select_previous();
                                }
                            }
                            2 => {}
                            _ => unreachable!(),
                        }
                    }
                    KeyCode::Home => match ctx.opendict_select_cursor {
                        0 => {
                            ctx.opendict_select_word_selected.select_first();
                            ctx.clear_page_list();
                        }
                        1 => {
                            ctx.opendict_select_page_selected.select_first();
                        }
                        2 => {}
                        _ => unreachable!(),
                    },
                    KeyCode::End => match ctx.opendict_select_cursor {
                        0 => {
                            ctx.opendict_select_word_selected.select_last();
                            ctx.clear_page_list();
                        }
                        1 => {
                            ctx.opendict_select_page_selected.select_last();
                        }
                        2 => {}
                        _ => unreachable!(),
                    },
                    KeyCode::Enter => {
                        let selected_word = ctx.opendict_select_word_selected.selected();
                        let selected_page = ctx.opendict_select_page_selected.selected();
                        if let (Some(selected_word), Some(selected_page)) =
                            (selected_word, selected_page)
                        {
                            let selected_word =
                                ctx.opendict_searched_word.get(selected_word).unwrap();
                            let selected_page =
                                ctx.opendict_select_page_list.as_ref().unwrap()[selected_page];
                            let query = ctx
                                .opendict_searched
                                .iter()
                                .find(|query| {
                                    query.keyword == *selected_word && query.page == selected_page
                                })
                                .unwrap();
                            let data = crate::prelude::get_opendict_data(query);
                            ctx.opendict_select_data = data;
                        }
                    }
                    _ => {}
                },
                3 => {}
                _ => unreachable!(),
            }
        }
    }
    ratatui::restore();
    Ok(())
}

fn opendict_select(frame: &mut Frame, mut area: Rect, ctx: &mut TuiContext) {
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
    frame.render_stateful_widget(
        &ctx.opendict_select_word,
        list_area,
        &mut ctx.opendict_select_word_selected,
    );

    if ctx.opendict_select_page.is_none() && ctx.opendict_select_word_selected.selected().is_some()
    {
        let selected_word = ctx.opendict_select_word_selected.selected().unwrap();
        let selected_word = ctx.opendict_searched_word.get(selected_word).unwrap();
        let mut pages = ctx
            .opendict_searched
            .iter()
            .filter(|query| query.keyword == *selected_word)
            .map(|x| x.page)
            .collect::<Vec<_>>();
        pages.sort();
        ctx.opendict_select_page = Some(
            pages
                .iter()
                .map(|page| format!("{:03}페이지", page))
                .collect::<widgets::List>()
                .block(widgets::Block::bordered())
                .highlight_style(Style::default().yellow()),
        );
        ctx.opendict_select_page_list = Some(pages);
    }

    if ctx.opendict_select_word_selected.selected().is_some() {
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
            ctx.opendict_select_page.as_ref().unwrap(),
            list_area,
            &mut ctx.opendict_select_page_selected,
        );
    }

    if let Some(data) = &ctx.opendict_select_data {
        let data = format!("{:#?}", data);
        let data = widgets::Paragraph::new(data).block(widgets::Block::bordered());
        frame.render_widget(data, area);
    }
}

impl<'a> Default for TuiContext<'a> {
    fn default() -> Self {
        let opendict_searched = crate::prelude::get_opendict_saved_queries();
        let mut opendict_searched_word = opendict_searched
            .iter()
            .map(|query| query.keyword.clone())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        debug_assert!(opendict_searched_word
            .iter()
            .all(|x| x.chars().count() == 1));
        opendict_searched_word.sort_by_cached_key(|x| x.chars().next().unwrap() as u32);
        let opendict_select_word = opendict_searched_word
            .iter()
            .cloned()
            .collect::<widgets::List>()
            .block(widgets::Block::bordered())
            .highlight_style(Style::default().yellow());
        Self {
            tab_selected: 0,
            opendict_searched,
            opendict_searched_word,
            opendict_select_cursor: 0,
            opendict_select_word,
            opendict_select_word_selected: widgets::ListState::default(),
            opendict_select_page_list: None,
            opendict_select_page: None,
            opendict_select_page_selected: widgets::ListState::default(),
            opendict_select_data: None,
        }
    }
}

impl<'a> TuiContext<'a> {
    const MAX_TAB: usize = 4;
    fn increase_tab(&mut self) {
        self.tab_selected += 1;
        if self.tab_selected >= Self::MAX_TAB {
            self.tab_selected = 0;
        }
    }
    fn decrease_tab(&mut self) {
        if self.tab_selected == 0 {
            self.tab_selected = Self::MAX_TAB - 1;
        }
        self.tab_selected -= 1;
    }
    fn clear_page_list(&mut self) {
        self.opendict_select_page_list.take();
        self.opendict_select_page.take();
        self.opendict_select_page_selected.select(None);
    }
}
