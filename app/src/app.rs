use app_ui::js_bind::on_trigger_upload;
use js_sys;
use leptonic::prelude::*;
use leptos::*;
use leptos_meta::{Meta, Title};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

use app_ui::components::{
    bookmark_dialog::BookmarkDialog, face_analysis_drawer::FaceAnalysisDrawer,
    navigation_bar::NavigationBar, screen::Screen, setting_menu::SettingMenu,
    web_cam_render::CameraCanvas,
};

use app_ui::{
    js_bind::{capture_and_predict, echo},
    tauri_bind::{attachConsole, info, invoke, scan},
};
use web_sys::*;
// import { trace, info, error, attachConsole } from '@tauri-apps/plugin-log';

// #[wasm_bindgen(module = "/public/index.bundle.js")]

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[derive(Serialize, Deserialize)]
struct ScanOptions<'a> {
    camera_direction: &'a str,
    formats: Vec<&'a str>,
    windowed: bool,
}

#[component]
pub fn App() -> impl IntoView {
    tracing::info!("Welcome to Leptonic");
    let _cargo_version = env!("CARGO_PKG_VERSION");
    let _browser_version = js_sys::eval("window.navigator.userAgent")
        .unwrap()
        .as_string()
        .unwrap();
    let (name, _set_name) = create_signal(String::new());
    let (greet_msg, set_greet_msg) = create_signal(String::new());

    let _greet = move || {
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

    let (_count, _set_count) = create_signal(0);

    let _greet_msg_display = move || greet_msg.get();

    let (is_booking_open, set_booking_open) = create_signal(false);
    let (is_face_analysis_open, set_face_analysis_open) = create_signal(false);

    // let on_booking_click = move |_| {
    //     set_face_analysis_open.set(false);
    //     set_booking_open.set(!is_booking_open.get());
    // };

    let on_face_analysis_click = move |_| {
        // spawn_local(async move {
        set_booking_open.set(false);
        set_face_analysis_open.set(!is_face_analysis_open.get());
        // capture_and_predict();
        capture_and_predict();
        // });
        //
        echo("world");
    };

    let is_mask_open =
        MaybeSignal::derive(move || is_booking_open.get() || is_face_analysis_open.get());

    let (is_bookmark_open, set_bookmark_open) = create_signal(false);
    let bookmark_close = move |_| set_bookmark_open.set(false);

    let (is_setting_open, set_setting_open) = create_signal(false);
    let setting_close = move |_| set_setting_open.set(false);
    // let (setting_menu_position, set_setting_menu_position) = create_signal(0.0);

    let _scan_qr_code = move || {
        spawn_local(async move {
            set_greet_msg.set("requesting".to_owned());
            info(to_value("scanning qr code").unwrap()).await;
            let args = to_value(
                r#"{
                cameraDirection: "front",
                windowed: false
            }"#,
            )
            .unwrap();
            set_greet_msg.set(args.as_string().unwrap());
            let result = scan(args).await;
            set_greet_msg.set(result.as_string().unwrap());
        });
    };

    let on_result_updated = move |result: Event| {
        set_face_analysis_open.set(true);
        logging::log!("on_result_updated in app.rs ");
        let value = result
            .target()
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();

        logging::log!("result updated: {}", value);
    };

    // let detech = create_rw_signal(JsValue::NULL);
    // create_effect(move |_| {
    //     spawn_local(async move {
    //         let detech_inst = attachConsole().await;
    //         detech.set(detech_inst);
    //         info(to_value("hwllow").unwrap()).await;
    //     });
    // });

    view! {
       <Meta name="charset" content="UTF-8"/>
       <Meta name="description" content="Leptonic Tauri template"/>
       <Meta name="viewport" content="width=device-width, initial-scale=1.0, user-scalable=0"/>
       <Meta name="theme-color" content="#e66956"/>

       <Title text="Leptonic Tauri template"/>

       <Root default_theme=LeptonicTheme::default()>
       <Screen is_mask_open  style="display:flex; flex-direction: row; justify-content: flex-start; align-items: flex-start;">
            <FaceAnalysisDrawer is_face_analysis_open on_close=move|_| set_face_analysis_open.set(false)  />
            <Drawer
                side=DrawerSide::Left
                shown=is_booking_open
                style="height:100%; width: 424px; background-color: #fff; position:relative;"
            >
                Booking
            </Drawer>
            <Box class="view" style="width: 100%; transition: all 0.2s ease-in-out;">
                <CameraCanvas on_result_updated=on_result_updated />
            </Box>
            <div id="logs"></div>
        </Screen>
        <BookmarkDialog is_open=is_bookmark_open on_close=bookmark_close />
        <SettingMenu is_open=is_setting_open on_close=setting_close />
        <NavigationBar
            on_goto_main_page_click = move |_| {
                set_booking_open.set(false);
                set_face_analysis_open.set(false);
                capture_and_predict();
                // set_face_analysis_open.set(true);
            }
            on_upload_click= move |_| {
                set_booking_open.set(false);
                set_face_analysis_open.set(false);
                on_trigger_upload();
                // set_face_analysis_open.set(true);
            }
            is_face_analysis_open
            on_face_analysis_click=on_face_analysis_click
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
