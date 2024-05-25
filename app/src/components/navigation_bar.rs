use super::icon::*;

use leptonic::prelude::*;
use leptos::{ev::MouseEvent, *};
#[component]
pub fn ButtonGotoMainPage(#[prop(into)] on_click: Consumer<MouseEvent>) -> impl IntoView {
    view! {
    <Button
        class="button-goto-main-page"
        on_click=on_click
    >
        <Icon width="40" height="40" icon=ICON_MAIN_PAGE/>
    </Button>
    }
}

#[component]
pub fn MenuButton(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] active: OptionalMaybeSignal<bool>,
    #[prop(into)] on_click: Consumer<MouseEvent>,
    #[prop(into, optional)] _ref: Option<NodeRef<leptos::html::Button>>,
    children: Children,
) -> impl IntoView {
    if let Some(node_ref) = _ref {
        return view! {
            <button
                class="leptonic-btn navigation-bar__menu-button"
                _ref=node_ref id=id active=active
                on:click:undelegated=move|e|on_click.consume(e)
            >
                {children()}
            </button>
        };
    }
    view! {
        <button
             class="leptonic-btn navigation-bar__menu-button "
            id=id active=active
            on:click:undelegated=move|e|on_click.consume(e)
        >
            {children()}
        </button>
    }
}

#[component]
pub fn NavigationBar(
    #[prop(into)] on_upload_click: Consumer<MouseEvent>,
    #[prop(into)] on_face_analysis_click: Consumer<MouseEvent>,
    #[prop(into)] on_bookmark_click: Consumer<MouseEvent>,
    #[prop(into)] on_setting_click: Consumer<MouseEvent>,
    #[prop(into)] on_goto_main_page_click: Consumer<MouseEvent>,

    #[prop(into)] is_face_analysis_open: OptionalMaybeSignal<bool>,
    #[prop(into)] is_bookmark_open: OptionalMaybeSignal<bool>,
    #[prop(into)] is_setting_open: OptionalMaybeSignal<bool>,
) -> impl IntoView {
    // let goto_main_page_fn = move |_| {};
    view! {
       <Stack
          spacing=Size::Em(2.0)
          orientation=StackOrientation::Horizontal
          class="navigation-bar"
        >
        <Box
            id="nav-bar-container"
            class="navigation-bar__container"
        >
            <ButtonGotoMainPage on_click=on_goto_main_page_click />
            <Stack
                id="nav-bar-sub-btn-group"
                spacing=Size::Em(0.0)
                orientation=StackOrientation::Horizontal
                class="navigation-bar__sub-btn-group"
            >
                <Stack
                    id="nav-bar-left-side"
                    spacing=Size::Em(0.0)
                    orientation=StackOrientation::Horizontal
                    class="navigation-bar__side-btn-group"
                >

                    <MenuButton
                        on_click=on_face_analysis_click
                        active=is_face_analysis_open
                    >
                        <Icon width="28" height="28" icon=ICON_NAV_FACE_ANALYSIS_PAGE/>
                    </MenuButton>
                    <MenuButton
                        on_click=on_upload_click
                    >
                        <Icon width="28" height="28" icon=ICON_NAV_BOOKING_PAGE/>
                    </MenuButton>
                </Stack>
                <Stack
                    id="nav-bar-left-side"
                    spacing=Size::Em(0.0)
                    orientation=StackOrientation::Horizontal
                    class="navigation-bar__side-btn-group"
                >
                    <MenuButton  on_click=on_bookmark_click active=is_bookmark_open>
                        <Icon width="28" height="28" icon=ICON_NAV_BOOKMARK_PAGE/>
                    </MenuButton>
                    <MenuButton id="nav-setting" on_click=on_setting_click active=is_setting_open>
                        <Icon width="28" height="28" icon=ICON_NAV_SETTING_PAGE/>
                    </MenuButton>
                </Stack>

            </Stack>
        </Box>
       </Stack>
    }
}
