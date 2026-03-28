Demonstrates the difference between multiple normal windows, a topmost window, an attached child window shown with `show_sublayer_of`, and Foreground overlays.

```sh
cargo run -p topmost_windows
```

Use the controls in the center panel to toggle individual normal windows, the topmost parent window, and its attached child window. Click inside the parent while the child overlaps it to verify that `show_sublayer_of` keeps the child directly above the parent instead of letting the parent cover it.
