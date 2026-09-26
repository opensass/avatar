# Group

`Group` provides a container to intuitively stack or grid multiple `Avatar` components together. It automatically passes configurations (like size, color, or shape variant) down to its children, ensuring visual consistency.

## Overview

Use `Group` when you exhibit a list of users, contributors, or participants. It pairs seamlessly with `Avatar` and optionally `Count` to clip surplus avatars and indicate the excess dynamically.

### Key Features

- **Context passing**: Configure `Avatar` styling centrally (e.g. `size`, `color`) via the group container instead of each child.
- **Auto-capping**: Limit max rendered avatars using the `max` prop, allowing trailing avatars to be collapsed (requires application-level integration).
- **Group Overflow Indicator**: `Count` allows seamless display of remaining avatars (e.g. `+3` or `+99`).
- **Flexible layouts**: Toggle between overlapping stacks (`Overlap: Clip | Ring`) or a non-overlapping flow grid (`is_grid`).

## API Reference

### Group Properties

| Property               | Type            | Default            | Description                                                                                 |
| ---------------------- | --------------- | ------------------ | ------------------------------------------------------------------------------------------- |
| `size`                 | `Size`          | `Size::Md`         | Default relative size passed down to child avatars.                                         |
| `color`                | `Color`         | `Color::Default`   | Default fallback color passed down to child avatars.                                        |
| `variant`              | `Variant`       | `Variant::Default` | Default generic style context applied to child avatars.                                     |
| `max`                  | `Option<usize>` | `None`             | Max cap representation (implementation specific per framework).                             |
| `total`                | `Option<usize>` | `None`             | Total number of elements. Required when using `max` to calculate overflow count accurately. |
| `overlap`              | `Overlap`       | `Overlap::Clip`    | How nested avatars overlap: `Clip` crops edges; `Ring` adds spacing.                        |
| `is_grid`              | `bool`          | `false`            | Sets spacing sequentially without overlap, rendering in a wrap grid.                        |
| `class`, `style`, `id` | `&'static str`  | `""`               | Advanced customizations on the container node.                                              |

### Count Properties

| Property               | Type              | Default | Description                                         |
| ---------------------- | ----------------- | ------- | --------------------------------------------------- |
| `count`                | `usize`           | `0`     | Displays as `+{count}` if no children are provided. |
| `size`                 | `Option<Size>`    | `None`  | Overrides parent group's size if set.               |
| `color`                | `Option<Color>`   | `None`  | Overrides parent group's color if set.              |
| `variant`              | `Option<Variant>` | `None`  | Overrides parent group's variant if set.            |
| `class`, `style`, `id` | `&'static str`    | `""`    | Advanced customizations on the container node.      |
