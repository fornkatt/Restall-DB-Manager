// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

/// A graphics API a game can be set to run on, before any translation layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GraphicsApi {
    D3d9,
    D3d10,
    D3d11,
    D3d12,
    OpenGl,
    Vulkan,
}
