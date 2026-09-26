// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc = include_str!("../LEPTOS.md")]

use crate::common::{
    Color, ImageLoadingStatus, Overlap, Size, Variant, base_container_style, base_fallback_style,
    base_group_grid_style, base_group_style, child_overlap_style,
};
use image_rs::common::{
    AriaLive, AriaPressed, CrossOrigin, Decoding, FetchPriority, Layout, Loading, ObjectFit,
    Position, ReferrerPolicy,
};
use image_rs::leptos::Image as ImageRS;
use leptos::callback::Callback;
use leptos::prelude::*;
use leptos::task::spawn_local;
use std::time::Duration;

/// Shared context propagated by [`Group`] to its children.
///
/// Passed via [`provide_context`] and consumed by descendant [`Avatar`] and
/// [`Count`] components.
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct GroupContext {
    /// Default size for all child avatars.
    pub size: Size,
    /// Default colour theme for child avatar fallbacks.
    pub color: Color,
    /// Default variant for child avatar fallbacks.
    pub variant: Variant,
    /// Overlap mode applied to child avatars in stacked layout.
    pub overlap: Overlap,
    /// When `true`, children are arranged in a wrapping grid.
    pub is_grid: bool,
    /// Number of avatars hidden by `max`. Automatically computed from `total - max`.
    /// [`Count`] reads this when its own `count` prop is `0`.
    pub overflow_count: usize,
}

/// A container that stacks or grids multiple [`Avatar`] components together.
///
/// Propagates size, colour, variant, and overlap configuration to its children
/// via Leptos context. When `max` is set, only the first `max` children are
/// displayed (the caller must slice their iterator before passing children).
///
/// # Accessibility
///
/// - Renders as a `<div role="group">` landmark.
/// - The `aria_label` prop is required for screen readers.
///
/// # Examples
///
/// ```rust
/// use avatar::leptos::{Avatar, Group, Image};
/// use avatar::Size;
/// use leptos::prelude::*;
///
/// #[component]
/// pub fn MyGroup() -> impl IntoView {
///     view! {
///         <Group size=Size::Lg aria_label="Project members">
///             <Avatar><Image src="https://i.pravatar.cc/150?u=1" alt="Alice" /></Avatar>
///             <Avatar><Image src="https://i.pravatar.cc/150?u=2" alt="Bob" /></Avatar>
///         </Group>
///     }
/// }
/// ```
#[component]
pub fn Group(
    /// Child [`Avatar`] elements and an optional [`Count`].
    children: Children,

    /// Maximum number of avatars to display before showing an overflow count.
    /// In Leptos, children cannot be slice, **manually** render only `max` children
    /// and pair this with the `total` prop so overflow is computed automatically.
    #[prop(optional)]
    max: Option<usize>,

    /// Total number of avatars (including hidden ones). Used with `max` to
    /// compute `overflow_count = total - max` stored in context and read by
    /// [`Count`] when it has no explicit `count`.
    #[prop(optional)]
    total: Option<usize>,

    /// Default size inherited by child avatars.
    #[prop(default = Size::Md)]
    size: Size,

    /// Default colour theme inherited by child avatar fallbacks.
    #[prop(default = Color::Default)]
    color: Color,

    /// Default style variant inherited by child avatar fallbacks.
    #[prop(default = Variant::Default)]
    variant: Variant,

    /// Stacking overlap style.
    #[prop(default = Overlap::Clip)]
    overlap: Overlap,

    /// When `true`, the group uses a wrapping grid layout.
    #[prop(default = false)]
    is_grid: bool,

    /// Additional CSS class names for the group container `<div>`.
    #[prop(default = "")]
    class: &'static str,

    /// Inline CSS added to the group container `<div>`.
    #[prop(default = "")]
    style: &'static str,

    /// `id` attribute on the group container element.
    #[prop(default = "")]
    id: &'static str,

    /// Accessible label announced by screen readers for the group.
    #[prop(default = "Avatar group")]
    aria_label: &'static str,

    /// ARIA `role` attribute.
    #[prop(default = "group")]
    role: &'static str,

    /// `tabindex` for keyboard focus management.
    #[prop(default = "0")]
    tabindex: &'static str,

    /// `data-testid` for automated testing.
    #[prop(default = "")]
    data_testid: &'static str,
) -> impl IntoView {
    let overflow_count = match (max, total) {
        (Some(max), Some(total)) => total.saturating_sub(max),
        _ => 0,
    };

    let ctx = GroupContext {
        size,
        color,
        variant,
        overlap,
        is_grid,
        overflow_count,
    };

    provide_context(ctx);

    let base_style = if is_grid {
        base_group_grid_style()
    } else {
        base_group_style()
    };

    let group_class = if is_grid {
        "avatar-group avatar-group--grid"
    } else {
        match overlap {
            Overlap::Clip => "avatar-group avatar-group--clip",
            Overlap::Ring => "avatar-group avatar-group--ring",
        }
    };

    let full_style = format!("{} {}", base_style, style);

    view! {
        <div
            id=id
            class=format!("{} {}", group_class, class)
            style=full_style
            role=role
            aria-label=aria_label
            tabindex=tabindex
            data-testid=data_testid
        >
            {children()}
        </div>
    }
}

/// Displays an overflow count badge inside an [`Group`].
///
/// Inherits size, colour, and variant from the surrounding [`GroupContext`].
///
/// # Accessibility
///
/// The badge renders inside an [`Avatar`] with `role="img"` and the `aria_label`
/// is announced by screen readers.
#[component]
pub fn Count(
    /// Explicit children to render as the label. Overrides `count`.
    #[prop(optional)]
    children: Option<ChildrenFn>,

    /// Numeric surplus count displayed as `+{count}` when `children` is empty.
    #[prop(default = 0)]
    count: usize,

    /// Overrides the parent group's size.
    #[prop(optional)]
    size: Option<Size>,

    /// Overrides the parent group's colour theme.
    #[prop(optional)]
    color: Option<Color>,

    /// Overrides the parent group's variant.
    #[prop(optional)]
    variant: Option<Variant>,

    /// Additional CSS class names on the indicator element.
    #[prop(default = "")]
    class: &'static str,

    /// Inline CSS on the indicator element.
    #[prop(default = "")]
    style: &'static str,

    /// `id` attribute on the indicator element.
    #[prop(default = "")]
    id: &'static str,

    /// Accessible label for the count indicator.
    #[prop(default = "Additional members")]
    aria_label: &'static str,
) -> impl IntoView {
    let group = use_context::<GroupContext>();
    let (group_size, group_color, group_variant, ctx_overflow) = group
        .map(|ctx| (ctx.size, ctx.color, ctx.variant, ctx.overflow_count))
        .unwrap_or((Size::Md, Color::Default, Variant::Default, 0));

    let final_size = size.unwrap_or(group_size);
    let final_color = color.unwrap_or(group_color);
    let final_variant = variant.unwrap_or(group_variant);

    // When count is 0 (not explicitly set), fall back to the overflow from context
    let effective_count = if count == 0 { ctx_overflow } else { count };

    let content_to_store = match children {
        Some(c) => leptos::either::Either::Left(c),
        None => leptos::either::Either::Right(format!("+{effective_count}")),
    };
    let stored_content = StoredValue::new_local(content_to_store);

    view! {
        <Avatar
            size=final_size
            container_class="avatar-group__count"
            class=class
            style=style
            id=id
            aria_label=aria_label
        >
            <Fallback color=final_color variant=final_variant>
                {
                    stored_content.with_value(|c| match c {
                        leptos::either::Either::Left(ch) => ch().into_any(),
                        leptos::either::Either::Right(s) => s.clone().into_any(),
                    })
                }
            </Fallback>
        </Avatar>
    }
}

/// A circular image container that renders either a photo or a fallback.
///
/// Context-aware: when nested inside [`Group`], size, colour, and overlap
/// are inherited automatically. Accepts [`Image`] and [`Fallback`]
/// as children.
///
/// # Accessibility
///
/// - Rendered as `<span role="img">` with a required `aria_label`.
/// - The fallback is hidden from assistive technology once the image loads.
///
/// # Examples
///
/// ```rust
/// use avatar::leptos::{Avatar, Image, Fallback};
/// use avatar::{Color, Variant};
/// use leptos::prelude::*;
///
/// #[component]
/// pub fn MyAvatar() -> impl IntoView {
///     view! {
///         <Avatar aria_label="Ferris Prophet">
///             <Image src="https://i.pravatar.cc/300" alt="Ferris Prophet" />
///             <Fallback color=Color::Accent variant=Variant::Soft>
///                 "FP"
///             </Fallback>
///         </Avatar>
///     }
/// }
/// ```
#[component]
pub fn Avatar(
    /// Content slotted inside the avatar.
    children: Children,

    /// Size of the avatar. Inherits from [`GroupContext`] if omitted.
    #[prop(optional)]
    size: Option<Size>,

    /// Additional CSS class names on the root `<span>`.
    #[prop(default = "")]
    class: &'static str,

    /// Inline CSS on the root `<span>`.
    #[prop(default = "")]
    style: &'static str,

    /// `id` attribute on the root `<span>`.
    #[prop(default = "")]
    id: &'static str,

    /// Accessible label for the avatar, announced by screen readers.
    #[prop(default = "Avatar")]
    aria_label: &'static str,

    /// `tabindex` for keyboard focus.
    #[prop(default = "0")]
    tabindex: &'static str,

    /// CSS class names on the inner container `<span>`.
    #[prop(default = "avatar")]
    container_class: &'static str,

    /// Inline CSS on the inner container `<span>`.
    #[prop(default = "")]
    container_style: &'static str,

    /// `data-testid` for automated testing.
    #[prop(default = "")]
    data_testid: &'static str,
) -> impl IntoView {
    let group = use_context::<GroupContext>();
    let status = RwSignal::new(ImageLoadingStatus::Idle);
    provide_context(status);

    let computed_size = size.unwrap_or_else(|| group.map(|ctx| ctx.size).unwrap_or(Size::Md));

    let overlap_style = match group {
        Some(ctx) if !ctx.is_grid => child_overlap_style(ctx.overlap, computed_size),
        _ => "",
    };

    let full_style = format!(
        "{} {} {} {} {}",
        base_container_style(),
        computed_size.to_style(),
        computed_size.to_font_style(),
        overlap_style,
        style
    );

    view! {
        <span
            id=id
            class=format!("{} {}", container_class, class)
            style=format!("{} {}", full_style, container_style)
            role="img"
            aria-label=aria_label
            tabindex=tabindex
            data-testid=data_testid
        >
            {children()}
        </span>
    }
}

/// Renders the `<img>` element inside a Leptos [`Avatar`].
///
/// Must be placed inside an [`Avatar`] component, it reads
/// `RwSignal<ImageLoadingStatus>` from context to update the parent's state.
/// The wrapper `<span>` is hidden (`display: none`) until the image loads.
///
/// # Accessibility
///
/// - `aria-hidden` is managed automatically based on loading state.
/// - `alt` text is forwarded to the underlying `<img>` element.
///
/// # Panics
///
/// Panics in debug mode when used outside an [`Avatar`] context.
#[component]
pub fn Image(
    /// URL of the image to display.
    #[prop(default = "")]
    src: &'static str,

    /// Fallback image URL shown when `src` fails to load.
    #[prop(default = "")]
    fallback_src: &'static str,

    /// `srcset` attribute for responsive images.
    #[prop(default = "")]
    srcset: &'static str,

    /// `sizes` attribute for responsive layout hints.
    #[prop(default = "")]
    sizes: &'static str,

    /// Alternative text for the image (required for accessibility).
    #[prop(default = "")]
    alt: &'static str,

    /// CSS class on the wrapper `<span>`.
    #[prop(default = "avatar__image")]
    class: &'static str,

    /// Inline CSS on the wrapper `<span>`.
    #[prop(default = "")]
    style: &'static str,

    /// `id` on the wrapper `<span>`.
    #[prop(default = "")]
    id: &'static str,

    /// `loading` hint: `Loading::Eager` (default) or `Loading::Lazy`.
    #[prop(default = Loading::Eager)]
    loading: Loading,

    /// Cross-origin attribute value.
    #[prop(default = CrossOrigin::None)]
    crossorigin: CrossOrigin,

    /// Image decoding hint.
    #[prop(default = Decoding::Auto)]
    decoding: Decoding,

    /// Referrer policy for the image fetch.
    #[prop(default = ReferrerPolicy::NoReferrer)]
    referrerpolicy: ReferrerPolicy,

    /// Fetch priority hint.
    #[prop(default = FetchPriority::Auto)]
    fetchpriority: FetchPriority,

    /// CSS `object-fit` value.
    #[prop(default = ObjectFit::Cover)]
    object_fit: ObjectFit,

    /// CSS `object-position` value.
    #[prop(default = Position::Center)]
    object_position: Position,

    /// Layout mode forwarded to `image-rs`.
    #[prop(default = Layout::Responsive)]
    layout: Layout,

    /// Image quality hint.
    #[prop(default = "")]
    quality: &'static str,

    /// Placeholder treatment while loading.
    #[prop(default = "")]
    placeholder: &'static str,

    /// Base64 blur data URL used when `placeholder` is `"blur"`.
    #[prop(default = "")]
    blur_data_url: &'static str,

    /// `elementtiming` attribute for LCP measurement.
    #[prop(default = "")]
    elementtiming: &'static str,

    /// `aria-live` region behaviour.
    #[prop(default = AriaLive::Off)]
    aria_live: AriaLive,

    /// `aria-pressed` state.
    #[prop(default = AriaPressed::Undefined)]
    aria_pressed: AriaPressed,

    /// `aria-controls` pointing to a related element's `id`.
    #[prop(default = "")]
    aria_controls: &'static str,

    /// `aria-labelledby` pointing to a labelling element's `id`.
    #[prop(default = "")]
    aria_labelledby: &'static str,

    /// `aria-describedby` pointing to a description element's `id`.
    #[prop(default = "")]
    aria_describedby: &'static str,

    /// `aria-expanded` state.
    #[prop(default = "")]
    aria_expanded: &'static str,

    /// Intrinsic `width` attribute.
    #[prop(default = "")]
    width: &'static str,

    /// Intrinsic `height` attribute.
    #[prop(default = "")]
    height: &'static str,

    /// Callback fired when the image finishes loading.
    #[prop(optional)]
    on_load: Option<Callback<()>>,

    /// Callback fired when the image fails to load.
    #[prop(optional)]
    on_error: Option<Callback<String>>,
) -> impl IntoView {
    let _ = quality;
    let _ = aria_live;
    let _ = aria_pressed;
    let _ = aria_controls;
    let _ = aria_labelledby;
    let _ = aria_describedby;
    let _ = aria_expanded;

    let status =
        use_context::<RwSignal<ImageLoadingStatus>>().expect("Image must be used inside Avatar");

    Effect::new(move |_| {
        if !src.is_empty() {
            status.set(ImageLoadingStatus::Loading);
        }
    });

    let is_loaded = move || status.get() == ImageLoadingStatus::Loaded;

    let wrapper_style = move || {
        let base = "width: 100%; height: 100%; object-fit: cover; border-radius: inherit;";
        if is_loaded() {
            format!("{} {}", base, style)
        } else {
            format!("{} display: none; {}", base, style)
        }
    };

    let load_cb = Callback::new(move |()| {
        status.set(ImageLoadingStatus::Loaded);
        if let Some(cb) = on_load {
            cb.run(());
        }
    });

    let error_cb = Callback::new(move |err: String| {
        status.set(ImageLoadingStatus::Error);
        if let Some(cb) = on_error {
            cb.run(err);
        }
    });

    view! {
        <span
            id=id
            class=class
            style=wrapper_style
            aria-hidden=move || if is_loaded() { "false" } else { "true" }
        >
            <ImageRS
                src=src
                fallback_src=fallback_src
                srcset=srcset
                sizes=sizes
                alt=alt
                style="width: 100%; height: 100%; object-fit: cover; border-radius: inherit;"
                loading=loading
                crossorigin=crossorigin
                decoding=decoding
                referrerpolicy=referrerpolicy
                fetchpriority=fetchpriority
                object_fit=object_fit
                object_position=object_position
                layout=layout
                placeholder=placeholder
                blur_data_url=blur_data_url
                elementtiming=elementtiming
                width=width
                height=height
                on_load=load_cb
                on_error=error_cb
            />
        </span>
    }
}

/// Shows textual initials or an icon when the [`Image`] has not loaded.
///
/// Must be inside an [`Avatar`], reads [`RwSignal<ImageLoadingStatus>`] from
/// context to determine visibility. Hidden once the image is visible.
///
/// # Examples
///
/// ```rust
/// use avatar::leptos::{Avatar, Fallback};
/// use avatar::Color;
/// use leptos::prelude::*;
///
/// #[component]
/// pub fn MyFallback() -> impl IntoView {
///     view! {
///         <Avatar aria_label="Ferris Prophet">
///             <Fallback color=Color::Accent>"FP"</Fallback>
///         </Avatar>
///     }
/// }
/// ```
#[component]
pub fn Fallback(
    /// Fallback content, initials, icon, or any view.
    children: ChildrenFn,

    /// Delay in milliseconds before the fallback becomes visible.
    #[prop(default = 0u32)]
    delay_ms: u32,

    /// Colour theme override. Inherits from [`GroupContext`] when `None`.
    #[prop(optional)]
    color: Option<Color>,

    /// Visual style override. Inherits from [`GroupContext`] when `None`.
    #[prop(optional)]
    variant: Option<Variant>,

    /// CSS class on the fallback `<span>`.
    #[prop(default = "avatar__fallback")]
    class: &'static str,

    /// Inline CSS on the fallback `<span>`.
    #[prop(default = "")]
    style: &'static str,

    /// `id` attribute on the fallback `<span>`.
    #[prop(default = "")]
    id: &'static str,

    /// `data-testid` for automated testing.
    #[prop(default = "")]
    data_testid: &'static str,
) -> impl IntoView {
    let status =
        use_context::<RwSignal<ImageLoadingStatus>>().expect("Fallback must be used inside Avatar");

    let group = use_context::<GroupContext>();
    let visible = RwSignal::new(delay_ms == 0);

    Effect::new(move |_| {
        if delay_ms > 0 {
            spawn_local(async move {
                gloo_timers::future::sleep(Duration::from_millis(u64::from(delay_ms))).await;
                visible.set(true);
            });
        }
    });

    let (group_color, group_variant) = group
        .map(|ctx| (ctx.color, ctx.variant))
        .unwrap_or((Color::Default, Variant::Default));

    let final_color = color.unwrap_or(group_color);
    let final_variant = variant.unwrap_or(group_variant);

    let color_style = match final_variant {
        Variant::Soft => final_color.to_soft_style(),
        Variant::Default => final_color.to_style(),
    };

    let full_style = format!("{} {} {}", base_fallback_style(), color_style, style);

    let is_image_loaded = move || status.get() == ImageLoadingStatus::Loaded;
    let should_show = move || visible.get() && !is_image_loaded();

    view! {
        <Show when=should_show>
            <span
                id=id
                class=class
                style=full_style.clone()
                aria-hidden=move || if is_image_loaded() { "true" } else { "false" }
                data-testid=data_testid
            >
                {children()}
            </span>
        </Show>
    }
}

// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
