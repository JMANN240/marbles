use serde::Serialize;
use yew::prelude::*;

use crate::util::button_classes;

#[derive(Copy, Clone, Default, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ButtonType {
    Submit,
    Reset,

    #[default]
    Button,
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Html,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub r#type: ButtonType,
}

#[function_component]
pub fn Button(
    ButtonProps {
        children,
        class,
        onclick,
        disabled,
        r#type,
    }: &ButtonProps,
) -> Html {
    html! {
        <button
            class={ classes!(button_classes(), class.clone()) }
            onclick={onclick}
            disabled={*disabled}
            type={serde_plain::to_string(r#type).unwrap()}
        >
            { children.clone() }
        </button>
    }
}
