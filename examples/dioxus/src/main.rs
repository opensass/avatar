// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use avatar::dioxus::{Avatar, Count, Fallback, Group, Image};
use avatar::{Color, Overlap, Size, Variant};
use dioxus::prelude::*;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: "https://unpkg.com/tailwindcss@2.2.19/dist/tailwind.min.css" }
        document::Stylesheet { href: asset!("assets/main.css") }
        LandingPage {}
    }
}

#[component]
fn UserIcon() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            width: "20",
            height: "20",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            "aria-hidden": "true",
            "focusable": "false",
            path { d: "M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" }
            circle { cx: "12", cy: "7", r: "4" }
        }
    }
}

#[component]
fn Example1() -> Element {
    rsx! {
        Avatar { aria_label: "Ferris Prophet",
            Image { src: "https://i.pravatar.cc/300?u=1", alt: "Ferris Prophet" }
            Fallback { "FP" }
        }
    }
}

#[component]
fn Example2() -> Element {
    rsx! {
        Avatar { aria_label: "No Image User",
            Fallback { "NA" }
        }
    }
}

#[component]
fn Example3() -> Element {
    rsx! {
        div { style: "display: flex; gap: 12px; align-items: center; flex-wrap: wrap;",
            Avatar { size: Some(Size::Xs), aria_label: "Extra Small", Fallback { "XS" } }
            Avatar { size: Some(Size::Sm), aria_label: "Small",       Fallback { "SM" } }
            Avatar { size: Some(Size::Md), aria_label: "Medium",      Fallback { "MD" } }
            Avatar { size: Some(Size::Lg), aria_label: "Large",       Fallback { "LG" } }
            Avatar { size: Some(Size::Xl), aria_label: "Extra Large", Fallback { "XL" } }
            Avatar { size: Some(Size::Xxl), aria_label: "2X Large",   Fallback { "2XL" } }
        }
    }
}

#[component]
fn Example4() -> Element {
    rsx! {
        div { style: "display: flex; gap: 12px; align-items: center; flex-wrap: wrap;",
            Avatar { aria_label: "Default",
                Fallback { color: Color::Default, "DF" }
            }
            Avatar { aria_label: "Accent",
                Fallback { color: Color::Accent, "AC" }
            }
            Avatar { aria_label: "Success",
                Fallback { color: Color::Success, "SC" }
            }
            Avatar { aria_label: "Warning",
                Fallback { color: Color::Warning, "WR" }
            }
            Avatar { aria_label: "Danger",
                Fallback { color: Color::Danger, "DG" }
            }
        }
    }
}

#[component]
fn Example5() -> Element {
    rsx! {
        div { style: "display: flex; gap: 12px; align-items: center; flex-wrap: wrap;",
            Avatar { aria_label: "Accent solid",
                Fallback { color: Color::Accent, variant: Variant::Default, "SO" }
            }
            Avatar { aria_label: "Accent soft",
                Fallback { color: Color::Accent, variant: Variant::Soft, "SF" }
            }
            Avatar { aria_label: "Success soft",
                Fallback { color: Color::Success, variant: Variant::Soft, "SS" }
            }
            Avatar { aria_label: "Danger soft",
                Fallback { color: Color::Danger, variant: Variant::Soft, "DS" }
            }
        }
    }
}

#[component]
fn Example6() -> Element {
    rsx! {
        Avatar { aria_label: "Delayed Fallback",
            Image { src: "https://invalid-url-to-trigger-fallback.example.com/img.jpg", alt: "Broken image" }
            Fallback { delay_ms: 600, "FB" }
        }
    }
}

#[component]
fn Example7() -> Element {
    rsx! {
        div { style: "display: flex; gap: 12px; align-items: center;",
            Avatar { size: Some(Size::Lg), aria_label: "User One",
                Image { src: "https://i.pravatar.cc/300?u=2", alt: "User 1" }
                Fallback { color: Color::Accent, "U1" }
            }
            Avatar { size: Some(Size::Lg), aria_label: "User Two",
                Image { src: "https://i.pravatar.cc/300?u=3", alt: "User 2" }
                Fallback { color: Color::Success, "U2" }
            }
            Avatar { size: Some(Size::Lg), aria_label: "User Three",
                Image { src: "https://i.pravatar.cc/300?u=4", alt: "User 3" }
                Fallback { color: Color::Warning, "U3" }
            }
        }
    }
}

#[component]
fn Example8() -> Element {
    rsx! {
        Avatar {
            aria_label: "Custom Border Avatar",
            style: "border: 3px solid #7c3aed;",
            size: Some(Size::Lg),
            Image { src: "https://i.pravatar.cc/300?u=5", alt: "Custom style user" }
            Fallback { color: Color::Accent, "CS" }
        }
    }
}

#[component]
fn Example9() -> Element {
    rsx! {
        Avatar {
            aria_label: "Square Avatar",
            style: "border-radius: 8px;",
            size: Some(Size::Lg),
            Fallback { color: Color::Success, style: "border-radius: 8px;", "SQ" }
        }
    }
}

#[component]
fn Example10() -> Element {
    rsx! {
        Avatar { size: Some(Size::Lg), aria_label: "Callback Avatar",
            Image {
                src: "https://i.pravatar.cc/300?u=6",
                alt: "Callback example user",
                on_load: Some(Callback::new(|_| {})),
                on_error: Some(Callback::new(|_: String| {})),
            }
            Fallback { "CB" }
        }
    }
}

#[component]
fn Example11() -> Element {
    let colors = [
        ("Accent", Color::Accent, "https://i.pravatar.cc/150?u=20"),
        ("Default", Color::Default, "https://i.pravatar.cc/150?u=21"),
        ("Success", Color::Success, "https://i.pravatar.cc/150?u=22"),
        ("Warning", Color::Warning, "https://i.pravatar.cc/150?u=23"),
        ("Danger", Color::Danger, "https://i.pravatar.cc/150?u=24"),
    ];
    rsx! {
        div { class: "w-full max-w-4xl overflow-x-auto bg-black text-white p-8 rounded-xl",
            role: "region",
            aria_label: "Avatar Matrix Overview",
            table { class: "w-full text-left border-collapse",
                caption { class: "sr-only", "Avatar style matrix, colors x variants x content types" }
                thead {
                    tr {
                        th { scope: "col", class: "p-4 border-b border-white/10 text-white/50 font-medium", "" }
                        for (name, _, _) in colors.iter() {
                            th { scope: "col", class: "p-4 border-b border-white/10 text-white/50 font-medium text-center", "{name}" }
                        }
                    }
                }
                tbody {
                    tr {
                        th { scope: "row", class: "p-4 border-b border-white/10 text-white/50", "letter" }
                        for (_, color, _) in colors.iter() {
                            td { class: "p-4 border-b border-white/10 text-center",
                                Avatar { size: Some(Size::Lg),
                                    Fallback { color: *color, variant: Variant::Default, "AG" }
                                }
                            }
                        }
                    }
                    tr {
                        th { scope: "row", class: "p-4 border-b border-white/10 text-white/50", "letter soft" }
                        for (_, color, _) in colors.iter() {
                            td { class: "p-4 border-b border-white/10 text-center",
                                Avatar { size: Some(Size::Lg),
                                    Fallback { color: *color, variant: Variant::Soft, "AG" }
                                }
                            }
                        }
                    }
                    tr {
                        th { scope: "row", class: "p-4 border-b border-white/10 text-white/50", "icon" }
                        for (_, color, _) in colors.iter() {
                            td { class: "p-4 border-b border-white/10 text-center",
                                Avatar { size: Some(Size::Lg),
                                    Fallback { color: *color, variant: Variant::Default, UserIcon {} }
                                }
                            }
                        }
                    }
                    tr {
                        th { scope: "row", class: "p-4 border-b border-white/10 text-white/50", "icon soft" }
                        for (_, color, _) in colors.iter() {
                            td { class: "p-4 border-b border-white/10 text-center",
                                Avatar { size: Some(Size::Lg),
                                    Fallback { color: *color, variant: Variant::Soft, UserIcon {} }
                                }
                            }
                        }
                    }
                    tr {
                        th { scope: "row", class: "p-4 border-b border-white/10 text-white/50", "img" }
                        for (_, _, url) in colors.iter() {
                            td { class: "p-4 border-b border-white/10 text-center",
                                Avatar { size: Some(Size::Lg), aria_label: "User photo",
                                    Image { src: *url, alt: "User" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn GroupExample1() -> Element {
    rsx! {
        Group { size: Size::Lg, aria_label: "Project team members",
            Avatar { aria_label: "Member 1",
                Image { src: "https://i.pravatar.cc/150?u=a042581f4e29026024d", alt: "Member 1" }
                Fallback { color: Color::Accent, "M1" }
            }
            Avatar { aria_label: "Member 2",
                Image { src: "https://i.pravatar.cc/150?u=a04258a2462d826712d", alt: "Member 2" }
                Fallback { color: Color::Success, "M2" }
            }
            Avatar { aria_label: "Member 3",
                Image { src: "https://i.pravatar.cc/150?u=a042581f4e29026704d", alt: "Member 3" }
                Fallback { color: Color::Warning, "M3" }
            }
            Avatar { aria_label: "Member 4",
                Image { src: "https://i.pravatar.cc/150?u=a04258114e29026302d", alt: "Member 4" }
                Fallback { color: Color::Danger, "M4" }
            }
            Avatar { aria_label: "Member 5",
                Image { src: "https://i.pravatar.cc/150?u=a04258114e29026702d", alt: "Member 5" }
                Fallback { "M5" }
            }
            Avatar { aria_label: "Member 6",
                Image { src: "https://i.pravatar.cc/150?u=a04258114e29026708c", alt: "Member 6" }
                Fallback { "M6" }
            }
        }
    }
}

#[component]
fn GroupExample2() -> Element {
    rsx! {
        Group { size: Size::Lg, max: 3, total: 6, aria_label: "Team with overflow (3 shown)",
            Avatar { aria_label: "Member 1",
                Image { src: "https://i.pravatar.cc/150?u=a042581f4e29026024d", alt: "Member 1" }
                Fallback { color: Color::Accent, "M1" }
            }
            Avatar { aria_label: "Member 2",
                Image { src: "https://i.pravatar.cc/150?u=a04258a2462d826712d", alt: "Member 2" }
                Fallback { color: Color::Success, "M2" }
            }
            Avatar { aria_label: "Member 3",
                Image { src: "https://i.pravatar.cc/150?u=a042581f4e29026704d", alt: "Member 3" }
                Fallback { color: Color::Warning, "M3" }
            }
            Count {}
        }
    }
}

#[component]
fn GroupExample3() -> Element {
    rsx! {
        Group { size: Size::Md, is_grid: true, aria_label: "Team grid layout",
            Avatar { aria_label: "Member 1",
                Image { src: "https://i.pravatar.cc/150?u=a042581f4e29026024d", alt: "Member 1" }
                Fallback { color: Color::Accent, "M1" }
            }
            Avatar { aria_label: "Member 2",
                Image { src: "https://i.pravatar.cc/150?u=a04258a2462d826712d", alt: "Member 2" }
                Fallback { color: Color::Success, "M2" }
            }
            Avatar { aria_label: "Member 3",
                Image { src: "https://i.pravatar.cc/150?u=a042581f4e29026704d", alt: "Member 3" }
                Fallback { color: Color::Warning, "M3" }
            }
            Avatar { aria_label: "Member 4",
                Image { src: "https://i.pravatar.cc/150?u=a04258114e29026302d", alt: "Member 4" }
                Fallback { color: Color::Danger, "M4" }
            }
            Avatar { aria_label: "Member 5",
                Image { src: "https://i.pravatar.cc/150?u=a04258114e29026702d", alt: "Member 5" }
                Fallback { "M5" }
            }
            Avatar { aria_label: "Member 6",
                Image { src: "https://i.pravatar.cc/150?u=a04258114e29026708c", alt: "Member 6" }
                Fallback { "M6" }
            }
            Avatar { aria_label: "Member 7",
                Image { src: "https://i.pravatar.cc/150?u=a042581f4e29026024e", alt: "Member 7" }
                Fallback { "M7" }
            }
        }
    }
}

#[component]
fn GroupExample4() -> Element {
    rsx! {
        Group { size: Size::Lg, overlap: Overlap::Ring, aria_label: "Team with ring overlap",
            Avatar { aria_label: "Member 1",
                Image { src: "https://i.pravatar.cc/150?u=a042581f4e29026024d", alt: "Member 1" }
                Fallback { color: Color::Accent, "M1" }
            }
            Avatar { aria_label: "Member 2",
                Image { src: "https://i.pravatar.cc/150?u=a04258a2462d826712d", alt: "Member 2" }
                Fallback { color: Color::Success, "M2" }
            }
            Avatar { aria_label: "Member 3",
                Image { src: "https://i.pravatar.cc/150?u=a042581f4e29026704d", alt: "Member 3" }
                Fallback { color: Color::Warning, "M3" }
            }
            Avatar { aria_label: "Member 4",
                Image { src: "https://i.pravatar.cc/150?u=a04258114e29026302d", alt: "Member 4" }
                Fallback { "M4" }
            }
        }
    }
}

#[component]
fn GroupExample5() -> Element {
    rsx! {
        Group { size: Size::Lg, max: 3, total: 5, aria_label: "Team with explicit count badge",
            Avatar { aria_label: "Member 1",
                Image { src: "https://picsum.photos/seed/m1/150/150", alt: "Member 1" }
                Fallback { color: Color::Accent, "M1" }
            }
            Avatar { aria_label: "Member 2",
                Image { src: "https://picsum.photos/seed/m2/150/150", alt: "Member 2" }
                Fallback { color: Color::Success, "M2" }
            }
            Avatar { aria_label: "Member 3",
                Image { src: "https://picsum.photos/seed/m3/150/150", alt: "Member 3" }
                Fallback { color: Color::Warning, "M3" }
            }
            Count {
                color: Some(Color::Danger),
                variant: Some(Variant::Soft),
                aria_label: "2 additional members",
            }
        }
    }
}

#[component]
pub fn LandingPage() -> Element {
    rsx! {
        div { class: "min-h-screen flex flex-col items-center justify-center",
              style: "color: #5e5c7f; background-color: #303030; font-family: 'Rubik', sans-serif; overflow-x: hidden;",

            h1 { class: "text-3xl font-bold mb-8 text-white", "Avatar Dioxus Examples" }

            section { aria_labelledby: "basic-heading", class: "w-full max-w-6xl mb-12",
                h2 { id: "basic-heading", class: "text-xl font-semibold text-white mb-6", "Basic Components" }
                div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8",

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Image + Fallback" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use avatar::dioxus::{{
    Avatar, Image, Fallback,
}};
use dioxus::prelude::*;

#[component]
fn BasicAvatar() -> Element {{
    rsx! {{
        Avatar {{ aria_label: "Ferris Prophet",
            Image {{
                src: "https://i.pravatar.cc/300?u=1",
                alt: "Ferris Prophet",
            }}
            Fallback {{ "FP" }}
        }}
    }}
}}"# }
                        Example1 {}
                    }

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Initials Only" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use avatar::dioxus::{{Avatar, Fallback}};
use dioxus::prelude::*;

#[component]
fn InitialsAvatar() -> Element {{
    rsx! {{
        Avatar {{ aria_label: "No Image User",
            Fallback {{ "NA" }}
        }}
    }}
}}"# }
                        Example2 {}
                    }

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Sizes" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use avatar::dioxus::{{Avatar, Fallback}};
use avatar::Size;
use dioxus::prelude::*;

#[component]
fn SizedAvatars() -> Element {{
    rsx! {{
        div {{ style: "display: flex; gap: 8px;",
            Avatar {{ size: Some(Size::Xs), Fallback {{ "XS" }} }}
            Avatar {{ size: Some(Size::Sm), Fallback {{ "SM" }} }}
            Avatar {{ size: Some(Size::Md), Fallback {{ "MD" }} }}
            Avatar {{ size: Some(Size::Lg), Fallback {{ "LG" }} }}
            Avatar {{ size: Some(Size::Xl), Fallback {{ "XL" }} }}
            Avatar {{ size: Some(Size::Xxl), Fallback {{ "2XL" }} }}
        }}
    }}
}}"# }
                        Example3 {}
                    }

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Colors" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use avatar::dioxus::{{Avatar, Fallback}};
use avatar::Color;
use dioxus::prelude::*;

#[component]
fn ColoredAvatars() -> Element {{
    rsx! {{
        div {{ style: "display: flex; gap: 8px;",
            Avatar {{ aria_label: "Default",
                Fallback {{ color: Color::Default, "DF" }}
            }}
            Avatar {{ aria_label: "Accent",
                Fallback {{ color: Color::Accent, "AC" }}
            }}
            Avatar {{ aria_label: "Success",
                Fallback {{ color: Color::Success, "SC" }}
            }}
            Avatar {{ aria_label: "Warning",
                Fallback {{ color: Color::Warning, "WR" }}
            }}
            Avatar {{ aria_label: "Danger",
                Fallback {{ color: Color::Danger, "DG" }}
            }}
        }}
    }}
}}"# }
                        Example4 {}
                    }

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Solid vs Soft" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use avatar::dioxus::{{Avatar, Fallback}};
use avatar::{{Color, Variant}};
use dioxus::prelude::*;

#[component]
fn VariantAvatars() -> Element {{
    rsx! {{
        div {{ style: "display: flex; gap: 8px;",
            Avatar {{ aria_label: "Accent solid",
                Fallback {{
                    color: Color::Accent,
                    variant: Variant::Default,
                    "SO"
                }}
            }}
            Avatar {{ aria_label: "Accent soft",
                Fallback {{
                    color: Color::Accent,
                    variant: Variant::Soft,
                    "SF"
                }}
            }}
            Avatar {{ aria_label: "Success soft",
                Fallback {{
                    color: Color::Success,
                    variant: Variant::Soft,
                    "SS"
                }}
            }}
            Avatar {{ aria_label: "Danger soft",
                Fallback {{
                    color: Color::Danger,
                    variant: Variant::Soft,
                    "DS"
                }}
            }}
        }}
    }}
}}"# }
                        Example5 {}
                    }

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Delayed Fallback" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use avatar::dioxus::{{
    Avatar, Image, Fallback,
}};
use dioxus::prelude::*;

#[component]
fn DelayedFallback() -> Element {{
    rsx! {{
        Avatar {{ aria_label: "Delayed",
            Image {{
                src: "https://invalid.example.com/img.jpg",
                alt: "Broken link",
            }}
            Fallback {{ delay_ms: 600, "FB" }}
        }}
    }}
}}"# }
                        Example6 {}
                    }

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Multiple Avatars" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use avatar::dioxus::{{
    Avatar, Image, Fallback,
}};
use avatar::{{Color, Size}};
use dioxus::prelude::*;

#[component]
fn MultiAvatar() -> Element {{
    rsx! {{
        div {{ style: "display: flex; gap: 12px;",
            Avatar {{ size: Some(Size::Lg), aria_label: "User One",
                Image {{ src: "https://i.pravatar.cc/300?u=2", alt: "U1" }}
                Fallback {{ color: Color::Accent, "U1" }}
            }}
            Avatar {{ size: Some(Size::Lg), aria_label: "User Two",
                Image {{ src: "https://i.pravatar.cc/300?u=3", alt: "U2" }}
                Fallback {{ color: Color::Success, "U2" }}
            }}
            Avatar {{ size: Some(Size::Lg), aria_label: "User Three",
                Image {{ src: "https://i.pravatar.cc/300?u=4", alt: "U3" }}
                Fallback {{ color: Color::Warning, "U3" }}
            }}
        }}
    }}
}}"# }
                        Example7 {}
                    }

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Custom Border" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use avatar::dioxus::{{
    Avatar, Image, Fallback,
}};
use avatar::{{Color, Size}};
use dioxus::prelude::*;

#[component]
fn StyledAvatar() -> Element {{
    rsx! {{
        Avatar {{
            aria_label: "Custom Border",
            style: "border: 3px solid #7c3aed;",
            size: Some(Size::Lg),
            Image {{ src: "https://i.pravatar.cc/300?u=5", alt: "User" }}
            Fallback {{ color: Color::Accent, "CS" }}
        }}
    }}
}}"# }
                        Example8 {}
                    }

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Square Shape" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use avatar::dioxus::{{Avatar, Fallback}};
use avatar::{{Color, Size}};
use dioxus::prelude::*;

#[component]
fn SquareAvatar() -> Element {{
    rsx! {{
        Avatar {{
            aria_label: "Square Avatar",
            style: "border-radius: 8px;",
            size: Some(Size::Lg),
            Fallback {{
                color: Color::Success,
                style: "border-radius: 8px;",
                "SQ"
            }}
        }}
    }}
}}"# }
                        Example9 {}
                    }

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Callbacks" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use avatar::dioxus::{{
    Avatar, Image, Fallback,
}};
use avatar::Size;
use dioxus::prelude::*;

#[component]
fn CallbackAvatar() -> Element {{
    rsx! {{
        Avatar {{ size: Some(Size::Lg), aria_label: "Callback Avatar",
            Image {{
                src: "https://i.pravatar.cc/300?u=6",
                alt: "Callback user",
                on_load: Some(Callback::new(|_| {{}})),
                on_error: Some(Callback::new(|_: String| {{}})),
            }}
            Fallback {{ "CB" }}
        }}
    }}
}}"# }
                        Example10 {}
                    }
                }
            }

            section { aria_labelledby: "group-heading", class: "w-full max-w-6xl mb-12",
                h2 { id: "group-heading", class: "text-xl font-semibold text-white mb-6", "Group" }
                div { class: "flex flex-col gap-8",

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Basic Group (Stacked)" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use avatar::dioxus::{{
    Avatar, Group, Image, Fallback,
}};
use avatar::{{Color, Size}};
use dioxus::prelude::*;

#[component]
fn BasicGroup() -> Element {{
    rsx! {{
        Group {{ size: Size::Lg, aria_label: "Project team members",
            Avatar {{ aria_label: "Member 1",
                Image {{
                    src: "https://i.pravatar.cc/150?u=a042581f4e29026024d",
                    alt: "Member 1",
                }}
                Fallback {{ color: Color::Accent, "M1" }}
            }}
            Avatar {{ aria_label: "Member 2",
                Image {{
                    src: "https://i.pravatar.cc/150?u=a04258a2462d826712d",
                    alt: "Member 2",
                }}
                Fallback {{ color: Color::Success, "M2" }}
            }}
            Avatar {{ aria_label: "Member 3",
                Image {{
                    src: "https://i.pravatar.cc/150?u=a042581f4e29026704d",
                    alt: "Member 3",
                }}
                Fallback {{ color: Color::Warning, "M3" }}
            }}
            Avatar {{ aria_label: "Member 4",
                Image {{
                    src: "https://i.pravatar.cc/150?u=a04258114e29026302d",
                    alt: "Member 4",
                }}
                Fallback {{ color: Color::Danger, "M4" }}
            }}
            Avatar {{ aria_label: "Member 5",
                Image {{
                    src: "https://i.pravatar.cc/150?u=a04258114e29026702d",
                    alt: "Member 5",
                }}
                Fallback {{ "M5" }}
            }}
            Avatar {{ aria_label: "Member 6",
                Image {{
                    src: "https://i.pravatar.cc/150?u=a04258114e29026708c",
                    alt: "Member 6",
                }}
                Fallback {{ "M6" }}
            }}
        }}
    }}
}}"# }
                        GroupExample1 {}
                    }

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Group with Max + Auto Count" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use avatar::dioxus::{{
    Avatar, Group, Image, Fallback,
}};
use avatar::{{Color, Size}};
use dioxus::prelude::*;

#[component]
fn MaxGroup() -> Element {{
    rsx! {{
        Group {{
            size: Size::Lg,
            max: 3,
            total: 6,
            aria_label: "Team, 3 shown with overflow",
            Avatar {{ aria_label: "M1",
                Image {{ src: "https://i.pravatar.cc/150?u=a042581f4e29026024d", alt: "M1" }}
                Fallback {{ color: Color::Accent, "M1" }}
            }}
            Avatar {{ aria_label: "M2",
                Image {{ src: "https://i.pravatar.cc/150?u=a04258a2462d826712d", alt: "M2" }}
                Fallback {{ color: Color::Success, "M2" }}
            }}
            Avatar {{ aria_label: "M3",
                Image {{ src: "https://i.pravatar.cc/150?u=a042581f4e29026704d", alt: "M3" }}
                Fallback {{ color: Color::Warning, "M3" }}
            }}
            Count {{}}
        }}
    }}
}}"# }
                        GroupExample2 {}
                    }

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Grid Layout" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use avatar::dioxus::{{
    Avatar, Group, Image, Fallback,
}};
use avatar::{{Color, Size}};
use dioxus::prelude::*;

#[component]
fn GridGroup() -> Element {{
    rsx! {{
        Group {{
            size: Size::Md,
            is_grid: true,
            aria_label: "Team grid layout",
            Avatar {{ aria_label: "M1",
                Image {{ src: "https://i.pravatar.cc/150?u=a042581f4e29026024d", alt: "M1" }}
                Fallback {{ color: Color::Accent, "M1" }}
            }}
            Avatar {{ aria_label: "M2",
                Image {{ src: "https://i.pravatar.cc/150?u=a04258a2462d826712d", alt: "M2" }}
                Fallback {{ color: Color::Success, "M2" }}
            }}
            Avatar {{ aria_label: "M3",
                Image {{ src: "https://i.pravatar.cc/150?u=a042581f4e29026704d", alt: "M3" }}
                Fallback {{ color: Color::Warning, "M3" }}
            }}
            Avatar {{ aria_label: "M4",
                Image {{ src: "https://i.pravatar.cc/150?u=a04258114e29026302d", alt: "M4" }}
                Fallback {{ color: Color::Danger, "M4" }}
            }}
        }}
    }}
}}"# }
                        GroupExample3 {}
                    }

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Ring Overlap" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use avatar::dioxus::{{
    Avatar, Group, Image, Fallback,
}};
use avatar::{{Color, Overlap, Size}};
use dioxus::prelude::*;

#[component]
fn RingGroup() -> Element {{
    rsx! {{
        Group {{
            size: Size::Lg,
            overlap: Overlap::Ring,
            aria_label: "Team with ring overlap",
            Avatar {{ aria_label: "M1",
                Image {{ src: "https://i.pravatar.cc/150?u=a042581f4e29026024d", alt: "M1" }}
                Fallback {{ color: Color::Accent, "M1" }}
            }}
            Avatar {{ aria_label: "M2",
                Image {{ src: "https://i.pravatar.cc/150?u=a04258a2462d826712d", alt: "M2" }}
                Fallback {{ color: Color::Success, "M2" }}
            }}
            Avatar {{ aria_label: "M3",
                Image {{ src: "https://i.pravatar.cc/150?u=a042581f4e29026704d", alt: "M3" }}
                Fallback {{ color: Color::Warning, "M3" }}
            }}
            Avatar {{ aria_label: "M4",
                Image {{ src: "https://i.pravatar.cc/150?u=a04258114e29026302d", alt: "M4" }}
                Fallback {{ "M4" }}
            }}
        }}
    }}
}}"# }
                        GroupExample4 {}
                    }

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Explicit Count Badge" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use avatar::dioxus::{{
    Avatar, Group, Count,
    Image, Fallback,
}};
use avatar::{{Color, Size, Variant}};
use dioxus::prelude::*;

#[component]
fn ExplicitCountGroup() -> Element {{
    rsx! {{
        Group {{
            size: Size::Lg,
            max: 3,
            total: 5,
            aria_label: "Team with count badge",
            Avatar {{ aria_label: "M1",
                Image {{ src: "https://picsum.photos/seed/m1/150/150", alt: "M1" }}
                Fallback {{ color: Color::Accent, "M1" }}
            }}
            Avatar {{ aria_label: "M2",
                Image {{ src: "https://picsum.photos/seed/m2/150/150", alt: "M2" }}
                Fallback {{ color: Color::Success, "M2" }}
            }}
            Avatar {{ aria_label: "M3",
                Image {{ src: "https://picsum.photos/seed/m3/150/150", alt: "M3" }}
                Fallback {{ color: Color::Warning, "M3" }}
            }}
            Count {{
                color: Some(Color::Danger),
                variant: Some(Variant::Soft),
                aria_label: "2 additional members",
            }}
        }}
    }}
}}"# }
                        GroupExample5 {}
                    }
                }
            }

            section { aria_labelledby: "matrix-heading", class: "w-full max-w-6xl mb-12",
                h2 { id: "matrix-heading", class: "text-xl font-semibold text-white mb-6", "Attribute Matrix" }
                article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-white border border-gray-800",
                    h3 { class: "text-xl font-bold mb-2 text-black", "Color x Variant x Content" }
                    pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use avatar::dioxus::{{Avatar, Fallback, Image}};
use avatar::{{Color, Size, Variant}};
use dioxus::prelude::*;

#[component]
fn AvatarMatrix() -> Element {{
    let colors = [
        ("Accent",  Color::Accent,  "https://i.pravatar.cc/150?u=20"),
        ("Default", Color::Default, "https://i.pravatar.cc/150?u=21"),
        ("Success", Color::Success, "https://i.pravatar.cc/150?u=22"),
        ("Warning", Color::Warning, "https://i.pravatar.cc/150?u=23"),
        ("Danger",  Color::Danger,  "https://i.pravatar.cc/150?u=24"),
    ];
    rsx! {{
        table {{
            thead {{
                tr {{
                    th {{ "" }}
                    for (name, _, _) in colors.iter() {{
                        th {{ "{{name}}" }}
                    }}
                }}
            }}
            tbody {{
                tr {{
                    th {{ scope: "row", "letter" }}
                    for (_, color, _) in colors.iter() {{
                        td {{
                            Avatar {{ size: Some(Size::Lg),
                                Fallback {{ color: *color, variant: Variant::Default, "AG" }}
                            }}
                        }}
                    }}
                }}
                tr {{
                    th {{ scope: "row", "letter soft" }}
                    for (_, color, _) in colors.iter() {{
                        td {{
                            Avatar {{ size: Some(Size::Lg),
                                Fallback {{ color: *color, variant: Variant::Soft, "AG" }}
                            }}
                        }}
                    }}
                }}
                tr {{
                    th {{ scope: "row", "img" }}
                    for (_, _, url) in colors.iter() {{
                        td {{
                            Avatar {{ size: Some(Size::Lg), aria_label: "User photo",
                                Image {{ src: *url, alt: "User" }}
                            }}
                        }}
                    }}
                }}
            }}
        }}
    }}
}}"# }
                    Example11 {}
                }
            }
        }
    }
}

// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
