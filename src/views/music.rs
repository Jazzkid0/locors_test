use loco_rs::prelude::*;

use crate::models::_entities::music;

/// Render a list view of music.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<music::Model>) -> Result<Response> {
    format::render().view(v, "music/list.html", data!({"items": items}))
}

/// Render a single music view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &music::Model) -> Result<Response> {
    format::render().view(v, "music/show.html", data!({"item": item}))
}

/// Render a music create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "music/create.html", data!({}))
}

/// Render a music edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &music::Model) -> Result<Response> {
    format::render().view(v, "music/edit.html", data!({"item": item}))
}
