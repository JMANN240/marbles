use gloo::storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_nav::use_hide_nav_menu;
use yew_router::hooks::use_navigator;

use crate::{
    Route,
    components::token_provider::{TokenAction, TokenContext},
};

#[function_component]
pub fn LogoutPage() -> Html {
    use_hide_nav_menu(());

    let navigator = use_navigator().unwrap();
    let token_reducer = use_context::<TokenContext>().expect("no token context found");

    token_reducer.dispatch(TokenAction::Clear);
    LocalStorage::delete("token");
    navigator.replace(&Route::Root);

    html! {}
}
