# Y Avatar Yew Usage

Adding Avatar to your project is simple:

1. Make sure your project is set up with **Yew**. Follow their [Getting Started Guide](https://yew.rs/docs/getting-started/introduction) for setup instructions.

1. Add the Avatar component to your dependencies by including it in your `Cargo.toml` file:

   ```sh
   cargo add avatar --features=yew
   ```

1. Import the `Avatar`, `Image`, and `Fallback` components into your Yew component and start using them in your app.

## 🛠️ Usage

### Basic Avatar with Image

```rust
use avatar::yew::{Avatar, Image, Fallback};
use yew::prelude::*;

#[function_component(MyAvatar)]
pub fn my_avatar() -> Html {
    html! {
        <Avatar aria_label="Ferris Prophet">
            <Image src="https://i.pravatar.cc/300" alt="Ferris Prophet" />
            <Fallback>{"FP"}</Fallback>
        </Avatar>
    }
}
```

### Initials Only (No Image)

```rust
use avatar::yew::{Avatar, Fallback};
use avatar::Color;
use yew::prelude::*;

#[function_component(InitialsAvatar)]
pub fn initials_avatar() -> Html {
    html! {
        <Avatar aria_label="Ferris Prophet">
            <Fallback color={Some(Color::Accent)}>{"FP"}</Fallback>
        </Avatar>
    }
}
```

### Different Sizes

```rust
use avatar::yew::{Avatar, Fallback};
use avatar::{Color, Size};
use yew::prelude::*;

#[function_component(SizedAvatars)]
pub fn sized_avatars() -> Html {
    html! {
        <>
            <Avatar size={Size::Sm}><Fallback color={Some(Color::Success)}>{"SM"}</Fallback></Avatar>
            <Avatar size={Size::Md}><Fallback color={Some(Color::Warning)}>{"MD"}</Fallback></Avatar>
            <Avatar size={Size::Lg}><Fallback color={Some(Color::Danger)}>{"LG"}</Fallback></Avatar>
        </>
    }
}
```

### Fallback Delay

Prevents a flash-of-fallback on fast image loads.

```rust
use avatar::yew::{Avatar, Image, Fallback};
use yew::prelude::*;

#[function_component(DelayedFallback)]
pub fn delayed_fallback() -> Html {
    html! {
        <Avatar aria_label="User">
            <Image src="https://i.pravatar.cc/300" alt="User" />
            <Fallback delay_ms={600}>{"US"}</Fallback>
        </Avatar>
    }
}
```

## 🔧 Props

### `Avatar`

| Property          | Type           | Description                            | Default            |
| ----------------- | -------------- | -------------------------------------- | ------------------ |
| `size`            | `Size`         | Dimensions of the avatar.              | `Size::Md`         |
| `color`           | `Color`        | Color theme for the fallback.          | `Color::Default`   |
| `variant`         | `Variant`      | `Default` (solid) or `Soft` (tinted).  | `Variant::Default` |
| `class`           | `&'static str` | Extra CSS classes on the root element. | `""`               |
| `style`           | `&'static str` | Extra inline CSS on the root element.  | `""`               |
| `id`              | `&'static str` | `id` attribute on the root element.    | `""`               |
| `aria_label`      | `&'static str` | Accessible name for screen readers.    | `"Avatar"`         |
| `tabindex`        | `&'static str` | Tab focus order.                       | `"-1"`             |
| `container_class` | `&'static str` | CSS class on the root `<span>`.        | `"avatar"`         |
| `container_style` | `&'static str` | Extra inline CSS on the root `<span>`. | `""`               |

### `Image`

| Property         | Type              | Description                           | Default           |
| ---------------- | ----------------- | ------------------------------------- | ----------------- |
| `src`            | `&'static str`    | Image source URL.                     | `""`              |
| `srcset`         | `&'static str`    | Responsive `srcset` attribute.        | `""`              |
| `sizes`          | `&'static str`    | Responsive `sizes` attribute.         | `""`              |
| `alt`            | `&'static str`    | Alternative text (required for a11y). | `""`              |
| `class`          | `&'static str`    | CSS class on the `<img>`.             | `"avatar__image"` |
| `style`          | `&'static str`    | Extra inline CSS on the `<img>`.      | `""`              |
| `loading`        | `&'static str`    | `"lazy"` or `"eager"`.                | `"eager"`         |
| `crossorigin`    | `&'static str`    | CORS setting.                         | `""`              |
| `decoding`       | `&'static str`    | `"auto"`, `"async"`, or `"sync"`.     | `"auto"`          |
| `referrerpolicy` | `&'static str`    | Referrer policy.                      | `""`              |
| `id`             | `&'static str`    | `id` on the `<img>`.                  | `""`              |
| `width`          | `Option<u32>`     | Pixel width.                          | `None`            |
| `height`         | `Option<u32>`     | Pixel height.                         | `None`            |
| `on_load`        | `Callback<Event>` | Called when image loads.              | no-op             |
| `on_error`       | `Callback<Event>` | Called when image fails.              | no-op             |

### `Fallback`

| Property   | Type              | Description                         | Default                         |
| ---------- | ----------------- | ----------------------------------- | ------------------------------- |
| `delay_ms` | `u32`             | Ms before fallback appears.         | `0`                             |
| `color`    | `Option<Color>`   | Color override.                     | `None` (inherits from `Avatar`) |
| `variant`  | `Option<Variant>` | Variant override.                   | `None` (inherits from `Avatar`) |
| `class`    | `&'static str`    | CSS class on the fallback `<span>`. | `"avatar__fallback"`            |
| `style`    | `&'static str`    | Extra inline CSS on the fallback.   | `""`                            |
| `id`       | `&'static str`    | `id` on the fallback `<span>`.      | `""`                            |

## 💡 Notes

- `Image` and `Fallback` **must** be used inside an `Avatar` component, they rely on its context.
- The fallback is shown until the image loads. Set `delay_ms` to prevent a flash on fast connections.
- Use `Color` and `Variant` props to match your design system.
- Looking for layered or clipped groups? Check out the **[Group API](GROUP.md)**!
