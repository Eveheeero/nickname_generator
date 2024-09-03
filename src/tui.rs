mod opendict_select;
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
    opendict_select: opendict_select::Data<'a>,
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
                2 => opendict_select::draw(frame, area, ctx),
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
                2 => opendict_select::pressed_event(ctx, key.code),
                3 => {}
                _ => unreachable!(),
            }
        }
    }
    ratatui::restore();
    Ok(())
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
        let opendict_select = opendict_select::Data::new(&opendict_searched_word);
        Self {
            tab_selected: 0,
            opendict_searched,
            opendict_searched_word,
            opendict_select,
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
