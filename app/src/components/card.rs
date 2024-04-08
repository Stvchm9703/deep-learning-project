use leptonic::prelude::*;
use leptos::{ev::MouseEvent, *};
use super::icon::*;

#[component]
pub fn Card(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] title: Option<String>,
    #[prop(into, optional)] image_url: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(into)] on_click: Consumer<MouseEvent>,
    #[prop(into, optional)] selected: OptionalMaybeSignal<bool>,
    #[prop(into, optional)] is_light: Option<bool>,
) -> impl IntoView {
    let (is_hovered, set_is_hovered) = create_signal(false);
    let is_active = move || selected.get() || is_hovered.get();
    let computed_style = move || {
        let mut style_str = r###"
            border-radius: 20px;
            border-style: solid;
            border-color: transparent;
            border-width: 2px;
            width: 150px;
            height: 150px;
            position: relative;
            overflow: hidden;
            transition: border-color 0.25s;
            "###
        .to_owned();
        if is_active() {
            style_str = style_str + "border-color: #588bd8;";
        }
       
        style_str
    };

    view! {
        <div style=r###"
            width: 150px;
            height: 180px;
            position: relative;
            display: flex; flex-direction: column; gap: 0.25em;
            "###
            id=id
            on:mouseenter=move |_| set_is_hovered.set(true)
            on:mouseleave=move |_| set_is_hovered.set(false)
            on:click=move |e| on_click.consume(e)
        >
            <div style={move||computed_style()}>
                <Show when= move || selected.get()>
                    <Icon style="position:absolute  ; right: 0.5em; top: 0.5em; overflow:visible; box-shadow: 0 0 4px #333; border-radius: 50%;" width="32" height="32" icon=IconBlueCheck/>
                </Show>

                <img style="
                position: absolute;
                height: 100%; width: 100%;
                object-fit: cover;
                " src=image_url />
            </div>
            <span style="font-size: 1.25em; padding: 0 0.25em;"> {title} </span>
        </div>
    }
}
