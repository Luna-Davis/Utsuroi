
# Utsuroi - Rust GUI Library

`Utsuroi` is a japanese word meaning `gentle change`. This project comes from a pain of wanting to code an app with a GUI library but looking at the current available options were building GUI apps.

## Why?

As I just mentioned, I was searching for a GUI library for writing native GUI apps with rust but there wasn't something good enough for me. I do acknowledge good projects like [egui](https://docs.rs/egui/latest/egui/), [tauri](https://tauri.app) and all such libraries but they had a thing or two in common, they talk about `web` and since I wasn't aiming to learn web dev in rust they fell off the picture.

Then there are the bindings for [gtk4-rs](https://docs.rs/gtk4/latest/gtk4/) and [gtk3-rs](https://github.com/gtk-rs/gtk3-rs) which was archived in 2024. I didn't want to use them since according to the docs, I quote:
`GTK is not thread-safe. Accordingly, none of this crate’s structs implement Send or Sync.` and using a gui library native to rust would feel nice since `GTK` is built in C by the [Gnome Project Devs](https://www.gnome.org/) so bindings is the closest thing we can get to GTK's full potential

## What it isn't

So as not to confuse Utsuroi with other crates, Utsuroi isn't the following:

- `Not a web renderer`. No HTML, no CSS, no JavaScript, no webview. It doesn't bundle a browser nor incorporate anything web related. It's just Pure Rust

- Not a binding to an existing toolkit. Nothing wraps GTK, Qt, Win32, or Cocoa underneath.

- Not a game engine UI layer. No scene graph, no ECS, not designed around a game loop.

- Not production ready. Experimental, in active early development, API will break.

- Not a widget library you drop into an existing app. It owns the window and the render loop.

- Not cross-platform yet. Vulkan on Linux first, other backends later. Since I'm on Linux, it's developed with Linux compatibility in mind. Though support will come in future for other OSs.

## How I planned to build it

I planned to create this libary in phases as follows:

- [X] Set the foundation with creating the window and setting up the GPU
- [ ] Rendering shapes on the window
- [ ] Rendering text measurements (Starting with English first)
- [ ] Writting the `LayoutEngine` - measuring and arrange phases
- [ ] Adding the core widgets and event handling
- [ ] Adding accessiblity via the `accesskit`
- [ ] Finalizing with the IME

## Contributing

Contributions are always welcome!

See `contributing.md` for ways to get started.

Please adhere to this project's `code of conduct`.


## Documentation

[Documentation](https://linktodocumentation)


