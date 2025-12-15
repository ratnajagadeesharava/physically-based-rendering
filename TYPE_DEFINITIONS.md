# Type Definitions

This document describes the centralized type definitions used throughout the project.

## Location

All common type aliases are defined in `src/util/types.rs` and re-exported through `src/util/mod.rs`.

## Available Types

### `Float`
- **Definition**: `pub type Float = f32;`
- **Usage**: The primary floating-point type used for all mathematical computations throughout the project
- **Import**: `use crate::util::Float;`

### `Int`
- **Definition**: `pub type Int = i32;`
- **Usage**: Integer type for indexing and counts
- **Import**: `use crate::util::Int;`

## Files Updated

The following files have been updated to use the centralized `Float` type definition:

1. `src/util/bounds/bounds3.rs` - Uses `Float` for bounding box calculations
2. `src/util/vector/vector3.rs` - Uses `Float` for vector components
3. `src/util/rays/ray.rs` - Uses `Float` for ray time parameter
4. `src/util/ray.rs` - Uses `Float` for ray time parameter
5. `src/util/math/spherical_geometry.rs` - Uses `Float` for spherical calculations

## Benefits

- **Consistency**: Single source of truth for type definitions
- **Easy to change**: Switching from `f32` to `f64` only requires changing one file
- **Clarity**: Makes it clear that certain types are project-wide conventions
- **Maintainability**: Reduces code duplication and potential inconsistencies

## How to Use

Simply import the type from `crate::util`:

```rust
use crate::util::Float;

fn my_function(x: Float) -> Float {
    x * 2.0
}
```

## Future Considerations

If you need to switch to double precision (f64), simply change the definition in `src/util/types.rs`:

```rust
pub type Float = f64;
```

All code using `Float` will automatically use the new precision.
