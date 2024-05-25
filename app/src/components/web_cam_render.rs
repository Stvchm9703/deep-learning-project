use leptonic::callback::Consumer;
use leptos::{
    component, create_effect, create_node_ref,
    ev::{change, Event},
    html::Input,
    logging, spawn_local, view, IntoView,
};
use leptos_meta::{Link, Script};
use leptos_use::use_event_listener;

use crate::js_bind::init_video_process;

#[component]
pub fn CameraCanvas(
    // capture the event when the result is updated
    #[prop(into)] on_result_updated: Consumer<Event>,
) -> impl IntoView {
    let result_ref = create_node_ref::<Input>();

    create_effect(|_| {
        spawn_local(async move {
            let _ = init_video_process().await;
        });
    });

    let d_on_result_updated = move |e| {
        logging::log!("d_on_result_updated: {:?}", e);
        on_result_updated.consume(e);
    };
    use_event_listener(result_ref, change, d_on_result_updated);

    view! {
        <div class="render_view">
            <video id="render_view_video"></video>
            <input id="render_view_input" type="file" accept="image/*"/>
            <input _ref=result_ref id="render_result" type="hidden"  />
            <canvas id="render_view_canvas"> </canvas>
            <Script src={"./public/jimp.js"}></Script>
            <Link rel="preload" href={"./public/end2end_b2.onnx"}></Link>
        </div>
    }
}
