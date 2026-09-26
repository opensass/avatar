// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use avatar::yew::{Avatar, Count, Fallback, Group, Image};
use avatar::{Color, Overlap, Size, Variant};
use yew::prelude::*;

#[function_component(Example1)]
pub fn example1() -> Html {
    html! {
        <Avatar aria_label="Ferris Prophet">
            <Image src="https://i.pravatar.cc/300?u=1" alt="Ferris Prophet" />
            <Fallback>{ "FP" }</Fallback>
        </Avatar>
    }
}

#[function_component(Example2)]
pub fn example2() -> Html {
    html! {
        <Avatar aria_label="No Image User">
            <Fallback>{ "NA" }</Fallback>
        </Avatar>
    }
}

#[function_component(Example3)]
pub fn example3() -> Html {
    html! {
        <div style="display: flex; gap: 12px; align-items: center; flex-wrap: wrap;">
            <Avatar size={Some(Size::Xs)} aria_label="Extra Small">
                <Fallback>{ "XS" }</Fallback>
            </Avatar>
            <Avatar size={Some(Size::Sm)} aria_label="Small">
                <Fallback>{ "SM" }</Fallback>
            </Avatar>
            <Avatar size={Some(Size::Md)} aria_label="Medium">
                <Fallback>{ "MD" }</Fallback>
            </Avatar>
            <Avatar size={Some(Size::Lg)} aria_label="Large">
                <Fallback>{ "LG" }</Fallback>
            </Avatar>
            <Avatar size={Some(Size::Xl)} aria_label="Extra Large">
                <Fallback>{ "XL" }</Fallback>
            </Avatar>
            <Avatar size={Some(Size::Xxl)} aria_label="2X Large">
                <Fallback>{ "2XL" }</Fallback>
            </Avatar>
        </div>
    }
}

#[function_component(Example4)]
pub fn example4() -> Html {
    html! {
        <div style="display: flex; gap: 12px; align-items: center; flex-wrap: wrap;">
            <Avatar aria_label="Default Color">
                <Fallback color={Some(Color::Default)}>{ "DF" }</Fallback>
            </Avatar>
            <Avatar aria_label="Accent Color">
                <Fallback color={Some(Color::Accent)}>{ "AC" }</Fallback>
            </Avatar>
            <Avatar aria_label="Success Color">
                <Fallback color={Some(Color::Success)}>{ "SC" }</Fallback>
            </Avatar>
            <Avatar aria_label="Warning Color">
                <Fallback color={Some(Color::Warning)}>{ "WR" }</Fallback>
            </Avatar>
            <Avatar aria_label="Danger Color">
                <Fallback color={Some(Color::Danger)}>{ "DG" }</Fallback>
            </Avatar>
        </div>
    }
}

#[function_component(Example5)]
pub fn example5() -> Html {
    html! {
        <div style="display: flex; gap: 12px; align-items: center; flex-wrap: wrap;">
            <Avatar aria_label="Accent Solid">
                <Fallback color={Some(Color::Accent)} variant={Some(Variant::Default)}>
                    { "SO" }
                </Fallback>
            </Avatar>
            <Avatar aria_label="Accent Soft">
                <Fallback color={Some(Color::Accent)} variant={Some(Variant::Soft)}>
                    { "SF" }
                </Fallback>
            </Avatar>
            <Avatar aria_label="Success Soft">
                <Fallback color={Some(Color::Success)} variant={Some(Variant::Soft)}>
                    { "SS" }
                </Fallback>
            </Avatar>
            <Avatar aria_label="Danger Soft">
                <Fallback color={Some(Color::Danger)} variant={Some(Variant::Soft)}>
                    { "DS" }
                </Fallback>
            </Avatar>
        </div>
    }
}

#[function_component(Example6)]
pub fn example6() -> Html {
    html! {
        <Avatar aria_label="Delayed Fallback">
            <Image
                src="https://invalid-url-to-trigger-fallback.example.com/img.jpg"
                alt="Broken image"
            />
            <Fallback delay_ms=600>{ "FB" }</Fallback>
        </Avatar>
    }
}

#[function_component(Example7)]
pub fn example7() -> Html {
    html! {
        <div style="display: flex; gap: 12px; align-items: center;">
            <Avatar size={Some(Size::Lg)} aria_label="User One">
                <Image src="https://i.pravatar.cc/300?u=2" alt="User 1" />
                <Fallback color={Some(Color::Accent)}>{ "U1" }</Fallback>
            </Avatar>
            <Avatar size={Some(Size::Lg)} aria_label="User Two">
                <Image src="https://i.pravatar.cc/300?u=3" alt="User 2" />
                <Fallback color={Some(Color::Success)}>{ "U2" }</Fallback>
            </Avatar>
            <Avatar size={Some(Size::Lg)} aria_label="User Three">
                <Image src="https://i.pravatar.cc/300?u=4" alt="User 3" />
                <Fallback color={Some(Color::Warning)}>{ "U3" }</Fallback>
            </Avatar>
        </div>
    }
}

#[function_component(Example8)]
pub fn example8() -> Html {
    html! {
        <Avatar
            aria_label="Custom Style Avatar"
            style="border: 3px solid #7c3aed;"
            size={Some(Size::Lg)}
        >
            <Image src="https://i.pravatar.cc/300?u=5" alt="Custom style user" />
            <Fallback color={Some(Color::Accent)}>{ "CS" }</Fallback>
        </Avatar>
    }
}

#[function_component(Example9)]
pub fn example9() -> Html {
    html! {
        <Avatar aria_label="Square Avatar" style="border-radius: 8px;" size={Some(Size::Lg)}>
            <Fallback color={Some(Color::Success)} style="border-radius: 8px;">{ "SQ" }</Fallback>
        </Avatar>
    }
}

#[function_component(Example10)]
pub fn example10() -> Html {
    let on_load = Callback::from(|_| {
        web_sys::console::log_1(&"Avatar image loaded".into());
    });
    let on_error = Callback::from(|_: String| {
        web_sys::console::warn_1(&"Avatar image failed".into());
    });
    html! {
        <Avatar aria_label="Callback Avatar" size={Some(Size::Lg)}>
            <Image
                src="https://i.pravatar.cc/300?u=6"
                alt="Callback example user"
                on_load={on_load}
                on_error={on_error}
            />
            <Fallback>{ "CB" }</Fallback>
        </Avatar>
    }
}

#[function_component(UserIcon)]
pub fn user_icon() -> Html {
    html! {
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

#[function_component(Example11)]
pub fn example11() -> Html {
    let colors: Vec<(&'static str, Color, &'static str)> = vec![
        ("Accent", Color::Accent, "https://i.pravatar.cc/150?u=20"),
        ("Default", Color::Default, "https://i.pravatar.cc/150?u=21"),
        ("Success", Color::Success, "https://i.pravatar.cc/150?u=22"),
        ("Warning", Color::Warning, "https://i.pravatar.cc/150?u=23"),
        ("Danger", Color::Danger, "https://i.pravatar.cc/150?u=24"),
    ];
    html! {
        <div
            class="w-full max-w-4xl overflow-x-auto bg-[#09090b] text-white p-8 rounded-xl"
            role="region"
            aria-label="Avatar Matrix Overview"
        >
            <table class="w-full text-left border-collapse">
                <caption class="sr-only">
                    { "Avatar style matrix - colors x variants x content types" }
                </caption>
                <thead>
                    <tr>
                        <th
                            scope="col"
                            class="p-4 border-b border-white/10 text-white/50 font-medium"
                        >
                            { "" }
                        </th>
                        { for colors.iter().map(|(name, _, _)| html! { <th scope="col" class="p-4 border-b border-white/10 text-white/50 font-medium text-center">{name}</th> }) }
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <th scope="row" class="p-4 border-b border-white/10 text-white/50">
                            { "letter" }
                        </th>
                        { for colors.iter().map(|(name, color, _)| html! {
                            <td class="p-4 border-b border-white/10 text-center">
                                <Avatar size={Some(Size::Lg)} aria_label={*name}><Fallback color={Some(*color)} variant={Some(Variant::Default)}>{"AG"}</Fallback></Avatar>
                            </td>
                        }) }
                    </tr>
                    <tr>
                        <th scope="row" class="p-4 border-b border-white/10 text-white/50">
                            { "letter soft" }
                        </th>
                        { for colors.iter().map(|(name, color, _)| html! {
                            <td class="p-4 border-b border-white/10 text-center">
                                <Avatar size={Some(Size::Lg)} aria_label={*name}><Fallback color={Some(*color)} variant={Some(Variant::Soft)}>{"AG"}</Fallback></Avatar>
                            </td>
                        }) }
                    </tr>
                    <tr>
                        <th scope="row" class="p-4 border-b border-white/10 text-white/50">
                            { "icon" }
                        </th>
                        { for colors.iter().map(|(name, color, _)| html! {
                            <td class="p-4 border-b border-white/10 text-center">
                                <Avatar size={Some(Size::Lg)} aria_label={*name}><Fallback color={Some(*color)} variant={Some(Variant::Default)}><UserIcon /></Fallback></Avatar>
                            </td>
                        }) }
                    </tr>
                    <tr>
                        <th scope="row" class="p-4 border-b border-white/10 text-white/50">
                            { "icon soft" }
                        </th>
                        { for colors.iter().map(|(name, color, _)| html! {
                            <td class="p-4 border-b border-white/10 text-center">
                                <Avatar size={Some(Size::Lg)} aria_label={*name}><Fallback color={Some(*color)} variant={Some(Variant::Soft)}><UserIcon /></Fallback></Avatar>
                            </td>
                        }) }
                    </tr>
                    <tr>
                        <th scope="row" class="p-4 border-b border-white/10 text-white/50">
                            { "img" }
                        </th>
                        { for colors.iter().map(|(_, _, url)| html! {
                            <td class="p-4 border-b border-white/10 text-center">
                                <Avatar size={Some(Size::Lg)} aria_label="User photo"><Image src={*url} alt="User" /></Avatar>
                            </td>
                        }) }
                    </tr>
                </tbody>
            </table>
        </div>
    }
}

#[function_component(GroupExample1)]
pub fn group_example1() -> Html {
    html! {
        <Group size={Size::Lg} aria_label="Project team members">
            <Avatar aria_label="Member 1">
                <Image src="https://i.pravatar.cc/150?u=a042581f4e29026024d" alt="Member 1" />
                <Fallback color={Some(Color::Accent)}>{ "M1" }</Fallback>
            </Avatar>
            <Avatar aria_label="Member 2">
                <Image src="https://i.pravatar.cc/150?u=a04258a2462d826712d" alt="Member 2" />
                <Fallback color={Some(Color::Success)}>{ "M2" }</Fallback>
            </Avatar>
            <Avatar aria_label="Member 3">
                <Image src="https://i.pravatar.cc/150?u=a042581f4e29026704d" alt="Member 3" />
                <Fallback color={Some(Color::Warning)}>{ "M3" }</Fallback>
            </Avatar>
            <Avatar aria_label="Member 4">
                <Image src="https://i.pravatar.cc/150?u=a04258114e29026302d" alt="Member 4" />
                <Fallback color={Some(Color::Danger)}>{ "M4" }</Fallback>
            </Avatar>
            <Avatar aria_label="Member 5">
                <Image src="https://i.pravatar.cc/150?u=a04258114e29026702d" alt="Member 5" />
                <Fallback>{ "M5" }</Fallback>
            </Avatar>
            <Avatar aria_label="Member 6">
                <Image src="https://i.pravatar.cc/150?u=a04258114e29026708c" alt="Member 6" />
                <Fallback>{ "M6" }</Fallback>
            </Avatar>
        </Group>
    }
}

#[function_component(GroupExample2)]
pub fn group_example2() -> Html {
    html! {
        <Group
            size={Size::Lg}
            max={Some(3)}
            total={Some(6)}
            aria_label="Team with overflow (3 shown)"
        >
            <Avatar aria_label="Member 1">
                <Image src="https://i.pravatar.cc/150?u=a042581f4e29026024d" alt="Member 1" />
                <Fallback color={Some(Color::Accent)}>{ "M1" }</Fallback>
            </Avatar>
            <Avatar aria_label="Member 2">
                <Image src="https://i.pravatar.cc/150?u=a04258a2462d826712d" alt="Member 2" />
                <Fallback color={Some(Color::Success)}>{ "M2" }</Fallback>
            </Avatar>
            <Avatar aria_label="Member 3">
                <Image src="https://i.pravatar.cc/150?u=a042581f4e29026704d" alt="Member 3" />
                <Fallback color={Some(Color::Warning)}>{ "M3" }</Fallback>
            </Avatar>
            <Count />
        </Group>
    }
}

#[function_component(GroupExample3)]
pub fn group_example3() -> Html {
    html! {
        <Group size={Size::Md} is_grid=true aria_label="Team grid layout">
            <Avatar aria_label="Member 1">
                <Image src="https://i.pravatar.cc/150?u=a042581f4e29026024d" alt="Member 1" />
                <Fallback color={Some(Color::Accent)}>{ "M1" }</Fallback>
            </Avatar>
            <Avatar aria_label="Member 2">
                <Image src="https://i.pravatar.cc/150?u=a04258a2462d826712d" alt="Member 2" />
                <Fallback color={Some(Color::Success)}>{ "M2" }</Fallback>
            </Avatar>
            <Avatar aria_label="Member 3">
                <Image src="https://i.pravatar.cc/150?u=a042581f4e29026704d" alt="Member 3" />
                <Fallback color={Some(Color::Warning)}>{ "M3" }</Fallback>
            </Avatar>
            <Avatar aria_label="Member 4">
                <Image src="https://i.pravatar.cc/150?u=a04258114e29026302d" alt="Member 4" />
                <Fallback color={Some(Color::Danger)}>{ "M4" }</Fallback>
            </Avatar>
            <Avatar aria_label="Member 5">
                <Image src="https://i.pravatar.cc/150?u=a04258114e29026702d" alt="Member 5" />
                <Fallback>{ "M5" }</Fallback>
            </Avatar>
            <Avatar aria_label="Member 6">
                <Image src="https://i.pravatar.cc/150?u=a04258114e29026708c" alt="Member 6" />
                <Fallback>{ "M6" }</Fallback>
            </Avatar>
            <Avatar aria_label="Member 7">
                <Image src="https://i.pravatar.cc/150?u=a042581f4e29026024e" alt="Member 7" />
                <Fallback>{ "M7" }</Fallback>
            </Avatar>
        </Group>
    }
}

#[function_component(GroupExample4)]
pub fn group_example4() -> Html {
    html! {
        <Group size={Size::Lg} overlap={Overlap::Ring} aria_label="Team with ring overlap">
            <Avatar aria_label="Member 1">
                <Image src="https://i.pravatar.cc/150?u=a042581f4e29026024d" alt="Member 1" />
                <Fallback color={Some(Color::Accent)}>{ "M1" }</Fallback>
            </Avatar>
            <Avatar aria_label="Member 2">
                <Image src="https://i.pravatar.cc/150?u=a04258a2462d826712d" alt="Member 2" />
                <Fallback color={Some(Color::Success)}>{ "M2" }</Fallback>
            </Avatar>
            <Avatar aria_label="Member 3">
                <Image src="https://i.pravatar.cc/150?u=a042581f4e29026704d" alt="Member 3" />
                <Fallback color={Some(Color::Warning)}>{ "M3" }</Fallback>
            </Avatar>
            <Avatar aria_label="Member 4">
                <Image src="https://i.pravatar.cc/150?u=a04258114e29026302d" alt="Member 4" />
                <Fallback>{ "M4" }</Fallback>
            </Avatar>
        </Group>
    }
}

#[function_component(GroupExample5)]
pub fn group_example5() -> Html {
    html! {
        <Group
            size={Size::Lg}
            max={Some(3)}
            total={Some(5)}
            aria_label="Team with explicit count badge"
        >
            <Avatar aria_label="Member 1">
                <Image src="https://picsum.photos/seed/m1/150/150" alt="Member 1" />
                <Fallback color={Some(Color::Accent)}>{ "M1" }</Fallback>
            </Avatar>
            <Avatar aria_label="Member 2">
                <Image src="https://picsum.photos/seed/m2/150/150" alt="Member 2" />
                <Fallback color={Some(Color::Success)}>{ "M2" }</Fallback>
            </Avatar>
            <Avatar aria_label="Member 3">
                <Image src="https://picsum.photos/seed/m3/150/150" alt="Member 3" />
                <Fallback color={Some(Color::Warning)}>{ "M3" }</Fallback>
            </Avatar>
            <Count
                color={Some(Color::Danger)}
                variant={Some(Variant::Soft)}
                aria_label="2 additional members"
            />
        </Group>
    }
}

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    html! {
        <div class="m-6 min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-3xl font-bold mb-8 text-white">{ "Avatar Yew Examples" }</h1>
            <section aria-labelledby="basic-heading" class="w-full max-w-6xl mb-12">
                <h2 id="basic-heading" class="text-xl font-semibold text-white mb-6">
                    { "Basic Components" }
                </h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">
                    <article
                        class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Image + Fallback" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use avatar::yew::{
    Avatar, Image, Fallback,
};
use yew::prelude::*;

#[function_component(BasicAvatar)]
pub fn basic_avatar() -> Html {
    html! {
        <Avatar aria_label="Ferris Prophet">
            <Image
                src="https://i.pravatar.cc/300?u=1"
                alt="Ferris Prophet"
            />
            <Fallback>{"FP"}</Fallback>
        </Avatar>
    }
}"# }
                        </pre>
                        <Example1 />
                    </article>
                    <article
                        class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Initials Only" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use avatar::yew::{Avatar, Fallback};
use yew::prelude::*;

#[function_component(InitialsAvatar)]
pub fn initials_avatar() -> Html {
    html! {
        <Avatar aria_label="No Image User">
            <Fallback>{"NA"}</Fallback>
        </Avatar>
    }
}"# }
                        </pre>
                        <Example2 />
                    </article>
                    <article
                        class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Sizes" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use avatar::yew::{Avatar, Fallback};
use avatar::Size;
use yew::prelude::*;

#[function_component(SizedAvatars)]
pub fn sized_avatars() -> Html {
    html! {
        <div style="display: flex; gap: 8px; align-items: center;">
            <Avatar size={Some(Size::Xs)}><Fallback>{"XS"}</Fallback></Avatar>
            <Avatar size={Some(Size::Sm)}><Fallback>{"SM"}</Fallback></Avatar>
            <Avatar size={Some(Size::Md)}><Fallback>{"MD"}</Fallback></Avatar>
            <Avatar size={Some(Size::Lg)}><Fallback>{"LG"}</Fallback></Avatar>
            <Avatar size={Some(Size::Xl)}><Fallback>{"XL"}</Fallback></Avatar>
            <Avatar size={Some(Size::Xxl)}><Fallback>{"2XL"}</Fallback></Avatar>
        </div>
    }
}"# }
                        </pre>
                        <Example3 />
                    </article>
                    <article
                        class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Colors" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use avatar::yew::{Avatar, Fallback};
use avatar::Color;
use yew::prelude::*;

#[function_component(ColoredAvatars)]
pub fn colored_avatars() -> Html {
    html! {
        <div style="display: flex; gap: 8px; align-items: center;">
            <Avatar aria_label="Default">
                <Fallback color={Some(Color::Default)}>{"DF"}</Fallback>
            </Avatar>
            <Avatar aria_label="Accent">
                <Fallback color={Some(Color::Accent)}>{"AC"}</Fallback>
            </Avatar>
            <Avatar aria_label="Success">
                <Fallback color={Some(Color::Success)}>{"SC"}</Fallback>
            </Avatar>
            <Avatar aria_label="Warning">
                <Fallback color={Some(Color::Warning)}>{"WR"}</Fallback>
            </Avatar>
            <Avatar aria_label="Danger">
                <Fallback color={Some(Color::Danger)}>{"DG"}</Fallback>
            </Avatar>
        </div>
    }
}"# }
                        </pre>
                        <Example4 />
                    </article>
                    <article
                        class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Variants: Solid vs Soft" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use avatar::yew::{Avatar, Fallback};
use avatar::{Color, Variant};
use yew::prelude::*;

#[function_component(VariantAvatars)]
pub fn variant_avatars() -> Html {
    html! {
        <div style="display: flex; gap: 8px; align-items: center;">
            <Avatar aria_label="Accent solid">
                <Fallback
                    color={Some(Color::Accent)}
                    variant={Some(Variant::Default)}
                >
                    {"SO"}
                </Fallback>
            </Avatar>
            <Avatar aria_label="Accent soft">
                <Fallback
                    color={Some(Color::Accent)}
                    variant={Some(Variant::Soft)}
                >
                    {"SF"}
                </Fallback>
            </Avatar>
            <Avatar aria_label="Success soft">
                <Fallback
                    color={Some(Color::Success)}
                    variant={Some(Variant::Soft)}
                >
                    {"SS"}
                </Fallback>
            </Avatar>
            <Avatar aria_label="Danger soft">
                <Fallback
                    color={Some(Color::Danger)}
                    variant={Some(Variant::Soft)}
                >
                    {"DS"}
                </Fallback>
            </Avatar>
        </div>
    }
}"# }
                        </pre>
                        <Example5 />
                    </article>
                    <article
                        class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Delayed Fallback" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use avatar::yew::{Avatar, Image, Fallback};
use yew::prelude::*;

#[function_component(DelayedFallback)]
pub fn delayed_fallback() -> Html {
    html! {
        <Avatar aria_label="Delayed Fallback">
            <Image
                src="https://invalid.example.com/img.jpg"
                alt="Broken link"
            />
            <Fallback delay_ms={600}>{"FB"}</Fallback>
        </Avatar>
    }
}"# }
                        </pre>
                        <Example6 />
                    </article>
                    <article
                        class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Multiple Avatars" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use avatar::yew::{Avatar, Image, Fallback};
use avatar::{Color, Size};
use yew::prelude::*;

#[function_component(MultiAvatar)]
pub fn multi_avatar() -> Html {
    html! {
        <div style="display: flex; gap: 12px; align-items: center;">
            <Avatar size={Some(Size::Lg)} aria_label="User One">
                <Image
                    src="https://i.pravatar.cc/300?u=2"
                    alt="User 1"
                />
                <Fallback color={Some(Color::Accent)}>
                    {"U1"}
                </Fallback>
            </Avatar>
            <Avatar size={Some(Size::Lg)} aria_label="User Two">
                <Image
                    src="https://i.pravatar.cc/300?u=3"
                    alt="User 2"
                />
                <Fallback color={Some(Color::Success)}>
                    {"U2"}
                </Fallback>
            </Avatar>
            <Avatar size={Some(Size::Lg)} aria_label="User Three">
                <Image
                    src="https://i.pravatar.cc/300?u=4"
                    alt="User 3"
                />
                <Fallback color={Some(Color::Warning)}>
                    {"U3"}
                </Fallback>
            </Avatar>
        </div>
    }
}"# }
                        </pre>
                        <Example7 />
                    </article>
                    <article
                        class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Custom Border Style" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use avatar::yew::{Avatar, Image, Fallback};
use avatar::{Color, Size};
use yew::prelude::*;

#[function_component(StyledAvatar)]
pub fn styled_avatar() -> Html {
    html! {
        <Avatar
            aria_label="Custom Border Avatar"
            style="border: 3px solid #7c3aed;"
            size={Some(Size::Lg)}
        >
            <Image
                src="https://i.pravatar.cc/300?u=5"
                alt="Custom style user"
            />
            <Fallback color={Some(Color::Accent)}>
                {"CS"}
            </Fallback>
        </Avatar>
    }
}"# }
                        </pre>
                        <Example8 />
                    </article>
                    <article
                        class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Square Shape" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use avatar::yew::{Avatar, Fallback};
use avatar::{Color, Size};
use yew::prelude::*;

#[function_component(SquareAvatar)]
pub fn square_avatar() -> Html {
    html! {
        <Avatar
            aria_label="Square Avatar"
            style="border-radius: 8px;"
            size={Some(Size::Lg)}
        >
            <Fallback
                color={Some(Color::Success)}
                style="border-radius: 8px;"
            >
                {"SQ"}
            </Fallback>
        </Avatar>
    }
}"# }
                        </pre>
                        <Example9 />
                    </article>
                    <article
                        class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Load / Error Callbacks" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use avatar::yew::{Avatar, Image, Fallback};
use avatar::Size;
use yew::prelude::*;

#[function_component(CallbackAvatar)]
pub fn callback_avatar() -> Html {
    let on_load = Callback::from(|_| {
        web_sys::console::log_1(&"Image loaded".into());
    });
    let on_error = Callback::from(|_: String| {
        web_sys::console::warn_1(&"Image failed".into());
    });
    html! {
        <Avatar aria_label="Callback Avatar" size={Some(Size::Lg)}>
            <Image
                src="https://i.pravatar.cc/300?u=6"
                alt="Callback user"
                on_load={on_load}
                on_error={on_error}
            />
            <Fallback>{"CB"}</Fallback>
        </Avatar>
    }
}"# }
                        </pre>
                        <Example10 />
                    </article>
                </div>
            </section>
            <section aria-labelledby="group-heading" class="w-full max-w-6xl mb-12">
                <h2 id="group-heading" class="text-xl font-semibold text-white mb-6">
                    { "Group" }
                </h2>
                <div class="flex flex-col gap-8">
                    <article
                        class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Basic Group (Stacked, No Max)" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use avatar::yew::{
    Avatar, Group, Image, Fallback,
};
use avatar::{Color, Size};
use yew::prelude::*;

#[function_component(BasicGroup)]
pub fn basic_group() -> Html {
    html! {
        <Group
            size={Size::Lg}
            aria_label="Project team members"
        >
            <Avatar aria_label="Member 1">
                <Image
                    src="https://i.pravatar.cc/150?u=a042581f4e29026024d"
                    alt="Member 1"
                />
                <Fallback color={Some(Color::Accent)}>{"M1"}</Fallback>
            </Avatar>
            <Avatar aria_label="Member 2">
                <Image
                    src="https://i.pravatar.cc/150?u=a04258a2462d826712d"
                    alt="Member 2"
                />
                <Fallback color={Some(Color::Success)}>{"M2"}</Fallback>
            </Avatar>
            <Avatar aria_label="Member 3">
                <Image
                    src="https://i.pravatar.cc/150?u=a042581f4e29026704d"
                    alt="Member 3"
                />
                <Fallback color={Some(Color::Warning)}>{"M3"}</Fallback>
            </Avatar>
            <Avatar aria_label="Member 4">
                <Image
                    src="https://i.pravatar.cc/150?u=a04258114e29026302d"
                    alt="Member 4"
                />
                <Fallback color={Some(Color::Danger)}>{"M4"}</Fallback>
            </Avatar>
            <Avatar aria_label="Member 5">
                <Image
                    src="https://i.pravatar.cc/150?u=a04258114e29026702d"
                    alt="Member 5"
                />
                <Fallback>{"M5"}</Fallback>
            </Avatar>
            <Avatar aria_label="Member 6">
                <Image
                    src="https://i.pravatar.cc/150?u=a04258114e29026708c"
                    alt="Member 6"
                />
                <Fallback>{"M6"}</Fallback>
            </Avatar>
        </Group>
    }
}"# }
                        </pre>
                        <GroupExample1 />
                    </article>
                    <article
                        class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Group with Max (Auto Count)" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use avatar::yew::{
    Avatar, Group, Image, Fallback,
};
use avatar::{Color, Size};
use yew::prelude::*;

#[function_component(MaxGroup)]
pub fn max_group() -> Html {
    html! {
        <Group
            size={Size::Lg}
            max={Some(3)}
            total={Some(6)}
            aria_label="Team with overflow (3 shown)"
        >
            <Avatar aria_label="Member 1">
                <Image
                    src="https://i.pravatar.cc/150?u=a042581f4e29026024d"
                    alt="Member 1"
                />
                <Fallback color={Some(Color::Accent)}>{"M1"}</Fallback>
            </Avatar>
            <Avatar aria_label="Member 2">
                <Image
                    src="https://i.pravatar.cc/150?u=a04258a2462d826712d"
                    alt="Member 2"
                />
                <Fallback color={Some(Color::Success)}>{"M2"}</Fallback>
            </Avatar>
            <Avatar aria_label="Member 3">
                <Image
                    src="https://i.pravatar.cc/150?u=a042581f4e29026704d"
                    alt="Member 3"
                />
                <Fallback color={Some(Color::Warning)}>{"M3"}</Fallback>
            </Avatar>
            <Count />
        </Group>
    }
}"# }
                        </pre>
                        <GroupExample2 />
                    </article>
                    <article
                        class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Grid Layout" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use avatar::yew::{
    Avatar, Group, Image, Fallback,
};
use avatar::{Color, Size};
use yew::prelude::*;

#[function_component(GridGroup)]
pub fn grid_group() -> Html {
    html! {
        <Group
            size={Size::Md}
            is_grid={true}
            aria_label="Team grid layout"
        >
            <Avatar aria_label="Member 1">
                <Image
                    src="https://i.pravatar.cc/150?u=a042581f4e29026024d"
                    alt="Member 1"
                />
                <Fallback color={Some(Color::Accent)}>{"M1"}</Fallback>
            </Avatar>
            <Avatar aria_label="Member 2">
                <Image
                    src="https://i.pravatar.cc/150?u=a04258a2462d826712d"
                    alt="Member 2"
                />
                <Fallback color={Some(Color::Success)}>{"M2"}</Fallback>
            </Avatar>
            <Avatar aria_label="Member 3">
                <Image
                    src="https://i.pravatar.cc/150?u=a042581f4e29026704d"
                    alt="Member 3"
                />
                <Fallback color={Some(Color::Warning)}>{"M3"}</Fallback>
            </Avatar>
            <Avatar aria_label="Member 4">
                <Image
                    src="https://i.pravatar.cc/150?u=a04258114e29026302d"
                    alt="Member 4"
                />
                <Fallback color={Some(Color::Danger)}>{"M4"}</Fallback>
            </Avatar>
            <Avatar aria_label="Member 5">
                <Image
                    src="https://i.pravatar.cc/150?u=a04258114e29026702d"
                    alt="Member 5"
                />
                <Fallback>{"M5"}</Fallback>
            </Avatar>
            <Avatar aria_label="Member 6">
                <Image
                    src="https://i.pravatar.cc/150?u=a04258114e29026708c"
                    alt="Member 6"
                />
                <Fallback>{"M6"}</Fallback>
            </Avatar>
            <Avatar aria_label="Member 7">
                <Image
                    src="https://i.pravatar.cc/150?u=a042581f4e29026024e"
                    alt="Member 7"
                />
                <Fallback>{"M7"}</Fallback>
            </Avatar>
        </Group>
    }
}"# }
                        </pre>
                        <GroupExample3 />
                    </article>
                    <article
                        class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Ring Overlap" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use avatar::yew::{
    Avatar, Group, Image, Fallback,
};
use avatar::{Color, Overlap, Size};
use yew::prelude::*;

#[function_component(RingGroup)]
pub fn ring_group() -> Html {
    html! {
        <Group
            size={Size::Lg}
            overlap={Overlap::Ring}
            aria_label="Team with ring overlap"
        >
            <Avatar aria_label="Member 1">
                <Image
                    src="https://i.pravatar.cc/150?u=a042581f4e29026024d"
                    alt="Member 1"
                />
                <Fallback color={Some(Color::Accent)}>{"M1"}</Fallback>
            </Avatar>
            <Avatar aria_label="Member 2">
                <Image
                    src="https://i.pravatar.cc/150?u=a04258a2462d826712d"
                    alt="Member 2"
                />
                <Fallback color={Some(Color::Success)}>{"M2"}</Fallback>
            </Avatar>
            <Avatar aria_label="Member 3">
                <Image
                    src="https://i.pravatar.cc/150?u=a042581f4e29026704d"
                    alt="Member 3"
                />
                <Fallback color={Some(Color::Warning)}>{"M3"}</Fallback>
            </Avatar>
            <Avatar aria_label="Member 4">
                <Image
                    src="https://i.pravatar.cc/150?u=a04258114e29026302d"
                    alt="Member 4"
                />
                <Fallback>{"M4"}</Fallback>
            </Avatar>
        </Group>
    }
}"# }
                        </pre>
                        <GroupExample4 />
                    </article>
                    <article
                        class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Explicit Count Badge" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use avatar::yew::{
    Avatar, Group, Count,
    Image, Fallback,
};
use avatar::{Color, Size, Variant};
use yew::prelude::*;

#[function_component(ExplicitCountGroup)]
pub fn explicit_count_group() -> Html {
    html! {
        <Group
            size={Size::Lg}
            max={Some(3)}
            total={Some(5)}
            aria_label="Team with count badge"
        >
            <Avatar aria_label="M1">
                <Image src="https://picsum.photos/seed/m1/150/150" alt="M1" />
                <Fallback color={Some(Color::Accent)}>{"M1"}</Fallback>
            </Avatar>
            <Avatar aria_label="M2">
                <Image src="https://picsum.photos/seed/m2/150/150" alt="M2" />
                <Fallback color={Some(Color::Success)}>{"M2"}</Fallback>
            </Avatar>
            <Avatar aria_label="M3">
                <Image src="https://picsum.photos/seed/m3/150/150" alt="M3" />
                <Fallback color={Some(Color::Warning)}>{"M3"}</Fallback>
            </Avatar>
            <Count
                color={Some(Color::Danger)}
                variant={Some(Variant::Soft)}
                aria_label="2 additional members"
            />
        </Group>
    }
}"# }
                        </pre>
                        <GroupExample5 />
                    </article>
                </div>
            </section>
            <section aria-labelledby="matrix-heading" class="w-full max-w-6xl mb-12">
                <h2 id="matrix-heading" class="text-xl font-semibold text-white mb-6">
                    { "Attribute Matrix" }
                </h2>
                <article
                    class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-white border border-gray-800"
                >
                    <h3 class="text-xl font-bold mb-2 text-black">
                        { "Color x Variant x Content Overview" }
                    </h3>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre"
                    >
                        { r#"use avatar::yew::{Avatar, Fallback, Image};
use avatar::{Color, Size, Variant};
use yew::prelude::*;

#[function_component(AvatarMatrix)]
pub fn avatar_matrix() -> Html {
    let colors = [
        ("Accent",  Color::Accent,  "https://i.pravatar.cc/150?u=20"),
        ("Default", Color::Default, "https://i.pravatar.cc/150?u=21"),
        ("Success", Color::Success, "https://i.pravatar.cc/150?u=22"),
        ("Warning", Color::Warning, "https://i.pravatar.cc/150?u=23"),
        ("Danger",  Color::Danger,  "https://i.pravatar.cc/150?u=24"),
    ];
    html! {
        <table class="w-full text-left border-collapse">
            <thead>
                <tr class="border-b border-gray-600">
                    <th class="p-4 text-white/50 font-normal">{""}</th>
                    { for colors.iter().map(|(name, _, _)| html!{ <th class="p-4 text-white/50 font-normal text-center">{name}</th> }) }
                </tr>
            </thead>
            <tbody>
                <tr class="border-b border-gray-600">
                    <th class="p-4 text-white/50 font-normal" scope="row">{"letter"}</th>
                    { for colors.iter().map(|(_, color, _)| html! {
                        <td class="p-4 text-center">
                            <div class="flex justify-center">
                            <Avatar size={Some(Size::Lg)}>
                                <Fallback
                                    color={Some(*color)}
                                    variant={Some(Variant::Default)}
                                >
                                    {"AG"}
                                </Fallback>
                            </Avatar>
                            </div>
                        </td>
                    }) }
                </tr>
                <tr class="border-b border-gray-600">
                    <th class="p-4 text-white/50 font-normal" scope="row">{"letter soft"}</th>
                    { for colors.iter().map(|(_, color, _)| html! {
                        <td class="p-4 text-center">
                            <div class="flex justify-center">
                            <Avatar size={Some(Size::Lg)}>
                                <Fallback
                                    color={Some(*color)}
                                    variant={Some(Variant::Soft)}
                                >
                                    {"AG"}
                                </Fallback>
                            </Avatar>
                            </div>
                        </td>
                    }) }
                </tr>
                <tr>
                    <th class="p-4 text-white/50 font-normal" scope="row">{"img"}</th>
                    { for colors.iter().map(|(_, _, url)| html! {
                        <td class="p-4 text-center">
                            <div class="flex justify-center">
                            <Avatar size={Some(Size::Lg)}>
                                <Image src={*url} alt="User photo" />
                            </Avatar>
                            </div>
                        </td>
                    }) }
                </tr>
            </tbody>
        </table>
    }
}"# }
                    </pre>
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
