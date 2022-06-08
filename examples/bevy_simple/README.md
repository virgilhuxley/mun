Note that at the time of writing, AMDVLK is experiencing an issue in
Bevy.  
See associated [Bevy
Issue](https://github.com/bevyengine/bevy/issues/3606)

If you experience a panic on run with an error message containing
something similar to: `src/view/window.rs:161:24` then this is a known
bug.

The following commands can be used to adjust how bevy will render.

```sh
WGPU_BACKEND=gl       cargo run # expect success
AMD_VULKAN_ICD=RADV   cargo run # expect success
AMD_VULKAN_ICD=AMDVLK cargo run # expect crash
```

## How to use this example

Open two terminals in the `bevy_simple` project.

In terminal 1 run the following:

```sh
mun build --watch
```

In terminal 2 run the following:

```sh
cargo run
```

If that fails try one of the following:
```sh
WGPU_BACKEND=gl       cargo run
AMD_VULKAN_ICD=RADV   cargo run
```
