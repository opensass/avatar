# 🌱 Avatar Leptos Usage

Adding Avatar to your project is simple:

1. Make sure your project is set up with **Leptos**. Follow their [Getting Started Guide](https://leptos.dev/getting-started) for setup instructions.

1. Add the Avatar component to your dependencies:

   ```sh
   cargo add avatar --features=lep
   ```

1. Import and use the `Avatar`, `Image`, and `Fallback` components.

## 🛠️ Usage

### Basic Avatar with Image

```rust
use avatar::leptos::{Avatar, Image, Fallback};
use leptos::prelude::*;

#[component]
fn MyAvatar() -> impl IntoView {
    view! {
        <Avatar aria_label="Ferris Prophet">
            <Image src="https://i.pravatar.cc/300" alt="Ferris Prophet" />
            <Fallback>"FP"</Fallback>
        </Avatar>
    }
}
```

### Initials Only

```rust
use avatar::leptos::{Avatar, Fallback};
use avatar::Color;
use leptos::prelude::*;

#[component]
fn InitialsAvatar() -> impl IntoView {
    view! {
        <Avatar aria_label="Ferris Prophet">
            <Fallback color=Color::Accent>"FP"</Fallback>
        </Avatar>
    }
}
```

### Sizes and Colors

```rust
use avatar::leptos::{Avatar, Fallback};
use avatar::{Color, Size};
use leptos::prelude::*;

#[component]
fn SizedAvatars() -> impl IntoView {
    view! {
        <Avatar size=Size::Sm><Fallback color=Color::Success>"SM"</Fallback></Avatar>
        <Avatar size=Size::Md><Fallback color=Color::Warning>"MD"</Fallback></Avatar>
        <Avatar size=Size::Lg><Fallback color=Color::Danger>"LG"</Fallback></Avatar>
    }
}
```

### Fallback Delay

```rust
use avatar::leptos::{Avatar, Image, Fallback};
use leptos::prelude::*;

#[component]
fn DelayedFallback() -> impl IntoView {
    view! {
        <Avatar aria_label="User">
            <Image src="https://i.pravatar.cc/300" alt="User" />
            <Fallback delay_ms=600>"US"</Fallback>
        </Avatar>
    }
}
```

## 🔧 Props

### `Avatar`

| Property          | Type           | Description               | Default    |
| ----------------- | -------------- | ------------------------- | ---------- |
| `size`            | `Size`         | Dimensions of the avatar. | `Size::Md` |
| `class`           | `&'static str` | Extra CSS classes.        | `""`       |
| `style`           | `&'static str` | Extra inline CSS.         | `""`       |
| `id`              | `&'static str` | `id` attribute.           | `""`       |
| `aria_label`      | `&'static str` | Accessible name.          | `"Avatar"` |
| `tabindex`        | `&'static str` | Tab order.                | `"-1"`     |
| `container_class` | `&'static str` | Class on root `<span>`.   | `"avatar"` |
| `container_style` | `&'static str` | CSS on root `<span>`.     | `""`       |

### `Image`

| Property      | Type                               | Description            | Default           |
| ------------- | ---------------------------------- | ---------------------- | ----------------- |
| `src`         | `&'static str`                     | Image URL.             | `""`              |
| `srcset`      | `&'static str`                     | Responsive srcset.     | `""`              |
| `sizes`       | `&'static str`                     | Responsive sizes.      | `""`              |
| `alt`         | `&'static str`                     | Alt text (required).   | `""`              |
| `class`       | `&'static str`                     | CSS class.             | `"avatar__image"` |
| `style`       | `&'static str`                     | Extra inline CSS.      | `""`              |
| `loading`     | `&'static str`                     | `"lazy"` or `"eager"`. | `"eager"`         |
| `crossorigin` | `&'static str`                     | CORS setting.          | `""`              |
| `decoding`    | `&'static str`                     | Decoding hint.         | `"auto"`          |
| `id`          | `&'static str`                     | `id` on the `<img>`.   | `""`              |
| `width`       | `Option<u32>`                      | Pixel width.           | `None`            |
| `height`      | `Option<u32>`                      | Pixel height.          | `None`            |
| `on_load`     | `Option<Callback<web_sys::Event>>` | Load callback.         | `None`            |
| `on_error`    | `Option<Callback<web_sys::Event>>` | Error callback.        | `None`            |

### `Fallback`

| Property   | Type           | Description          | Default              |
| ---------- | -------------- | -------------------- | -------------------- |
| `delay_ms` | `u32`          | Ms before visible.   | `0`                  |
| `color`    | `Color`        | Color theme.         | `Color::Default`     |
| `variant`  | `Variant`      | `Default` or `Soft`. | `Variant::Default`   |
| `class`    | `&'static str` | CSS class.           | `"avatar__fallback"` |
| `style`    | `&'static str` | Extra inline CSS.    | `""`                 |
| `id`       | `&'static str` | `id` attribute.      | `""`                 |

## 💡 Notes

- Child components must be nested inside `Avatar`, they rely on a shared `RwSignal` context.
- Use `delay_ms` to suppress the fallback flash on fast connections.
- Looking for layered or clipped groups? Check out the **[Group API](GROUP.md)**!
