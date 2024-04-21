use super::icon::*;
use leptonic::prelude::*;
use leptos::{ev::MouseEvent, *};

#[component]
pub fn Card(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] title: Option<String>,
    #[prop(into, optional)] image_url: Option<AttributeValue>,
    #[prop(into, optional)] _style: Option<AttributeValue>,
    #[prop(into)] on_click: Consumer<MouseEvent>,
    #[prop(into, optional)] selected: OptionalMaybeSignal<bool>,
    #[prop(into, optional)] _is_light: Option<bool>,
) -> impl IntoView {
    let (is_hovered, set_is_hovered) = create_signal(false);
    let is_active = move || selected.get() || is_hovered.get();

    let computed_class = move || {
        if is_active() {
            return "card__content card__content--is-active";
        }
        "card__content"
    };

    view! {
        <div
            id=id
            class="card"
            on:mouseenter=move |_| set_is_hovered.set(true)
            on:mouseleave=move |_| set_is_hovered.set(false)
            on:click=move |e| on_click.consume(e)
        >
            <div class={move||computed_class()}>
                <Show when= move || selected.get()>
                    <Icon class="card__icon" width="32" height="32" icon=ICON_BLUE_CHECK/>
                </Show>

                <img
                class="card__image"
                src=image_url />
            </div>
            <span class="card__title"> {title} </span>
        </div>
    }
}
