use js_sys;
use leptonic::prelude::*;
use leptos::*;
use leptos_meta::{Meta, Title};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

use app_ui::components::{
    bookmark_dialog::BookmarkDialog, navigation_bar::NavigationBar, screen::Screen,
    setting_menu::SettingMenu,
};

#[wasm_bindgen]
extern "C" {
    // basic tauri invoke function
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    tracing::info!("Welcome to Leptonic");
    let cargo_version = env!("CARGO_PKG_VERSION");
    let browser_version = js_sys::eval("window.navigator.userAgent")
        .unwrap()
        .as_string()
        .unwrap();
    let (name, set_name) = create_signal(String::new());
    let (greet_msg, set_greet_msg) = create_signal(String::new());

    let greet = move || {
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = to_value(&GreetArgs { name: &name }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            let new_msg = invoke("greet", args.clone()).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    let (count, set_count) = create_signal(0);

    let greet_msg_display = move || greet_msg.get();

    let on_mask_click = move |_| {
        tracing::info!("Mask clicked");
    };

    let (is_bookmark_open, set_bookmark_open) = create_signal(false);
    let bookmark_close = move |_| set_bookmark_open.set(false);

    let (is_setting_open, set_setting_open) = create_signal(false);
    let setting_close = move |_| set_setting_open.set(false);
    let (setting_menu_position, set_setting_menu_position) = create_signal( 0.0);
    view! { 
       <Meta name="charset" content="UTF-8"/>
       <Meta name="description" content="Leptonic Tauri template"/>
       <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
       <Meta name="theme-color" content="#e66956"/>

       <Title text="Leptonic Tauri template"/>

       <Root default_theme=LeptonicTheme::default()>
       <Screen on_mask_click>
           <H2>"Welcome to Leptonic"</H2>
           cargo : { cargo_version } <br/>
           { browser_version }
           <Stack spacing=Size::Em(2.0)>
               <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(1.2)>
                   <div>"Count: " {move || count.get()}</div>
                   <Button on_click=move|_| set_count.update(|c| *c += 1)>
                       "Increase"
                   </Button>
               </Stack>
               <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(1.2)>
                   <div>
                       <TextInput get=name set=set_name placeholder="Enter a name..."/>
                       <span><b>{ greet_msg_display }</b></span>
                   </div>
                   <Button on_click=move|_| greet()>
                       "Greet"
                   </Button>
               </Stack>
           </Stack>

            <Button on_click=move|_| set_bookmark_open.set(true)> "Open Bookmark Dialog" </Button>
       </Screen>
       <BookmarkDialog is_open=is_bookmark_open on_close=bookmark_close />
       <SettingMenu is_open=is_setting_open on_close=setting_close />
       <NavigationBar
            is_bookmark_open
            on_bookmark_click=move |_| {
                set_setting_open.set(false);
                set_bookmark_open.set(!is_bookmark_open.get());
            }
            is_setting_open
            on_setting_click=move |_| {
                set_bookmark_open.set(false);
                set_setting_open.set( !is_setting_open.get() );
            }
       />

       </Root>
    }
}
