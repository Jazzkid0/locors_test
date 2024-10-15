use loco_rs::prelude::*;

use crate::models::_entities::songs;

/// Render a list view of songs.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<songs::Model>) -> Result<Response> {
    format::render().view(v, "songs/list.html", data!({"items": items}))
}

/// Render a single songs view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &songs::Model) -> Result<Response> {
    format::render().view(v, "songs/show.html", data!({"item": item}))
}

/// Render a songs create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "songs/create.html", data!({}))
}

/// Render a songs edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &songs::Model) -> Result<Response> {
    format::render().view(v, "songs/edit.html", data!({"item": item}))
}
