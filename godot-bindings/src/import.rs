/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! Versions to be updated whenever Godot releases a new patch version we support.
//!
//! This file contains several templating comments, who are substituted by the machinery itest/repo-tweak.
//! When modifying those, make sure to rerun.

/// All stable Godot releases _and_ upcoming next minor release.
pub const ALL_VERSIONS: &[(u8, u8, u8)] = &[
    // [version-sync] [[
    //  [repeat] past+current+future
    //  [fmt] \t$triple,
    (4, 0, 0),
    (4, 0, 1),
    (4, 0, 2),
    (4, 0, 3),
    (4, 0, 4),
    (4, 1, 0),
    (4, 1, 1),
    (4, 1, 2),
    (4, 1, 3),
    (4, 1, 4),
    (4, 2, 0),
    (4, 2, 1),
    (4, 2, 2),
    (4, 3, 0),
    // ]]
];

// [version-sync] [[
//  [repeat] past+current
//  [fmt] #[cfg(feature = "api-$kebabVersion")]\npub use prebuilt_$snakeVersion as godot4_prebuilt;\n
#[cfg(feature = "api-4-0")]
pub use prebuilt_4_0 as godot4_prebuilt;

#[cfg(feature = "api-4-0-1")]
pub use prebuilt_4_0_1 as godot4_prebuilt;

#[cfg(feature = "api-4-0-2")]
pub use prebuilt_4_0_2 as godot4_prebuilt;

#[cfg(feature = "api-4-0-3")]
pub use prebuilt_4_0_3 as godot4_prebuilt;

#[cfg(feature = "api-4-0-4")]
pub use prebuilt_4_0_4 as godot4_prebuilt;

#[cfg(feature = "api-4-1")]
pub use prebuilt_4_1 as godot4_prebuilt;

#[cfg(feature = "api-4-1-1")]
pub use prebuilt_4_1_1 as godot4_prebuilt;

#[cfg(feature = "api-4-1-2")]
pub use prebuilt_4_1_2 as godot4_prebuilt;

#[cfg(feature = "api-4-1-3")]
pub use prebuilt_4_1_3 as godot4_prebuilt;

#[cfg(feature = "api-4-1-4")]
pub use prebuilt_4_1_4 as godot4_prebuilt;

#[cfg(feature = "api-4-2")]
pub use prebuilt_4_2 as godot4_prebuilt;

#[cfg(feature = "api-4-2-1")]
pub use prebuilt_4_2_1 as godot4_prebuilt;

#[cfg(feature = "api-4-2-2")]
pub use prebuilt_4_2_2 as godot4_prebuilt;

// ]]

// If none of the api-* features are provided, use default prebuilt version (typically latest Godot stable release).

// [version-sync] [[
//  [repeat] past+current+future
//  [fmt] \tfeature = "api-$kebabVersion",
//  [pre] #[cfg(not(any(
//  [post] \tfeature = "api-custom",\n)))]
#[cfg(not(any(
    feature = "api-4-0",
    feature = "api-4-0-1",
    feature = "api-4-0-2",
    feature = "api-4-0-3",
    feature = "api-4-0-4",
    feature = "api-4-1",
    feature = "api-4-1-1",
    feature = "api-4-1-2",
    feature = "api-4-1-3",
    feature = "api-4-1-4",
    feature = "api-4-2",
    feature = "api-4-2-1",
    feature = "api-4-2-2",
    feature = "api-4-3",
    feature = "api-custom",
)))]
// ]]

// [version-sync] [[
//  [repeat] current.minor
//  [fmt] pub use prebuilt_$snakeVersion as godot4_prebuilt;
pub use prebuilt_4_2 as godot4_prebuilt;
// ]]
