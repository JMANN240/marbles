
use api::marble::Marble;
use reqwest::Client;
use yew::prelude::*;
use yew_nav::use_hide_nav_menu;

use crate::{components::marble_card::MarbleCard, util::get_marbles};

#[function_component]
pub fn RootPage() -> Html {
    use_hide_nav_menu(());

    let marbles_state: UseStateHandle<Option<Vec<Marble>>> = use_state(Option::default);

    use_effect_with((), {
        let marbles_state = marbles_state.clone();

        move |_| {
            let marbles_state = marbles_state.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let marbles = get_marbles(Client::new()).await.unwrap();

                marbles_state.set(Some(marbles));
            });
        }
    });

    html! {
        <main class="grid gap-2 sm:gap-4 auto-rows-fr grid-cols-2 sm:grid-cols-8 p-4 sm:p-8">
            if let Some(marbles) = &*marbles_state {
                {
                    marbles.iter().map(|marble| {
                        html! {
                            <MarbleCard marble={marble.clone()} />
                        }
                    }).collect::<Html>()
                }
            }
        </main>
    }
}
