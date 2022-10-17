use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use std::cell::RefCell;
use std::collections::VecDeque;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::JsCast;

use crate::contants::color::{
    BOARD_DEFAULT_COLOR, BOARD_STROKE_DEFAULT_COLOR, HOLD_DEFAULT_COLOR, HOLD_STROKE_DEFAULT_COLOR,
    NEXT_DEFAULT_COLOR, NEXT_STROKE_DEFAULT_COLOR,
};
use crate::game::tetris_board::TetrisBoard;
use crate::game::tetris_cell::TetrisCell;
use crate::game::MinoShape;
use crate::js_bind::body::body;
use crate::js_bind::request_animation_frame::request_animation_frame;

use super::draw::draw_block;

#[wasm_bindgen]
pub fn render_board(
    board_unfolded: Vec<i32>,
    board_width: u32,
    board_height: u32,
    column_count: u32,
    row_count: u32,
    hidden_row_count: u32,
) {
    let visible_row_count = row_count - hidden_row_count;

    let block_width_size = (board_width / column_count as u32) as f64;
    let block_height_size = (board_height / visible_row_count as u32) as f64;

    let tetris_board = TetrisBoard::from_unfold(
        board_unfolded,
        board_width,
        board_height,
        column_count,
        row_count,
        hidden_row_count,
    );

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("game-canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    context.set_fill_style(&JsValue::from_str(BOARD_DEFAULT_COLOR));
    context.fill_rect(0.0, 0.0, board_width as f64, board_height as f64);
    context.set_stroke_style(&JsValue::from_str(BOARD_STROKE_DEFAULT_COLOR));
    context.stroke_rect(0.0, 0.0, board_width as f64, board_height as f64);

    for x in 0..column_count {
        let x = x as usize;

        for y in 0..(visible_row_count) {
            let y = y as usize;

            let hidden_row_count = hidden_row_count as usize;

            if tetris_board.cells[y + hidden_row_count][x] != TetrisCell::Empty {
                let cell = tetris_board.cells[y + hidden_row_count][x];

                let x = x as f64 * block_width_size;
                let y = y as f64 * block_height_size;
                draw_block(
                    context.clone(),
                    x,
                    y,
                    block_width_size,
                    block_height_size,
                    cell.to_color(),
                );
            } else {
                let x = x as f64 * block_width_size;
                let y = y as f64 * block_height_size;
                draw_block(
                    context.clone(),
                    x,
                    y,
                    block_width_size,
                    block_height_size,
                    BOARD_DEFAULT_COLOR,
                );
            }
        }
    }
}

#[wasm_bindgen]
pub fn render_next(
    mino_list: Vec<i32>,
    board_width: u32,
    board_height: u32,
    column_count: u8,
    row_count: u8,
) {
    let block_width_size = (board_width / column_count as u32) as f64;
    let block_height_size = (board_height / row_count as u32) as f64;

    let mino_shapes = mino_list
        .into_iter()
        .map(|e| e.into())
        .collect::<Vec<MinoShape>>();

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("next-canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    // 검은색으로 세팅
    context.set_fill_style(&JsValue::from_str(NEXT_DEFAULT_COLOR));
    context.fill_rect(0.0, 0.0, board_width as f64, board_height as f64);
    context.set_stroke_style(&JsValue::from_str(NEXT_STROKE_DEFAULT_COLOR));
    context.stroke_rect(0.0, 0.0, board_width as f64, board_height as f64);

    let mut mino_iter = mino_shapes.iter();
    let mut current_mino = VecDeque::new();

    for y in 0..row_count {
        if current_mino.is_empty() {
            match mino_iter.next() {
                Some(mino) => {
                    current_mino = mino.cells.iter().cloned().collect();
                    continue;
                }
                None => {
                    break;
                }
            }
        }

        let current_mino_row = current_mino.pop_front().unwrap();

        let y = y as usize;

        for x in 1..(column_count - 1) {
            let x = x as usize;

            let cell = current_mino_row.get(x - 1);

            if cell != Some(&TetrisCell::Empty) && cell.is_some() {
                let cell = current_mino_row[x - 1];

                let x = x as f64 * block_width_size;
                let y = y as f64 * block_height_size;
                draw_block(
                    context.clone(),
                    x,
                    y,
                    block_width_size,
                    block_height_size,
                    cell.to_color(),
                );
            } else {
                let x = x as f64 * block_width_size;
                let y = y as f64 * block_height_size;
                draw_block(
                    context.clone(),
                    x,
                    y,
                    block_width_size,
                    block_height_size,
                    NEXT_DEFAULT_COLOR,
                );
            }
        }
    }
}

#[wasm_bindgen]
pub fn render_hold(
    mino: Option<i32>,
    board_width: u32,
    board_height: u32,
    column_count: u8,
    row_count: u8,
) {
    let block_width_size = (board_width / column_count as u32) as f64;
    let block_height_size = (board_height / row_count as u32) as f64;

    let mino_shapes = match mino {
        Some(mino) => [mino]
            .into_iter()
            .map(|e| e.into())
            .collect::<Vec<MinoShape>>(),
        None => vec![],
    };

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("hold-canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    // 검은색으로 세팅
    context.set_fill_style(&JsValue::from_str(HOLD_DEFAULT_COLOR));
    context.fill_rect(0.0, 0.0, board_width as f64, board_height as f64);
    context.set_stroke_style(&JsValue::from_str(HOLD_STROKE_DEFAULT_COLOR));
    context.stroke_rect(0.0, 0.0, board_width as f64, board_height as f64);

    let mut mino_iter = mino_shapes.iter();
    let mut current_mino = VecDeque::new();

    for y in 0..row_count {
        if current_mino.is_empty() {
            match mino_iter.next() {
                Some(mino) => {
                    current_mino = mino.cells.iter().cloned().collect();
                    continue;
                }
                None => {
                    break;
                }
            }
        }

        let current_mino_row = current_mino.pop_front().unwrap();

        let y = y as usize;

        for x in 1..(column_count - 1) {
            let x = x as usize;

            let cell = current_mino_row.get(x - 1);

            if cell != Some(&TetrisCell::Empty) && cell.is_some() {
                let cell = current_mino_row[x - 1];

                let x = x as f64 * block_width_size;
                let y = y as f64 * block_height_size;
                draw_block(
                    context.clone(),
                    x,
                    y,
                    block_width_size,
                    block_height_size,
                    cell.to_color(),
                );
            } else {
                let x = x as f64 * block_width_size;
                let y = y as f64 * block_height_size;
                draw_block(
                    context.clone(),
                    x,
                    y,
                    block_width_size,
                    block_height_size,
                    HOLD_DEFAULT_COLOR,
                );
            }
        }
    }
}

#[wasm_bindgen]
pub fn run_render() -> Result<(), JsValue> {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut i = 0;
    *g.borrow_mut() = Some(Closure::new(move || {
        if i > 300 {
            body().set_text_content(Some("All done!"));

            // Drop our handle to this closure so that it will get cleaned
            // up once we return.
            let _ = f.borrow_mut().take();
            return;
        }

        // Set the body's text content to how many times this
        // requestAnimationFrame callback has fired.
        i += 1;
        let text = format!("requestAnimationFrame has been called {} times.", i);
        body().set_text_content(Some(&text));

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
