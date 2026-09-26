// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use avatar::leptos::{Avatar, Count, Fallback, Group, Image};
use avatar::{Color, Overlap, Size, Variant};
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! { <LandingPage /> }
}

#[component]
fn UserIcon() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            width="20"
            height="20"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
            focusable="false"
        >
            <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" />
            <circle cx="12" cy="7" r="4" />
        </svg>
    }
}

#[component]
pub fn Example1() -> impl IntoView {
    view! {
        <Avatar aria_label="Ferris Prophet">
            <Image src="https://i.pravatar.cc/300?u=1" alt="Ferris Prophet" />
            <Fallback>"FP"</Fallback>
        </Avatar>
    }
}

#[component]
pub fn Example2() -> impl IntoView {
    view! {
        <Avatar aria_label="No Image User">
            <Fallback>"NA"</Fallback>
        </Avatar>
    }
}

#[component]
pub fn Example3() -> impl IntoView {
    view! {
        <div style="display: flex; gap: 12px; align-items: center; flex-wrap: wrap;">
            <Avatar size=Size::Xs aria_label="Extra Small"><Fallback>"XS"</Fallback></Avatar>
            <Avatar size=Size::Sm aria_label="Small"><Fallback>"SM"</Fallback></Avatar>
            <Avatar size=Size::Md aria_label="Medium"><Fallback>"MD"</Fallback></Avatar>
            <Avatar size=Size::Lg aria_label="Large"><Fallback>"LG"</Fallback></Avatar>
            <Avatar size=Size::Xl aria_label="Extra Large"><Fallback>"XL"</Fallback></Avatar>
            <Avatar size=Size::Xxl aria_label="2X Large"><Fallback>"2XL"</Fallback></Avatar>
        </div>
    }
}

#[component]
pub fn Example4() -> impl IntoView {
    view! {
        <div style="display: flex; gap: 12px; align-items: center; flex-wrap: wrap;">
            <Avatar aria_label="Default"><Fallback color=Color::Default>"DF"</Fallback></Avatar>
            <Avatar aria_label="Accent"><Fallback color=Color::Accent>"AC"</Fallback></Avatar>
            <Avatar aria_label="Success"><Fallback color=Color::Success>"SC"</Fallback></Avatar>
            <Avatar aria_label="Warning"><Fallback color=Color::Warning>"WR"</Fallback></Avatar>
            <Avatar aria_label="Danger"><Fallback color=Color::Danger>"DG"</Fallback></Avatar>
        </div>
    }
}

#[component]
pub fn Example5() -> impl IntoView {
    view! {
        <div style="display: flex; gap: 12px; align-items: center; flex-wrap: wrap;">
            <Avatar aria_label="Accent solid">
                <Fallback color=Color::Accent variant=Variant::Default>"SO"</Fallback>
            </Avatar>
            <Avatar aria_label="Accent soft">
                <Fallback color=Color::Accent variant=Variant::Soft>"SF"</Fallback>
            </Avatar>
            <Avatar aria_label="Success soft">
                <Fallback color=Color::Success variant=Variant::Soft>"SS"</Fallback>
            </Avatar>
            <Avatar aria_label="Danger soft">
                <Fallback color=Color::Danger variant=Variant::Soft>"DS"</Fallback>
            </Avatar>
        </div>
    }
}

#[component]
pub fn Example6() -> impl IntoView {
    view! {
        <Avatar aria_label="Delayed Fallback">
            <Image src="https://invalid-url-to-trigger-fallback.example.com/img.jpg" alt="Broken image" />
            <Fallback delay_ms=600>"FB"</Fallback>
        </Avatar>
    }
}

#[component]
pub fn Example7() -> impl IntoView {
    view! {
        <div style="display: flex; gap: 12px; align-items: center;">
            <Avatar size=Size::Lg aria_label="User One">
                <Image src="https://i.pravatar.cc/300?u=2" alt="User 1" />
                <Fallback color=Color::Accent>"U1"</Fallback>
            </Avatar>
            <Avatar size=Size::Lg aria_label="User Two">
                <Image src="https://i.pravatar.cc/300?u=3" alt="User 2" />
                <Fallback color=Color::Success>"U2"</Fallback>
            </Avatar>
            <Avatar size=Size::Lg aria_label="User Three">
                <Image src="https://i.pravatar.cc/300?u=4" alt="User 3" />
                <Fallback color=Color::Warning>"U3"</Fallback>
            </Avatar>
        </div>
    }
}

#[component]
pub fn Example8() -> impl IntoView {
    view! {
        <Avatar aria_label="Custom Border Avatar" style="border: 3px solid #7c3aed;" size=Size::Lg>
            <Image src="https://i.pravatar.cc/300?u=5" alt="Custom style user" />
            <Fallback color=Color::Accent>"CS"</Fallback>
        </Avatar>
    }
}

#[component]
pub fn Example9() -> impl IntoView {
    view! {
        <Avatar aria_label="Square Avatar" style="border-radius: 8px;" size=Size::Lg>
            <Fallback color=Color::Success style="border-radius: 8px;">"SQ"</Fallback>
        </Avatar>
    }
}

#[component]
pub fn Example10() -> impl IntoView {
    let on_load = Callback::new(|_: ()| {
        leptos::logging::log!("Avatar image loaded");
    });
    let on_error = Callback::new(|_: String| {
        leptos::logging::warn!("Avatar image failed");
    });
    view! {
        <Avatar aria_label="Callback Avatar" size=Size::Lg>
            <Image
                src="https://i.pravatar.cc/300?u=6"
                alt="Callback example user"
                on_load=on_load
                on_error=on_error
            />
            <Fallback>"CB"</Fallback>
        </Avatar>
    }
}

#[component]
pub fn Example11() -> impl IntoView {
    let colors = vec![
        ("Accent", Color::Accent, "https://i.pravatar.cc/150?u=20"),
        ("Default", Color::Default, "https://i.pravatar.cc/150?u=21"),
        ("Success", Color::Success, "https://i.pravatar.cc/150?u=22"),
        ("Warning", Color::Warning, "https://i.pravatar.cc/150?u=23"),
        ("Danger", Color::Danger, "https://i.pravatar.cc/150?u=24"),
    ];
    view! {
        <div
            class="w-full max-w-4xl overflow-x-auto bg-black text-white p-8 rounded-xl"
            role="region"
            aria-label="Avatar Matrix Overview"
        >
            <table class="w-full text-left border-collapse">
                <caption class="sr-only">"Avatar style matrix, colors x variants x content types"</caption>
                <thead>
                    <tr>
                        <th scope="col" class="p-4 border-b border-white/10 text-white/50 font-medium">{""}</th>
                        {colors.iter().map(|(name, _, _)| view! {
                            <th scope="col" class="p-4 border-b border-white/10 text-white/50 font-medium text-center">{*name}</th>
                        }).collect_view()}
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <th scope="row" class="p-4 border-b border-white/10 text-white/50">"letter"</th>
                        {colors.iter().map(|(_, color, _)| { let c = *color; view! {
                            <td class="p-4 border-b border-white/10 text-center">
                                <Avatar size=Size::Lg>
                                    <Fallback color=c variant=Variant::Default>"AG"</Fallback>
                                </Avatar>
                            </td>
                        }}).collect_view()}
                    </tr>
                    <tr>
                        <th scope="row" class="p-4 border-b border-white/10 text-white/50">"letter soft"</th>
                        {colors.iter().map(|(_, color, _)| { let c = *color; view! {
                            <td class="p-4 border-b border-white/10 text-center">
                                <Avatar size=Size::Lg>
                                    <Fallback color=c variant=Variant::Soft>"AG"</Fallback>
                                </Avatar>
                            </td>
                        }}).collect_view()}
                    </tr>
                    <tr>
                        <th scope="row" class="p-4 border-b border-white/10 text-white/50">"icon"</th>
                        {colors.iter().map(|(_, color, _)| { let c = *color; view! {
                            <td class="p-4 border-b border-white/10 text-center">
                                <Avatar size=Size::Lg>
                                    <Fallback color=c variant=Variant::Default><UserIcon /></Fallback>
                                </Avatar>
                            </td>
                        }}).collect_view()}
                    </tr>
                    <tr>
                        <th scope="row" class="p-4 border-b border-white/10 text-white/50">"icon soft"</th>
                        {colors.iter().map(|(_, color, _)| { let c = *color; view! {
                            <td class="p-4 border-b border-white/10 text-center">
                                <Avatar size=Size::Lg>
                                    <Fallback color=c variant=Variant::Soft><UserIcon /></Fallback>
                                </Avatar>
                            </td>
                        }}).collect_view()}
                    </tr>
                    <tr>
                        <th scope="row" class="p-4 border-b border-white/10 text-white/50">"img"</th>
                        {colors.iter().map(|(_, _, url)| { let u = *url; view! {
                            <td class="p-4 border-b border-white/10 text-center">
                                <Avatar size=Size::Lg aria_label="User photo">
                                    <Image src=u alt="User" />
                                </Avatar>
                            </td>
                        }}).collect_view()}
                    </tr>
                </tbody>
            </table>
        </div>
    }
}

#[component]
pub fn GroupExample1() -> impl IntoView {
    view! {
        <Group size=Size::Lg aria_label="Project team members">
            <Avatar aria_label="Member 1">
                <Image src="https://i.pravatar.cc/150?u=a042581f4e29026024d" alt="Member 1" />
                <Fallback color=Color::Accent>"M1"</Fallback>
            </Avatar>
            <Avatar aria_label="Member 2">
                <Image src="https://i.pravatar.cc/150?u=a04258a2462d826712d" alt="Member 2" />
                <Fallback color=Color::Success>"M2"</Fallback>
            </Avatar>
            <Avatar aria_label="Member 3">
                <Image src="https://i.pravatar.cc/150?u=a042581f4e29026704d" alt="Member 3" />
                <Fallback color=Color::Warning>"M3"</Fallback>
            </Avatar>
            <Avatar aria_label="Member 4">
                <Image src="https://i.pravatar.cc/150?u=a04258114e29026302d" alt="Member 4" />
                <Fallback color=Color::Danger>"M4"</Fallback>
            </Avatar>
            <Avatar aria_label="Member 5">
                <Image src="https://i.pravatar.cc/150?u=a04258114e29026702d" alt="Member 5" />
                <Fallback>"M5"</Fallback>
            </Avatar>
            <Avatar aria_label="Member 6">
                <Image src="https://i.pravatar.cc/150?u=a04258114e29026708c" alt="Member 6" />
                <Fallback>"M6"</Fallback>
            </Avatar>
        </Group>
    }
}

#[component]
pub fn GroupExample2() -> impl IntoView {
    view! {
        <Group size=Size::Lg max=3 total=6 aria_label="Team with overflow (3 shown)">
            <Avatar aria_label="Member 1">
                <Image src="https://i.pravatar.cc/150?u=a042581f4e29026024d" alt="Member 1" />
                <Fallback color=Color::Accent>"M1"</Fallback>
            </Avatar>
            <Avatar aria_label="Member 2">
                <Image src="https://i.pravatar.cc/150?u=a04258a2462d826712d" alt="Member 2" />
                <Fallback color=Color::Success>"M2"</Fallback>
            </Avatar>
            <Avatar aria_label="Member 3">
                <Image src="https://i.pravatar.cc/150?u=a042581f4e29026704d" alt="Member 3" />
                <Fallback color=Color::Warning>"M3"</Fallback>
            </Avatar>
            <Count />
        </Group>
    }
}

#[component]
pub fn GroupExample3() -> impl IntoView {
    view! {
        <Group size=Size::Md is_grid=true aria_label="Team grid layout">
            <Avatar aria_label="Member 1">
                <Image src="https://i.pravatar.cc/150?u=a042581f4e29026024d" alt="Member 1" />
                <Fallback color=Color::Accent>"M1"</Fallback>
            </Avatar>
            <Avatar aria_label="Member 2">
                <Image src="https://i.pravatar.cc/150?u=a04258a2462d826712d" alt="Member 2" />
                <Fallback color=Color::Success>"M2"</Fallback>
            </Avatar>
            <Avatar aria_label="Member 3">
                <Image src="https://i.pravatar.cc/150?u=a042581f4e29026704d" alt="Member 3" />
                <Fallback color=Color::Warning>"M3"</Fallback>
            </Avatar>
            <Avatar aria_label="Member 4">
                <Image src="https://i.pravatar.cc/150?u=a04258114e29026302d" alt="Member 4" />
                <Fallback color=Color::Danger>"M4"</Fallback>
            </Avatar>
            <Avatar aria_label="Member 5">
                <Image src="https://i.pravatar.cc/150?u=a04258114e29026702d" alt="Member 5" />
                <Fallback>"M5"</Fallback>
            </Avatar>
            <Avatar aria_label="Member 6">
                <Image src="https://i.pravatar.cc/150?u=a04258114e29026708c" alt="Member 6" />
                <Fallback>"M6"</Fallback>
            </Avatar>
            <Avatar aria_label="Member 7">
                <Image src="https://i.pravatar.cc/150?u=a042581f4e29026024e" alt="Member 7" />
                <Fallback>"M7"</Fallback>
            </Avatar>
        </Group>
    }
}

#[component]
pub fn GroupExample4() -> impl IntoView {
    view! {
        <Group size=Size::Lg overlap=Overlap::Ring aria_label="Team with ring overlap">
            <Avatar aria_label="Member 1">
                <Image src="https://i.pravatar.cc/150?u=a042581f4e29026024d" alt="Member 1" />
                <Fallback color=Color::Accent>"M1"</Fallback>
            </Avatar>
            <Avatar aria_label="Member 2">
                <Image src="https://i.pravatar.cc/150?u=a04258a2462d826712d" alt="Member 2" />
                <Fallback color=Color::Success>"M2"</Fallback>
            </Avatar>
            <Avatar aria_label="Member 3">
                <Image src="https://i.pravatar.cc/150?u=a042581f4e29026704d" alt="Member 3" />
                <Fallback color=Color::Warning>"M3"</Fallback>
            </Avatar>
            <Avatar aria_label="Member 4">
                <Image src="https://i.pravatar.cc/150?u=a04258114e29026302d" alt="Member 4" />
                <Fallback>"M4"</Fallback>
            </Avatar>
        </Group>
    }
}

#[component]
pub fn GroupExample5() -> impl IntoView {
    view! {
        <Group size=Size::Lg max=3 total=5 aria_label="Team with explicit count badge">
            <Avatar aria_label="Member 1">
                <Image src="https://picsum.photos/seed/m1/150/150" alt="Member 1" />
                <Fallback color=Color::Accent>"M1"</Fallback>
            </Avatar>
            <Avatar aria_label="Member 2">
                <Image src="https://picsum.photos/seed/m2/150/150" alt="Member 2" />
                <Fallback color=Color::Success>"M2"</Fallback>
            </Avatar>
            <Avatar aria_label="Member 3">
                <Image src="https://picsum.photos/seed/m3/150/150" alt="Member 3" />
                <Fallback color=Color::Warning>"M3"</Fallback>
            </Avatar>
            <Count
                color=Color::Danger
                variant=Variant::Soft
                aria_label="2 additional members"
            />
        </Group>
    }
}

#[component]
pub fn LandingPage() -> impl IntoView {
    view! {
        <div class="m-6 min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-3xl font-bold mb-8 text-white">"Avatar Leptos Examples"</h1>

            <section aria-labelledby="basic-heading" class="w-full max-w-6xl mb-12">
                <h2 id="basic-heading" class="text-xl font-semibold text-white mb-6">"Basic Components"</h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">

                    <article class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black">
                        <h3 class="text-xl font-bold mb-2">"Image + Fallback"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre">{
r#"use avatar::leptos::{
    Avatar, Image, Fallback,
};
use leptos::prelude::*;

#[component]
pub fn BasicAvatar() -> impl IntoView {
    view! {
        <Avatar aria_label="Ferris Prophet">
            <Image
                src="https://i.pravatar.cc/300?u=1"
                alt="Ferris Prophet"
            />
            <Fallback>"FP"</Fallback>
        </Avatar>
    }
}"#}</pre>
                        <Example1 />
                    </article>

                    <article class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black">
                        <h3 class="text-xl font-bold mb-2">"Initials Only"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre">{
r#"use avatar::leptos::{Avatar, Fallback};
use leptos::prelude::*;

#[component]
pub fn InitialsAvatar() -> impl IntoView {
    view! {
        <Avatar aria_label="No Image User">
            <Fallback>"NA"</Fallback>
        </Avatar>
    }
}"#}</pre>
                        <Example2 />
                    </article>

                    <article class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black">
                        <h3 class="text-xl font-bold mb-2">"Sizes"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre">{
r#"use avatar::leptos::{Avatar, Fallback};
use avatar::Size;
use leptos::prelude::*;

#[component]
pub fn SizedAvatars() -> impl IntoView {
    view! {
        <div style="display: flex; gap: 8px; align-items: center;">
            <Avatar size=Size::Xs><Fallback>"XS"</Fallback></Avatar>
            <Avatar size=Size::Sm><Fallback>"SM"</Fallback></Avatar>
            <Avatar size=Size::Md><Fallback>"MD"</Fallback></Avatar>
            <Avatar size=Size::Lg><Fallback>"LG"</Fallback></Avatar>
            <Avatar size=Size::Xl><Fallback>"XL"</Fallback></Avatar>
            <Avatar size=Size::Xxl><Fallback>"2XL"</Fallback></Avatar>
        </div>
    }
}"#}</pre>
                        <Example3 />
                    </article>

                    <article class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black">
                        <h3 class="text-xl font-bold mb-2">"Colors"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre">{
r#"use avatar::leptos::{Avatar, Fallback};
use avatar::Color;
use leptos::prelude::*;

#[component]
pub fn ColoredAvatars() -> impl IntoView {
    view! {
        <div style="display: flex; gap: 8px; align-items: center;">
            <Avatar aria_label="Default">
                <Fallback color=Color::Default>"DF"</Fallback>
            </Avatar>
            <Avatar aria_label="Accent">
                <Fallback color=Color::Accent>"AC"</Fallback>
            </Avatar>
            <Avatar aria_label="Success">
                <Fallback color=Color::Success>"SC"</Fallback>
            </Avatar>
            <Avatar aria_label="Warning">
                <Fallback color=Color::Warning>"WR"</Fallback>
            </Avatar>
            <Avatar aria_label="Danger">
                <Fallback color=Color::Danger>"DG"</Fallback>
            </Avatar>
        </div>
    }
}"#}</pre>
                        <Example4 />
                    </article>

                    <article class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black">
                        <h3 class="text-xl font-bold mb-2">"Solid vs Soft"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre">{
r#"use avatar::leptos::{Avatar, Fallback};
use avatar::{Color, Variant};
use leptos::prelude::*;

#[component]
pub fn VariantAvatars() -> impl IntoView {
    view! {
        <div style="display: flex; gap: 8px; align-items: center;">
            <Avatar aria_label="Accent solid">
                <Fallback
                    color=Color::Accent
                    variant=Variant::Default
                >
                    "SO"
                </Fallback>
            </Avatar>
            <Avatar aria_label="Accent soft">
                <Fallback
                    color=Color::Accent
                    variant=Variant::Soft
                >
                    "SF"
                </Fallback>
            </Avatar>
            <Avatar aria_label="Success soft">
                <Fallback
                    color=Color::Success
                    variant=Variant::Soft
                >
                    "SS"
                </Fallback>
            </Avatar>
            <Avatar aria_label="Danger soft">
                <Fallback
                    color=Color::Danger
                    variant=Variant::Soft
                >
                    "DS"
                </Fallback>
            </Avatar>
        </div>
    }
}"#}</pre>
                        <Example5 />
                    </article>

                    <article class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black">
                        <h3 class="text-xl font-bold mb-2">"Delayed Fallback"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre">{
r#"use avatar::leptos::{
    Avatar, Image, Fallback,
};
use leptos::prelude::*;

#[component]
pub fn DelayedFallback() -> impl IntoView {
    view! {
        <Avatar aria_label="Delayed Fallback">
            <Image
                src="https://invalid.example.com/img.jpg"
                alt="Broken link"
            />
            <Fallback delay_ms=600>"FB"</Fallback>
        </Avatar>
    }
}"#}</pre>
                        <Example6 />
                    </article>

                    <article class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black">
                        <h3 class="text-xl font-bold mb-2">"Multiple Avatars"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre">{
r#"use avatar::leptos::{
    Avatar, Image, Fallback,
};
use avatar::{Color, Size};
use leptos::prelude::*;

#[component]
pub fn MultiAvatar() -> impl IntoView {
    view! {
        <div style="display: flex; gap: 12px; align-items: center;">
            <Avatar size=Size::Lg aria_label="User One">
                <Image src="https://i.pravatar.cc/300?u=2" alt="U1" />
                <Fallback color=Color::Accent>"U1"</Fallback>
            </Avatar>
            <Avatar size=Size::Lg aria_label="User Two">
                <Image src="https://i.pravatar.cc/300?u=3" alt="U2" />
                <Fallback color=Color::Success>"U2"</Fallback>
            </Avatar>
            <Avatar size=Size::Lg aria_label="User Three">
                <Image src="https://i.pravatar.cc/300?u=4" alt="U3" />
                <Fallback color=Color::Warning>"U3"</Fallback>
            </Avatar>
        </div>
    }
}"#}</pre>
                        <Example7 />
                    </article>

                    <article class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black">
                        <h3 class="text-xl font-bold mb-2">"Custom Border"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre">{
r#"use avatar::leptos::{
    Avatar, Image, Fallback,
};
use avatar::{Color, Size};
use leptos::prelude::*;

#[component]
pub fn StyledAvatar() -> impl IntoView {
    view! {
        <Avatar
            aria_label="Custom Border"
            style="border: 3px solid #7c3aed;"
            size=Size::Lg
        >
            <Image src="https://i.pravatar.cc/300?u=5" alt="User" />
            <Fallback color=Color::Accent>"CS"</Fallback>
        </Avatar>
    }
}"#}</pre>
                        <Example8 />
                    </article>

                    <article class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black">
                        <h3 class="text-xl font-bold mb-2">"Square Shape"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre">{
r#"use avatar::leptos::{Avatar, Fallback};
use avatar::{Color, Size};
use leptos::prelude::*;

#[component]
pub fn SquareAvatar() -> impl IntoView {
    view! {
        <Avatar
            aria_label="Square Avatar"
            style="border-radius: 8px;"
            size=Size::Lg
        >
            <Fallback
                color=Color::Success
                style="border-radius: 8px;"
            >
                "SQ"
            </Fallback>
        </Avatar>
    }
}"#}</pre>
                        <Example9 />
                    </article>

                    <article class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black">
                        <h3 class="text-xl font-bold mb-2">"Load / Error Callbacks"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre">{
r#"use avatar::leptos::{
    Avatar, Image, Fallback,
};
use avatar::Size;
use leptos::prelude::*;

#[component]
pub fn CallbackAvatar() -> impl IntoView {
    let on_load = Callback::new(|_: ()| {
        leptos::logging::log!("Image loaded");
    });
    let on_error = Callback::new(|_: String| {
        leptos::logging::warn!("Image failed");
    });
    view! {
        <Avatar aria_label="Callback Avatar" size=Size::Lg>
            <Image
                src="https://i.pravatar.cc/300?u=6"
                alt="Callback user"
                on_load=Some(on_load)
                on_error=Some(on_error)
            />
            <Fallback>"CB"</Fallback>
        </Avatar>
    }
}"#}</pre>
                        <Example10 />
                    </article>

                </div>
            </section>

            <section aria-labelledby="group-heading" class="w-full max-w-6xl mb-12">
                <h2 id="group-heading" class="text-xl font-semibold text-white mb-6">"Group"</h2>
                <div class="flex flex-col gap-8">

                    <article class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black">
                        <h3 class="text-xl font-bold mb-2">"Basic Group (Stacked)"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre">{
r#"use avatar::leptos::{
    Avatar, Group, Image, Fallback,
};
use avatar::{Color, Size};
use leptos::prelude::*;

#[component]
pub fn BasicGroup() -> impl IntoView {
    view! {
        <Group size=Size::Lg aria_label="Project team members">
            <Avatar aria_label="Member 1">
                <Image
                    src="https://i.pravatar.cc/150?u=a042581f4e29026024d"
                    alt="Member 1"
                />
                <Fallback color=Color::Accent>"M1"</Fallback>
            </Avatar>
            <Avatar aria_label="Member 2">
                <Image
                    src="https://i.pravatar.cc/150?u=a04258a2462d826712d"
                    alt="Member 2"
                />
                <Fallback color=Color::Success>"M2"</Fallback>
            </Avatar>
            <Avatar aria_label="Member 3">
                <Image
                    src="https://i.pravatar.cc/150?u=a042581f4e29026704d"
                    alt="Member 3"
                />
                <Fallback color=Color::Warning>"M3"</Fallback>
            </Avatar>
            <Avatar aria_label="Member 4">
                <Image
                    src="https://i.pravatar.cc/150?u=a04258114e29026302d"
                    alt="Member 4"
                />
                <Fallback color=Color::Danger>"M4"</Fallback>
            </Avatar>
            <Avatar aria_label="Member 5">
                <Image
                    src="https://i.pravatar.cc/150?u=a04258114e29026702d"
                    alt="Member 5"
                />
                <Fallback>"M5"</Fallback>
            </Avatar>
            <Avatar aria_label="Member 6">
                <Image
                    src="https://i.pravatar.cc/150?u=a04258114e29026708c"
                    alt="Member 6"
                />
                <Fallback>"M6"</Fallback>
            </Avatar>
        </Group>
    }
}"#}</pre>
                        <GroupExample1 />
                    </article>

                    <article class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black">
                        <h3 class="text-xl font-bold mb-2">"Group with Max + Auto Count"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre">{
r#"use avatar::leptos::{
    Avatar, Group, Image, Fallback,
};
use avatar::{Color, Size};
use leptos::prelude::*;

#[component]
pub fn MaxGroup() -> impl IntoView {
    view! {
        <Group
            size=Size::Lg
            max=3
            total=6
            aria_label="Team, 3 shown with overflow"
        >
            <Avatar aria_label="M1">
                <Image
                    src="https://i.pravatar.cc/150?u=a042581f4e29026024d"
                    alt="M1"
                />
                <Fallback color=Color::Accent>"M1"</Fallback>
            </Avatar>
            <Avatar aria_label="M2">
                <Image
                    src="https://i.pravatar.cc/150?u=a04258a2462d826712d"
                    alt="M2"
                />
                <Fallback color=Color::Success>"M2"</Fallback>
            </Avatar>
            <Avatar aria_label="M3">
                <Image
                    src="https://i.pravatar.cc/150?u=a042581f4e29026704d"
                    alt="M3"
                />
                <Fallback color=Color::Warning>"M3"</Fallback>
            </Avatar>
            <Count />
        </Group>
    }
}"#}</pre>
                        <GroupExample2 />
                    </article>

                    <article class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black">
                        <h3 class="text-xl font-bold mb-2">"Grid Layout"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre">{
r#"use avatar::leptos::{
    Avatar, Group, Image, Fallback,
};
use avatar::{Color, Size};
use leptos::prelude::*;

#[component]
pub fn GridGroup() -> impl IntoView {
    view! {
        <Group size=Size::Md is_grid=true aria_label="Team grid">
            <Avatar aria_label="M1">
                <Image src="https://i.pravatar.cc/150?u=a042581f4e29026024d" alt="M1" />
                <Fallback color=Color::Accent>"M1"</Fallback>
            </Avatar>
            <Avatar aria_label="M2">
                <Image src="https://i.pravatar.cc/150?u=a04258a2462d826712d" alt="M2" />
                <Fallback color=Color::Success>"M2"</Fallback>
            </Avatar>
            <Avatar aria_label="M3">
                <Image src="https://i.pravatar.cc/150?u=a042581f4e29026704d" alt="M3" />
                <Fallback color=Color::Warning>"M3"</Fallback>
            </Avatar>
            <Avatar aria_label="M4">
                <Image src="https://i.pravatar.cc/150?u=a04258114e29026302d" alt="M4" />
                <Fallback color=Some(Color::Danger)>"M4"</Fallback>
            </Avatar>
        </Group>
    }
}"#}</pre>
                        <GroupExample3 />
                    </article>

                    <article class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black">
                        <h3 class="text-xl font-bold mb-2">"Ring Overlap"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre">{
r#"use avatar::leptos::{
    Avatar, Group, Image, Fallback,
};
use avatar::{Color, Overlap, Size};
use leptos::prelude::*;

#[component]
pub fn RingGroup() -> impl IntoView {
    view! {
        <Group
            size=Size::Lg
            overlap=Overlap::Ring
            aria_label="Team with ring overlap"
        >
            <Avatar aria_label="M1">
                <Image src="https://i.pravatar.cc/150?u=a042581f4e29026024d" alt="M1" />
                <Fallback color=Color::Accent>"M1"</Fallback>
            </Avatar>
            <Avatar aria_label="M2">
                <Image src="https://i.pravatar.cc/150?u=a04258a2462d826712d" alt="M2" />
                <Fallback color=Color::Success>"M2"</Fallback>
            </Avatar>
            <Avatar aria_label="M3">
                <Image src="https://i.pravatar.cc/150?u=a042581f4e29026704d" alt="M3" />
                <Fallback color=Color::Warning>"M3"</Fallback>
            </Avatar>
            <Avatar aria_label="M4">
                <Image src="https://i.pravatar.cc/150?u=a04258114e29026302d" alt="M4" />
                <Fallback>"M4"</Fallback>
            </Avatar>
        </Group>
    }
}"#}</pre>
                        <GroupExample4 />
                    </article>

                    <article class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black">
                        <h3 class="text-xl font-bold mb-2">"Explicit Count Badge"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre">{
r#"use avatar::leptos::{
    Avatar, Group, Count,
    Image, Fallback,
};
use avatar::{Color, Size, Variant};
use leptos::prelude::*;

#[component]
pub fn ExplicitCountGroup() -> impl IntoView {
    view! {
        <Group
            size=Size::Lg
            max=3
            total=5
            aria_label="Team with count badge"
        >
            <Avatar aria_label="M1">
                <Image src="https://picsum.photos/seed/m1/150/150" alt="M1" />
                <Fallback color=Color::Accent>"M1"</Fallback>
            </Avatar>
            <Avatar aria_label="M2">
                <Image src="https://picsum.photos/seed/m2/150/150" alt="M2" />
                <Fallback color=Color::Success>"M2"</Fallback>
            </Avatar>
            <Avatar aria_label="M3">
                <Image src="https://picsum.photos/seed/m3/150/150" alt="M3" />
                <Fallback color=Color::Warning>"M3"</Fallback>
            </Avatar>
            <Count
                color=Color::Danger
                variant=Variant::Soft
                aria_label="2 additional members"
            />
        </Group>
    }
}"#}</pre>
                        <GroupExample5 />
                    </article>
                </div>
            </section>

            <section aria-labelledby="matrix-heading" class="w-full max-w-6xl mb-12">
                <h2 id="matrix-heading" class="text-xl font-semibold text-white mb-6">"Attribute Matrix"</h2>
                <article class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-white border border-gray-800">
                    <h3 class="text-xl font-bold mb-2 text-black">"Color x Variant x Content"</h3>
                    <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre">{
r#"use avatar::leptos::{Avatar, Fallback, Image};
use avatar::{Color, Size, Variant};
use leptos::prelude::*;

#[component]
pub fn AvatarMatrix() -> impl IntoView {
    let colors = vec![
        ("Accent",  Color::Accent,  "https://i.pravatar.cc/150?u=20"),
        ("Default", Color::Default, "https://i.pravatar.cc/150?u=21"),
        ("Success", Color::Success, "https://i.pravatar.cc/150?u=22"),
        ("Warning", Color::Warning, "https://i.pravatar.cc/150?u=23"),
        ("Danger",  Color::Danger,  "https://i.pravatar.cc/150?u=24"),
    ];
    view! {
        <table class="w-full text-left border-collapse">
            <thead>
                <tr class="border-b border-gray-600">
                    <th class="p-4 text-white/50 font-normal">{""}</th>
                    {colors.iter().map(|(name, _, _)| view! { <th class="p-4 text-white/50 font-normal text-center">{*name}</th> }).collect_view()}
                </tr>
            </thead>
            <tbody>
                <tr class="border-b border-gray-600">
                    <th class="p-4 text-white/50 font-normal" scope="row">"letter"</th>
                    {colors.iter().map(|(_, color, _)| { let c = *color; view! {
                        <td class="p-4">
                            <div class="flex justify-center">
                            <Avatar size=Size::Lg>
                                <Fallback
                                    color=c
                                    variant=Variant::Default
                                >
                                    "AG"
                                </Fallback>
                            </Avatar>
                            </div>
                        </td>
                    }}).collect_view()}
                </tr>
                <tr class="border-b border-gray-600">
                    <th class="p-4 text-white/50 font-normal" scope="row">"letter soft"</th>
                    {colors.iter().map(|(_, color, _)| { let c = *color; view! {
                        <td class="p-4">
                            <div class="flex justify-center">
                            <Avatar size=Size::Lg>
                                <Fallback
                                    color=c
                                    variant=Variant::Soft
                                >
                                    "AG"
                                </Fallback>
                            </Avatar>
                            </div>
                        </td>
                    }}).collect_view()}
                </tr>
                <tr>
                    <th class="p-4 text-white/50 font-normal" scope="row">"img"</th>
                    {colors.iter().map(|(_, _, url)| { let u = *url; view! {
                        <td class="p-4">
                            <div class="flex justify-center">
                            <Avatar size=Size::Lg aria_label="User photo">
                                <Image src=u alt="User" />
                            </Avatar>
                            </div>
                        </td>
                    }}).collect_view()}
                </tr>
            </tbody>
        </table>
    }
}"#}</pre>
                    <Example11 />
                </article>
            </section>
        </div>
    }
}

// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
