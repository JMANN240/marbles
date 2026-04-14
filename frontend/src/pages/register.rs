use api::{RegisterRequest, TokenRequest};
use gloo::storage::{LocalStorage, Storage};
use reqwest::Client;
use web_sys::{HtmlInputElement, wasm_bindgen::JsCast};
use yew::prelude::*;
use yew_nav::use_hide_nav_menu;
use yew_router::hooks::use_navigator;

use crate::{
    Route,
    components::{
        button::{Button, ButtonType},
        token_provider::{TokenAction, TokenContext},
    },
    util::with_origin,
};

#[function_component]
pub fn RegisterPage() -> Html {
    use_hide_nav_menu(());

    let navigator = use_navigator().unwrap();
    let token_reducer = use_context::<TokenContext>().expect("no token context found");

    let error_text_state = use_state(Option::default);

    let username_state = use_state(String::default);
    let password_state = use_state(String::default);
    let confirm_password_state = use_state(String::default);

    let on_username_input = {
        let username_state = username_state.clone();

        Callback::from(move |event: InputEvent| {
            if let Some(username_input) = event
                .target()
                .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
            {
                username_state.set(username_input.value());
            }
        })
    };

    let on_password_input = {
        let password_state = password_state.clone();

        Callback::from(move |event: InputEvent| {
            if let Some(password_input) = event
                .target()
                .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
            {
                password_state.set(password_input.value());
            }
        })
    };

    let on_confirm_password_input = {
        let confirm_password_state = confirm_password_state.clone();

        Callback::from(move |event: InputEvent| {
            if let Some(confirm_password_input) = event
                .target()
                .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
            {
                confirm_password_state.set(confirm_password_input.value());
            }
        })
    };

    let on_submit = {
        let error_text_state = error_text_state.clone();

        let username_state = username_state.clone();
        let password_state = password_state.clone();
        let confirm_password_state = confirm_password_state.clone();

        Callback::from(move |event: SubmitEvent| {
            let navigator = navigator.clone();
            let token_reducer = token_reducer.clone();

            let error_text_state = error_text_state.clone();

            let username_state = username_state.clone();
            let password_state = password_state.clone();
            let confirm_password_state = confirm_password_state.clone();

            event.prevent_default();

            wasm_bindgen_futures::spawn_local(async move {
                let client = Client::new();

                let register_request = RegisterRequest {
                    username: (*username_state).clone(),
                    password: (*password_state).clone(),
                    confirm_password: (*confirm_password_state).clone(),
                };

                let register_response = client
                    .post(with_origin("/api/register"))
                    .json(&register_request)
                    .send()
                    .await
                    .unwrap();

                if register_response.status().is_success() {
                    let token_request = TokenRequest {
                        username: (*username_state).clone(),
                        password: (*password_state).clone(),
                    };

                    let token_response = client
                        .post(with_origin("/api/token"))
                        .json(&token_request)
                        .send()
                        .await
                        .unwrap();

                    let token_response_status = token_response.status();
                    let token_response_text = token_response.text().await.unwrap();

                    if token_response_status.is_success() {
                        LocalStorage::set("token", &token_response_text).unwrap();

                        token_reducer.dispatch(TokenAction::Set(token_response_text.clone()));

                        navigator.push(&Route::Root);
                    } else {
                        error_text_state.set(Some(token_response_text));
                    }
                } else {
                    let register_response_text = register_response.text().await.unwrap();

                    error_text_state.set(Some(register_response_text));
                }
            });
        })
    };

    html! {
        <main class="flex flex-col items-center p-8">
            <div class="border p-4 w-full sm:w-64">
                <form class="flex flex-col gap-2" onsubmit={on_submit}>
                    if let Some(error_text) = &*error_text_state {
                        <p class="text-red-500">{error_text}</p>
                    }
                    <input class="outline-offset-1 focus:outline-1 border p-1" value={(*username_state).clone()} oninput={on_username_input} type="text" name="username" placeholder="Username" required=true />
                    <input class="outline-offset-1 focus:outline-1 border p-1" value={(*password_state).clone()} oninput={on_password_input} type="password" name="password" placeholder="Password" required=true />
                    <input class="outline-offset-1 focus:outline-1 border p-1" value={(*confirm_password_state).clone()} oninput={on_confirm_password_input} type="password" name="confirm-password" placeholder="Confirm Password" required=true />
                    <Button r#type={ButtonType::Submit}>{ "Register" }</Button>
                </form>
            </div>
        </main>
    }
}
