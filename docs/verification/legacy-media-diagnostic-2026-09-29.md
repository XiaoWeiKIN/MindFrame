# Optional legacy media encoding diagnostic — 2026-09-29

## Observed failure

Run36569380103 / verify job109409249723 completed with failure during actual media integration.
Its compiler, Rust/renderer static checks and contract generation passed. This does not invalidate
the independently passed chat-materials job; it does block claiming all optional-video acceptance.

Downloaded artifact11033467585, mindframe-v1-verification, from the failed run and independently
verified its ZIP digest locally:

`da9c3f4cb3d3dc49ab1f22d403d1664805a566eace7dea30590bae4e4af1efbf`

Actual ffprobe results from both retained MP4 files:

| File | Video codec | Pixel format | Range | Dimensions | FPS | Audio |
|---|---|---|---|---|---|---|
| bilibili.mp4 | h264 | yuvj420p | pc | 480 x 270 | 30/1 | aac |
| douyin.mp4 | h264 | yuvj420p | pc | 270 x 480 | 30/1 | aac |

The failure is not a missing MP4 or wrong H.264 codec: output is full-range yuvj420p rather than
the required yuv420p. The verified artifact package lock resolved Remotion 4.0.421.

## Bounded correction

The existing renderMedia call already had pixelFormat:'yuv420p'. Add imageFormat:'png' and
colorSpace:'bt709' instead of weakening the integration assertion or introducing an extra transcode.
This selects lossless captured input and explicit Remotion color conversion. The setting is documented
in the official [renderMedia colorSpace reference](https://www.remotion.dev/docs/renderer/render-media#colorspace).

The change is confined to the optional legacy renderer. The chat-material Rust application and tests
remain byte-identical to the passing c04cb33 revision. The existing full-media integration test remains
the acceptance gate for both layouts, codec/pixel format, AAC, frame rate, duration and full decode.

This note records the observed artifact and submitted correction, **not a claim that the correction
already passed CI**. Inspect the correction revision's actual workflow before marking MEDIA-1 resolved.
The earlier primary verification record remains a dated observation, not a rolling workflow status.
