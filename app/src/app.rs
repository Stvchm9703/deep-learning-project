use js_sys;
use leptonic::prelude::*;
use leptos::{server_fn::client::browser, *};
use leptos_meta::{Meta, Title};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

use app_ui::components::navigation_bar::NavigationBar;
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
    let (camera_device, set_camera_device) = create_signal(String::new());


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

    view! {
        <Meta name="charset" content="UTF-8"/>
        <Meta name="description" content="Leptonic Tauri template"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="theme-color" content="#e66956"/>

        <Title text="Leptonic Tauri template"/>

        <Root default_theme=LeptonicTheme::default()>
            <Box style="display: flex; flex-direction: column; align-items: center; padding: 1em; min-height: 100%; min-width: 100%">
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
               camera_display: {  move || camera_device.get() }
            </Box>
            <NavigationBar />
        </Root>
    }
}
