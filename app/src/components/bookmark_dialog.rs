use super::icon::*;
use leptonic::prelude::*;
use leptos::{ev::MouseEvent, logging, *};
use leptos_animated_for::AnimatedFor;
use serde::{Deserialize, Serialize};

use super::card::Card;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
struct BookmarkItem {
    id: i32,
    selected: bool,
    title: String,
    // #[serde(skip)]
    // view: View,
}

#[component]
pub fn BookmarkDialog(
    #[prop(into, optional)] is_open: OptionalMaybeSignal<bool>,
    #[prop(into)] on_close: Consumer<MouseEvent>,
) -> impl IntoView {
    let computed_style = move || {
        let mut style_str = r###"
            padding: 1.5em;
            position: absolute; 
            left: 0; right: 0; bottom: 0; 
            border-radius: 20px 20px 0px 0px;
            box-shadow: 4px 0px 4px 0px rgba(0, 0, 0, 0.25);
            overflow-y: auto;
            z-index: 2;
            background: #fff;
            transition: height 150ms ease-in-out;   
        "###
        .to_owned();
        if is_open.get() {
            style_str = style_str + "height: calc(100% - 5em);";
        } else {
            style_str = style_str + "height: 0;";
        }
        style_str
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
        items.set(vec![
            BookmarkItem {
                id: 1,
                selected: false,
                title: "Bookmark 1".to_string(),
            },
            BookmarkItem {
                id: 2,
                selected: false,
                title: "Bookmark 2".to_string(),
            },
            BookmarkItem {
                id: 3,
                selected: false,
                title: "Bookmark 3".to_string(),
            },
        ]);


    });

    view! {
        <Box
            style={move||computed_style()}
            >
            <Box style=r###"
                position: absolute;
                top: 0; left: 0; right: 0;
                padding: 0 1.5em;
                height: 4em;
                background: #fff;
                display: flex;
            "###>
                <H1 style="margin: auto 0;"> {"Bookmark"} </H1>
                <Button
                    style=r###"
                    background: transparent;
                    border: none;
                    color: #000;
                    position: absolute;
                    right: 1em; top: 1em;
                    "###
                    on_click=on_close
                >
                    <Icon style="margin-right:0;" width="32" height="32" icon=IconClose/>
                </Button>
            </Box>
            <Box style=r###"
                padding: 4em 0; 
                gap: 1em; display: grid; grid-auto-flow: column; justify-content: start; 
                background: transparent;
            "###
            >
                <AnimatedFor
                    each=move || items.get()
                    key=|item| item.id.clone()
                    children=move |item| {
                        view! {
                            <Card
                                id=item.id.to_string()
                                title=item.title.clone()
                                on_click=move |_| set_selected_card(item.id)
                            />
                        }
                    }
                />
            </Box>
        </Box>
    }
}
