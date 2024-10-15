use loco_rs::prelude::*;

use crate::models::_entities::tests;

/// Render a list view of tests.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<tests::Model>) -> Result<Response> {
    format::render().view(v, "test/list.html", data!({"items": items}))
}

/// Render a single test view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &tests::Model) -> Result<Response> {
    format::render().view(v, "test/show.html", data!({"item": item}))
}

/// Render a test create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "test/create.html", data!({}))
}

/// Render a test edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &tests::Model) -> Result<Response> {
    format::render().view(v, "test/edit.html", data!({"item": item}))
}
