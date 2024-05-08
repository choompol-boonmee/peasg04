use crate::sg::ldp::base;
use crate::sg::wk1;
use askama::Template;
use askama_axum;
use axum::body::Body;
use axum::extract::{Path, Query};
use axum::http::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use plotters::prelude::*;
use std::fs;
use tokio::task::spawn_blocking;
use uuid::Uuid;
use xlsxwriter::prelude::*;

fn drawline(data: Vec<(f64, f64)>) -> Vec<u8> {
    let tnm = Uuid::new_v4().to_string();
    let tnm = format!("{}.png", tnm);

    let mut minv = 0_f64;
    let mut maxv = 0_f64;
    for (_x, y) in &data {
        if *y > maxv {
            maxv = *y as f64;
        }
        if *y < minv {
            minv = *y as f64;
        }
    }
    {
        let drawing_area = BitMapBackend::new(&tnm, (600, 400)).into_drawing_area();
        //let drawing_area = SVGBackend::new(&tnm, (800, 600)).into_drawing_area();
        drawing_area.fill(&WHITE).unwrap();
        let mut chart_builder = ChartBuilder::on(&drawing_area);
        chart_builder
            .margin(10)
            .set_left_and_bottom_label_area_size(20);
        let mut ctx = chart_builder
            .build_cartesian_2d(0.0..24.0, minv..maxv)
            .unwrap();
        ctx.configure_mesh().draw().unwrap();
        ctx.draw_series(LineSeries::new(data.iter().map(|(x, y)| (*x, *y)), &BLACK))
            .unwrap();
    }

    let bytes = fs::read(tnm.as_str()).expect("?");
    std::fs::remove_file(tnm.as_str());
    bytes
}

pub async fn handler(Path(xlsx): Path<String>) -> impl IntoResponse {
    let mut dv = Vec::new();
    for i in 0..48 {
        let x = i as f64 / 2f64;
        let y = i as f64 / 2f64;
        dv.push((x, y));
    }
    print!("png:{}\n", xlsx);
    let bytes = match spawn_blocking(move || drawline(dv)).await {
        Ok(b) => b,
        Err(e) => return Err((StatusCode::NOT_FOUND, format!("{}", e))),
    };
    let body = Body::from(bytes);
    let name = format!("attachment; filename=\"{}.png\"", xlsx);
    let head = [
        (CONTENT_TYPE, "image/png".to_string()),
        //(CONTENT_DISPOSITION, name),
    ];
    Ok((head, body))
}
