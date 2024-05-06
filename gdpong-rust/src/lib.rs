use godot::prelude::*;

struct GodotExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GodotExtension {}
