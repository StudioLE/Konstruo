## About

With Konstruo you can design, plan, and build villages, towns, and cities.

Konstruo is many things:

### For Gamers

Konstruo is a standalone open-ended city-building game giving you the freedom to build villages, towns, and cities with intuitive [urban planning](https://en.wikipedia.org/wiki/Urban_planning) tools developed by industry experts.

### For AEC Design Professionals

Konstruo is an intuitive [urban planning](https://en.wikipedia.org/wiki/Urban_planning) tool developed with the expertise of someone who once did this themselves. Wether you're completing a simple capacity study or a detailed urban masterplan Konstruo has the tools to make it happen quickly, intuitively, and without tedium. 

### For Game Developers and Modders

Konstruo is implemented as a collection of plugins for the [Bevy](https://bevyengine.org/) game engine. You are encouraged to build on the foundations of Konstruo to enhance the experience for gamers. Implement new building methods, enhance the UI, or add new game and simulation logic. 

Konstruo itself is opinionated but with your expertise you can change that. If you want to add simulation or challenges then you're encouraged to do so.

### For AEC Software Developers

## Getting Started

Konstruo is available for Linux, Windows, and Mac.

### Linux

Binaries for Linux are compiled by the CI pipeline and included as GitHub release assets. Simply find the latest [GitHub Release](https://github.com/StudioLE/Konstruo/releases) and download the asset targetting `x86_64-unknown-linux-gnu`.

### Windows

Binaries for Windows aren't currently compiled but you can do so quickly by following the [build instructions](BUILD.md).

### Mac

As an Apple user you appreciate that good design comes at great cost. Therefore the first step is to send me an exorbitant amount of money.

Binaries for Mac aren't currently compiled but you can do so quickly by following the [build instructions](BUILD.md).

## Bevy Plugins and Libraries


> [!NOTE]
> Please be aware that Konstruo deviates from the Bevy conventions for [coordiante systems](https://bevy-cheatbook.github.io/fundamentals/coords.html).
>
> Bevy and Konstruo both implement right hand coordinate systems, however, Konstruo follows the AEC software convention of **Z up**. The reasons for this are simple. In the real world buildings, roads etc are always designed primarily in plan (looking from the sky down onto the ground), then they're extruded up (one floor placed upon another). 
>
> Therefore for the geometry of most buildings, roads etc it's drawn first as a 2D polygon on the ground before being extruded up into a 3D prism. It's therefore logical to draw first on the XY plane before extruded up on the Z axis.

Konstruo is implemented as a series of [Bevy](https://bevyengine.org/) plugins complimented by core libraries.

## Core Libraries

#### Bezier library [`src/core/beziers`](tree/main/src/core/beziers)

Cubic béziers are the primary curve used by vector graphics applications such as Inkscape or Adobe Illustrator. They're intuitive to work with and provide incredible freedom for drawing curves that exceed the historic reliance on lines and arcs that are the standard in AEC software.

Cubic béziers are geometrically and mathematically simple so from a software programming perspective they're ideal to work with, however, some simple operations - such as offsetting parallel curves - that we rely on geometric design aren't simple.

The bezier library provides `CubicBezier` and `CubicBezierSpline` struct for Bevy with essential geometric manipulation methods integrated from the [Kurbo](https://github.com/linebender/kurbo) library. Kurbo was developed by Raph Levien and Nicolas Silva and provides a Rust implemention of some of Raph Levien's primary research such as [Parallel curves of cubic Béziers](https://raphlinus.github.io/curves/2022/09/09/parallel-beziers.html).

#### Distribution library [`src/core/distribution`](tree/main/src/core/distribution)

Logic to distribute enitites in 3D based on [CSS flexbox layout](https://css-tricks.com/snippets/css/a-guide-to-flexbox/). 

The distribution library provides an intuitive fluent `FlexboxBuilder` struct. You provide the size and margin of the items you want to distribute, set the various alignment options, including optional cuboid bound or container constraints and the layout engine returns the center points for each item distributed according to your instructions, along with the size of the container they fit within. 

The distribution library is powered by the [taffy](https://github.com/DioxusLabs/taffy) layout library that already powers Bevy's 2D UI layout.

#### Geometry library [`src/core/geometry`](tree/main/src/core/geometry)

Includes struct for `Polyline` (`LineStrip`), `LineList`, `TriangleList` and `TriangleStrip` to simplify the generation of `Mesh` without the abstract complexity of Bevy's `PrimitiveTopology`.

#### Kinematics library [`src/core/kinematics`](tree/main/src/core/kinematics)

A basic `Translation` struct to move an entity towards a target location on every frame update.

#### Mathematics library [`src/core/mathematics`](tree/main/src/core/mathematics)

Basic essentials for `f32` along with methods for working with [spherical coordiantes](https://mathworld.wolfram.com/SphericalCoordinates.html).

### Bevy Plugins

#### Architecture plugins [`src/architecture`](tree/main/src/architecture)

Generate modular buildings.

#### Environment plugins [`src/environment`](tree/main/src/environment)

Graphical components for the sun, sky, and ground.

#### Infrastructure plugins [`src/infrastructure`](tree/main/src/infrastructure)

`Way` components for Bevy's entity component system that are drawn as a cubic bézier.

Way are the foundational component of Konstruo.In its most basic use a way defines the centerline of a road so the way is used to generate the 3D representation of the road surface, the footway (pavement or sidewalk) alongside, and then is defines the zones where buildings are distributed alongside the road.

#### UI plugins [`src/ui`](tree/main/src/ui)

A pan and orbit based camera using spherical coordiantes.

Axis markers, grid, view cube and other UI components essential to working in 3D.

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
