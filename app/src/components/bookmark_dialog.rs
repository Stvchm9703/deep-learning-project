use super::icon::*;
use leptonic::prelude::*;
use leptos::{ev::MouseEvent, logging, *};
use leptos_animated_for::AnimatedFor;
use serde::{Deserialize, Serialize};

use crate::data::{face_hair_style::*, face_result_set::get_all_hair_style};

use super::card::Card;

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
struct BookmarkItem {
    id: i32,
    selected: bool,
    title: String,
    snapshot_image: Option<String>,
}

// impl Default for BookmarkItem {
//     fn default() -> Self {
//         Self {
//             id: -1,
//             selected: false,
//             title: "".to_string(),
//             snapshot_image: None,
//         }
//     }
// }

impl BookmarkItem {
    fn from_face_hair_style(face_hair_style: &FaceHairStyle) -> Self {
        Self {
            id: face_hair_style.uid as i32,
            selected: false,
            title: face_hair_style.style.to_string(),
            snapshot_image: Some(face_hair_style.image_url.to_string()),
        }
    }
}

#[component]
pub fn BookmarkDialog(
    #[prop(into, optional)] is_open: OptionalMaybeSignal<bool>,
    #[prop(into)] on_close: Consumer<MouseEvent>,
) -> impl IntoView {
    let computed_class = move || {
        if is_open.get() {
            return "bookmark-dialog bookmark-dialog--open";
        }
        "bookmark-dialog"
    };

    let items = create_rw_signal(Vec::<BookmarkItem>::new());
    let set_selected_card = move |id: i32| {
        logging::log!("set_selected_card: {:?}", id);
        items.update(|item_list| {
            item_list.iter_mut().for_each(|item| {
                item.selected = item.id == id;
            });
        });

        for item in items.get() {
            logging::log!("item: {:#?}", item);
        }
    };

    create_effect(move |_| {
        logging::log!("is_open: {:?}", is_open.get());

        items.set(
            get_all_hair_style()
                .iter()
                .map(|style| BookmarkItem::from_face_hair_style(style.clone()))
                .collect::<Vec<BookmarkItem>>(),
        );
    });

    view! {
        <Box class=move||computed_class()>
            <Box class="bookmark-dialog__content">
                <H1 class="bookmark-dialog__header"> {"Bookmark"} </H1>
                <Button class="bookmark-dialog__close-btn" on_click=on_close>
                    <Icon width="32" height="32" icon=ICON_CLOSE/>
                </Button>
            </Box>
            <Box class="bookmark-dialog__card-container">
                <AnimatedFor
                    each=move || items.get()
                    key=|item| item.id.clone()
                    children=move |item| {
                        view! {
                            <Card
                                id=item.id.to_string()
                                title=item.title.clone()
                                on_click=move |_| set_selected_card(item.id)
                                image_url=item.snapshot_image.clone()
                            />
                        }
                    }
                />
            </Box>
        </Box>
    }
}
