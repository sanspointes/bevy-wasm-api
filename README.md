# Bevy Wasm Api

Plugin + proc macro to easily build a typescript api for your bevy app when running in the browser.

# How to use

## 1. Implement your rust api

```rust
// Add BevyWasmApiPlugin to your app
use bevy_wasm_api::BevyWasmApiPlugin;
app.add_plugins(BevyWasmApiPlugin);

// Define an api
#[wasm_bindgen(skip_typescript)]
struct MyApi;

#[bevy_wasm_api]
impl MyApi {
    //                  The `&mut World` must be the first argument.
    pub fn spawn_entity(world: &mut World, x: f32, y: f32, z: f32) {
        world.spawn(TransformBundle {
            translation: Vec3::new(x, y, z),
            ..Default::default(),
        });
    }

    pub fn 
}
```

## 2. Integrate with typescript

```typescript
import { MyApi } from 'my-bevy-wasm-app';

const myApi = new MyApi();
myApi.spawn_entity_js(1, 1, 1);

```

# Features

- [ ] Type inference / handling
    - [x] Infers any number (`i32`, ...) as `number` type
    - [x] Infers `bool` as `bool` type
    - [x] Correctly handles custom struct returns (must implement From/IntoWasmAbi) (use [tsify](https://github.com/madonoharu/tsify) to generate typescript types).
    - [x] Infers `&str`/`String` as `string`
    - [x] Infers `Result<T, E>` as `Promise<T>`
    - [ ] Infers `i32[]`, and other number arrays as `Int32Array`
    - [ ] Infers `Vec<T>` as `T[]` type
    - [ ] Infers tuples (i.e. `(Type1, Type2)`) as `[Type1, Type2]` type
    - [ ] Infers `Option<T>` as `T | undefined` type
- [ ] Targets:
    - [x] Generates JS -> Wasm app
    - [ ] Generates JS -> Native app via RPC over HTTP
