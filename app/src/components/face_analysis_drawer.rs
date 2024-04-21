use super::card::Card;
use super::icon::*;
use leptonic::prelude::*;
use leptos::{ev::MouseEvent, *};
use leptos_animated_for::AnimatedFor;


#[component]
pub fn FaceAnalysisContentLoading() -> impl IntoView {
    view! {
    <div class="face-analysis-content_loading" >
        <div class="face-analysis-content_loading__container">
            <svg
                class="frame"
                width="200"
                height="200"
                viewBox="0 0 200 200"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
            >
                <path
                d="M100 137.5C92.5832 137.5 85.333 135.301 79.1661 131.18C72.9993 127.06 68.1928 121.203 65.3545 114.351C62.5162 107.498 61.7736 99.9584 63.2206 92.6841C64.6675 85.4098 68.239 78.728 73.4835 73.4835C78.728 68.239 85.4098 64.6675 92.6841 63.2206C99.9584 61.7736 107.498 62.5162 114.351 65.3545C121.203 68.1928 127.06 72.9993 131.18 79.1661C135.301 85.333 137.5 92.5832 137.5 100C137.489 109.942 133.534 119.474 126.504 126.504C119.474 133.535 109.942 137.489 100 137.5ZM100 75C95.0555 75 90.222 76.4662 86.1107 79.2133C81.9995 81.9603 78.7952 85.8648 76.903 90.4329C75.0108 95.0011 74.5157 100.028 75.4804 104.877C76.445 109.727 78.826 114.181 82.3223 117.678C85.8186 121.174 90.2732 123.555 95.1227 124.52C99.9723 125.484 104.999 124.989 109.567 123.097C114.135 121.205 118.04 118 120.787 113.889C123.534 109.778 125 104.945 125 100C124.993 93.3719 122.356 87.0174 117.669 82.3306C112.983 77.6438 106.628 75.0075 100 75Z"
                fill="#404040"
                />
                <path
                d="M100 162.5C91.2083 162.494 82.5165 160.636 74.4906 157.047C66.4647 153.459 59.2847 148.22 53.4181 141.672L62.7306 133.334C68.2909 139.551 75.3178 144.277 83.1715 147.084C91.0253 149.89 99.4563 150.688 107.697 149.404C115.938 148.119 123.726 144.794 130.353 139.731C136.981 134.667 142.236 128.026 145.641 120.413C149.046 112.799 150.492 104.455 149.848 96.14C149.204 87.8247 146.491 79.8026 141.954 72.8041C137.418 65.8055 131.203 60.0529 123.875 56.07C116.548 52.0871 108.34 50.0004 100 50V37.5C116.576 37.5 132.473 44.0848 144.194 55.8058C155.915 67.5268 162.5 83.424 162.5 100C162.5 116.576 155.915 132.473 144.194 144.194C132.473 155.915 116.576 162.5 100 162.5Z"
                fill="#404040"
                />
                <path
                d="M100 187.5C82.6941 187.5 65.7769 182.368 51.3876 172.754C36.9983 163.139 25.7832 149.473 19.1605 133.485C12.5379 117.496 10.8051 99.903 14.1813 82.9296C17.5575 65.9563 25.8911 50.3653 38.1282 38.1282C50.3653 25.8911 65.9563 17.5575 82.9296 14.1813C99.9029 10.8051 117.496 12.5379 133.485 19.1606C149.473 25.7832 163.139 36.9983 172.754 51.3876C182.368 65.7769 187.5 82.6942 187.5 100C187.474 123.198 178.247 145.439 161.843 161.843C145.439 178.247 123.198 187.474 100 187.5ZM100 25C85.1664 25 70.6659 29.3987 58.3322 37.6398C45.9986 45.8809 36.3856 57.5943 30.709 71.2988C25.0325 85.0032 23.5472 100.083 26.4411 114.632C29.335 129.18 36.4781 142.544 46.967 153.033C57.4559 163.522 70.8197 170.665 85.3682 173.559C99.9168 176.453 114.997 174.968 128.701 169.291C142.406 163.614 154.119 154.001 162.36 141.668C170.601 129.334 175 114.834 175 100C174.977 80.1158 167.068 61.0524 153.008 46.9921C138.948 32.9317 119.884 25.0227 100 25Z"
                fill="#404040"
                />
            </svg>
            <div class="looking-similar-result">
                "Looking Similar Result..."
            </div>
        </div>
    </div>
    }
}

#[component]
fn FaceAnalysisContent(
    #[prop(into)] title: MaybeSignal<String>,
    #[prop(into)] description: MaybeSignal<String>,
    #[prop(into)] cover_img: MaybeSignal<String>,
) -> impl IntoView {
    view! {
    <div class="face-analysis-content">
    <div class="face-analysis-content__header header">
      <img class="face-analysis-content__header-cover" src=cover_img />
      <div class="face-analysis-content__header-cover-mask"></div>
      <h1 class="face-analysis-content__header-title">{title.get()}</h1>
    </div>

        <div class="face-analysis-content__content content">
            <div class="face-analysis-content__description-block">
                <p class="face-analysis-content__description-content">
                    {description.get()}
                </p>
            </div>
            <div class="block face-analysis-content__similar-actor-content">
                <h2 class="face-analysis-content__similar-actor-content__title">{"Similar Actors"}</h2>
                <div class="face-analysis-content__similar-actor-content__slider">
                <AnimatedFor
                    each=move || (0..5)
                    key=|item| item.clone()
                    children=move |item| {
                        view! {
                            <Card
                                id=item.to_string()
                                title="Actor Name"
                                image_url="https://picsum.photos/150"
                                on_click=move |_| {}
                            />
                        }
                    }
                />

                </div>
                
            </div>


             <div class="block face-analysis-content__similar-actor-content">
                <h2 class="face-analysis-content__similar-actor-content__title">{"Recommend Hair Style"}</h2>
                <p class="face-analysis-content__similar-actor-content__sub-text">click to preview now</p>
                <div class="face-analysis-content__similar-actor-content__slider">
                <AnimatedFor
                    each=move || (0..5)
                    key=|item| item.clone()
                    children=move |item| {
                        view! {
                            <Card
                                id=item.to_string()
                                title="Actor Name"
                                image_url="https://picsum.photos/150"
                                on_click=move |_| {}
                            />
                        }
                    }
                />

                </div>
                
            </div>
        </div>
    </div>
    }
}

#[component]
pub fn FaceAnalysisDrawer(
    #[prop(into)] is_face_analysis_open: Signal<bool>,
    #[prop(into)] on_close: Consumer<MouseEvent>,
) -> impl IntoView {
    // let target = create_node_ref::<Div>();

    // on_click_outside(target, move |event| {
    //     log!("{:?}", event);
    // });

    // let is_ready = create_rw_signal(false);
    // let cb_data = create_resource(
    //     || (),
    //     |_| async move {
    //         let result = reqwest::get("https://api.github.com/users/defuncart")
    //             .await
    //             .unwrap()
    //             .json::<serde_json::Value>()
    //             .await
    //             .unwrap();
    //         log!("{:?}", result);
    //         return result;
    //     },
    // );
    // create_action(|_| async move{
    //     let cb_data = cb_data.clone();
    //     let result = cb_data.read().await;
    //     log!("{:?}", result);
    // });
    let sample_result = create_rw_signal(
        "    The heart-shaped face is characterized by a broader forehead and cheekbones
    that taper down to a narrow, pointed chin, resembling the silhouette of a
    heart when observed from the front. This facial shape often features high
    and defined cheekbones, creating a soft and feminine appearance. The
    forehead is usually the widest part of the face, gradually narrowing down
    towards the chin, which may have a slightly pointed or angular shape. People
    with heart-shaped faces often have delicate features and a youthful look.
    This face shape is versatile and can complement a variety of hairstyles and
    makeup looks, as well as different types of eyewear and accessories."
            .to_string(),
    );
    view! {
        <Drawer
            side=DrawerSide::Left
            shown=is_face_analysis_open
            style="height:100%; min-width: 424px; background-color: #fff; position:relative; z-index:2;"
        >

            <Button
                class="fill-white"
                style=r###"
                background: transparent;
                border: none;
                color: #fff;
                position: absolute;
                right: -3.25em; top: 0;
                "###
                on_click=on_close
            >
                <Icon style="margin-right:0; color:#fff; fill:#fff;" width="32" height="32" icon=ICON_CLOSE/>
            </Button>
            // content
            <Box class="content" style="
                position:absolute; top:0; bottom:0; left:0; right:0; 
                background:#fff; overflow-y:scroll;  
                scrollbar-width: none;
                ">
                <Transition
                    fallback=|| view!{ <FaceAnalysisContentLoading/> }
                >
                    <FaceAnalysisContent
                        title="hello world"
                        cover_img="https://picsum.photos/600/300"
                        description=sample_result
                    />
                </Transition>
            </Box>
        </Drawer>
    }
}
