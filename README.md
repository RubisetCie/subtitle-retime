# Subtitle Retime

A CLI tool to act on timing of common subtitles format. It can perform classic operations like **shift**, **speed change** and **validation**.

## Usage

It works by chaining operations to run on given input(s) subtitle file(s):

```
subtitle-retime <options> <operation-1> <operation-2> <...> <input-1> <input-2> <...>
```

For example, to shift all subtitles by an amount:

```
subtitle-retime -o output-dir -shift -0.25 my-subtitle-1.srt my-subtitle-2.srt
```

Or, you can "validate" a subtitle, to check issues with the timing:

```
subtitle-retime -n -validate my-subtitle.srt
```

## Features

- **Selecting portions of subtitles**, to run operations on a more targetted subset. For example, in order to shift only the subtitles from 0:05:67.420.
- **Shifting** subtitles uniformly.
- **Changing speed of subtitles**, by a speed factor.
- **Changing framerate** of subtitles.
- **Copying all timings from a reference** subtitle file.
- **Creating a gap** between subtitles when it's below a threshold.

## Supported formats

- SubRip (.srt)
- WebVTT (.vtt)
- Advanced SubStation Alpha (.ass)
- SubStation Alpha (.ssa)
- MicroDVD (.sub)
- SubViewer
- TTML/IMSC (.ttml, .xml)
- YouTube SBV (.sbv)
- SCC (.scc)
- SAMI (.smi)
- Lyrics LRC (.lrc)
- MPL2 (.mpl)
- EBU STL (.stl)
- DFXP
- WHISPER

## Third-party

- [Subtitler](https://github.com/subtitle-rs/subtitler/)
