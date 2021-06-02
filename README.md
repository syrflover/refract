# Refract

Refract is a guided image conversion tool. It takes [JPEG](https://en.wikipedia.org/wiki/JPEG) and [PNG](https://en.wikipedia.org/wiki/Portable_Network_Graphics) image sources and produces [AVIF](https://en.wikipedia.org/wiki/AV1#AV1_Image_File_Format_(AVIF)), [JPEG XL](https://en.wikipedia.org/wiki/JPEG_XL), and [WebP](https://en.wikipedia.org/wiki/WebP) clones.

The program is named after — and works like — eye doctor Refraction Tests. It generates candidate images at various qualities, asking at each step how it looks, and uses that feedback to arrive at the smallest possible "acceptable" output.

Hence "guided".

The beauty of this approach is that it moots the need for exhaustive testing. Refract's quality stepping works by selecting the mid-point between moving min/max boundaries. Each answer you provide adjusts the range, allowing the final, perfect result to be discovered in just 5-10 steps instead of 100+.



## Why?

Every image is different. The idea of a "Magic Bullet" format is a myth.

If you want to truly maximize the quality and size of next-gen copies, you cannot rely on hardcoded constants or fancy [SSIM](https://en.wikipedia.org/wiki/Structural_similarity) analysis. That will result in frequent over- or under-compression, and some images will just come out looking… worse.

You have to actually _use your eyes_. And you have to pay attention to the resulting file sizes. Sometimes newer formats will result in _larger_ output than the original source, defeating the purpose!

While you can do all of this manually — running multiple programs hundreds of times for each and every source you want to convert — that would be incredibly tedious and easy to screw up.

Refract helps makes that manual process _less_ tedious and _more_ foolproof.

It automatically uses the strongest (slowest) possible compression settings for each format, keeps track of file sizes and qualities along the way, can process inputs en masse, and reduces the number of conversion tests by around 90%.

Should you use it for every image ever? No, probably not. The next generation formats, particularly AVIF and JPEG XL, require a lot of computation to discover byte savings. All those minutes will add up quickly.

But if you're looking to obsessively optimize a small project or single web page, Refract is the way to go!



## Features

Only JPEG and PNG input sources are supported. They can have RGB, RGBA, greyscale, or greyscale w/ alpha color spaces, but CMYK is not supported.

Conversion is done at pixel level; gamma and other metadata profile information is not factored or present in the converted copies, so is not supported.

Refract implements [`libavif`](https://github.com/AOMediaCodec/libavif), [`libjxl`](https://gitlab.com/wg1/jpeg-xl), and [`libwebp`](https://chromium.googlesource.com/webm/libwebp/) directly. This means the images it produces will match those produced by the official `avifenc`, `cjxl`, and `cwebp` binaries, but it also means you don't _need_ those official programs installed to use it.

Refract supports both lossless and lossy encoding for each of the three formats, and by default will try both. (Sometimes lossless actually wins!)

Refract also supports both full-range `RGB` and limited-range `YCbCr` encoding modes for AVIF.



## Flavors

Refract comes in two flavors — [CLI](#cli-version) and [GTK](#gtk-version) (GUI) — differing only in how they manage image previews.

The GTK version, being graphical, simply displays source and candidate images within its interface, allowing you to easily toggle between the two to quickly identify good encodings from bad ones.

The CLI version instead saves each candidate image to a temporary file so you can open and inspect them in any program of your choosing, such as a web browser.

Other than that, the two versions share the same features and process images the same way.



## CLI Version

The original Refract is a simple CLI tool that saves each candidate image to a temporary location (which you can open and inspect in any program of your choosing).

![Example CLI screen.](https://github.com/Blobfolio/refract/raw/master/skel/prompt.png)

Final, "best" candidate images are saved next to the source(s) with the appropriate extension tacked onto the end, e.g. `image.jpeg.webp`.

### Usage

Just run `refract [FLAGS] [OPTIONS] <PATH(S)>…`.

The following flags are available:

```bash
-h, --help          Prints help information.
    --no-avif       Skip AVIF conversion.
    --no-jxl        Skip JPEG XL conversion.
    --no-webp       Skip WebP conversion.
    --skip-lossless Only perform lossy encoding.
    --skip-lossy    Only perform lossless encoding.
    --skip-ycbcr    Only test full-range RGB AVIF encoding (when encoding
                    AVIFs).
-V, --version       Prints version information.
```

By default, Refract CLI will generate copies in every next-gen format. If you want to skip one, use the corresponding `--no-*` flag.

You can specify any number of input paths. The paths can be individual JPEG or PNG images, or directories containing such images. You can also/alternatively specify paths using the `-l`/`--list` option, which should point to a text file containing any number of paths, one per line.

```bash
# Handle one image.
refract /path/to/image.jpg

# Example pulling paths from a text file.
refract --list /path/to/list.txt /path/to/another/image.jpg

# Skip WebP.
refract --no-webp /path/to/image.jpg
```

### Installation

Debian and Ubuntu users can just grab the pre-built `refract_#.#.#_amd64.deb` package from the [release page](https://github.com/Blobfolio/refract/releases/latest).

Refract can also be built from source on other x86-64 Unix systems with `Rust`/`Cargo`:

```
# Clone the repository.
git clone https://github.com/Blobfolio/refract.git

# Move into the directory.
cd refract

# Build with Cargo.
# Note: you should specify both --bin and -p to avoid binary bloat.
cargo build \
    --bin refract \
    -p refract \
    --release
```

Image formats are… complicated, and many of the decoders have `build.rs` system dependencies that need to be present on the system when compiling from source, including up-to-date versions of Clang, GCC, NASM, and Ninja. (If you end up needing to install anything else, please open a ticket so I can update this README.)



## GTK Version

The guided generation offered by Refract CLI is a big time saver, but previewing the images it generates can still be quite the time suck.

The GUI version of Refract helps streamline the process by providing a built-in A/B image viewer, letting you quickly toggle between source and candidate images to inspect the differences.

![Example GTK screen.](https://github.com/Blobfolio/refract/raw/master/skel/gtk.webp)

### Usage

As with the CLI version, you can load and encode a single source image, or queue up an entire directory to process multiple images en masse.

Unlike the CLI version, a `Save` prompt is popped whenever a "best" candidate has been found, allowing you to specify any arbitrary output destination.

For keyboard afficionados, the following hot-keys are provided:

| Action | Key(s) |
| ------ | ------ |
| Open File | `CTRL + o` |
| Open Directory | `SHIFT + CTRL + o` |
| Toggle View | `SPACE` |
| Discard Candidate | `d` |
| Keep Candidate | `k` |

### Installation

Debian and Ubuntu users can just grab the pre-built `refract-gtk_#.#.#_amd64.deb` package from the [release page](https://github.com/Blobfolio/refract/releases/latest).

Refract GTK can also be built from source on other x86-64 Unix systems with `Rust`/`Cargo`:

```
# Clone the repository.
git clone https://github.com/Blobfolio/refract.git

# Move into the directory.
cd refract

# Build with Cargo.
# Note: you should specify both --bin and -p to avoid binary bloat.
cargo build \
    --bin refract-gtk \
    -p refract-gtk \
    --release
```

This comes with the same build caveats as the [CLI version](#cli-version), but with the additional monster that is GTK. To build this version from source, you're going to need the `-dev` packages for ATK, Cairo, GDK, GLIB, GTK, Pango, Pixbuf, and probably a few other things.

Thankfully many distributions offer meta packages to bundle away the tedium. On Debian Buster, for example, installing `librust-gtk-dev` and `librust-gdk-dev` should just about cover it.



## License

See also: [CREDITS.md](CREDITS.md)

Copyright © 2021 [Blobfolio, LLC](https://blobfolio.com) &lt;hello@blobfolio.com&gt;

This work is free. You can redistribute it and/or modify it under the terms of the Do What The Fuck You Want To Public License, Version 2.

    DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
    Version 2, December 2004
    
    Copyright (C) 2004 Sam Hocevar <sam@hocevar.net>
    
    Everyone is permitted to copy and distribute verbatim or modified
    copies of this license document, and changing it is allowed as long
    as the name is changed.
    
    DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
    TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
    
    0. You just DO WHAT THE FUCK YOU WANT TO.
