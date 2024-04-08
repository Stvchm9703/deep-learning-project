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
      style=r###"
        border-style: solid;
        border-color: rgba(0, 0, 0, 0.25);
        border-width: 0px 0px 1px 0px;
        border-radius: 0px;
        padding: 10px;
        display: flex;
        flex-direction: row;
        gap: 25px;
        align-items: center;
        justify-content: flex-start;
        align-self: stretch;
        flex-shrink: 0;
        height: 70px;
        position: relative;
        overflow: hidden;
        
        "### 
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
    let computed_style = move || {
        let mut style_str = r###"
            padding: 12px 16px;
            position: absolute; 
            right: 0; bottom: 0; 
            border-radius: 8px 8px 0px 0px;
            box-shadow: -2px 0px 8px 0px rgba(0, 0, 0, 0.25);
            overflow-y: auto;
            z-index: 2;
            background: #fff;
            transition: height 150ms ease-in-out;   
            min-width: 390px;
            display: flex;
            flex-direction: column;
            gap: 10px;
            align-items: flex-start;
            justify-content: flex-start;
            overflow: hidden;
            max-height: 585px;
        "###
        .to_owned();
        if is_open.get() {
            style_str = style_str + "height: 100%;"
        } else {
            style_str = style_str + "height: 0;";
        }
        style_str
    };

    view! {
        <Box style={move||computed_style()}>
            <SettingMenuItem on_click=on_close>
                <Icon width="32" height="32" icon=IconAccount/>
                <span>Account</span>
            </SettingMenuItem>
             <SettingMenuItem on_click=on_close>
                <Icon width="32" height="32" icon=IconAbout/>
                <span>About</span>
            </SettingMenuItem>
            <SettingMenuItem on_click=on_close>
                <Icon width="32" height="32" icon=IconHelp/>
                <span>Help</span>
            </SettingMenuItem>
              <SettingMenuItem on_click=on_close>
                <Icon width="32" height="32" icon=IconSetting/>
                <span>Setting</span>
            </SettingMenuItem>
        </Box>
    }
}
