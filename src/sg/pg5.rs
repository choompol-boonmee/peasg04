use crate::sg::ldp::base;
use crate::sg::wk1;
use askama::Template;
use askama_axum;
use axum::body::Body;
use axum::extract::{Path, Query};
use axum::http::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::fs;
use tokio::task::spawn_blocking;
use uuid::Uuid;
use xlsxwriter::prelude::*;

fn gen_xlsx() -> Vec<u8> {
    let tnm = Uuid::new_v4().to_string();
    let wb = Workbook::new(tnm.as_str()).expect("?");
    let mut sh = wb.add_worksheet(None).unwrap();

    sh.write_string(
        0,
        0,
        "Red text",
        Some(&Format::new().set_font_color(FormatColor::Red)),
    )
    .expect("");
    sh.write_number(0, 1, 20., None).expect("");
    sh.write_formula_num(1, 0, "=10+B1", None, 30.).expect("");
    let u = "https://github.com/informationsea/xlsxwriter-rs";
    sh.write_url(
        1,
        1,
        u,
        Some(
            &Format::new()
                .set_font_color(FormatColor::Blue)
                .set_underline(FormatUnderline::Single),
        ),
    )
    .expect("");
    sh.merge_range(
        2,
        0,
        3,
        2,
        "Hello, world",
        Some(
            &Format::new()
                .set_font_color(FormatColor::Green)
                .set_align(FormatAlignment::CenterAcross)
                .set_vertical_align(FormatVerticalAlignment::VerticalCenter),
        ),
    )
    .expect("");

    sh.set_selection(1, 0, 1, 2);
    sh.set_tab_color(FormatColor::Cyan);

    sh.write_string(0, 0, "Test", None).unwrap();
    wb.close().expect("?");
    let bytes = fs::read(tnm.as_str()).expect("?");
    std::fs::remove_file(tnm.as_str());
    bytes
}

pub async fn handler(Path(xlsx): Path<String>) -> impl IntoResponse {
    print!("xlsx:{}\n", xlsx);
    let bytes = match spawn_blocking(move || gen_xlsx()).await {
        Ok(b) => b,
        Err(e) => return Err((StatusCode::NOT_FOUND, format!("{}", e))),
    };
    let body = Body::from(bytes);
    let name = format!("attachment; filename=\"{}.xlsx\"", xlsx);
    let head = [
        (CONTENT_TYPE, "application/vnd.ms-excel".to_string()),
        (CONTENT_DISPOSITION, name),
    ];
    Ok((head, body))
}
