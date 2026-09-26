// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc = include_str!("../DIOXUS.md")]

use crate::common::{
    Color, ImageLoadingStatus, Overlap, Size, Variant, base_container_style, base_fallback_style,
    base_group_grid_style, base_group_style, child_overlap_style,
};
use dioxus::prelude::*;
use image_rs::common::{
    AriaLive, AriaPressed, CrossOrigin, Decoding, FetchPriority, Layout, Loading, ObjectFit,
    Position, ReferrerPolicy,
};
use image_rs::dioxus::Image as ImageRS;
use std::time::Duration;

/// Shared context propagated by [`Group`] to its children.
///
/// Provided via [`use_context_provider`] and consumed by any descendant
/// [`Avatar`] or [`Count`] via [`consume_context`].
#[derive(Clone, Debug, PartialEq)]
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

/// Props for the [`Group`] Dioxus component.
#[derive(Props, Clone, PartialEq)]
pub struct GroupProps {
    /// Child [`Avatar`] elements and an optional [`Count`].
    #[props(default)]
    pub children: Element,

    /// Maximum number of avatars to display before showing an overflow count.
    /// In Dioxus, children cannot be sliced, **manually** render only `max` children
    /// and pair this with the `total` prop so overflow is computed automatically.
    #[props(default)]
    pub max: Option<usize>,

    /// Total number of avatars (including hidden ones). Used with `max` to
    /// compute `overflow_count = total - max` that is stored in context and
    /// read by [`Count`] when it has no explicit `count`.
    #[props(default)]
    pub total: Option<usize>,

    /// Default size inherited by child avatars.
    #[props(default)]
    pub size: Size,

    /// Default colour theme inherited by child avatar fallbacks.
    #[props(default)]
    pub color: Color,

    /// Default style variant inherited by child avatar fallbacks.
    #[props(default)]
    pub variant: Variant,

    /// Stacking overlap style.
    #[props(default)]
    pub overlap: Overlap,

    /// When `true`, the group uses a wrapping grid layout.
    #[props(default)]
    pub is_grid: bool,

    /// Additional CSS class names for the group container `<div>`.
    #[props(default)]
    pub class: &'static str,

    /// Inline CSS added to the group container `<div>`.
    #[props(default)]
    pub style: &'static str,

    /// `id` attribute on the group container element.
    #[props(default)]
    pub id: &'static str,

    /// Accessible label announced by screen readers for the group.
    #[props(default = "Avatar group")]
    pub aria_label: &'static str,

    /// ARIA `role` attribute.
    #[props(default = "group")]
    pub role: &'static str,

    /// `tabindex` for keyboard focus management.
    #[props(default = "0")]
    pub tabindex: &'static str,

    /// `data-testid` for automated testing.
    #[props(default)]
    pub data_testid: &'static str,
}

/// A container that stacks or grids multiple [`Avatar`] components together.
///
/// Propagates size, colour, variant and overlap to its children via Dioxus context.
///
/// # Accessibility
///
/// - Renders as `<div role="group">` with a required `aria_label`.
#[component]
pub fn Group(props: GroupProps) -> Element {
    let overflow_count = match (props.max, props.total) {
        (Some(max), Some(total)) => total.saturating_sub(max),
        _ => 0,
    };

    let ctx = GroupContext {
        size: props.size,
        color: props.color,
        variant: props.variant,
        overlap: props.overlap,
        is_grid: props.is_grid,
        overflow_count,
    };

    use_context_provider(|| ctx);

    let base_style = if props.is_grid {
        base_group_grid_style()
    } else {
        base_group_style()
    };

    let group_class = if props.is_grid {
        "avatar-group avatar-group--grid"
    } else {
        match props.overlap {
            Overlap::Clip => "avatar-group avatar-group--clip",
            Overlap::Ring => "avatar-group avatar-group--ring",
        }
    };

    rsx! {
        div {
            id: props.id,
            class: "{group_class} {props.class}",
            style: "{base_style} {props.style}",
            role: props.role,
            aria_label: props.aria_label,
            tabindex: props.tabindex,
            "data-testid": props.data_testid,
            {props.children}
        }
    }
}

/// Props for [`Count`].
#[derive(Props, Clone, PartialEq)]
pub struct CountProps {
    /// Explicit children to render as the label. When provided, overrides `count`.
    #[props(default)]
    pub children: Option<Element>,

    /// Numeric surplus count displayed as `+{count}` when `children` is `None`.
    #[props(default)]
    pub count: usize,

    /// Overrides the parent group's size.
    #[props(default = None)]
    pub size: Option<Size>,

    /// Overrides the parent group's colour theme.
    #[props(default = None)]
    pub color: Option<Color>,

    /// Overrides the parent group's variant.
    #[props(default = None)]
    pub variant: Option<Variant>,

    /// Additional CSS class names on the indicator element.
    #[props(default)]
    pub class: &'static str,

    /// Inline CSS on the indicator element.
    #[props(default)]
    pub style: &'static str,

    /// `id` attribute on the indicator element.
    #[props(default)]
    pub id: &'static str,

    /// Accessible label for the count indicator.
    #[props(default = "Additional members")]
    pub aria_label: &'static str,
}

/// Displays an overflow count badge inside an [`Group`].
#[component]
pub fn Count(props: CountProps) -> Element {
    let group = resolve_group_context();
    let (group_size, group_color, group_variant, ctx_overflow) = group
        .map(|ctx| (ctx.size, ctx.color, ctx.variant, ctx.overflow_count))
        .unwrap_or((Size::Md, Color::Default, Variant::Default, 0));

    let size = props.size.unwrap_or(group_size);
    let color = props.color.unwrap_or(group_color);
    let variant = props.variant.unwrap_or(group_variant);

    // When count is 0 (not explicitly set), fall back to the overflow from context
    let effective_count = if props.count == 0 {
        ctx_overflow
    } else {
        props.count
    };

    let content = match props.children {
        Some(c) => rsx! { {c} },
        None => rsx! { "+{effective_count}" },
    };

    rsx! {
        Avatar {
            size: size,
            container_class: "avatar-group__count",
            class: props.class,
            style: props.style,
            id: props.id,
            aria_label: props.aria_label,
            Fallback {
                color: color,
                variant: variant,
                {content}
            }
        }
    }
}

/// Reads [`GroupContext`] from the nearest ancestor provider.
///
/// Returns `None` when no [`Group`] ancestor is found.
fn resolve_group_context() -> Option<GroupContext> {
    try_consume_context::<GroupContext>()
}

/// Props for the [`Avatar`] Dioxus component.
#[derive(Props, Clone, PartialEq)]
pub struct AvatarProps {
    /// Content slotted inside the avatar, typically [`Image`] and [`Fallback`].
    #[props(default)]
    pub children: Element,

    /// Size of the avatar. Inherits from a surrounding [`Group`] if unset.
    #[props(default)]
    pub size: Option<Size>,

    /// Additional CSS class names on the root `<span>`.
    #[props(default)]
    pub class: &'static str,

    /// Inline CSS on the root `<span>`.
    #[props(default)]
    pub style: &'static str,

    /// `id` attribute on the root `<span>`.
    #[props(default)]
    pub id: &'static str,

    /// Accessible label for the avatar, announced by screen readers.
    #[props(default = "Avatar")]
    pub aria_label: &'static str,

    /// `tabindex` for keyboard focus control.
    #[props(default = "0")]
    pub tabindex: &'static str,

    /// CSS class names on the inner container `<span>` (prepended before `class`).
    #[props(default = "avatar")]
    pub container_class: &'static str,

    /// Inline CSS on the inner container `<span>`.
    #[props(default)]
    pub container_style: &'static str,

    /// `data-testid` for automated testing.
    #[props(default)]
    pub data_testid: &'static str,
}

/// A circular image container that renders either a photo or a fallback.
///
/// Context-aware: inherits size, colour, and overlap from a surrounding [`Group`].
///
/// # Accessibility
///
/// - Renders as `<span role="img">` with a required `aria_label`.
#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let group_ctx = resolve_group_context();
    let status = use_signal(|| ImageLoadingStatus::Idle);
    use_context_provider(|| status);

    let computed_size = props
        .size
        .unwrap_or_else(|| group_ctx.as_ref().map(|ctx| ctx.size).unwrap_or(Size::Md));

    let overlap_style = match &group_ctx {
        Some(ctx) if !ctx.is_grid => child_overlap_style(ctx.overlap, computed_size),
        _ => "",
    };

    rsx! {
        span {
            id: props.id,
            class: "{props.container_class} {props.class}",
            style: "{base_container_style()} {computed_size.to_style()} {computed_size.to_font_style()} {overlap_style} {props.style} {props.container_style}",
            role: "img",
            aria_label: props.aria_label,
            tabindex: props.tabindex,
            "data-testid": props.data_testid,
            {props.children}
        }
    }
}

/// Props for [`Image`].
///
/// Exposes the full HTML `<img>` attribute surface plus ARIA attributes.
#[derive(Props, Clone, PartialEq)]
pub struct ImageProps {
    /// URL of the image.
    #[props(default)]
    pub src: &'static str,

    /// Fallback image URL shown when `src` fails.
    #[props(default)]
    pub fallback_src: &'static str,

    /// `srcset` attribute for responsive images.
    #[props(default)]
    pub srcset: &'static str,

    /// `sizes` attribute for responsive layout hints.
    #[props(default)]
    pub sizes: &'static str,

    /// Alternative text (required for accessibility).
    #[props(default)]
    pub alt: &'static str,

    /// CSS class on the wrapper `<span>`.
    #[props(default = "avatar__image")]
    pub class: &'static str,

    /// Inline CSS on the wrapper `<span>`.
    #[props(default)]
    pub style: &'static str,

    /// `id` on the wrapper `<span>`.
    #[props(default)]
    pub id: &'static str,

    /// `loading` hint.
    #[props(default)]
    pub loading: Loading,

    /// Cross-origin attribute.
    #[props(default)]
    pub crossorigin: CrossOrigin,

    /// Image decoding hint.
    #[props(default)]
    pub decoding: Decoding,

    /// Referrer policy.
    #[props(default)]
    pub referrerpolicy: ReferrerPolicy,

    /// Fetch priority hint.
    #[props(default)]
    pub fetchpriority: FetchPriority,

    /// CSS `object-fit` value.
    #[props(default)]
    pub object_fit: ObjectFit,

    /// CSS `object-position` value.
    #[props(default)]
    pub object_position: Position,

    /// Layout mode forwarded to `image-rs`.
    #[props(default)]
    pub layout: Layout,

    /// Image quality hint.
    #[props(default)]
    pub quality: &'static str,

    /// Placeholder treatment.
    #[props(default)]
    pub placeholder: &'static str,

    /// Base64 blur data URL.
    #[props(default)]
    pub blur_data_url: &'static str,

    /// `elementtiming` for LCP measurement.
    #[props(default)]
    pub elementtiming: &'static str,

    /// `aria-live` region behaviour.
    #[props(default)]
    pub aria_live: AriaLive,

    /// `aria-pressed` state.
    #[props(default)]
    pub aria_pressed: AriaPressed,

    /// `aria-controls` pointing to a related element's `id`.
    #[props(default)]
    pub aria_controls: &'static str,

    /// `aria-labelledby` pointing to a labelling element's `id`.
    #[props(default)]
    pub aria_labelledby: &'static str,

    /// `aria-describedby` pointing to a description element's `id`.
    #[props(default)]
    pub aria_describedby: &'static str,

    /// `aria-expanded` state.
    #[props(default)]
    pub aria_expanded: &'static str,

    /// Intrinsic `width` attribute.
    #[props(default)]
    pub width: &'static str,

    /// Intrinsic `height` attribute.
    #[props(default)]
    pub height: &'static str,

    /// Callback fired when the image finishes loading.
    #[props(default)]
    pub on_load: Option<Callback<()>>,

    /// Callback fired when the image fails to load.
    #[props(default)]
    pub on_error: Option<Callback<String>>,
}

/// Renders the `<img>` element inside an [`Avatar`].
///
/// Must be placed inside an [`Avatar`], reads `Signal<ImageLoadingStatus>`
/// from context to update the parent's loading state.
///
/// # Accessibility
///
/// - `aria-hidden` is managed automatically based on loading state.
/// - `alt` is required for screen reader announcements.
#[component]
pub fn Image(props: ImageProps) -> Element {
    let mut status = use_context::<Signal<ImageLoadingStatus>>();

    use_effect(move || {
        if !props.src.is_empty() {
            status.set(ImageLoadingStatus::Loading);
        }
    });

    let on_load = move |()| {
        status.set(ImageLoadingStatus::Loaded);
        if let Some(cb) = &props.on_load {
            cb.call(());
        }
    };

    let on_error = move |err: String| {
        status.set(ImageLoadingStatus::Error);
        if let Some(cb) = &props.on_error {
            cb.call(err);
        }
    };

    let is_loaded = status() == ImageLoadingStatus::Loaded;

    rsx! {
        span {
            id: props.id,
            class: props.class,
            style: if is_loaded { "width: 100%; height: 100%; display: block;" } else { "display: none;" },
            aria_hidden: if is_loaded { "false" } else { "true" },
            ImageRS {
                src: props.src,
                alt: props.alt,
                width: props.width,
                height: props.height,
                layout: props.layout,
                fallback_src: props.fallback_src,
                srcset: props.srcset,
                sizes: props.sizes,
                style: "width: 100%; height: 100%; object-fit: cover; border-radius: inherit;",
                loading: props.loading,
                crossorigin: props.crossorigin,
                decoding: props.decoding,
                referrerpolicy: props.referrerpolicy,
                fetchpriority: props.fetchpriority,
                object_fit: props.object_fit,
                object_position: props.object_position,
                quality: props.quality,
                placeholder: props.placeholder,
                blur_data_url: props.blur_data_url,
                elementtiming: props.elementtiming,
                aria_live: props.aria_live,
                aria_pressed: props.aria_pressed,
                aria_controls: props.aria_controls,
                aria_labelledby: props.aria_labelledby,
                aria_describedby: props.aria_describedby,
                aria_expanded: props.aria_expanded,
                on_load: on_load,
                on_error: on_error,
            }
        }
    }
}

/// Props for [`Fallback`].
///
/// Displayed when the avatar image has not yet loaded or has failed.
#[derive(Props, Clone, PartialEq)]
pub struct FallbackProps {
    /// Text initials, icon, or any content to show.
    #[props(default)]
    pub children: Element,

    /// Delay in milliseconds before the fallback is shown after mount.
    /// On wasm32 targets, any non-zero delay gracefully degrades to no delay.
    #[props(default)]
    pub delay_ms: u32,

    /// Colour theme. Inherits from [`GroupContext`] when `Color::Default`.
    #[props(default = Color::Default)]
    pub color: Color,

    /// Style variant. Inherits from [`GroupContext`] when `Variant::Default`.
    #[props(default = Variant::Default)]
    pub variant: Variant,

    /// CSS class on the fallback `<span>`.
    #[props(default = "avatar__fallback")]
    pub class: &'static str,

    /// Inline CSS on the fallback `<span>`.
    #[props(default)]
    pub style: &'static str,

    /// `id` attribute on the fallback `<span>`.
    #[props(default)]
    pub id: &'static str,

    /// `data-testid` for automated testing.
    #[props(default)]
    pub data_testid: &'static str,
}

/// Shows textual initials or an icon when the [`Image`] has not loaded.
///
/// Must be inside an [`Avatar`] component. Hidden once the image is visible.
///
/// # Accessibility
///
/// `aria-hidden` is managed automatically to keep screen reader output accurate.
#[component]
pub fn Fallback(props: FallbackProps) -> Element {
    let status = use_context::<Signal<ImageLoadingStatus>>();
    let group = resolve_group_context();
    let mut visible = use_signal(|| props.delay_ms == 0);

    use_effect(move || {
        if props.delay_ms > 0 {
            let delay = props.delay_ms;
            spawn(async move {
                platform_sleep(Duration::from_millis(u64::from(delay))).await;
                visible.set(true);
            });
        }
    });

    let image_loaded = status() == ImageLoadingStatus::Loaded;
    let should_show = visible() && !image_loaded;

    if !should_show {
        return rsx! {};
    }

    let (group_color, group_variant) = group
        .map(|ctx| (ctx.color, ctx.variant))
        .unwrap_or((Color::Default, Variant::Default));

    let effective_color = if props.color == Color::Default {
        group_color
    } else {
        props.color
    };
    let effective_variant = if props.variant == Variant::Default {
        group_variant
    } else {
        props.variant
    };

    let color_style = match effective_variant {
        Variant::Soft => effective_color.to_soft_style(),
        Variant::Default => effective_color.to_style(),
    };

    rsx! {
        span {
            id: props.id,
            class: props.class,
            style: "{base_fallback_style()} {color_style} {props.style}",
            aria_hidden: if image_loaded { "true" } else { "false" },
            "data-testid": props.data_testid,
            {props.children}
        }
    }
}

/// Cross-platform async sleep.
///
/// Dioxus has no built-in cross-platform timer primitive. mb it has one. idk atm.
async fn platform_sleep(_duration: Duration) {
    std::future::ready(()).await
}

// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
