use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::{fa_brands_icons, fa_solid_icons},
    Icon,
};
use dioxus_router::Link;

use crate::hooks::mode::{is_dark, mode};

pub fn Footer(cx: Scope) -> Element {
    log::info!("dark mode: {:?}", is_dark(&cx));

    let dark_mode = is_dark(&cx);

    cx.render(rsx! {
        div {
            div {
                class: "mt-8 flex space-x-4 justify-center font-semibold",
                Link {
                    class: "text-black dark:text-white hover:text-gray-800 dark:hover:text-gray-200",
                    to: "/",
                    "Home"
                }
                Link {
                    class: "text-black dark:text-white hover:text-gray-800 dark:hover:text-gray-200",
                    to: "/projects",
                    "Projects"
                }
                Link {
                    class: "text-black dark:text-white hover:text-gray-800 dark:hover:text-gray-200",
                    to: "/blog",
                    "Blog"
                }
                Link {
                    class: "text-black dark:text-white hover:text-gray dark:hover:text-gray-200",
                    to: "/about",
                    "About"
                }
            }
            div {
                class: "mt-3 flex space-x-4 justify-center",
                a {
                    class: "text-black dark:text-white hover:text-gray-800 dark:hover:text-gray-200",
                    href: "javascript:;",
                    onclick: move |_| {
                        mode(&cx, !dark_mode);
                        cx.needs_update();
                    },
                    if is_dark(&cx) {
                        rsx! { 
                            Icon {
                                height: 26,
                                width: 26,
                                icon: fa_solid_icons::FaSun
                            }
                         }
                    } else {
                        rsx! { 
                            Icon {
                                height: 26,
                                width: 26,
                                icon: fa_solid_icons::FaMoon
                            }
                         }
                    }
                }
                a {
                    class: "text-black dark:text-white hover:text-gray-800 dark:hover:text-gray-200",
                    href: "https://github.com/mrxiaozhuox",
                    Icon {
                        height: 26,
                        width: 26,
                        icon: fa_brands_icons::FaGithub
                    }
                }
                a {
                    class: "text-black dark:text-white hover:text-gray-800 dark:hover:text-gray-200",
                    href: "https://www.zhihu.com/people/mrxiao-zhuo-x",
                    Icon {
                        height: 26,
                        width: 26,
                        icon: fa_brands_icons::FaZhihu
                    }
                }
                a {
                    class: "text-black dark:text-white hover:text-gray-800 dark:hover:text-gray-200",
                    href: "https://www.instagram.com/mrxiaozhuox/",
                    Icon {
                        height: 26,
                        width: 26,
                        icon: fa_brands_icons::FaInstagram
                    }
                }
                a {
                    class: "text-black dark:text-white hover:text-gray-800 dark:hover:text-gray-200",
                    href: "https://twitter.com/mrxiaozhuox",
                    Icon {
                        height: 26,
                        width: 26,
                        icon: fa_brands_icons::FaTwitter
                    }
                }
            }
        }
        br {}
    })
}
