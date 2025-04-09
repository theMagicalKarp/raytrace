CARGO := cargo
BUILD_DIR := target
RELEASE_DIR := $(BUILD_DIR)/release
DEBUG_DIR := $(BUILD_DIR)/debug

# Default target
all: test lint release

# Build the project in release mode
release:
	$(CARGO) build --release

# Build the project in debug mode
build:
	$(CARGO) build

# Run tests
test:
	$(CARGO) test

# Format the code
fmt:
	$(CARGO) fmt

# Check for linting issues
lint:
	$(CARGO) clippy

# Re-generate example renders
examples: release
	$(RELEASE_DIR)/raytrace --config examples/basic/render.toml --output examples/basic/render.png
	$(RELEASE_DIR)/raytrace --config examples/bouncing_spheres/render.toml --output examples/bouncing_spheres/render.png
	$(RELEASE_DIR)/raytrace --config examples/cornell_box/render.toml --output examples/cornell_box/render.png
	$(RELEASE_DIR)/raytrace --config examples/earth/render.toml --output examples/earth/render.png
	$(RELEASE_DIR)/raytrace --config examples/perlin/render.toml --output examples/perlin/render.png
	$(RELEASE_DIR)/raytrace --config examples/quads/render.toml --output examples/quads/render.png
	$(RELEASE_DIR)/raytrace --config examples/smoke/render.toml --output examples/smoke/render.png
	$(RELEASE_DIR)/raytrace --config examples/sweet_dreams/render.toml --output examples/sweet_dreams/render.png

	$(RELEASE_DIR)/raytrace --config examples/primitives/geometry/sphere/render.toml --output examples/primitives/geometry/sphere/render.png
	$(RELEASE_DIR)/raytrace --config examples/primitives/geometry/cube/render.toml --output examples/primitives/geometry/cube/render.png
	$(RELEASE_DIR)/raytrace --config examples/primitives/geometry/quad/render.toml --output examples/primitives/geometry/quad/render.png
	$(RELEASE_DIR)/raytrace --config examples/primitives/geometry/triangle/render.toml --output examples/primitives/geometry/triangle/render.png
	$(RELEASE_DIR)/raytrace --config examples/primitives/geometry/wavefront/render.toml --output examples/primitives/geometry/wavefront/render.png

	$(RELEASE_DIR)/raytrace --config examples/primitives/material/lambertian/render.toml --output examples/primitives/material/lambertian/render.png
	$(RELEASE_DIR)/raytrace --config examples/primitives/material/metal/render.toml --output examples/primitives/material/metal/render.png
	$(RELEASE_DIR)/raytrace --config examples/primitives/material/checkered/render.toml --output examples/primitives/material/checkered/render.png
	$(RELEASE_DIR)/raytrace --config examples/primitives/material/dielectric/render.toml --output examples/primitives/material/dielectric/render.png
	$(RELEASE_DIR)/raytrace --config examples/primitives/material/light/render.toml --output examples/primitives/material/light/render.png
	$(RELEASE_DIR)/raytrace --config examples/primitives/material/noise/render.toml --output examples/primitives/material/noise/render.png
	$(RELEASE_DIR)/raytrace --config examples/primitives/material/water/render.toml --output examples/primitives/material/water/render.png
	$(RELEASE_DIR)/raytrace --config examples/primitives/material/glass/render.toml --output examples/primitives/material/glass/render.png
	$(RELEASE_DIR)/raytrace --config examples/primitives/material/texture/render.toml --output examples/primitives/material/texture/render.png

.PHONY: all release build test fmt lint examples
