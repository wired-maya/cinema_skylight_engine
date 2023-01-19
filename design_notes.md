# Engine Design

## Philosophies

### Relative Simplicity

The engine should be easy to use for someone reasonably comfortable with computers, but doesn't necessarily know how to program. It should allow for at the most basic level a relatively simple and efficient creation of visual novels similar to writing a script.

### Optional Complexity

Despite the highest level of the engine being simple, it contains access to many lower level implementations to allow for extreme customizability of the engine, with the intention of allowing the developer to create whatever effects they can imagine.

### Expanding on Inspiration

Being inspired by the Film-Window system, this engine intends to improve and expand its function to create something new and original.

## Approach to Building

Build the engine bottom up then outwards. What this means, is that first the background widget is created, then the easier widgets (text, images), then the more complex widgets (3D view, nametag), all with default styling (animations, fonts, colours, etc). Ensure the basic functionality works (can advance text, resizes properly, etc), then fill in the ways to customize the styles. Look at Flutter widgets for inspiration when creating these expansive styles.

Then, implement sound, user interaction, etc.

After that, do as much research as possible on how to optimize OpenGL and Rust, and then move on to other libraries such as DX12 and Vulkan.

## General Functionality

### Widgets

#### Background

- The background to the game
- Generally a visualizer for currently playing music
- By default rendered as an orthographic 3D image for complex visualizers that look 2D, but later allow for perspective projection and so on
- This has its own shader program for the 3D view, and by default has a light at the camera, though this can be changed

#### Text

- Width and height is defined by characters and lines respectively
- Implements word wrapping with various options
- Any characters past limit are cropped
- User can define the sound that plays during drawing of the text box
    - Has support for per-letter sound as well as one that plays during the entire drawing process
    - Supports separate sounds for different stages of the drawing, for example each newline or a sound that plays when a text block has completed animation
    - Markdown support, with extra features to change colours
    - The general sounds are in the style, but then there is a property in the actual widget struct that overrides that stlye in case one wants to quickly provide voice acting.
- Also has functionality for TTS executable to generate voice or voice mumble

#### Image

- Simple boxed image that is loaded from hard-drive at runtime
- Has options to fit, fill, cover, etc.
- Can change pictures without being re-animated in to simulate the flipping of slides or changing of expressions in a sprite

#### Nametag

- Custom x, y, width, and height
- Essentially a square that annotates a custom area
- Usually to annotate someone's name onto a bigger picture of a scene by wrapping their face

#### Portrait

- Combination of image and nametag widgets, default way to represent characters

#### Menu

- A widget that allows the user to choose between multiple options
- The way it is written will be an async/await paradigm, allowing the main code to wait until an option is chosen, and then return a value for the developer to easily work with it and create branching paths

#### 3D Environment

- A window that displays a 3D environment
- This is where a character could move to interact with the world, solve puzzles, etc.
- Interaction is planned but not currently a goal

### Sound Functionality

#### Background Sound System

- Plays audio on loop while it is enabled
- Intended to play music in the background, as well as ambience, etc.

#### Single Sound System

- Plays sound file once then exits
- Intention is to play sound effect in conjunction with other events

### Simultaneous Blocks

- Their own function or macro that contains information on what to do
- Has options on how to behave, e.g. whether they all animate in at the same time, or with x delay in order of declaration, or one after another, etc.

## Engine Implementations

### General Engine Structure

The general structure is defined as such:

1. An underlying GL backend that can be switched out depending on which GL is used. Could be a struct implementing an extremely extensive amount of traits. This would be handled by the developer.
2. The core of the engine that runs most of the code. This would include multiple threads, the render loop, etc.
3. API abstraction layer over the core engine that enables the simple top level creation of games.

#### Top-Level Developer API Syntax

The top-level API's syntax will be similar to the "stateless instance" way the GL crate works. Instead of being a limitation however, here it is intentional for simplicity to the end developer. The general structure of a game written in this API will be as such:

```rust
use cinema_skylight_engine::*;

fn main() {
	let config = CSEngineCongfig { ... };

	CSEngine::init(config); // Global state is created here

	let text = widget::Text::new(widget::TextConfig { ... }); // Can be any widget here
	text.display("Foo");
	text.close();

	// ...Rest of game here...
}
```

The top level API handles blocking between `.display()` calls so everything is executed sequentially and to allow for developer defined logic inbetween displaying things on the screen to take place, possibly even mutating the widgets themselves. The `.display()` call also handles animating the widget in if it is not present on the screen yet, and updating values if the widget's config is updated (for example it is resized).

Behind the scenes these structs are a thin wrapper over a weak reference to the struct on the UI thread, and is dropped if the struct it is referencing is dropped. The functions implemented on the wrapper struct handle all dereferencing, thread sending, and blocking. This blocking behaviour also allows for menu widgets to return their result for the developer to handle that logic.

Multiple events happening with one advance of the state (e.g. when you advance the visual novel, a second text box pops up at the same time as the text advancing in the main text box) can be expressed with macros which translate into similar code to what is in the `.display()` function, just with only one block:

```rust
simul!(
	SimulOptions { ... }, // Says how the widgets are animated in, see definition
	text1.display("Foo"), // Variable length arguments similar to println!()
	text2.display("Bar")
)
```

Could later support a text file that is either interpreted or trans-piled into rust code to make writing this even simpler.

##### Loading screens

Loading screens are handled by allowing the user to create whatever light-weight screen they want using the widget system, then wait until all the widgets specified have completed loading and returned weak references before advancing.

The `new()` constructor on widgets passes widgets to the loading thread where they are loaded and return a weak reference. Certain widgets would not need to be loaded, for example text widgets, provided there isn't a feature later on where you can load them from text files.

There should be a wait macro/function that waits until the weak reference is returned back, to ensure everything works correctly. This would be program thread blocking, and could be used to ensure a widget is loaded before it is displayed. The `.display()` method calls the same function to ensure that the widget is loaded before displaying, allowing you to also define the widget earlier in the script and bypass loading screens by hoping it's loaded by then. If it isn't it just waits for a bit. Could look into adding some default widget that appears on screen to signify if the game is still waiting for a widget to load outside of a loading screen.

Because a loading screen would finish when the loading has completed instead of on user input, add in the config for a given widget an option as to what the `.display()` waits for when blocking, if at all. By default this would still be waiting for the user to advance the text.

#### Core Engine

##### Multiple Threads

The engine will consist of at least 3 threads that handle all logic:

1. Main thread that handles interaction, moving between states, feeding the other threads, loading assets etc.
	- See if you can change this to solely an input handling thread with everything else handled by the program thread
1. UI thread that contains the render loop, this contains the render stack as well handles everything OpenGL except loading assets.
2. Loading thread that handles quietly loading everything in the background to have a smoother experience.

This will work by having the `main()` function send jobs to the Main thread, which then handles them sequentially and sends off jobs to the loading thread, which returns the main thread a weak reference and sends the actual widget to the UI thread, and then waiting until some user interaction advances the game's logic. The program thread (or the `main()` function) is read sequentially and blocked until the user advances the game state to allow for a simple API for developers.

An optional fourth thread to handle all 3D environment widget rendering calls. Possibly also more threading for multithreaded asset loading. [Ensure extra threads aren't created when they can't be](https://doc.rust-lang.org/std/thread/struct.Builder.html), as well as [create a drop trait to ensure the threads complete their work before exiting](https://doc.rust-lang.org/book/ch20-03-graceful-shutdown-and-cleanup.html).

Have each function that updates UI (maybe that's its own trait?) wait until user input before proceeding, allowing the declarative rust-native style for using the engine.

#### Render Stack

Each widget to be rendered on the screen is contained in a growable list called the render stack. Every frame it is iterated over, and each item in the stack is drawn on top of eachother sequentially (first element is the first drawn, etc), so that the closer to the end an element is on the render stack, the more on top it is.

This will be the main data structure that owns the widgets, and will decide whether they need to be dropped (usually upon recieving input from the interaction thread).

This is performed without Depth Testing, so first in first drawn is always obeyed.

#### Rust Crate

The engine is a Rust crate, allowing for a developer to import it and have a deep API that offers all levels of abstractions to sllow for the developer to decide the complexity of their code. This also allows the exploitation of Rust's quirks to create a more extensible engine, and an extremely performant game.

Be sure to document as much as possible and use the excellent documentation features Rust provides.

Use Rust's system to be extremely aggressive to remove unused code, for example Vulkan support if not enabled, to keep speed up and size down.

#### Render Loop

The engine has a dedicated render loop and a closure you can set to be run during the render loop, allowing to either replace the render loop entirely or just have something be calculated each frame.

### Asset Loading

#### Feedback

While loading screens are completely custom, allow for an engine-wide feedback string that explains what is currently happening, if one desires to use it in some way.

### Useful Crates

- Glutin is a pure Rust alternative to GLFW (window system)
- [Find more useful packages here](https://arewegameyet.rs/#ecosystem)
- Egui is a pure Rust GUI library which could be used to provide debug information within the actual engine
    - Would provide information that RenderDoc couldn't, for example the render stack
    - Make sure to have this not included in the release executable to not bloat final game, though if the developer desires they have to option to include it anyways (make a setting for it)
- Clap is the most featureful command line parsing tool, could be used for debug config for the game and not the engine
- Rayon allows parallel iterators to maximize performance when unpacking data for example
- Eyre/Color eyre avoid the mess of different error types by making one unified one that then has detailed and amazing runtime exceptions (probably won't be needed as well as shouldn't be used because of speed considerations)
- Tokio is a very good async library that has zero-cost abstractions, pretty much necessary

### Widgets

#### Specific Properties

- Depth parameter
    - Represents position of widget in draw stack, if not specified defaults to last item in the stack
    - If a position is set, the widget stack handles it by inserting the widget into that position of the array and shifting the other widgets accordingly
    - Most likely needs to be a getter and setter function rather than a struct property
- Each widget struct implements a Widget trait which contains useful functions such as `.draw()`, which uses `&self: Box<Self>` to ensure it's only drawn when in the render stack

#### General Structure

The general method of widgets is to be a struct with information on style, location, size, etc. When it is created and added to the render stack it is animated in, and when it is dropped it is animated out. Each individual widget is its own self contained struct, and so each individual widget can be styled differently. 

Allow creation of one single style struct later on that is stored by a widget of that type (different style struct per each widget) to allow for less code duplication. As well, allow the changing of the default styles of the engine to further cut down on code duplication (e.g. most text is drawn according to style defined at the top of the game, and then specific different text needs to then be given an overrided style struct). Default trait could take from default of the engine instance to ensure least amount of code duplication.

#### 3D View and Background Widgets

As these are the only widgets (for now) that contain 3D scenes inside of them, have them act as a wrapper window to a 3D `scene` struct. This is to say, the styling and animation logic is in the widget specific struct, which holds a more general `scene` struct that has the job of rendering the scene.

This scene struct will have a vector of scene objects, a skybox, and so on, and handles in-scene animations, transformations, and rendering via a draw function/trait.

### OpenGL Implementations

Currently the engine will only support OpenGL, but in the future plan to support Vulkan and DX12 as well. This will come with the option to restrict which graphics libraries are used when compiling if for example you want to support ray tracing (which would restrict it to Vulkan and DX12). Maybe even WebGL and mobile OpenGL as well in the far future to allow for easy porting.

#### Abstractions

- Extremely important that safe abstractions are made over the unsafe OpenGL code
- Consolidate as many OpenGL calls as possible, for example have every model generate an array of texture IDs with one call rather than doing it every mesh, etc.
- To ensure safety, the IDs and direct references to OpenGL objects on the GPU need to be private to ensure that all unsafety is properly abstracted out.
- These abstractions could be treated like smart pointers
- Drop trait de-allocates ALL objects automatically unless hot-reloading

#### 3D Environment Widget

- Rendered to a different framebuffer and bound to texture that is drawn onto a quad representing a widget
- Implemented fully using deffered shading and other such framebuffer tricks to allow for full post processing, or in other words full game functionlity

### Hot Reload/Restart

Create a system for hot reloading the game when you change something small like the position of a widget without the need to completely reload all assets. This would still build the application, but takes advancage of Rust not building the entire executable again from scratch and the fact that OpenGL objects are still on the GPU to significantly speed up development time by removing the need to wait for asset loading between small changes.

- Would be a button integrated into the IDE like compile is now
- Still requires full reload when lower level changes happen, but that shouldn't matter as it is expected
- Ensure this framework is compiled out of release executables to further optimize the engine
- Find a way to transfer GL contexts between applications, for example a command like argument that contains a ton of assets and their associated object IDs which are then re-used in the re-compiled profram instead of loading the assets again
    - This would be implemented as a check when creating new assets on whether it exists already on the graphics card, if it doesn't then it is created again normally, or if it an object is no longer used, clean it up normally
    - If a hot reload is triggered, make sure to not automatically clean up resources in the drop trait like they would be normally
- See if there is a way to keep OpenGL and window contexts and just transfer them between runtimes
- Opt-in as it would be extremely experimental and potentialy unstable/unsafe for your dev environment
- After exe is recompiled and the old one's process is terminated completely, simply remove it (to get around accidentally rewriting, could append timestamp to the built exe)

If the above system is unfeasable, you could have an engine that throws all changes into an interpreted file for hot restart between compilations, but only for editing the script. This system could be combined with the previous as well to enable a hot-reload/restart paradigm similar to Flutter's

#### Shader Hot Reload

Shader hot reload is simple, just tie a button to the recompilation of shaders on the GPU.

### Error Handling

In the engine, there are two kinds of error handling. The core engine uses `Result<T, E>` types that propagate up through the engine, so be handled by anyone who is writing a program using the core engine. The abstract wrapper around the core engine handles these errors by logging them to console, simplifying the process.

#### Logging

Logging should be able to changed based on the level of logging desired. This extends to OpenGL logging and errors, as well as debug for the general engine. Logging should default to minimum level when building for release and maximum level when building for debug. A basic 3 levels could be the following:

- Level 1: Only severe errors
- Level 2: Errors and warnings
- Level 3: Errors, warnings, and debug statements

### Other Notes

- Enum with `Rgb(i32, i32, i32)`, `Hsv(i32, i32, i32)`, etc. values could be very useful as a design pattern
- Might be able to use mutable [static vars](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#accessing-or-modifying-a-mutable-static-variable)?
- Aliases to cut down on repetition, e.g. `type Widget = Box<dyn Draw, etc>`
- It’s best to write functions using a generic type and one of the closure traits so your functions can accept either functions or closures

## Possible Scripting Syntax

Most likely not going to be used, but is useful to write down incase it is needed later for a possible secondary system for declaring a script.

### General Syntax

```
[<widget name> <widget id>]
(param1 = foo, param 2 = bar)
<text, if applicable>
```

### Example Text Widget

```
[Text sample_text]
(width = 20, height = 1, x = 0, y = 0, depth = 0)
Hello, how are you?	
wonderful weather we are having, huh
[Close sample_text]
```

### Other Notes

- Text is the only widget that has the strings following declaration
- Text always starts on first line after it has animated in, then remains on last text until closed
- Will need to find a way to tag text blocks, e.g. sample_text="test", or sample_text=[]
- Force multi-line text blocks would be written surrounded with "" (This would override word wrapping on >1 height text widgets)
- You can do multiple things are once by wrapping them in curly braces
- Has header with window size and other information

## Visual Editor

A visual editor will exist that streamlines creating files with the scripting syntax, as well as 3D scenes. It will use a modified engine window for the preview (which can be switched out by you if you implement the proper functions in your game's debug exe).

The editor will have two modes, the VN mode, and the 3D scene mode:

In the VN mode, you have a video editor-like timeline at the bottom of various widgets, which can be live previewed in the window. These are separated with "steps" instead of seconds. The user has multiple sizing options, which are interchangeable based on each widget. The options include pixels (there is a standard size the project expects to be, and they are scaled relatively based on size of the window), absolute pixels (these are absolute and do not scale with the window, use wisely!), percents (0-100% width and height wise, very difficult to make squares), and "cells" (these take your aspect ratio you choose at the start of the project, and multiply them by a set amount, default 100 e.g. 16:9 => 1600x900 cells)

In 3D mode, you get to create a scene by importing models, nesting gameobjects, maybe even enabling physics, navigation nodes, etc. These scenes are used in the 3D game view widgets and can interact with them to produce a dynamic UI.

The 3D scenes are saved to scene files, similar to the script files of the VN portion, which contain all necessary data for the engine to display.