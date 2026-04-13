use yew::prelude::*;
use yew_nav::NavLink;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct QuantumNavLinkProps<R: PartialEq> {
    pub to: R,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn QuantumNavLink<R: Routable + 'static>(
    QuantumNavLinkProps {
        to,
        classes,
        children,
    }: &QuantumNavLinkProps<R>,
) -> Html {
    html! {
        <NavLink<R>
            classes={classes!("border-y", "border-t-transparent", classes.clone())}
            inactive_classes={classes!("border-b-transparent")}
            active_classes={classes!("border-b")}
            to={to.clone()}
        >
            { children.clone() }
        </NavLink<R>>
    }
}
