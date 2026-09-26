# 🧬 Avatar Dioxus Usage

Adding Avatar to your project is simple:

1. Make sure your project is set up with **Dioxus**. Follow their [Getting Started Guide](https://dioxuslabs.com/learn/0.7/getting_started) for setup instructions.

1. Add the Avatar component to your dependencies:

   ```sh
   cargo add avatar --features=dio
   ```

1. Import and use the `Avatar`, `Image`, and `Fallback` components.

## 🛠️ Usage

### Basic Avatar with Image

```rust
use avatar::dioxus::{Avatar, Image, Fallback};
use dioxus::prelude::*;

fn MyAvatar() -> Element {
    rsx! {
        Avatar { aria_label: "Ferris Prophet",
            Image { src: "https://i.pravatar.cc/300", alt: "Ferris Prophet" }
            Fallback { "FP" }
        }
    }
}
```

### Initials Only

```rust
use avatar::dioxus::{Avatar, Fallback};
use avatar::Color;
use dioxus::prelude::*;

fn InitialsAvatar() -> Element {
    rsx! {
        Avatar { aria_label: "Ferris Prophet",
            Fallback { color: Color::Accent, "FP" }
        }
    }
}
```

### Sizes and Colors

```rust
use avatar::dioxus::{Avatar, Fallback};
use avatar::{Color, Size};
use dioxus::prelude::*;

fn SizedAvatars() -> Element {
    rsx! {
        Avatar { size: Size::Sm, Fallback { color: Color::Success, "SM" } }
        Avatar { size: Size::Md, Fallback { color: Color::Warning, "MD" } }
        Avatar { size: Size::Lg, Fallback { color: Color::Danger, "LG" } }
    }
}
```

### Fallback Delay

```rust
use avatar::dioxus::{Avatar, Image, Fallback};
use dioxus::prelude::*;

fn DelayedFallback() -> Element {
    rsx! {
        Avatar { aria_label: "User",
            Image { src: "https://i.pravatar.cc/300", alt: "User" }
            Fallback { delay_ms: 600, "US" }
        }
    }
}
```

## 🔧 Props

### `Avatar`

| Property          | Type           | Description                  | Default            |
| ----------------- | -------------- | ---------------------------- | ------------------ |
| `size`            | `Size`         | Dimensions of the avatar.    | `Size::Md`         |
| `color`           | `Color`        | Fallback color theme.        | `Color::Default`   |
| `variant`         | `Variant`      | `Default` or `Soft`.         | `Variant::Default` |
| `class`           | `&'static str` | Extra CSS classes.           | `""`               |
| `style`           | `&'static str` | Extra inline CSS.            | `""`               |
| `id`              | `&'static str` | `id` attribute.              | `""`               |
| `aria_label`      | `&'static str` | Accessible name.             | `"Avatar"`         |
| `tabindex`        | `&'static str` | Tab order.                   | `"-1"`             |
| `container_class` | `&'static str` | Class on root `<span>`.      | `"avatar"`         |
| `container_style` | `&'static str` | Inline CSS on root `<span>`. | `""`               |

### `Image`

| Property      | Type                        | Description            | Default           |
| ------------- | --------------------------- | ---------------------- | ----------------- |
| `src`         | `&'static str`              | Image URL.             | `""`              |
| `srcset`      | `&'static str`              | Responsive srcset.     | `""`              |
| `sizes`       | `&'static str`              | Responsive sizes.      | `""`              |
| `alt`         | `&'static str`              | Alt text (required).   | `""`              |
| `class`       | `&'static str`              | CSS class.             | `"avatar__image"` |
| `style`       | `&'static str`              | Extra inline CSS.      | `""`              |
| `loading`     | `&'static str`              | `"lazy"` or `"eager"`. | `"eager"`         |
| `crossorigin` | `&'static str`              | CORS setting.          | `""`              |
| `decoding`    | `&'static str`              | Decoding hint.         | `"auto"`          |
| `id`          | `&'static str`              | `id` on the `<img>`.   | `""`              |
| `width`       | `Option<u32>`               | Pixel width.           | `None`            |
| `height`      | `Option<u32>`               | Pixel height.          | `None`            |
| `on_load`     | `Option<EventHandler<...>>` | Load callback.         | `None`            |
| `on_error`    | `Option<EventHandler<...>>` | Error callback.        | `None`            |

### `Fallback`

| Property   | Type           | Description        | Default              |
| ---------- | -------------- | ------------------ | -------------------- |
| `delay_ms` | `u32`          | Ms before visible. | `0`                  |
| `color`    | `Color`        | Color override.    | `Color::Default`     |
| `variant`  | `Variant`      | Variant override.  | `Variant::Default`   |
| `class`    | `&'static str` | CSS class.         | `"avatar__fallback"` |
| `style`    | `&'static str` | Extra inline CSS.  | `""`                 |
| `id`       | `&'static str` | `id` attribute.    | `""`                 |

## 💡 Notes

- Child components must be nested inside `Avatar`, they rely on context.
- `delay_ms` prevents a flash-of-fallback on fast connections.
- Looking for layered or clipped groups? Check out the **[Group API](GROUP.md)**!
