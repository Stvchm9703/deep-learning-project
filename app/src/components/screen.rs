use leptonic::prelude::*;
use leptos::{ev::MouseEvent, *};

#[component]
pub fn Screen(
    #[prop(into, optional)] is_mask_open: OptionalMaybeSignal<bool>,
    #[prop(into)] on_mask_click: Consumer<MouseEvent>,
    children: Children,
) -> impl IntoView {
    view! {
      <Box
        style=r###"
        position: fixed;
        top: 0; left: 0; right: 0; bottom: 0;
        padding: 0 0 3.5em;
        "###
        >
        <Show
          when= move || is_mask_open.get()
        >

          <div class="mask" on:click=move |e| {
              // if !disabled.get_untracked() {
              e.stop_propagation();
              on_mask_click.consume(e);
              // }
          }></div>
        </Show>
        {children()}
      </Box>
    }
}
