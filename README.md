# Rust Ray Tracer

A physically-based, interactive ray tracer built entirely from scratch in Rust. It renders a 3D scene in real time — no GPU required — using Monte Carlo path tracing, parallel CPU rendering via Rayon, and a live window powered by Winit and the Pixels crate.

The renderer accumulates samples over time: the longer you let it sit still, the cleaner and more photorealistic the image becomes. When you orbit the camera, it switches to a fast low-resolution preview mode that keeps the motion smooth, then automatically returns to full HD quality the moment you stop.

## Download

| Platform | Link |
|---|---|
| **Windows (x86-64)** | [⬇ raytracer.exe — v1.0.0](https://github.com/Juliodvp29/rust-raytracer/releases/download/v1.0.0/raytracer.exe) |

> No installation needed. Download the `.exe`, double-click, and a window will open with the live render.

---

## Features

- **Monte Carlo path tracing** — physically correct lighting through recursive ray scattering
- **Three material types** — diffuse (Lambertian), reflective (Metal), and refractive (Dielectric/glass)
- **Depth of field** — thin-lens camera model with configurable aperture and focus distance
- **Bounding Volume Hierarchy (BVH)** — spatial acceleration structure for O(log n) ray traversal
- **Progressive accumulation** — samples stack on top of each other each frame; image quality improves indefinitely while the camera is still
- **Dynamic resolution scaling** — automatically drops to half-resolution preview during camera movement to reduce noise, then returns to full 1600×900 HD when idle
- **Fully parallel rendering** — every pixel is computed in parallel across all CPU cores using Rayon
- **Gamma correction** — linear-to-sRGB conversion (gamma 2.0) applied during output
- **Interactive orbit camera** — click and drag to rotate around the scene; Escape releases the cursor
- **Custom xorshift RNG** — fast, lock-free random number generation per thread, no external RNG dependency

---

## Demo Scene

The default scene contains 16 spheres arranged to showcase every material and lighting phenomenon:

| Object | Position | Material | Notes |
|---|---|---|---|
| Ground | (0, −1000, 0) | Lambertian gray | Giant sphere acting as a flat floor |
| Center | (0, 1.8, 0) | Dielectric (IOR 1.5) | Large glass sphere, borosilicate glass |
| Left accent | (−3.2, 0.7, 0.5) | Metal gold (fuzz 0.0) | Perfect mirror, gold color |
| Right accent | (3.2, 0.7, 0.5) | Metal silver (fuzz 0.02) | Near-perfect silver mirror |
| Front-left | (−1.5, 0.45, 2.2) | Lambertian red | Matte red diffuse |
| Front-right | (1.5, 0.45, 2.2) | Lambertian blue | Matte blue diffuse |
| Front-center | (0, 0.3, 2.8) | Dielectric (IOR 1.7) | Dense glass, higher refraction |
| Back-left | (−2.2, 0.4, −1.8) | Metal copper (fuzz 0.1) | Warm metallic with slight roughness |
| Back-right | (2.2, 0.4, −1.8) | Metal bronze (fuzz 0.05) | Nearly specular gold-bronze |
| Back-center | (0, 0.3, −2.5) | Lambertian green | Matte green |
| 6 small spheres | scattered | Mixed (see below) | Yellow diffuse, light-blue metal, glass, purple diffuse, green metal, orange diffuse |

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2021, stable toolchain)
- No GPU is required — the renderer runs entirely on the CPU

### Running

```bash
# Debug build (slower rendering, faster compile)
cargo run

# Release build — strongly recommended for real use
cargo run --release
```

The release build enables full compiler optimizations (`opt-level = 3`) and runs several times faster than the debug build.

### Controls

| Input | Action |
|---|---|
| **Left click** | Capture the mouse and enter orbit mode |
| **Mouse drag** | Orbit the camera around the scene center |
| **Escape** | Release the mouse cursor |

The window title bar shows the current render state:

- **`preview 2x`** — camera is moving, rendering at half resolution for smooth motion
- **`N spp HD`** — camera is still, showing N accumulated samples per pixel

---

## Architecture

The project is structured as a Rust library (`lib.rs`) with a binary entry point (`main.rs`). The library exposes all the rendering subsystems as public modules.

```
src/
├── main.rs              # Entry point: builds the scene, launches the app
├── lib.rs               # Library root; re-exports all modules
│
├── app/
│   ├── mod.rs           # Winit event loop, render orchestration, dynamic resolution
│   ├── accumulator.rs   # Frame accumulator: averages samples and converts to RGBA
│   └── controller.rs    # Orbital camera controller (spherical coordinates → Camera)
│
├── render/
│   ├── camera.rs        # Thin-lens perspective camera with depth-of-field
│   ├── renderer.rs      # Parallel raycast: render_sample / render_sample_scaled
│   └── mod.rs
│
├── geometry/
│   ├── sphere.rs        # Sphere primitive with analytic ray intersection
│   ├── aabb.rs          # Axis-Aligned Bounding Box as a renderable box primitive
│   ├── hittable.rs      # HitRecord and Hittable trait
│   ├── bvh.rs           # Bounding Volume Hierarchy tree + Bounded trait
│   └── mod.rs
│
├── materials/
│   ├── lambertian.rs    # Diffuse material (true Lambertian scattering)
│   ├── metal.rs         # Reflective material with configurable fuzz
│   ├── dielectric.rs    # Refractive material (glass) with Schlick approximation
│   ├── material.rs      # Material trait
│   └── mod.rs
│
├── scene/
│   ├── world.rs         # Scene container; builds the BVH from object list
│   └── mod.rs
│
├── core/
│   ├── ray.rs           # Ray type (origin + direction)
│   └── mod.rs
│
├── math/
│   ├── vec3.rs          # Vec3 with operator overloading; aliases Color and Point3
│   └── mod.rs
│
└── utils/
    ├── image.rs         # Image save utilities
    └── mod.rs
```

---

## How It Works

### Path Tracing

Each frame, the renderer fires one ray per pixel. The ray travels into the scene and, upon hitting a surface, scatters according to the material's BRDF (Bidirectional Reflectance Distribution Function). This process repeats recursively up to `MAX_DEPTH = 50` bounces. Background (sky) rays that don't hit anything return a gradient from white to light blue.

Because the scattering directions are randomly sampled, each frame produces a slightly different (noisy) image. The `Accumulator` averages all frames together: after N frames the image is the average of N random samples per pixel, which converges to the ground-truth integral of the rendering equation as N → ∞.

### Progressive Quality

```
Frame 1:   1 spp  →  noisy, grainy
Frame 10:  10 spp →  recognizable, some noise
Frame 100: 100 spp → clean, soft shadows visible
Frame 500: 500 spp → near-photorealistic
```

### Dynamic Resolution Scaling

While the camera is in motion, rendering at 1600×900 with 1 sample per pixel produces a very noisy image — every pixel shows a single random ray result. To fix this, the renderer drops to an internal resolution of 800×450 (scale factor = 2) and upscales the result via nearest-neighbor interpolation. At half resolution, each "pixel block" is twice as large, so the image appears blocky but not random-noisy. The moment the mouse stops, the system returns to full 1600×900 and begins accumulating HD samples.

The scale factor is controlled by `PREVIEW_SCALE` in `app/mod.rs`.

### Bounding Volume Hierarchy

Before rendering begins, all scene objects are inserted into a BVH tree. During traversal, if a ray misses a node's bounding box, the entire subtree underneath it is skipped. This reduces ray-object intersection tests from O(n) to O(log n), which is critical for scenes with many objects.

Construction splits objects by the longest spatial axis at each level, sorting them by bounding box centroid and dividing in half, recursively.

### Camera Model

The camera uses a thin-lens model. Instead of firing every ray from a single point (pinhole), rays originate from random points on a virtual disk of radius `aperture / 2`. All rays for a given pixel converge on the same point in the focal plane (at distance `focus_dist`), which creates a natural depth-of-field blur for objects outside the focus plane.

The `CameraController` maintains the camera as a point on a sphere of radius `radius` centered on the target, described by two angles:
- `theta` — horizontal orbit (azimuth)
- `phi` — vertical orbit (elevation), clamped so the camera can't flip over

### Materials

**Lambertian (diffuse):** Scatters the incoming ray in a random direction near the surface normal, weighted by a cosine distribution. This physically models surfaces that scatter light equally in all directions (matte paint, chalk, etc.).

**Metal:** Reflects the incoming ray around the surface normal using the standard reflection formula `r = d - 2(d·n)n`. A `fuzz` parameter adds a small random perturbation to the reflected direction, simulating microscopic surface roughness. `fuzz = 0` is a perfect mirror.

**Dielectric (glass):** Combines refraction using Snell's law and reflection using the Schlick approximation for Fresnel reflectance. At steep (grazing) angles, glass surfaces become increasingly reflective — this is physically accurate. Total internal reflection is also handled: when the refraction ratio makes refraction geometrically impossible, the ray reflects instead.

### Math & RNG

`Vec3` is the single vector type used for positions (`Point3`), directions, and colors (`Color`). It is `Copy` and implements all standard arithmetic operators. The custom random number generator uses the xorshift64 algorithm seeded from the system clock, stored in a thread-local cell — this means each Rayon worker thread has its own independent, lock-free RNG.

---

## Configuration

Key constants that can be tuned without breaking anything:

| Location | Constant | Default | Effect |
|---|---|---|---|
| `app/mod.rs` | `WIDTH` | `1600` | Render and window width in pixels |
| `app/mod.rs` | `HEIGHT` | `900` | Render and window height in pixels |
| `app/mod.rs` | `MAX_DEPTH` | `50` | Maximum ray bounce depth |
| `app/mod.rs` | `PREVIEW_SCALE` | `2` | Downscale during movement (2 = half-res, 4 = quarter-res) |
| `app/mod.rs` | `FRAMES_AFTER_MOVE` | `0` | Extra preview frames after mouse stops |
| `app/controller.rs` | `radius` | `6.0` | Initial camera distance from scene center |
| `app/controller.rs` | `vfov` | `45.0` | Vertical field of view in degrees |
| `app/controller.rs` | `aperture` | `0.05` | Lens aperture (depth of field strength) |
| `app/controller.rs` | `sensitivity` | `0.003` | Mouse orbit sensitivity |

---

## Dependencies

| Crate | Version | Purpose |
|---|---|---|
| [`rayon`](https://github.com/rayon-rs/rayon) | 1.8 | Data-parallel iteration across CPU cores |
| [`winit`](https://github.com/rust-windowing/winit) | 0.28 | Cross-platform window creation and event loop |
| [`pixels`](https://github.com/parasyte/pixels) | 0.12 | GPU-accelerated pixel buffer for the window surface |
| [`image`](https://github.com/image-rs/image) | 0.25 | Image encoding utilities (PNG, etc.) |

---

## License

This project is open source and available under the [MIT License](LICENSE).

---

*Built from scratch by [Julio Otero](https://github.com/Juliodvp29)