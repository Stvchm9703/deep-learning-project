use leptonic::prelude::*;
use leptos::*;
use leptos_meta::{Meta, Title};

use leptos_use::use_window;
// use leptos_router::{create_query_signal, use_query_map};
use wasm_bindgen::prelude::*;

use app_ui::components::{
    bookmark_dialog::BookmarkDialog, cover_view::MainCoverView,
    face_analysis_drawer::FaceAnalysisDrawer, navigation_bar::NavigationBar, screen::Screen,
    setting_menu::SettingMenu, web_cam_render::CameraCanvas,
};

use app_ui::js_bind::{capture_and_predict, get_url_query_map, init_model};

use web_sys::*;

// #[wasm_bindgen]
fn on_trigger_upload() {
    let input_elem = document().get_element_by_id("render_view_input").unwrap();
    input_elem.dyn_into::<HtmlInputElement>().unwrap().click();
}

#[component]
pub fn App() -> impl IntoView {
    tracing::info!("Welcome to Leptonic");
    let (is_model_inited, set_model_inited) = create_signal(false);
    // let root = create_node_ref();
    let (is_app_acted, set_app_acted) = create_signal(false);

    let (is_booking_open, set_booking_open) = create_signal(false);
    let (is_face_analysis_open, set_face_analysis_open) = create_signal(false);

    let on_face_analysis_click = move |_| {
        // spawn_local(async move {
        set_booking_open.set(false);
        set_face_analysis_open.set(!is_face_analysis_open.get());
        capture_and_predict();
    };

    let is_mask_open =
        MaybeSignal::derive(move || is_booking_open.get() || is_face_analysis_open.get());

    let (is_bookmark_open, set_bookmark_open) = create_signal(false);
    let bookmark_close = move |_| set_bookmark_open.set(false);

    let (is_setting_open, set_setting_open) = create_signal(false);
    let setting_close = move |_| set_setting_open.set(false);
    // let (setting_menu_position, set_setting_menu_position) = create_signal(0.0);

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

    // let window_inst = use_window();

    // let url_string = get_url_query_map();

    // let dummy_result = ReadSignal::new(get_url_query_map());
    create_effect(move |_| {
        let query_map = get_url_query_map();
        let result = query_map.all("result");
        if result.is_some() {
            let result_list = result.unwrap().get(0).unwrap().to_string();
            match result_list.as_str() {
                "square" | "oblong" | "round" | "heart" | "rectangular" | "oval" => {
                    document()
                        .get_element_by_id("render_result")
                        .expect("should be rendered")
                        .dyn_into::<HtmlInputElement>()
                        .expect("should be input element")
                        .set_value(&result_list);
                    set_face_analysis_open.set(true);
                }
                _ => {}
            }
        };
    });

    create_effect(move |_| {
        spawn_local(async move {
            let _ = init_model().await;
            set_model_inited.set(true);
        });
    });

    view! {
       <Meta name="charset" content="UTF-8"/>
       <Meta name="description" content="Leptonic Tauri template"/>
       <Meta name="viewport" content="width=device-width, initial-scale=1.0, user-scalable=0"/>
       <Meta name="theme-color" content="#e66956"/>

       <Title text="Leptonic Tauri template"/>

       <Root default_theme=LeptonicTheme::default()>

       <Screen is_mask_open style="display:flex; flex-direction: row; justify-content: flex-start; align-items: flex-start;">
            <FaceAnalysisDrawer is_face_analysis_open on_close=move|_| set_face_analysis_open.set(false)  />
            <Drawer
                side=DrawerSide::Left
                shown=is_booking_open
                style="height:100%; width: 424px; background-color: #fff; position:relative;"
            >
                Booking
            </Drawer>

            <Show when= move || !is_app_acted.get()>
                <MainCoverView></MainCoverView>
            </Show>
            // <Show when= move || is_app_acted.get()>
            <Box class="view" style=move||{
                if is_app_acted.get(){
                    "width: 100%; transition: all 0.2s ease-in-out;"
                }
                else{
                    "width: 100%; transition: all 0.2s ease-in-out; display:none;"
                }
            }>
                <CameraCanvas on_result_updated=on_result_updated />
            </Box>
            // </Show>
        </Screen>
        <BookmarkDialog is_open=is_bookmark_open on_close=bookmark_close />
        <SettingMenu is_open=is_setting_open on_close=setting_close />
        <NavigationBar
            on_goto_main_page_click = move |_| {
                set_app_acted.set(true);
                set_booking_open.set(false);
                set_face_analysis_open.set(false);
                capture_and_predict();
            }
            on_upload_click= move |_| {
                set_app_acted.set(false);
                set_booking_open.set(false);
                set_face_analysis_open.set(false);
                on_trigger_upload();
                // set_face_analysis_open.set(true);
            }
            is_face_analysis_open
            on_face_analysis_click=on_face_analysis_click
            is_bookmark_open
            on_bookmark_click=move |_| {
                // set_app_acted.set(true);
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
