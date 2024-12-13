use crate::tui::TuiContext;
use ratatui::{crossterm::event::KeyCode, prelude::*, widgets, Frame};
use rayon::prelude::*;

#[derive(Debug)]
pub(super) struct Data<'a> {
    /// 몇번째 탭에 데이터가 있는지
    tab_cursor: u8,
    /// 검색된 단어 목록
    keyword: widgets::List<'a>,
    /// 선택된 단어 목록
    keyword_selected: widgets::ListState,

    page_origin: Option<Vec<u16>>,
    /// 몇번째 페이지를 선택했는지
    page: Option<widgets::List<'a>>,
    /// 선택된 페이지 목록
    page_selected: widgets::ListState,
    /// 상세 내용
    detail: Option<crate::data_collector::opendict::v1::OpendictResult>,
}

impl<'a> Data<'a> {
    pub(super) fn new(opendict_searched_word: &Vec<String>) -> Self {
        let opendict_query_word = opendict_searched_word
            .iter()
            .cloned()
            .collect::<widgets::List>()
            .block(widgets::Block::bordered())
            .highlight_style(Style::default().yellow());
        Self {
            tab_cursor: 0,
            keyword: opendict_query_word,
            keyword_selected: widgets::ListState::default(),
            page_origin: None,
            page: None,
            page_selected: widgets::ListState::default(),
            detail: None,
        }
    }
    fn clear_page_list(&mut self) {
        self.page_origin.take();
        self.page.take();
        self.page_selected.select(None);
    }
}

pub(super) fn draw(frame: &mut Frame, mut area: Rect, parent_ctx: &mut TuiContext) {
    let ctx = &mut parent_ctx.opendict_query;
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
            .par_iter()
            .filter(|query| query.keyword == *selected_word)
            .map(|x| x.page)
            .collect::<Vec<_>>();
        pages.sort();
        ctx.page = Some(
            pages
                .iter()
                .map(|page| format!("{:04}페이지", page))
                .collect::<widgets::List>()
                .block(widgets::Block::bordered())
                .highlight_style(Style::default().yellow()),
        );
        ctx.page_origin = Some(pages);
    }

    if ctx.keyword_selected.selected().is_some() {
        let list_area = Rect {
            x: area.x,
            y: area.y,
            width: 12,
            height: area.height,
        };
        area = Rect {
            x: area.x + 13,
            y: area.y,
            width: area.width - 13,
            height: area.height,
        };
        frame.render_stateful_widget(
            parent_ctx.opendict_query.page.as_ref().unwrap(),
            list_area,
            &mut parent_ctx.opendict_query.page_selected,
        );
    }

    if let Some(data) = &parent_ctx.opendict_query.detail {
        let data = format!("{:#?}", data);
        let data = widgets::Paragraph::new(data).block(widgets::Block::bordered());
        frame.render_widget(data, area);
    }
}

pub(super) fn pressed_event(parent_ctx: &mut TuiContext, pressed: KeyCode) {
    let ctx = &mut parent_ctx.opendict_query;
    const REPEAT_COUNT: i32 = 5;
    let set_detail_if_can = |ctx: &mut Data| {
        /* 최대값을 벗어났으면 설정 */
        ctx.keyword_selected.selected().inspect(|i| {
            if *i >= ctx.keyword.len() {
                ctx.keyword_selected.select(Some(ctx.keyword.len() - 1));
            }
        });
        ctx.page_selected.selected().inspect(|i| {
            if *i >= ctx.page.as_ref().unwrap().len() {
                ctx.page_selected
                    .select(Some(ctx.page.as_ref().unwrap().len() - 1));
            }
        });

        let selected_word = ctx.keyword_selected.selected();
        let selected_page = ctx.page_selected.selected();

        if let (Some(selected_word), Some(selected_page)) = (selected_word, selected_page) {
            let selected_word = parent_ctx
                .opendict_searched_word
                .get(selected_word)
                .unwrap();
            let selected_page = ctx.page_origin.as_ref().unwrap()[selected_page];
            let query = parent_ctx
                .opendict_searched
                .par_iter()
                .find_any(|query| query.keyword == *selected_word && query.page == selected_page)
                .unwrap();
            let data = crate::prelude::get_opendict_data(query);
            ctx.detail = data;
        }
    };
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
                set_detail_if_can(ctx);
            }
            1 => {
                ctx.page_selected.select_next();
                set_detail_if_can(ctx);
            }
            2 => {}
            _ => unreachable!(),
        },
        KeyCode::Up => match ctx.tab_cursor {
            0 => {
                ctx.keyword_selected.select_previous();
                ctx.clear_page_list();
                set_detail_if_can(ctx);
            }
            1 => {
                ctx.page_selected.select_previous();
                set_detail_if_can(ctx);
            }
            2 => {}
            _ => unreachable!(),
        },
        KeyCode::PageDown => match ctx.tab_cursor {
            0 => {
                for _ in 0..REPEAT_COUNT {
                    ctx.keyword_selected.select_next();
                }
                ctx.clear_page_list();
                set_detail_if_can(ctx);
            }
            1 => {
                for _ in 0..REPEAT_COUNT {
                    ctx.page_selected.select_next();
                }
                set_detail_if_can(ctx);
            }
            2 => {}
            _ => unreachable!(),
        },
        KeyCode::PageUp => match ctx.tab_cursor {
            0 => {
                for _ in 0..REPEAT_COUNT {
                    ctx.keyword_selected.select_previous();
                }
                ctx.clear_page_list();
                set_detail_if_can(ctx);
            }
            1 => {
                for _ in 0..REPEAT_COUNT {
                    ctx.page_selected.select_previous();
                }
                set_detail_if_can(ctx);
            }
            2 => {}
            _ => unreachable!(),
        },
        KeyCode::Home => match ctx.tab_cursor {
            0 => {
                ctx.keyword_selected.select_first();
                ctx.clear_page_list();
                set_detail_if_can(ctx);
            }
            1 => {
                ctx.page_selected.select_first();
                set_detail_if_can(ctx);
            }
            2 => {}
            _ => unreachable!(),
        },
        KeyCode::End => match ctx.tab_cursor {
            0 => {
                ctx.keyword_selected.select_last();
                ctx.clear_page_list();
                set_detail_if_can(ctx);
            }
            1 => {
                ctx.page_selected.select_last();
                set_detail_if_can(ctx);
            }
            2 => {}
            _ => unreachable!(),
        },
        _ => {}
    }
}
