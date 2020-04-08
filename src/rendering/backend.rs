#[cfg(feature = "dx11")]
pub extern crate gfx_backend_dx11 as backend;
#[cfg(feature = "dx12")]
pub extern crate gfx_backend_dx12 as backend;
#[cfg(any(feature = "gl", feature = "wgl"))]
pub extern crate gfx_backend_gl as backend;
#[cfg(feature = "metal")]
pub extern crate gfx_backend_metal as backend;
#[cfg(feature = "vulkan")]
pub extern crate gfx_backend_vulkan as backend;

#[cfg(not(any(
    feature = "vulkan",
    feature = "dx11",
    feature = "dx12",
    feature = "metal",
    feature = "gl",
    feature = "wgl"
)))]
pub extern crate gfx_backend_empty as backend;

#[cfg(feature = "dx11")]
pub const BACKEND_NAME: &'static str = "DirectX 11";
#[cfg(feature = "dx12")]
pub const BACKEND_NAME: &'static str = "DirectX 12";
#[cfg(any(feature = "gl", feature = "wgl"))]
pub const BACKEND_NAME: &'static str = "OpenGL";
#[cfg(feature = "metal")]
pub const BACKEND_NAME: &'static str = "Metal";
#[cfg(feature = "vulkan")]
pub const BACKEND_NAME: &'static str = "Vulkan";

#[cfg(not(any(
    feature = "vulkan",
    feature = "dx11",
    feature = "dx12",
    feature = "metal",
    feature = "gl",
    feature = "wgl"
)))]
pub const BACKEND_NAME: &'static str = "No Backend";
