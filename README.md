# Bevy Wasm Api

Plugin + proc macro to easily build a typescript api for your bevy app when running in the browser.

### How to use

#### 1. Implement your rust api

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

    pub fn count_entities(world: &mut World) -> usize {
        world.query::<Entity>().iter(world).len()
    }
}
```

#### 2. Integrate with typescript

```typescript
import { MyApi } from 'my-bevy-wasm-app';

const myApi = new MyApi();
// Call the same method, omitting the world argument.
// This will be run at the start of the next frame.
myApi.spawn_entity(1, 1, 1);

const entityCount = await myApi.count_entities();
console.log(entityCount) // number

```

### Examples

#### `vite-app` 

This is your "kitchen sink" example showcasing a lot of the features of the crate.  It's deployed to [TODO](https://www.google.com/).
This is how I am personally using the package to develop my app (a CAD/design program).

#### `wasm-app`

This shows how to use the crate purely from the bevy side.  
Showcasing the changes you'd make / dependencies you'd need in bevy.


### Features

Here's an outline of the currently supported feature set + features that I'd like to implement.

- [ ] Type inference / handling of return types
    - [x] Infers any number (`i32`, ...) as `number` type
    - [x] Infers `bool` as `bool` type
    - [x] Correctly handles custom struct returns (must implement From/IntoWasmAbi) (use [tsify](https://github.com/madonoharu/tsify) to generate typescript types).
    - [x] Infers `&str`/`String` as `string`
    - [x] Infers `Result<T, E>` as `Promise<T>`
        - [ ] Use a Result polyfill so the final return type is `Result<JsResult<T, E>>`
    - [ ] Infers `i32[]`, and other number arrays as `Int32Array`
    - [ ] Infers `Vec<T>` as `T[]` type
    - [ ] Infers tuples (i.e. `(Type1, Type2)`) as `[Type1, Type2]` type
    - [ ] Infers `Option<T>` as `T | undefined` type
    - [ ] Handle `Future<T>` as `Promise<T>`
- [ ] Type inference / handling of argument types
    - [x] Input parameters handled entirely by `wasm_bindgen`. [tsify](https://github.com/madonoharu/tsify) is good for making this more ergonomic.
    - [ ] Implement custom handling supporting the same typed parameters as return types (above)
- [ ] Targets:
    - [x] Exposes an api in JS that communicates directly with the bevy wasm app. (For use in browser contexts)
    - [ ] Exposes an api in JS that communicates with a desktop app over a HTTP endpoint + RPC layer. (For use in desktop contexts with ui in [bevy_wry_webview](https://github.com/hytopiagg/bevy_wry_webview))
- [ ] Support systems as the Api handler.  Make use of [`In<T>`](https://docs.rs/bevy/latest/bevy/ecs/system/struct.In.html) and [`Out<T>`](https://docs.rs/bevy/latest/bevy/ecs/prelude/trait.System.html#associatedtype.Out) for args / return value.
- [ ] Support multiple bevy apps
- [ ] Less restrictive dependency versions
- [ ] Adding proc macro attributes to declare when in the frame lifecycle we want to execute the api method.

### Contributing

This crate is an ends to a means for developing an app so I am not sure what level of support I will be
able to provide and I might not be able to support a lot of additional features.  That being said, if you
run into bugs or have ideas for improvements/features feel free to create an issue or, even better, submit a PR. 

> :warning: If the PR is fairly large and complex it could be worth submitting an issue introducing the desired
> changes + the usecase so I can verify if it's something that belongs in this crate.

### Help me out?

This is also my first proc_macro and I am not that experience with the "bevy" way of doing things so
if you know have some technical ideas on how this crate can be improved (improve modularity/adaptability,
performance, simplify code) I would be very grateful to hear it in an issue.

Some things I'd love feedback on is:

- Making the dependency versions lest restrictive.
- Adding proc macro attributes on each function to declare when the ApiMethod should run.
- Making better use of bevy paradigms
- Making better use of wasm_bindgen type inference (currently duplicating logic converting `str` (rust) -> `string` (typescript))
- All of this is only tested with my depenencies, anything that makes it more versatile (I might be a bit too dumb to make it fully generic)
- Generalising the type inference improvements into its own crate (could be useful outside of the bevy ecosystem)
