# docx-you-want
`docx-you-want` is a tool to convert a PDF or Typst document into a `.docx` file ... in an unusual way.
Since these formats are inherently different, it is impossible to get a `.docx` file from a PDF/Typst source without a noticeable difference in their appearances.
`docx-you-want` on the other hand, sort of preserves the look of the original document.

# Packages
[![AUR](https://shields.io/aur/version/docx-you-want)](https://aur.archlinux.org/packages/docx-you-want/)

## What does it really do?
1. It converts each page into an SVG image, then inserts those into a minimal `.docx` file.
   - For **PDF** input: it calls [Inkscape](https://inkscape.org/) to convert every individual page.  
     This means `inkscape` must be installed and in your `PATH`.
   - For **Typst** (`.typ`) input: it calls [Typst](https://typst.app/) to compile directly to SVG via `typst compile --format svg`.  
     This means `typst` must be installed and in your `PATH`.
2. Then it inserts those images into a minimal `.docx` file, adding a PNG version of each also so that programs that don't support SVG in a `.docx` file have something to fall back on.
3. Finally, it zips the files and gives you the `.docx` (you want?).

## When to use this tool?
Hopefully never.

However, if someone asks you to send them a `.docx` version of your document and refuses to accept the PDF (Typst) version that you only have, consider using it.
The next thing should be the person being very sad about your fake `.docx` document and wondering: is this *really* the `.docx` he wants?

## Why Rust?
My bad.

I really should have written it in bash or Python, none of which, including Rust, I am good at, though.
