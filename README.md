<div align="center">

# 👦 Avatar

[![Crates.io](https://img.shields.io/crates/v/avatar)](https://crates.io/crates/avatar)
[![Crates.io Downloads](https://img.shields.io/crates/d/avatar)](https://crates.io/crates/avatar)
![Crates.io License](https://img.shields.io/crates/l/avatar)
[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Rust](https://img.shields.io/badge/Rust-1.89%2B-blue.svg)](https://www.rust-lang.org)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/opensass)

[![Open SASS Discord](https://dcbadge.limes.pink/api/server/b5JbvHW5nv)](https://discord.gg/b5JbvHW5nv)

<!-- absolute url for docs.rs cause assets is excluded from crate -->

![logo](https://raw.githubusercontent.com/opensass/avatar/refs/heads/main/assets/logo.png)

</div>

## 🎬 Demo

| Framework | Live Demo                                                                                                                                     |
| --------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| Yew       | [![Netlify Status](https://api.netlify.com/api/v1/badges/b213132a-d8b6-494b-8a5f-7290682a1a95/deploy-status)](https://avatar-rs.netlify.app)  |
| Dioxus    | [![Netlify Status](https://api.netlify.com/api/v1/badges/b213132a-d8b6-494b-8a5f-7290682a1a95/deploy-status)](https://avatar-dio.netlify.app) |
| Leptos    | [![Netlify Status](https://api.netlify.com/api/v1/badges/b213132a-d8b6-494b-8a5f-7290682a1a95/deploy-status)](https://avatar-lep.netlify.app) |

## 📜 Intro

A highly customizable, accessible avatar component for WASM frameworks: Yew, Dioxus, and Leptos.
Supports image loading with automatic fallback content, multiple sizes, color themes, and full WCAG compliance.

## 🤔 Why Avatar?

1. **🎨 Fully Customizable**: Control every attribute, class, style, size, color, variant, ARIA labels and more.
1. **🖼️ Smart Image Loading**: Tracks `idle → loading → loaded/error` status with context. Fallback is shown until the image is ready.
1. **⏱ Delay Support**: Prevent flash-of-fallback on fast connections with `delay_ms`.
1. **♿ Accessible by Default**: `role="img"`, `aria-label`, `aria-hidden` wired up automatically.
1. **🧩 Framework Agnostic**: Same API semantics across Yew, Dioxus, and Leptos.

## Y Yew Usage

<!-- absolute url for docs.rs cause YEW.md is not included in crate -->

Refer to [our guide](https://github.com/opensass/avatar/blob/main/YEW.md) to integrate this component into your Yew app.

## 🧬 Dioxus Usage

<!-- absolute url for docs.rs cause DIOXUS.md is not included in crate -->

Refer to [our guide](https://github.com/opensass/avatar/blob/main/DIOXUS.md) to integrate this component into your Dioxus app.

## 🌱 Leptos Usage

<!-- absolute url for docs.rs cause LEPTOS.md is not included in crate -->

Refer to [our guide](https://github.com/opensass/avatar/blob/main/LEPTOS.md) to integrate this component into your Leptos app.

## 🤝 Contributions

Contributions are welcome! Whether it's bug fixes, feature requests, or examples, we would love your help to make Avatar better.

1. Fork the repository.
1. Create a new branch for your feature/bugfix.
1. Submit a pull request for review.

## 📜 License

Avatar is licensed under the [MIT License](https://github.com/opensass/avatar/blob/main/LICENSE). You are free to use, modify, and distribute this library in your projects.
