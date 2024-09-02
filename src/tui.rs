use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    prelude::*,
    widgets,
};

pub(super) fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    let mut tab_selected = 0;
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
                .select(tab_selected)
                .divider(symbols::DOT),
                frame.area(),
            );
        })?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Tab => {
                        tab_selected += 1;
                        if tab_selected >= 4 {
                            tab_selected = 0;
                        }
                    }
                    KeyCode::BackTab => {
                        if tab_selected == 0 {
                            tab_selected = 3;
                        }
                        tab_selected -= 1;
                    }
                    _ => {}
                }
            }
        }
    }
    ratatui::restore();
    Ok(())
}
