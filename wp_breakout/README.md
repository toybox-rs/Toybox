## Toybox-WASM-Gameplay

To Build:

### Manual Step: Add to workspace.

Add this crate, e.g., ``wasm`` to the toplevel Cargo.toml. Otherwise you'll get an error like:

```
error: current package believes it's in a workspace when it's not:
current:   /Users/jfoley/code/practice-release/toybox-core/wasm/Cargo.toml
workspace: /Users/jfoley/code/practice-release/toybox-core/Cargo.toml

this may be fixable by adding `wasm` to the `workspace.members` array of the manifest located at: /Users/jfoley/code/practice-release/toybox-core/Cargo.toml
```

### Install Dependencies:

Run ``deps.sh``.


### Compile WASM, Bindgen, and build Javascript:

Run ``web.sh``. If it all succeeds, you should have a bunch of files inside ``target`` in this directory. If not, file an issue with as many details as you can!

### Copy target contents to webserver

You need a server that understands the application/wasm mime type.

