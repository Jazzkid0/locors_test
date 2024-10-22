#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use std::{convert::From, path::{self, PathBuf}};

use loco_rs::prelude::*;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use axum::{extract::Form, response::Redirect, http::StatusCode};
use sea_orm::{sea_query::Order, QueryOrder};
use axum::debug_handler;

use crate::{
    models::_entities::tests::{ActiveModel, Column, Entity, Model},
    views,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub title: Option<String>,
    pub pdf: Option<Vec<u8>>,
    pub cost: Option<Decimal>,
    pub downloads: Option<i32>,
    pub description: Option<String>,
    pub date: Option<Date>,
    }

impl Params {
    fn update(&self, item: &mut ActiveModel) {
      item.title = Set(self.title.clone());
      item.pdf = Set(self.pdf.clone());
      item.cost = Set(self.cost.clone());
      item.downloads = Set(self.downloads.clone());
      item.description = Set(self.description.clone());
      item.date = Set(self.date.clone());
      }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let item = Entity::find()
        .order_by(Column::Id, Order::Desc)
        .all(&ctx.db)
        .await?;
    views::test::list(&v, &item)
}

#[debug_handler]
pub async fn new(
    ViewEngine(v): ViewEngine<TeraView>,
    State(_ctx): State<AppContext>,
) -> Result<Response> {
    views::test::create(&v)
}

#[debug_handler]
pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Form(params): Form<Params>,
) -> Result<Redirect> {
    let item = load_item(&ctx, id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    item.update(&ctx.db).await?;
    Ok(Redirect::to("../tests"))
}

#[debug_handler]
pub async fn edit(
    Path(id): Path<i32>,
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    views::test::edit(&v, &item)
}

#[debug_handler]
pub async fn show(
    Path(id): Path<i32>,
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    views::test::show(&v, &item)
}

#[debug_handler]
pub async fn add(
    State(ctx): State<AppContext>,
    Form(params): Form<Params>,
) -> Result<Redirect> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    item.insert(&ctx.db).await?;
    Ok(Redirect::to("tests"))
}

#[debug_handler]
pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

#[debug_handler]
pub async fn upload_file(
    State(ctx): State<AppContext>,
    content: Bytes
) -> Result<StatusCode> {
    let path = PathBuf::from("test.txt");
    let testcontent = Bytes::from("boobs!");
    let storage_response = ctx.storage.upload(path.as_path(), &testcontent).await;
    print!("\nTried storing file... ");
    match storage_response {
        Ok(_) => {
            println!("Success\n");
            Ok(StatusCode::OK) // 200
            },
        Err(_) => {
            println!("Error\n");
            Err(loco_rs::Error::InternalServerError) // 500
        }
    }
}

pub async fn download_file_text(
    State(ctx): State<AppContext>
) -> Result<StatusCode> {
    let storage_response: loco_rs::storage::StorageResult<String> = loco_rs::storage::Storage::download(&ctx.storage, PathBuf::from("test.txt").as_path()).await;
    match storage_response  {
        Ok(out) => {
            println!("Success\n");
            println!("\n\n{out}\n\n");
            Ok(StatusCode::OK) // 200
            },
        Err(_) => {
            println!("Error\n");
            Err(loco_rs::Error::InternalServerError) // 500
        }
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("tests/")
        .add("/", get(list))
        .add("/", post(add))
        .add("new", get(new))
        .add(":id", get(show))
        .add(":id/edit", get(edit))
        .add(":id", post(update))
        .add(":id", delete(remove))
        .add("pdf", get(download_file_text))
        .add("pdf", post(upload_file))
}
