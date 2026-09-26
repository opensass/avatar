// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Controls the rendered size of an [`Avatar`] or [`Group`] component.
///
/// Each named variant maps to a fixed pair of CSS dimension values on the
/// outermost container element. Choose [`Size::Custom`] to pass an arbitrary
/// inline-CSS width/height pair as a `&'static str`.
///
/// # Default
///
/// [`Size::Md`] is the default variant.
///
/// # Examples
///
/// ```rust
/// use avatar::Size;
///
/// let style = Size::Lg.to_style();
/// assert!(style.contains("48px"));
///
/// let custom = Size::Custom("width: 56px; height: 56px;");
/// assert!(custom.to_style().contains("56px"));
/// ```
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum Size {
    /// Extra-small: 24 x 24 px.
    Xs,

    /// Small: 32 x 32 px.
    Sm,

    /// Medium: 40 x 40 px. This is the default.
    #[default]
    Md,

    /// Large: 48 x 48 px.
    Lg,

    /// Extra-large: 64 x 64 px.
    Xl,

    /// Double extra-large: 80 x 80 px.
    Xxl,

    /// Arbitrary inline CSS width/height pair, e.g. `"width: 56px; height: 56px;"`.
    Custom(&'static str),
}

impl Size {
    /// Returns the inline CSS `width` and `height` string for this size variant.
    ///
    /// # Returns
    ///
    /// A `&'static str` containing `width: <N>px; height: <N>px;`.
    pub fn to_style(self) -> &'static str {
        match self {
            Self::Xs => "width: 24px; height: 24px;",
            Self::Sm => "width: 32px; height: 32px;",
            Self::Md => "width: 40px; height: 40px;",
            Self::Lg => "width: 48px; height: 48px;",
            Self::Xl => "width: 64px; height: 64px;",
            Self::Xxl => "width: 80px; height: 80px;",
            Self::Custom(s) => s,
        }
    }

    /// Returns the font-size inline CSS string appropriate for this size variant.
    ///
    /// # Returns
    ///
    /// A `&'static str` containing `font-size: <N>px;`.
    pub fn to_font_style(self) -> &'static str {
        match self {
            Self::Xs => "font-size: 10px;",
            Self::Sm => "font-size: 12px;",
            Self::Md => "font-size: 14px;",
            Self::Lg => "font-size: 16px;",
            Self::Xl => "font-size: 20px;",
            Self::Xxl => "font-size: 24px;",
            Self::Custom(_) => "font-size: 14px;",
        }
    }

    /// Returns the numeric pixel value used for group overlap calculations.
    ///
    /// # Returns
    ///
    /// A pixel value as `u32` representing the avatar's diameter.
    pub fn to_px(self) -> u32 {
        match self {
            Self::Xs => 24,
            Self::Sm => 32,
            Self::Md => 40,
            Self::Lg => 48,
            Self::Xl => 64,
            Self::Xxl => 80,
            Self::Custom(_) => 40,
        }
    }
}

/// Controls the color theme of the [`Avatar`] fallback element and the
/// [`Count`] overflow indicator.
///
/// Each named variant maps to a background-color + text-color pair. Use
/// [`Color::Custom`] to pass an arbitrary inline-CSS color declaration.
///
/// # Default
///
/// [`Color::Default`] is the default variant.
///
/// # Examples
///
/// ```rust
/// use avatar::Color;
///
/// let style = Color::Accent.to_style();
/// assert!(style.contains("#7c3aed"));
///
/// let custom = Color::Custom("background-color: #ff6b6b; color: #fff;");
/// assert!(custom.to_style().contains("#ff6b6b"));
/// ```
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum Color {
    /// Neutral gray: `#e5e7eb` background, `#374151` text.
    #[default]
    Default,

    /// Accent purple: `#7c3aed` background, `#ffffff` text.
    Accent,

    /// Success green: `#16a34a` background, `#ffffff` text.
    Success,

    /// Warning amber: `#d97706` background, `#ffffff` text.
    Warning,

    /// Danger red: `#dc2626` background, `#ffffff` text.
    Danger,

    /// Arbitrary inline CSS color declaration, e.g.
    /// `"background-color: #ff6b6b; color: #fff;"`.
    Custom(&'static str),
}

impl Color {
    /// Returns the solid-fill inline CSS for this color variant.
    ///
    /// # Returns
    ///
    /// A `&'static str` containing `background-color: <hex>; color: <hex>;`.
    pub fn to_style(self) -> &'static str {
        match self {
            Self::Default => "background-color: #4b5563; color: #ffffff;",
            Self::Accent => "background-color: #7c3aed; color: #ffffff;",
            Self::Success => "background-color: #16a34a; color: #ffffff;",
            Self::Warning => "background-color: #d97706; color: #ffffff;",
            Self::Danger => "background-color: #dc2626; color: #ffffff;",
            Self::Custom(s) => s,
        }
    }

    /// Returns the soft-tinted inline CSS for this color variant.
    ///
    /// The soft variant uses a lightly tinted background with the full accent
    /// text color, following the `avatar--soft` BEM modifier pattern.
    ///
    /// # Returns
    ///
    /// A `&'static str` containing `background-color: <light>; color: <dark>;`.
    pub fn to_soft_style(self) -> &'static str {
        match self {
            Self::Default => "background-color: #374151; color: #d1d5db;",
            Self::Accent => "background-color: #ede9fe; color: #7c3aed;",
            Self::Success => "background-color: #dcfce7; color: #16a34a;",
            Self::Warning => "background-color: #fef3c7; color: #d97706;",
            Self::Danger => "background-color: #fee2e2; color: #dc2626;",
            Self::Custom(s) => s,
        }
    }
}

/// Controls the visual style variant of the avatar fallback and count indicator.
///
/// # Default
///
/// [`Variant::Default`] is the default variant.
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum Variant {
    /// Uses the full-color background from [`Color::to_style`].
    #[default]
    Default,

    /// Uses the soft, lightly-tinted background from [`Color::to_soft_style`].
    Soft,
}

/// Controls the stacked-overlap mode for [`Group`] in non-grid layout.
///
/// This has no effect when `is_grid` is `true`.
///
/// # Default
///
/// [`Overlap::Clip`] is the default variant.
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum Overlap {
    /// Clips overlapped avatars with a crescent mask (transparent seam). This
    /// is the default stacked appearance.
    #[default]
    Clip,

    /// Uses a thin box-shadow outline via the container background to create
    /// a visible ring between stacked avatars.
    Ring,
}

impl Overlap {
    /// Returns the CSS class name string for this overlap mode.
    ///
    /// # Returns
    ///
    /// `"avatar-group--clip"` or `"avatar-group--ring"`.
    pub fn to_class(self) -> &'static str {
        match self {
            Self::Clip => "avatar-group--clip",
            Self::Ring => "avatar-group--ring",
        }
    }
}

/// Tracks the lifecycle of an `<img>` tag's loading state within [`Image`].
///
/// The state machine transitions: `Idle → Loading → Loaded | Error`.
///
/// # Default
///
/// [`ImageLoadingStatus::Idle`] is the default variant.
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum ImageLoadingStatus {
    /// The image fetch has not started yet.
    #[default]
    Idle,

    /// The image element has been mounted and the fetch is in progress.
    Loading,

    /// The image has fully loaded and is displayable.
    Loaded,

    /// The image fetch failed or the URL was invalid.
    Error,
}

impl ImageLoadingStatus {
    /// Returns `true` when the image has successfully loaded.
    pub fn is_loaded(self) -> bool {
        self == Self::Loaded
    }

    /// Returns `true` when the image is in a non-displayable state.
    ///
    /// The fallback content should be rendered whenever this returns `true`.
    pub fn should_show_fallback(self) -> bool {
        !self.is_loaded()
    }
}

/// Returns the base container CSS string used by `Avatar` across all frameworks.
///
/// # Returns
///
/// A `&'static str` containing the common flex container styles for an avatar.
pub fn base_container_style() -> &'static str {
    "display: inline-flex; align-items: center; justify-content: center; border-radius: 50%; overflow: hidden; vertical-align: middle; user-select: none; flex-shrink: 0; position: relative;"
}

/// Returns the base image CSS string used by `Image` across all frameworks.
///
/// # Returns
///
/// A `&'static str` containing the full-size object-cover image styles.
pub fn base_image_style() -> &'static str {
    "width: 100%; height: 100%; object-fit: cover; border-radius: inherit;"
}

/// Returns the base fallback CSS string used by `Fallback` across all frameworks.
///
/// # Returns
///
/// A `&'static str` containing the flex-centered fallback span styles.
pub fn base_fallback_style() -> &'static str {
    "display: flex; align-items: center; justify-content: center; width: 100%; height: 100%; font-weight: 600; line-height: 1; border-radius: inherit; letter-spacing: 0.025em;"
}

/// Returns the base stacked group container CSS (used when `is_grid` is `false`).
///
/// Children receive a negative `margin-inline-start` to create the overlap effect.
/// The exact overlap distance is controlled per-child by the framework component.
///
/// # Returns
///
/// A `&'static str` with `display: flex; align-items: center;`.
pub fn base_group_style() -> &'static str {
    "display: inline-flex; align-items: center; flex-direction: row; flex-shrink: 0;"
}

/// Returns the grid group container CSS (used when `is_grid` is `true`).
///
/// # Returns
///
/// A `&'static str` with `display: flex; flex-wrap: wrap; gap: 4px;`.
pub fn base_group_grid_style() -> &'static str {
    "display: inline-flex; align-items: center; flex-wrap: wrap; gap: 4px;"
}

/// Returns the overlap offset CSS for a stacked child avatar.
///
/// Each subsequent avatar in the stack is shifted left by the overlap amount.
/// The clip mode also adds a box-shadow-to-transparent ring for the crescent
/// seam; ring mode uses a solid outline.
///
/// # Arguments
///
/// * `overlap`: the overlap style.
/// * `size`: the avatar size (used to compute the ring width).
///
/// # Returns
///
/// An owned `String` with `margin-inline-start` and optional `box-shadow`.
pub fn child_overlap_style(overlap: Overlap, _size: Size) -> &'static str {
    match overlap {
        Overlap::Clip => "margin-inline-start: -0.5rem; filter: drop-shadow(-2px 0 0 transparent);",
        Overlap::Ring => "margin-inline-start: -0.5rem; box-shadow: 0 0 0 2px #1a1a1a;",
    }
}

// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
