

use super::icon::*;
use leptonic::prelude::*;
use leptos::{ev::MouseEvent, *};

#[component]
fn SettingMenuItem(
    #[prop(into)] on_click: Consumer<MouseEvent>,
    children: Children,
) -> impl IntoView {
    view! {
      <div
        class="setting-menu-item"
        on:click=move|e| on_click.consume(e)>
        {children()}
      </div>
    }
}

#[component]
pub fn SettingMenu(
    #[prop(into, optional)] is_open: OptionalMaybeSignal<bool>,
    #[prop(into)] on_close: Consumer<MouseEvent>,
) -> impl IntoView {
    // let style_str = create_rw_signal("".to_owned());
    let style_str = move || {
        if is_open.get() {
            return "height:100%;".to_owned();
        }
        "".to_owned()
    };

    view! {
        <Box class="setting-menu" style={move|| style_str()}>
            <SettingMenuItem on_click=on_close>
                <Icon width="32" height="32" icon=ICON_ACCOUNT/>
                <span>Account</span>
            </SettingMenuItem>
             <SettingMenuItem on_click=on_close>
                <Icon width="32" height="32" icon=ICON_ABORT/>
                <span>About</span>
            </SettingMenuItem>
            <SettingMenuItem on_click=on_close>
                <Icon width="32" height="32" icon=ICON_HELP/>
                <span>Help</span>
            </SettingMenuItem>
              <SettingMenuItem on_click=on_close>
                <Icon width="32" height="32" icon=ICON_SETTING/>
                <span>Setting</span>
            </SettingMenuItem>
        </Box>
    }
}
