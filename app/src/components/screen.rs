use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn Screen(
    #[prop(into, optional)] is_mask_open: OptionalMaybeSignal<bool>,
    #[prop(into, optional)] style: OptionalMaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let computed_style = move || {
        let style_content = style.get();
        {
            return style_content;
        }
    };
    view! {
      <Box
        class="screen"
        style={computed_style()}
        >
        <Show when= move || is_mask_open.get() >
          <div class="mask screen__mask"></div>
        </Show>
        {children()}
      </Box>
    }
}
