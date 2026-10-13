# Curves dialog layout evidence (issue #5113)

Actual offscreen PhotoCraft snapshots, rendered with the repository snapshot example on a synthetic 128x128 white document, Russian UI, 1280x900 points at 1.5x scale. No personal photos or proprietary assets.

Before: origin/main cdfec72. After: curves-specific 300-point content width. The graph stays the same size; the hint wraps and controls remain visible.

These images show the real application, not a mocked dialog. Additional captures verified every theme; the regression test covers all 14 themes, English/Russian and 1280x900/900x720 sizes.
