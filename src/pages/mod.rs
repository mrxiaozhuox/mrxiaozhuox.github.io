use dioxus::prelude::*;

use crate::components::footer::Footer;

pub mod _404;

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        section {
            class: "h-screen bg-cover bg-white dark:bg-gray-600",
            div {
                class: "flex h-full w-full items-center justify-center container mx-auto px-8",
                div {
                    class: "max-w-2xl text-center",
                    img {
                        class: "rounded-lg w-32 mb-4 mx-auto",
                        src: "https://avatars.githubusercontent.com/u/41265098?v=4"
                    }
                    h5 {
                        class: "text-2xl dark:text-white font-medium leading-tight mb-2",
                        "YuKun Liu"
                    }
                    p {
                        class: "text-gray-500 dark:text-gray-300",
                        "Computer Science Student | Keen on new technologies."
                    }

                    Footer {}
                }
            }
        }
    })
}