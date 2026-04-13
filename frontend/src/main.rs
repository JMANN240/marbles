use components::token_provider::TokenProvider;
use pages::root::RootPage;
use yew::prelude::*;
use yew_nav::NavMenuStateProvider;
use yew_router::prelude::*;

use crate::{components::nav::navbar::QuantumNavBar, pages::{login::LoginPage, logout::LogoutPage, register::RegisterPage}};

pub mod components;
pub mod pages;
pub mod util;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Root,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/logout")]
    Logout,
    #[at("/marbles/:id")]
    Marble { id: i64 },
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Root => {
            html! { <RootPage /> }
        }
        Route::Login => {
            html! { <LoginPage /> }
        }
        Route::Register => {
            html! { <RegisterPage /> }
        }
        Route::Logout => {
            html! { <LogoutPage /> }
        }
        Route::Marble { id } => {
            html! { <RootPage /> }
        }
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <TokenProvider>
            <NavMenuStateProvider>
                <BrowserRouter>
                    <header class="border-b bg-inherit">
                        <QuantumNavBar />
                    </header>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </NavMenuStateProvider>
        </TokenProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
