# Swatches grid evidence (#5103)

Real offscreen PhotoCraft snapshots using the unchanged snapshot example, 1280x900 at 1.5x scale, English UI, default clean-room swatch library, Color group height 470 points. No personal photos or proprietary reference assets.

Before: main 99e1138, 4-point inter-tile gutters and 8-point inner grid inset.
After: zero inter-tile gaps and no additional grid inset; square tiles and inside selection/hover outlines. Other preset browsers retain their existing spacing.

Reproduce with the snapshot example and this script:

```json
[["engine.execute",{"command":"prefs.set","params":{"path":"interface.language","value":"en"}}],["ui.menu.invoke",{"id":"window.panel.swatches"}],["ui.set",{"dock":{"heights":{"color":470}}}]]
```

Additional actual snapshots inspected in all 14 themes at 1280x900 / 900x720. The real-panel regression covers 14 themes, 180/320-point widths and 22/40-point thumbnail sizes, checking horizontal/vertical adjacency and left alignment.
