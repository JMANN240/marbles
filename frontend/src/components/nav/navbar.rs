use yew::prelude::*;
use yew_nav::{NavMenuButton, NavMenuStateContext};
use yew_router::prelude::*;

use crate::{
    Route,
    components::{
        nav::navlink::QuantumNavLink,
        token_provider::TokenContext,
    },
};

#[function_component]
pub fn QuantumNavBar() -> Html {
    let token_reducer = use_context::<TokenContext>().expect("no token context found");
    let nav_menu_state_context =
        use_context::<NavMenuStateContext>().expect("no nav menu state context found");

    html! {
        <nav class="flex justify-between items-center relative px-4 py-2 bg-inherit">
            <div class="flex items-center gap-4">
                <Link<Route> to={Route::Root}>
                    <h1 class="text-xl">
                        { "Quantum Marble Racing" }
                    </h1>
                </Link<Route>>
            </div>
            <div class={classes!("flex", "items-center", "gap-4", "max-sm:hidden")}>
                if token_reducer.0.is_some() {
                    <QuantumNavLink<Route> to={Route::Logout}>
                        <h2>{ "Logout" }</h2>
                    </QuantumNavLink<Route>>
                } else {
                    <QuantumNavLink<Route> to={Route::Login}>
                        <h2>{ "Login" }</h2>
                    </QuantumNavLink<Route>>
                    <QuantumNavLink<Route> to={Route::Register}>
                        <h2>{ "Register" }</h2>
                    </QuantumNavLink<Route>>
                }
            </div>
            <NavMenuButton classes="sm:hidden">
                { "Menu" }
            </NavMenuButton>
            <div class={classes!("absolute", "top-full", "left-0", "right-0", "bg-inherit", "sm:hidden", "overflow-y-hidden", "duration-200", "border-b", if nav_menu_state_context.shown { "h-64 border-t" } else { "h-0" })}>
                <div class={classes!("flex", "flex-col", "p-2", "gap-2")}>
                    if token_reducer.0.is_some() {
                        <QuantumNavLink<Route> to={Route::Logout}>
                            <h2>{ "Logout" }</h2>
                        </QuantumNavLink<Route>>
                    } else {
                        <QuantumNavLink<Route> to={Route::Login}>
                            <h2>{ "Login" }</h2>
                        </QuantumNavLink<Route>>
                        <QuantumNavLink<Route> to={Route::Register}>
                            <h2>{ "Register" }</h2>
                        </QuantumNavLink<Route>>
                    }
                </div>
            </div>
        </nav>
    }
}
