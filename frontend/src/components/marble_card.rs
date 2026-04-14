use api::marble::Marble;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MarbleCardProps {
    pub marble: Marble,

    #[prop_or_default]
    pub classes: Classes,
}

#[function_component]
pub fn MarbleCard(MarbleCardProps { marble, classes }: &MarbleCardProps) -> Html {
    html! {
        <div
            class={ classes!("border", "flex", "flex-col", "p-4", "rounded-xl", "break-all", classes.clone()) }
            style={format!("border-color: rgb({} {} {})", (marble.color.red * 255.0) as u8, (marble.color.green * 255.0) as u8, (marble.color.blue * 255.0) as u8)}
        >
            if let Some(image_path) = &marble.maybe_image_path {
                <img src={ format!("/api/images/{}", image_path.to_str().unwrap()) } />
            }
            <h1 class="text-xl">{ &marble.name }</h1>
        </div>
    }
}
