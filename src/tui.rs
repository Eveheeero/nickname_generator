use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    prelude::*,
    widgets,
};
use std::cell::{LazyCell, OnceCell};

#[derive(Debug)]
struct TuiContext<'a> {
    tab_selected: usize,
    opendict_searched: LazyCell<Vec<crate::data_collector::opendict::OpendictQuery>>,
    opendict_select_list: OnceCell<widgets::List<'a>>,
    opendict_select_selected: widgets::ListState,
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
                    KeyCode::Down => {
                        ctx.opendict_select_selected.select_next();
                    }
                    KeyCode::Up => {
                        ctx.opendict_select_selected.select_previous();
                    }
                    KeyCode::Enter => {
                        if let Some(selected) = ctx.opendict_select_selected.selected() {
                            let query = ctx.opendict_searched.get(selected).unwrap();
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

fn opendict_select(frame: &mut Frame, area: Rect, ctx: &mut TuiContext) {
    let searched_string = ctx.opendict_select_list.get_or_init(|| {
        ctx.opendict_searched
            .iter()
            .map(|query| format!("{} {:3}페이지", query.keyword, query.page))
            .collect::<widgets::List>()
            .block(widgets::Block::bordered())
            .highlight_style(Style::default().yellow())
    });
    let list_area = Rect {
        x: area.x,
        y: area.y,
        width: 14,
        height: area.height,
    };
    let area = Rect {
        x: area.x + 15,
        y: area.y,
        width: area.width - 15,
        height: area.height,
    };
    frame.render_stateful_widget(
        searched_string,
        list_area,
        &mut ctx.opendict_select_selected,
    );
    if let Some(data) = &ctx.opendict_select_data {
        let data = format!("{:#?}", data);
        let data = widgets::Paragraph::new(data).block(widgets::Block::bordered());
        frame.render_widget(data, area);
    }
}

impl<'a> Default for TuiContext<'a> {
    fn default() -> Self {
        Self {
            tab_selected: 0,
            opendict_searched: LazyCell::new(|| crate::prelude::get_opendict_saved_queries()),
            opendict_select_list: OnceCell::new(),
            opendict_select_selected: widgets::ListState::default(),
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
}
