use godot::prelude::*;

pub mod sprites;

struct GodotExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GodotExtension {}
