# XcodePBXParser

XcodePBXParser is a Rust application that fully parses Xcode `.pbxproj` files. These project files control target links, build settings, and file references, but they are difficult to interpret manually. This parser provides a structured view that can be inspected without working directly with the raw text.

**Core idea**: Model the `.pbxproj` syntax with a `pest` grammar so the parser can fully recognize the format and generate consistent Rust representations.

**What we parse**: dictionaries delimited by `{}`, arrays delimited by `()`, quoted strings, numeric values, identifiers, and the inline comments associated with object identifiers.

**How we parse**: read the project file, pass it through the grammar, traverse the generated parse tree, map each node to Rust types (`Dictionary`, `Array`, `String`, `Identifier`, `Number`), and serialize the result to formatted JSON.

**What we expect**: an accurate JSON snapshot that preserves the order and hierarchy of the original file, enabling analysis of targets, groups, and build settings without editing the `.pbxproj` source directly.

