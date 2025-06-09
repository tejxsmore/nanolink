use crate::templates::{home_template, signin_template, signup_template, profile_template};
use axum::response::Html;

pub async fn home_handler() -> Html<String> {
    Html(home_template::get_template())
}

pub async fn signin_page_handler() -> Html<String> {
    Html(signin_template::get_template())
}

pub async fn signup_page_handler() -> Html<String> {
    Html(signup_template::get_template())
}

pub async fn profile_page_handler() -> Html<String> {
    Html(profile_template::get_template())
}