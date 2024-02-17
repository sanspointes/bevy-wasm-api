<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a name="readme-top"></a>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->
<p align="center">
  <img src="" title="">
</p>

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/sanspointes/bevy-wasm-api/">
    <img src="https://private-user-images.githubusercontent.com/7402063/305640938-ec1b5504-9077-4d8e-a448-127376db901c.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3MDgyMTE3MTksIm5iZiI6MTcwODIxMTQxOSwicGF0aCI6Ii83NDAyMDYzLzMwNTY0MDkzOC1lYzFiNTUwNC05MDc3LTRkOGUtYTQ0OC0xMjczNzZkYjkwMWMucG5nP1gtQW16LUFsZ29yaXRobT1BV1M0LUhNQUMtU0hBMjU2JlgtQW16LUNyZWRlbnRpYWw9QUtJQVZDT0RZTFNBNTNQUUs0WkElMkYyMDI0MDIxNyUyRnVzLWVhc3QtMSUyRnMzJTJGYXdzNF9yZXF1ZXN0JlgtQW16LURhdGU9MjAyNDAyMTdUMjMxMDE5WiZYLUFtei1FeHBpcmVzPTMwMCZYLUFtei1TaWduYXR1cmU9MzdmNGI0YThjNmY3OTA2MmFlNDUzMjA2NDJmZTljMjQ2ODY2YjlhNmUwMGQ1NmU1YjZjMGQ1NGUxM2MwMDNmZiZYLUFtei1TaWduZWRIZWFkZXJzPWhvc3QmYWN0b3JfaWQ9MCZrZXlfaWQ9MCZyZXBvX2lkPTAifQ.RovEVadRIq0G7lYUQ02OvWxsh3q9UBoyaUowl797WOg" alt="Image of typed wasm api returning an optional tuple asyncronously.">
  </a>

  <h3 align="center">bevy-wasm-api</h3>

  <p align="center">
    Opinionated plugin and proc macro to easily builded typed APIs for Js -> Wasm -> Js communication in the browser.
    <!-- <br /> -->
    <!-- <a href="https://github.com/othneildrew/Best-README-Template"><strong>Explore the docs »</strong></a> -->
    <!-- <br /> -->
    <br />
    <a href="https://sanspointes.github.io/bevy-wasm-api/">View Demo</a>
    ·
    <a href="https://github.com/sanspointes/bevy-wasm-api/issues">Report Bug</a>
    ·
    <a href="https://github.com/sanspointesbevy-wasm-api/issues">Request Feature</a>
  </p>
</div>

<!-- GETTING STARTED -->
## Getting Started

This is an example of how you may give instructions on setting up your project locally.
To get a local copy up and running follow these simple example steps.

### Installation 

Install the `bevy-wasm-api` crate.

```bash

cargo add --git https://github.com/sanspointes/bevy-wasm-api
cargo add wasm-bindgen                                  
```

Install optional dependencies to help your development

```bash
# Required if you want to return custom structs from your api
cargo add serde --features derive
# Helpful crate for generating typescript types for custom structs
cargo add tsify --features js --no-default-features
```

### Add Plugin to app

```rust
use bevy_wasm_api::BevyWasmApiPlugin;

#[wasm_bindgen]
pub fn setup_app(canvas_selector: String) {
    let mut app = App::new();
    app.add_plugins(BevyWasmApiPlugin).run();
}
```

### Define an Api

:warning: The first argument must be a `world: &mut World`.

```rust

#[wasm_bindgen(skip_typescript)] // Let bevy-wasm-api generate the types
struct MyApi;

#[bevy_wasm_api]
impl MyApi {
    pub fn spawn_entity(world: &mut World, x: f32, y: f32, z: f32) -> Entity {
        world.spawn(
            TransformBundle {
                transform: Transform {
                    translation: Vec3::new(x, y, z),
                    ..Default::default(),
                },
                ..Default::default(),
            }
        ).id()
    }

    pub fn set_entity_position(world: &mut World, entity: u32, x: f32, y: f32, z: f32) -> Result<(), String> {
        let entity = Entity::from_raw(entity);
        let mut transform = world.get_mut::<Transform>(entity).ok_or("Could not find entity".to_string())?;
        transform.translation.x = x;
        transform.translation.y = y;
        transform.translation.z = z;
        Ok(())
    }
    pub fn get_entity_position(world: &mut World, entity: u32) -> Option<(f32, f32, f32)> {
        let transform = world.get::<Transform>(Entity::from_raw(entity));
        transform.map(|transform| {
            let pos = transform.translation;
            (pos.x, pos.y, pos.z)
        })
    }
}
```

### Use your api in typescript

```typescript

import { setup_app, MyApi } from 'bevy-app';

async function start() {
    try {
        setup_app('#canvas-element');
    } catch (error) {
        // Ignore, Bevy apps for wasm error for control flow.
    }

    const api = new MyApi();

    const id = await api.spawn_entity(0, 0, 0);

    await api.set_entity_position(10, 0, 0);

    const pos = await api.get_entity_position(id)
    console.log(pos) // [10, 0, 0]

    const otherPos = await api.get_entity_position(1000) // (Made up entity)
    console.log(pos) // undefined
}

```


<p align="right">(<a href="#readme-top">back to top</a>)</p>

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

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Examples

### `vite-app` 

This is your "kitchen sink" example showcasing a lot of the features of the crate.
This is how I am personally using the package to develop my app (a CAD/design program).

### `wasm-app`

This shows how to use the crate purely from the bevy side.  
Showcasing the changes you'd make / dependencies you'd need in bevy.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

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

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Contributing

This crate is an ends to a means for developing an app so I am not sure what level of support I will be
able to provide and I might not be able to support a lot of additional features.  That being said, if you
run into bugs or have ideas for improvements/features feel free to create an issue or, even better, submit a PR. 

> :warning: If the PR is fairly large and complex it could be worth submitting an issue introducing the desired
> changes + the usecase so I can verify if it's something that belongs in this crate.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

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
