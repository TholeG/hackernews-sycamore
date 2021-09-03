mod apis;
mod components;
mod pages;

use anyhow::Result;
use apis::types::{StoryItem, StorySorting, UserData};
use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router, RouterProps};

#[derive(Debug, Route)]
enum AppRoutes {
    #[to("/")]
    #[preload(|_| apis::get_stories(StorySorting::Top))]
    Top { data: Result<Vec<StoryItem>> },
    #[to("/new")]
    #[preload(|_| apis::get_stories(StorySorting::New))]
    New { data: Result<Vec<StoryItem>> },
    #[to("/best")]
    #[preload(|_| apis::get_stories(StorySorting::Best))]
    Best { data: Result<Vec<StoryItem>> },
    #[to("/show")]
    #[preload(|_| apis::get_stories(StorySorting::Show))]
    Show { data: Result<Vec<StoryItem>> },
    #[to("/user/<username>")]
    #[preload(|params: Vec<String>| async move { apis::get_user_page(&params[1]).await })]
    User {
        username: String,
        data: Result<UserData>,
    },
    #[not_found]
    NotFound,
}

#[component(App<G>)]
fn app() -> Template<G> {
    template! {
        Router(RouterProps::new(HistoryIntegration::new(), |route: AppRoutes| {
            let t = match route {
                AppRoutes::Top { data } => template! {
                    pages::stories::Stories(data)
                },
                AppRoutes::New { data } => template! {
                    pages::stories::Stories(data)
                },
                AppRoutes::Best { data } => template! {
                    pages::stories::Stories(data)
                },
                AppRoutes::Show { data } => template! {
                    pages::stories::Stories(data)
                },
                AppRoutes::User { username: _, data } => template! {
                    pages::user::User(data)
                },
                AppRoutes::NotFound => template! {
                    "Page not found."
                },
            };
            template! {
                div(class="app") {
                    div(class="container mx-auto") {
                        components::header::Header()
                        (t)
                    }
                    components::footer::Footer()
                }
            }
        }))
    }
}

fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    console_log::init().expect("error initializing logger");

    sycamore::render(|| {
        template! {
            App()
        }
    });
}
