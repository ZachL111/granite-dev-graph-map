# Granite Dev Graph Map Walkthrough

This note is the quickest way to read the extra review model in `granite-dev-graph-map`.

| Case | Focus | Score | Lane |
| --- | --- | ---: | --- |
| baseline | change width | 137 | watch |
| stress | diagnostic quality | 204 | ship |
| edge | review cost | 158 | ship |
| recovery | safe rewrite | 180 | ship |
| stale | change width | 238 | ship |

Start with `stale` and `baseline`. They create the widest contrast in this repository's fixture set, which makes them better review anchors than the middle cases.

The useful comparison is `change width` against `change width`, not the raw score alone.
