# Bevy Wasm Api

Opinionated plugin and proc macro to easily builded typed APIs for Js -> Wasm -> Js communication in the browser.

![https://private-user-images.githubusercontent.com/7402063/305640938-ec1b5504-9077-4d8e-a448-127376db901c.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3MDgxNzEwMTMsIm5iZiI6MTcwODE3MDcxMywicGF0aCI6Ii83NDAyMDYzLzMwNTY0MDkzOC1lYzFiNTUwNC05MDc3LTRkOGUtYTQ0OC0xMjczNzZkYjkwMWMucG5nP1gtQW16LUFsZ29yaXRobT1BV1M0LUhNQUMtU0hBMjU2JlgtQW16LUNyZWRlbnRpYWw9QUtJQVZDT0RZTFNBNTNQUUs0WkElMkYyMDI0MDIxNyUyRnVzLWVhc3QtMSUyRnMzJTJGYXdzNF9yZXF1ZXN0JlgtQW16LURhdGU9MjAyNDAyMTdUMTE1MTUzWiZYLUFtei1FeHBpcmVzPTMwMCZYLUFtei1TaWduYXR1cmU9YmMxZGMyNDMzYzdmMmNiMjU1ZTk0YjA0YjBlODQyOTliMjk3MTYyODAwNGFlMTkyNDM5ZTI2ZGFiOTA0N2UzNCZYLUFtei1TaWduZWRIZWFkZXJzPWhvc3QmYWN0b3JfaWQ9MCZrZXlfaWQ9MCZyZXBvX2lkPTAifQ.RDmzYYnx9oIJmpVSQGZcv5ezTjWSuIBAeiEoYIdvhCY](Image of typed wasm api returning an optional tuple asyncronously.)

## How to use

### 1. Implement your rust api

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

### 2. Integrate with typescript

```typescript
import { MyApi } from 'my-bevy-wasm-app';

const myApi = new MyApi();
// Call the same method, omitting the world argument.
// This will be run at the start of the next frame.
myApi.spawn_entity(1, 1, 1);

const entityCount = await myApi.count_entities();
console.log(entityCount) // number

```

## How it works

The crate uses a similar approach to the [deferred promise](https://dev.to/webduvet/deferred-promise-pattern-2j59)
by parking the function that we want to execute (See `Task` in [`sync.rs`](./src/sync.rs)),
executing all the parked tasks, and then converts the result back to a JsValue.

The real complexity is in the effort to support typed returns in typescript which is handled in the [bevy-wasm-api-macro-core`](./bevy-wasm-api-macro-core/src/analyze/) crate.

Given the following input

```rust
#[bevy_wasm_api]
impl MyApi {
    pub fn my_function(world: &mut World, x:f32, y: f32) -> bool {
        // Do anything with your &mut World
        true
    }
}
```

The output will look something like this.

```rust
// Exposes `MyApiWasmApi` as `MyApi` in javascript
#[wasm_bindgen(js_class = "MyApi")]
impl MyApiWasmApi {
    // Skips wasm_bindgen typescript types so we can generate better typescript types.
    #[wasm_bindgen(skip_typescript)]
    pub fn my_function(x: f32, y: f32) -> js_sys::Promise {
        // Uses execute_in_world to get a `world: &mut World`, converts the future to a Js Promise
        wasm_bindgen_futures::future_to_promise(bevy_wasm_api::execute_in_world(bevy_wasm_api::ExecutionChannel::FrameStart, |world| {
            // Calls the original method 
            let ret_val = MyApi::my_function(world, x, y);
            // Return the original return type as a JsValue
            // The real code that's generated here is actually dependent on the return type but I'll keep it simple in this example.
            Ok(JsValue::from(ret_val))
        }))
    }
}
```


## Examples

### `vite-app` 

This is your "kitchen sink" example showcasing a lot of the features of the crate.
This is how I am personally using the package to develop my app (a CAD/design program).

### `wasm-app`

This shows how to use the crate purely from the bevy side.  
Showcasing the changes you'd make / dependencies you'd need in bevy.

## Features

Here's an outline of the currently supported feature set + features that I'd like to implement.

- [ ] Type inference / handling of return types
    - [x] Infers any number (`i32`, ...) as typescript `number` type
    - [x] Infers `bool` as typescript `bool` type
    - [x] Correctly handles custom struct returns (must implement From/IntoWasmAbi) (use [tsify](https://github.com/madonoharu/tsify) to generate typescript types).
    - [x] Infers `&str`/`String` as typescript `string`
    - [x] Infers `Result<T, E>` as typescript `Promise<T>`
        - [ ] Use a Result polyfill so the final return type is `Result<JsResult<T, E>>`
    - [x] Infers `Vec<T>` as typescript typescript `Array<T>` type
        - [ ] Infers an `Iter<T>` as typescript `Array<T>`?
    - [x] Infers `Option<T>` as typescript `T | undefined` type
    - [x] Infers tuples (i.e. `(f32, String)`) as typescript `[number, String]` type
    - [ ] Infers `i32[]`, and other number arrays as typescript `Int32Array`
    - [ ] Handle `Future<T>` as typescript `Promise<T>`?
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

## Contributing

This crate is an ends to a means for developing an app so I am not sure what level of support I will be
able to provide and I might not be able to support a lot of additional features.  That being said, if you
run into bugs or have ideas for improvements/features feel free to create an issue or, even better, submit a PR. 

> :warning: If the PR is fairly large and complex it could be worth submitting an issue introducing the desired
> changes + the usecase so I can verify if it's something that belongs in this crate.

## Help me out?

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
