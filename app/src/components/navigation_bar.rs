use super::icon::*;
use leptonic::prelude::*;
use leptos::{ev::MouseEvent, logging, *};

#[component]
fn ButtonGotoMainPage(#[prop(into)] on_click: Consumer<MouseEvent>) -> impl IntoView {
    view! {
        <Button
            style="
                background: #588bd8;
                border-color: transparent;
                --button-border-radius: 50%;
                position: absolute;
                width: 70px; height: 70px;
                box-shadow: 0px 0px 8px 0px rgba(0, 0, 0, 0.25);
                left: calc(50% - 35px);
                bottom: 0px;
                z-index: 1;
            "
            on_click=on_click
        >
            <Icon width="40" height="40" icon=IconMainPage/>
    </Button>

    }
}

#[component]
fn MenuButton(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] active: OptionalMaybeSignal<bool>,
    #[prop(into)] on_click: Consumer<MouseEvent>,
    #[prop(into, optional)] _ref: Option<NodeRef<leptos::html::Button>>,
    children: Children,
) -> impl IntoView {
    // let Some(node_ref) = _ref;
    let style_set = r###"
                flex-shrink: 0;
                width: 40px;
                height: 40px;
                position: relative;
                background: #fff;
                border-color: transparent;
            "###;

    if let Some(node_ref) = _ref {
        return view! {
            <button
                class="leptonic-btn"
                _ref=node_ref id=id active=active
                style=style_set
                on:click=move|e|on_click.consume(e)
            >
                {children()}
            </button>
        };
    }
    view! {
        <button
            class="leptonic-btn"
            id=id active=active
            style=style_set
            on:click=move|e|on_click.consume(e)
        >
            {children()}
        </button>
    }
}

#[component]
pub fn NavigationBar(
    #[prop(into)] on_booking_click: Consumer<MouseEvent>,
    #[prop(into)] on_face_analysis_click: Consumer<MouseEvent>,
    #[prop(into)] on_bookmark_click: Consumer<MouseEvent>,
    #[prop(into)] on_setting_click: Consumer<MouseEvent>,
    #[prop(into)] is_booking_open: OptionalMaybeSignal<bool>,
    #[prop(into)] is_face_analysis_open: OptionalMaybeSignal<bool>,
    #[prop(into)] is_bookmark_open: OptionalMaybeSignal<bool>,
    #[prop(into)] is_setting_open: OptionalMaybeSignal<bool>,
) -> impl IntoView {
    let goto_main_page_fn = move |_| {
        spawn_local(async move {});
    };
    view! {
       <Stack
          spacing=Size::Em(2.0)
          orientation=StackOrientation::Horizontal
          style=r###"
                background: #ffffff; 
                align-items: center; 
                justify-content: center; 
                position: relative; 
                box-shadow: 0px 0px 8px 0px rgba(0, 0, 0, 0.25); 
                position: fixed; 
                bottom: 0; left: 0; right: 0; 
                box-shadow: 0px 0px 8px 0px rgba(0, 0, 0, 0.25);
                padding: 4px;
                z-index: 100;
              "###>
        <Box
            id="nav-bar-container"
            style=r###"
              background: transparent; 
              position: relative;
              width: 100%; height: 48px;
              min-width: 390px; max-width: 600px;
              flex-shrink: 0;
            "###
        >
            <ButtonGotoMainPage on_click=goto_main_page_fn />
            <Stack
                id="nav-bar-sub-btn-group"
                spacing=Size::Em(0.0)
                orientation=StackOrientation::Horizontal
                style=r###"
                    padding: 0px 16px;
                    display: flex;
                    flex-direction: row;
                    gap: 140px;
                    align-items: between;
                    justify-content: center;
                    position: absolute;
                    right: 0px; left: 0px; bottom: 0px; top: 0px;
                "###
            >
                <Stack
                    id="nav-bar-left-side"
                    spacing=Size::Em(0.0)
                    orientation=StackOrientation::Horizontal
                    style=r###"
                        display: flex;
                        align-items: center;
                        justify-content: space-between;
                        align-self: stretch;
                        flex: 1;
                        position: relative; 
                    "###
                >
                    <MenuButton
                        on_click=on_booking_click
                        active=is_booking_open
                    >
                        <Icon style="margin-right:0;" width="28" height="28" icon=IconNavBookingPage/>
                    </MenuButton>
                    <MenuButton 
                        on_click=on_face_analysis_click
                        active=is_face_analysis_open
                    >
                        <Icon style="margin-right:0;" width="28" height="28" icon=IconNavFaceAnalysisPage/>
                    </MenuButton>
                </Stack>
                <Stack
                    id="nav-bar-left-side"
                    spacing=Size::Em(0.0)
                    orientation=StackOrientation::Horizontal
                    style=r###"
                        display: flex;
                        align-items: center;
                        justify-content: space-between;
                        align-self: stretch;
                        flex: 1;
                        position: relative;
                    "###
                >
                    <MenuButton  on_click=on_bookmark_click active=is_bookmark_open>
                        <Icon style="margin-right:0;" width="28" height="28" icon=IconNavBookmarkPage/>
                    </MenuButton>
                    <MenuButton id="nav-setting" on_click=on_setting_click active=is_setting_open>
                        <Icon style="margin-right:0;" width="28" height="28" icon=IconNavSettingPage/>
                    </MenuButton>
                </Stack>

            </Stack>
        </Box>
       </Stack>
    }
}
