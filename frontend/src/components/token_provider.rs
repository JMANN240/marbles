use gloo::storage::{LocalStorage, Storage, errors::StorageError};
use yew::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token(pub Option<String>);

pub enum TokenAction {
    Set(String),
    Clear,
}

impl Reducible for Token {
    type Action = TokenAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            TokenAction::Set(token) => Token(Some(token)),
            TokenAction::Clear => Token(None),
        }
        .into()
    }
}

pub type TokenContext = UseReducerHandle<Token>;

#[derive(Properties, Debug, PartialEq)]
pub struct TokenProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn TokenProvider(props: &TokenProviderProps) -> Html {
    let token = use_reducer(|| {
        let token_result = LocalStorage::get::<String>("token");

        match token_result {
            Ok(token) => Token(Some(token)),
            Err(StorageError::KeyNotFound(_)) => Token(None),
            Err(StorageError::SerdeError(error)) => panic!("{error:?}"),
            Err(StorageError::JsError(error)) => panic!("{error:?}"),
        }
    });

    html! {
        <ContextProvider<TokenContext> context={token}>
            {props.children.clone()}
        </ContextProvider<TokenContext>>
    }
}
