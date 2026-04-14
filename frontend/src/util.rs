use api::marble::Marble;
use gloo::utils::window;
use reqwest::Client;
use yew::prelude::*;

pub fn button_classes() -> Classes {
    classes!("border")
}

pub fn pluralize(word: impl AsRef<str>, quantity: f64) -> String {
    let pluralize = quantity != 1.0;

    format!("{}{}", word.as_ref(), if pluralize { "s" } else { "" })
}

pub fn with_origin(path: impl AsRef<str>) -> String {
    format!("{}{}", window().location().origin().unwrap(), path.as_ref())
}

pub async fn get_marbles(client: Client) -> Result<Vec<Marble>, reqwest::Error> {
    client
        .get(with_origin("/api/marbles"))
        .send()
        .await?
        .json::<Vec<Marble>>()
        .await
}
