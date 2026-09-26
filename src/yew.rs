// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc = include_str!("../YEW.md")]

use crate::common::{
    Color, ImageLoadingStatus, Overlap, Size, Variant, base_container_style, base_fallback_style,
    base_group_grid_style, base_group_style, child_overlap_style,
};
use image_rs::common::{
    AriaLive, AriaPressed, CrossOrigin, Decoding, FetchPriority, Layout, Loading, ObjectFit,
    Position, ReferrerPolicy,
};
use image_rs::yew::Image as ImageRS;
use std::time::Duration;
use yew::platform::spawn_local;
use yew::platform::time::sleep;
use yew::prelude::*;

/// Shared image-loading status context passed from [`Avatar`] down to
/// [`Image`] and [`Fallback`].
pub type AvatarContext = UseStateHandle<ImageLoadingStatus>;

/// Shared context propagated by [`Group`] to its children.
///
/// This context contains the default configuration values inherited by all
/// child [`Avatar`] components unless they override them explicitly.
#[derive(Clone, Debug, PartialEq)]
pub struct GroupContext {
    /// Default size for all child avatars.
    pub size: Size,
    /// Default color theme for child avatar fallbacks.
    pub color: Color,
    /// Default variant for child avatar fallbacks.
    pub variant: Variant,
    /// Overlap mode applied to child avatars in stacked layout.
    pub overlap: Overlap,
    /// When `true`, children are arranged in a wrapping grid.
    pub is_grid: bool,
    /// Number of avatars hidden by `max`. Set automatically by [`Group`].
    /// [`Count`] reads this when its own `count` prop is `0`.
    pub overflow_count: usize,
}

/// Props for the [`Group`] Yew component.
#[derive(Properties, PartialEq, Clone)]
pub struct GroupProps {
    /// Child [`Avatar`] elements and an optional [`Count`].
    #[prop_or_default]
    pub children: Children,

    /// Maximum number of avatars to display before showing an overflow count.
    #[prop_or_default]
    pub max: Option<usize>,

    /// Total number of avatars (including hidden ones). Used with `max` to
    /// compute `overflow_count = total - max` stored in context and read by
    /// [`Count`] when it has no explicit `count`.
    #[prop_or_default]
    pub total: Option<usize>,

    /// Default size inherited by child avatars.
    #[prop_or_default]
    pub size: Size,

    /// Default colour theme inherited by child avatar fallbacks.
    #[prop_or_default]
    pub color: Color,

    /// Default style variant inherited by child avatar fallbacks.
    #[prop_or_default]
    pub variant: Variant,

    /// Stacking overlap style, [`Overlap::Clip`] or [`Overlap::Ring`].
    #[prop_or_default]
    pub overlap: Overlap,

    /// When `true`, the group uses a wrapping grid layout.
    #[prop_or_default]
    pub is_grid: bool,

    /// Additional CSS class names for the group container `<div>`.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline CSS added to the group container `<div>`.
    #[prop_or_default]
    pub style: &'static str,

    /// `id` attribute on the group container element.
    #[prop_or_default]
    pub id: &'static str,

    /// Accessible label announced by screen readers for the group.
    #[prop_or("Avatar group")]
    pub aria_label: &'static str,

    /// ARIA `role` attribute. Defaults to `"group"` when unset.
    #[prop_or("group")]
    pub role: &'static str,

    /// `tabindex` for keyboard focus management.
    #[prop_or("0")]
    pub tabindex: &'static str,

    /// `data-testid` for automated testing.
    #[prop_or_default]
    pub data_testid: &'static str,
}

/// A container that stacks or grids multiple [`Avatar`] components together.
///
/// Propagates size, colour, variant, and overlap configuration to its children
/// via Yew context. When [`GroupProps::max`] is set, surplus avatars are
/// replaced with an auto-generated [`Count`] indicator.
///
/// # Accessibility
///
/// - Renders as a `<div role="group">` landmark.
/// - The `aria_label` prop is required for screen readers to announce the group.
/// - Keyboard navigation is supported via `tabindex`.
///
/// # Examples
///
/// ```rust
/// use avatar::yew::{Avatar, Group, Image};
/// use avatar::Size;
/// use yew::prelude::*;
///
/// #[function_component(MyGroup)]
/// pub fn my_group() -> Html {
///     html! {
///         <Group size={Size::Lg} max={3} aria_label="Project members">
///             <Avatar><Image src="https://i.pravatar.cc/150?u=1" alt="Alice" /></Avatar>
///             <Avatar><Image src="https://i.pravatar.cc/150?u=2" alt="Bob" /></Avatar>
///             <Avatar><Image src="https://i.pravatar.cc/150?u=3" alt="Carol" /></Avatar>
///             <Avatar><Image src="https://i.pravatar.cc/150?u=4" alt="Dan" /></Avatar>
///         </Group>
///     }
/// }
/// ```
#[function_component(Group)]
pub fn avatar_group(props: &GroupProps) -> Html {
    let overflow_count = match (props.max, props.total) {
        (Some(max), Some(total)) => total.saturating_sub(max),
        _ => 0,
    };

    let visible_children: Vec<_> = props.children.iter().collect();

    let ctx = GroupContext {
        size: props.size,
        color: props.color,
        variant: props.variant,
        overlap: props.overlap,
        is_grid: props.is_grid,
        overflow_count,
    };

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

    let full_style = format!("{} {}", base_style, props.style);

    html! {
        <ContextProvider<GroupContext> context={ctx}>
            <div
                id={props.id}
                class={format!("{} {}", group_class, props.class)}
                style={full_style}
                role={props.role}
                aria-label={props.aria_label}
                tabindex={props.tabindex}
                data-testid={props.data_testid}
            >
                { for visible_children }
            </div>
        </ContextProvider<GroupContext>>
    }
}

/// Props for [`Count`].
///
/// When placed inside an [`Group`], this component inherits the group's
/// size, colour, and variant unless overridden by explicit props.
#[derive(Properties, PartialEq, Clone)]
pub struct CountProps {
    /// Explicit children to render as the label. Overrides the `count` prop.
    #[prop_or_default]
    pub children: Children,

    /// Numeric surplus count displayed as `+{count}` when `children` is empty.
    #[prop_or_default]
    pub count: usize,

    /// Overrides the parent group's size.
    #[prop_or_default]
    pub size: Option<Size>,

    /// Overrides the parent group's colour theme.
    #[prop_or_default]
    pub color: Option<Color>,

    /// Overrides the parent group's variant.
    #[prop_or_default]
    pub variant: Option<Variant>,

    /// Additional CSS class names on the indicator element.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline CSS on the indicator element.
    #[prop_or_default]
    pub style: &'static str,

    /// `id` attribute on the indicator element.
    #[prop_or_default]
    pub id: &'static str,

    /// Accessible label for the count indicator.
    #[prop_or("Additional members")]
    pub aria_label: &'static str,
}

/// Displays an overflow count badge inside an [`Group`].
///
/// Inherits size, colour, and variant from the surrounding [`GroupContext`].
/// The displayed label is either the `children` slot or `+{count}`.
///
/// # Accessibility
///
/// The surrounding [`Avatar`] renders with `role="img"` and passes the
/// `aria_label` prop through. Screen readers will announce the count badge
/// contents and its label.
#[function_component(Count)]
pub fn avatar_group_count(props: &CountProps) -> Html {
    let group = use_context::<GroupContext>();
    let (group_size, group_color, group_variant, ctx_overflow) = group
        .as_ref()
        .map(|ctx| (ctx.size, ctx.color, ctx.variant, ctx.overflow_count))
        .unwrap_or((Size::Md, Color::Default, Variant::Default, 0));

    let size = props.size.unwrap_or(group_size);
    let color = props.color.unwrap_or(group_color);
    let variant = props.variant.unwrap_or(group_variant);

    // When count is 0 (not set), fall back to the overflow computed by Group
    let effective_count = if props.count == 0 {
        ctx_overflow
    } else {
        props.count
    };

    let label = format!("+{}", effective_count);
    let content = if props.children.is_empty() {
        html! { { label.clone() } }
    } else {
        html! { { for props.children.iter() } }
    };

    html! {
        <Avatar
            size={Some(size)}
            class={classes!("avatar-group__count", props.class)}
            style={props.style}
            id={props.id}
            aria_label={props.aria_label}
        >
            <Fallback color={Some(color)} variant={Some(variant)}>
                { content }
            </Fallback>
        </Avatar>
    }
}

/// Props for the [`Avatar`] Yew component.
///
/// All ARIA and HTML attributes on the root `<span>` element are exposed.
#[derive(Properties, PartialEq, Clone)]
pub struct AvatarProps {
    /// Content slotted inside the avatar, typically [`Image`] and [`Fallback`].
    #[prop_or_default]
    pub children: Children,

    /// Size of the avatar. Inherits from a surrounding [`Group`] if unset.
    #[prop_or_default]
    pub size: Option<Size>,

    /// Additional CSS class names on the root `<span>`.
    #[prop_or_default]
    pub class: Classes,

    /// Inline CSS on the root `<span>`.
    #[prop_or_default]
    pub style: &'static str,

    /// `id` attribute on the root `<span>`.
    #[prop_or_default]
    pub id: &'static str,

    /// Accessible label for the avatar, announced by screen readers.
    #[prop_or("Avatar")]
    pub aria_label: &'static str,

    /// `tabindex` for keyboard focus control.
    #[prop_or("0")]
    pub tabindex: &'static str,

    /// Additional CSS class names on the inner container `<span>`.
    #[prop_or_default]
    pub container_class: &'static str,

    /// Inline CSS on the inner container `<span>`.
    #[prop_or_default]
    pub container_style: &'static str,

    /// `data-testid` for automated testing.
    #[prop_or_default]
    pub data_testid: &'static str,
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
/// - The fallback is visually hidden from assistive technology when the image
///   has loaded (`aria-hidden="true"`).
///
/// # Examples
///
/// ```rust
/// use avatar::yew::{Avatar, Image, Fallback};
/// use avatar::Color;
/// use yew::prelude::*;
///
/// #[function_component(MyAvatar)]
/// pub fn my_avatar() -> Html {
///     html! {
///         <Avatar aria_label="Ferris Prophet">
///             <Image src="https://i.pravatar.cc/300" alt="Ferris Prophet" />
///             <Fallback color={Some(Color::Accent)}>{"FP"}</Fallback>
///         </Avatar>
///     }
/// }
/// ```
#[function_component(Avatar)]
pub fn avatar(props: &AvatarProps) -> Html {
    let group_ctx = use_context::<GroupContext>();
    let status = use_state(|| ImageLoadingStatus::Idle);

    let computed_size = props
        .size
        .unwrap_or_else(|| group_ctx.as_ref().map(|ctx| ctx.size).unwrap_or(Size::Md));

    let overlap_style = match &group_ctx {
        Some(ctx) if !ctx.is_grid => child_overlap_style(ctx.overlap, computed_size),
        _ => "",
    };

    let full_style = format!(
        "{} {} {} {} {}",
        base_container_style(),
        computed_size.to_style(),
        computed_size.to_font_style(),
        overlap_style,
        props.style
    );

    html! {
        <ContextProvider<AvatarContext> context={status}>
            <span
                id={props.id}
                class={classes!("avatar", props.container_class, props.class.clone())}
                style={format!("{} {}", full_style, props.container_style)}
                role="img"
                aria-label={props.aria_label}
                tabindex={props.tabindex}
                data-testid={props.data_testid}
            >
                { for props.children.iter() }
            </span>
        </ContextProvider<AvatarContext>>
    }
}

/// Props for [`Image`].
///
/// Exposes the full HTML `<img>` attribute surface plus ARIA attributes,
/// mirroring `image-rs::yew::Image`.
#[derive(Properties, PartialEq, Clone)]
pub struct ImageProps {
    /// URL of the image to display.
    #[prop_or_default]
    pub src: &'static str,

    /// Fallback image URL shown when `src` fails to load.
    #[prop_or_default]
    pub fallback_src: &'static str,

    /// `srcset` attribute for responsive images.
    #[prop_or_default]
    pub srcset: &'static str,

    /// `sizes` attribute for responsive layout hints.
    #[prop_or_default]
    pub sizes: &'static str,

    /// Alternative text for the image (required for accessibility).
    #[prop_or_default]
    pub alt: &'static str,

    /// CSS class on the wrapper `<span>`.
    #[prop_or("avatar__image")]
    pub class: &'static str,

    /// Inline CSS on the wrapper `<span>`.
    #[prop_or_default]
    pub style: &'static str,

    /// `id` on the wrapper `<span>`.
    #[prop_or_default]
    pub id: &'static str,

    /// `loading` hint: `Loading::Eager` (default) or `Loading::Lazy`.
    #[prop_or_default]
    pub loading: Loading,

    /// Cross-origin attribute value.
    #[prop_or_default]
    pub crossorigin: CrossOrigin,

    /// Image decoding hint.
    #[prop_or_default]
    pub decoding: Decoding,

    /// Referrer policy for the image fetch.
    #[prop_or_default]
    pub referrerpolicy: ReferrerPolicy,

    /// Fetch priority hint.
    #[prop_or_default]
    pub fetchpriority: FetchPriority,

    /// CSS `object-fit` value.
    #[prop_or_default]
    pub object_fit: ObjectFit,

    /// CSS `object-position` value.
    #[prop_or_default]
    pub object_position: Position,

    /// Layout mode forwarded to `image-rs`.
    #[prop_or_default]
    pub layout: Layout,

    /// Image quality hint forwarded to `image-rs`.
    #[prop_or_default]
    pub quality: &'static str,

    /// Placeholder treatment while loading.
    #[prop_or_default]
    pub placeholder: &'static str,

    /// Base64 blur data URL used when `placeholder` is `"blur"`.
    #[prop_or_default]
    pub blur_data_url: &'static str,

    /// `elementtiming` attribute for Largest Contentful Paint measurement.
    #[prop_or_default]
    pub elementtiming: &'static str,

    /// `aria-live` region behaviour.
    #[prop_or_default]
    pub aria_live: AriaLive,

    /// `aria-pressed` state for toggle-button images.
    #[prop_or_default]
    pub aria_pressed: AriaPressed,

    /// `aria-controls` pointing to a related element's `id`.
    #[prop_or_default]
    pub aria_controls: &'static str,

    /// `aria-labelledby` pointing to a labelling element's `id`.
    #[prop_or_default]
    pub aria_labelledby: &'static str,

    /// `aria-describedby` pointing to a description element's `id`.
    #[prop_or_default]
    pub aria_describedby: &'static str,

    /// `aria-expanded` state.
    #[prop_or_default]
    pub aria_expanded: &'static str,

    /// Intrinsic `width` attribute (pixels).
    #[prop_or_default]
    pub width: &'static str,

    /// Intrinsic `height` attribute (pixels).
    #[prop_or_default]
    pub height: &'static str,

    /// Callback fired when the image finishes loading.
    #[prop_or_default]
    pub on_load: Callback<()>,

    /// Callback fired when the image fails to load, receiving the error message.
    #[prop_or_default]
    pub on_error: Callback<String>,
}

/// Renders the `<img>` element inside an [`Avatar`].
///
/// Must be placed inside an [`Avatar`] component, it reads [`AvatarContext`]
/// to update the parent's loading state. The wrapper `<span>` is hidden
/// (`display: none`) until the image loads, then revealed.
///
/// # Accessibility
///
/// - `aria-hidden` is set to `"true"` while loading and `"false"` after load.
/// - `alt` text is forwarded to the underlying `<img>` element.
///
/// # Panics
///
/// Panics in debug mode if used outside an [`Avatar`] context.
#[function_component(Image)]
pub fn avatar_image(props: &ImageProps) -> Html {
    let status = use_context::<AvatarContext>().expect("Image must be inside Avatar");

    let on_load = {
        let status = status.clone();
        let user_cb = props.on_load.clone();
        Callback::from(move |()| {
            status.set(ImageLoadingStatus::Loaded);
            user_cb.emit(());
        })
    };

    let on_error = {
        let status = status.clone();
        let user_cb = props.on_error.clone();
        Callback::from(move |err: String| {
            status.set(ImageLoadingStatus::Error);
            user_cb.emit(err);
        })
    };

    let node_ref = use_node_ref();

    {
        let status = status.clone();
        let node_ref = node_ref.clone();
        use_effect_with(props.src, move |src| {
            if !src.is_empty() {
                let mut is_cached = false;
                if let Some(img) = node_ref.cast::<web_sys::HtmlImageElement>()
                    && img.complete()
                {
                    is_cached = true;
                }

                if is_cached {
                    status.set(ImageLoadingStatus::Loaded);
                } else {
                    status.set(ImageLoadingStatus::Loading);
                }
            }
            || ()
        });
    }

    let is_loaded = *status == ImageLoadingStatus::Loaded;
    let wrapper_style = if is_loaded {
        "width: 100%; height: 100%; display: block;"
    } else {
        "display: none;"
    };

    html! {
        <span
            id={props.id}
            class={props.class}
            style={format!("{} {}", wrapper_style, props.style)}
            aria-hidden={if is_loaded { "false" } else { "true" }}
        >
            <ImageRS
                node_ref={node_ref}
                src={props.src}
                alt={props.alt}
                width={props.width}
                height={props.height}
                layout={props.layout}
                fallback_src={props.fallback_src}
                srcset={props.srcset}
                sizes={props.sizes}
                style="width: 100%; height: 100%; object-fit: cover; border-radius: inherit;"
                loading={props.loading.clone()}
                crossorigin={props.crossorigin.clone()}
                decoding={props.decoding}
                referrerpolicy={props.referrerpolicy}
                fetchpriority={props.fetchpriority.clone()}
                object_fit={props.object_fit}
                object_position={props.object_position}
                quality={props.quality}
                placeholder={props.placeholder}
                blur_data_url={props.blur_data_url}
                elementtiming={props.elementtiming}
                aria_live={props.aria_live}
                aria_pressed={props.aria_pressed}
                aria_controls={props.aria_controls}
                aria_labelledby={props.aria_labelledby}
                aria_describedby={props.aria_describedby}
                aria_expanded={props.aria_expanded}
                on_load={on_load}
                on_error={on_error}
            />
        </span>
    }
}

/// Props for [`Fallback`].
///
/// Displayed when the avatar image has not yet loaded or has failed. Supports
/// optional delay to avoid flash-of-fallback on fast connections.
#[derive(Properties, PartialEq, Clone)]
pub struct FallbackProps {
    /// Text initials, icon, or any content to show in the fallback.
    #[prop_or_default]
    pub children: Children,

    /// Delay in milliseconds before the fallback is shown after mount.
    #[prop_or_default]
    pub delay_ms: u32,

    /// Colour theme for the fallback. Inherits from [`GroupContext`] if unset.
    #[prop_or_default]
    pub color: Option<Color>,

    /// Style variant. Inherits from [`GroupContext`] if unset.
    #[prop_or_default]
    pub variant: Option<Variant>,

    /// CSS class on the fallback `<span>`.
    #[prop_or_else(|| classes!("avatar__fallback"))]
    pub class: Classes,

    /// Inline CSS on the fallback `<span>`.
    #[prop_or_default]
    pub style: &'static str,

    /// `id` attribute on the fallback `<span>`.
    #[prop_or_default]
    pub id: &'static str,

    /// `data-testid` for automated testing.
    #[prop_or_default]
    pub data_testid: &'static str,
}

/// Shows textual initials or an icon when the [`Image`] has not loaded.
///
/// Must be inside an [`Avatar`], reads [`AvatarContext`] to determine whether
/// the image is loaded. Hidden (`aria-hidden="true"`) once the image is visible.
///
/// # Examples
///
/// ```rust
/// use avatar::yew::{Avatar, Fallback};
/// use avatar::{Color, Variant};
/// use yew::prelude::*;
///
/// #[function_component(MyFallback)]
/// pub fn my_fallback() -> Html {
///     html! {
///         <Avatar aria_label="Ferris Prophet">
///             <Fallback color={Some(Color::Accent)} variant={Some(Variant::Soft)}>
///                 {"FP"}
///             </Fallback>
///         </Avatar>
///     }
/// }
/// ```
#[function_component(Fallback)]
pub fn avatar_fallback(props: &FallbackProps) -> Html {
    let status = use_context::<AvatarContext>().expect("Fallback must be inside Avatar");
    let group = use_context::<GroupContext>();
    let visible = use_state(|| props.delay_ms == 0);

    {
        let visible = visible.clone();
        let delay_ms = props.delay_ms;
        use_effect_with(delay_ms, move |_| {
            if delay_ms > 0 {
                let visible = visible.clone();
                spawn_local(async move {
                    sleep(Duration::from_millis(u64::from(delay_ms))).await;
                    visible.set(true);
                });
            }
            || ()
        });
    }

    let image_loaded = *status == ImageLoadingStatus::Loaded;
    let should_show = *visible && !image_loaded;

    if !should_show {
        return html! {};
    }

    let (group_color, group_variant) = group
        .as_ref()
        .map(|ctx| (ctx.color, ctx.variant))
        .unwrap_or((Color::Default, Variant::Default));

    let color = props.color.unwrap_or(group_color);
    let variant = props.variant.unwrap_or(group_variant);

    let color_style = match variant {
        Variant::Soft => color.to_soft_style(),
        Variant::Default => color.to_style(),
    };

    html! {
        <span
            id={props.id}
            class={props.class.clone()}
            style={format!("{} {} {}", base_fallback_style(), color_style, props.style)}
            aria-hidden={if image_loaded { "true" } else { "false" }}
            data-testid={props.data_testid}
        >
            { for props.children.iter() }
        </span>
    }
}

// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
