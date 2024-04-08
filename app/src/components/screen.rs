use leptonic::prelude::*;
use leptos::{*};

#[component]
pub fn Screen(
    #[prop(into, optional)] is_mask_open: OptionalMaybeSignal<bool>,
    #[prop(into)] style: OptionalMaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let computed_style = move || {
        let mut style_str = r###"
            position: fixed;
            top: 0; left: 0; right: 0; bottom: 0;
            padding: 0 0 3.5em; 
            background: #ececec;
            overflow: hidden;
            "###
        .to_owned();
        if let style = style.get() {
            style_str = style_str + &style;
        }
        style_str
    };
    view! {
      <Box
        style=computed_style()
        >
        <Show when= move || is_mask_open.get() >
          <div class="mask" style="background: rgba(0,0,0,0.25); position:absolute; top:0; bottom:0; left:0; right:0;  backdrop-filter: blur(4px); z-index:1;"></div>
        </Show>
        {children()}
      </Box>
    }
}
