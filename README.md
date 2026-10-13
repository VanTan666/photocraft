# Brush preset drag preview evidence

Before: upstream main at 99e1138. After: fix/brush-drag-preview.

Both PNGs render the actual PhotoCraft app offscreen at 1200 x 900 logical points (1.5 scale), with its default synthetic brush library and real pointer press/move events through egui_kittest. No personal document or photograph is loaded. The ProMedium theme is used for the before/after pair.

The included brush-drag-evidence.rs capture harness was temporarily compiled as a ui-egui example and is not part of the product change. The permanent regression test covers all 14 themes in list, grid and popup modes, release cleanup, unchanged brush/library state, and cached rendering. Additional real offscreen snapshots were inspected in all 14 themes.
