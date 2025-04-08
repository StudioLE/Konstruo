## Getting Started

Try the [WebAssembly demo](http://konstruo.studiole.uk/latest).

Download binaries for Windows and Linux from [GitHub releases](https://github.com/StudioLE/Konstruo/releases).

Or build from source by following the [build instructions](BUILD.md).

## About

With Konstruo you can design, plan, and build villages, towns, and cities.

### For Gamers

Konstruo is a standalone scenario based town building game giving you the freedom to build villages, towns, and cities with intuitive [urban planning](https://en.wikipedia.org/wiki/Urban_planning) tools.

You'll need to overcome complex constraints to build an optimal community for different sites.

### For AEC Design Professionals

Konstruo is an intuitive [urban planning](https://en.wikipedia.org/wiki/Urban_planning) tool developed with the expertise of someone who once did this themselves. Wether you're completing a simple capacity study or a detailed urban masterplan Konstruo has the tools to make it happen quickly, intuitively, and without the tedium of conventional methods.

### For Game Developers and Modders

Konstruo is implemented as a collection of plugins for the [Bevy](https://bevyengine.org/) game engine. You are encouraged to build on the foundations of Konstruo to enhance the experience for gamers. Implement new building methods, enhance the UI, or add new game and simulation logic.

Konstruo itself is opinionated but with your expertise you can change that. If you want to add simulation or challenges then you're encouraged to do so.

## Features

Right now Konstruo has the absolute basics of a working prototype:

- Pan and orbit behaviours for the camera are based on [spherical coordinates](https://mathworld.wolfram.com/SphericalCoordinates.html). They work with either the middle mouse button or the WASD keys (with shift for orbit).

https://github.com/user-attachments/assets/dd268532-60c1-4e45-bf82-f3c88dfd60c8

https://github.com/user-attachments/assets/20d0a58f-b0c2-44fe-972e-2af98f0b18c8

- An interactive View Cube gives an alternate and more precise method of orientation.

https://github.com/user-attachments/assets/8d9618a6-efd9-4227-a833-9d93336f8f45

- Roads can be selected to adjust the bézier spline.

https://github.com/user-attachments/assets/7ccd00a4-b999-422a-98db-61fa7c896efe

- Roads and paths can be drawn as bézier splines (click and drag to place the handle).

https://github.com/user-attachments/assets/9a177f4b-45d0-4533-bc2f-b34eca606dfa

- UI is loosely based on Material Design 3. The intention is to keep the UI as minimal as possible so buttons are only shown in specific contexts (ie after selecting a road).

- Houses can be distributed alongside a road using FlexBox. Currently the UI doesn't allow adding houses to a new road but I'll probably add that next.

- Houses are procedurally generated without any external models/assets. Each house is simply an array of cuboid modules. Window and door openings are distributed across the facade with FlexBox. See: https://github.com/StudioLE/Konstruo/blob/ab59a6a7aa5174585696c7d48b1f0b0f2f3ac89f/src/architecture/modular/templates.rs#L97-L160

## Bevy Plugins and Libraries

> [!NOTE]
> Please be aware that Konstruo deviates from the Bevy conventions for [coordiante systems](https://bevy-cheatbook.github.io/fundamentals/coords.html).
>
> Bevy and Konstruo both implement right hand coordinate systems, however, Konstruo follows the AEC software convention of **Z up**. The reasons for this are simple. In the real world buildings, roads etc are always designed primarily in plan (looking from the sky down onto the ground), then they're extruded up (one floor placed upon another).
>
> Therefore for the geometry of most buildings, roads etc it's drawn first as a 2D polygon on the ground before being extruded up into a 3D prism. It's therefore logical to draw first on the XY plane before extruded up on the Z axis.

Konstruo is implemented as a series of [Bevy](https://bevyengine.org/) plugins complimented by core libraries.

### [Architecture](tree/main/crates/architecture/src)

Procedurally generate modular buildings.

### [Béziers](tree/main/crates/beziers/src)

Cubic béziers are the primary curve used by vector graphics applications such as Inkscape or Adobe Illustrator. They're intuitive to work with and provide incredible freedom for drawing curves that exceed the historic reliance on lines and arcs that are the standard in AEC software.

Cubic béziers are geometrically and mathematically simple so from a software programming perspective they're ideal to work with, however, some simple operations - such as offsetting parallel curves - that we rely on geometric design aren't simple.

The bezier library provides `CubicBezier` and `CubicBezierSpline` struct for Bevy with essential geometric manipulation methods integrated from the [Kurbo](https://github.com/linebender/kurbo) library. Kurbo was developed by Raph Levien and Nicolas Silva and provides a Rust implemention of some of Raph Levien's primary research such as [Parallel curves of cubic Béziers](https://raphlinus.github.io/curves/2022/09/09/parallel-beziers.html).

### [Core](tree/main/crates/core/src)

#### [Extensions library](tree/main/crates/core/extensions)

Extension methods for `Entity`, `Result`, `Vec3`.

#### [Kinematics library](tree/main/crates/core/kinematics)

A basic `Translation` struct to move an entity towards a target location on every frame update.

#### [Mathematics library](tree/main/crates/core/mathematics)

Basic essentials for `f32` along with methods for working with [spherical coordiantes](https://mathworld.wolfram.com/SphericalCoordinates.html).

### [Distribution](tree/main/crates/distribution/src)

Logic to distribute enitites in 3D based on [CSS flexbox layout](https://css-tricks.com/snippets/css/a-guide-to-flexbox/).

The distribution library provides an intuitive fluent `FlexboxBuilder` struct. You provide the size and margin of the items you want to distribute, set the various alignment options, including optional cuboid bound or container constraints and the layout engine returns the center points for each item distributed according to your instructions, along with the size of the container they fit within.

The distribution library is powered by the [taffy](https://github.com/DioxusLabs/taffy) layout library that already powers Bevy's 2D UI layout.

### [Environment](tree/main/crates/environment)

Environmental components to handle lighting and shadows based on sun, sky, and ground.

### [Geometry](tree/main/crates/geometry/src)

#### [Primitive geometry](tree/main/crates/geometry/src/primitives)

Includes struct for `Polyline` (`LineStrip`), `LineList`, `TriangleList` and `TriangleStrip` to simplify the generation of `Mesh` without the abstract complexity of Bevy's `PrimitiveTopology`.

#### [Transform based 3D shape library](tree/main/crates/geometry/src/shapes_3d)

Includes struct for `Cuboid`, `TrianglularPrism`, and `Sweep` that are transformable and have methods to get specific vertices, edges, and faces.

#### [Geometry topolology library](tree/main/crates/geometry/src/topology)

Includes:
- `Orientation` enum for identifting individual faces of a cuboid (eg: `Front`), edges as a pair (eg: `[Front, Left]`), or vertices as a triplet: (`[Top, Front Left]`).
- `Subdivision` logic for creating a rectangular openings in a rectangle.
- `Vec6` a struct for defining 3D margins or offsets.
- `Edge`, `Solid`, `Wireframe` marker components.

### [Paths](tree/main/crates/geometry/src/paths)

`Path` components for Bevy's entity component system that are drawn as a cubic bézier.

`Path` are the foundational component of Konstruo. In its most basic use a way defines the centerline of a road so the way is used to generate the 3D representation of the road surface, the footway (pavement or sidewalk) alongside, and then is defines the zones where buildings are distributed alongside the road.

### [UI](tree/main/crates/ui/src)

Components that are visible to the user or visualize other entities.

A pan and orbit based camera using spherical coordiantes.

Axis markers, grid, view cube and other UI components essential to working in 3D.

### [UX](tree/main/crates/ux/src)

Logic controlling how users interact.

## License

Konstruo and its libraries are provided open source with the [AGPL-3.0](https://www.gnu.org/licenses/agpl-3.0.en.html) license that requires you must disclose your source code when you distribute, publish, or provide access to modified or derivative software.

Developers who wish to keep modified or derivative software proprietary or closed source can [get in touch for a commercial license agreements](https://studiole.uk/contact/)

> Copyright © Laurence Elsdon 2025
>
> This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
>
> This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.
>
> You should have received a copy of the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.

→ [GNU Affero General Public License](LICENSE.md)
