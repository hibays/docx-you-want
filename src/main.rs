/* This file is part of docx-you-want.

   docx-you-want is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   docx-you-want is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with docx-you-want.  If not, see <https://www.gnu.org/licenses/>.
*/

use clap::Parser;
use docx_you_want as dyw;
use docx_you_want::Error;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::exit;

#[derive(Parser)]
#[command(
    name = "docx-you-want",
    about = "Convert PDF or Typst documents to DOCX"
)]
struct Cli {
    /// Embed SVG only (skip PNG fallback)
    #[arg(long)]
    svg_only: bool,

    /// Path to the input PDF or Typst file
    input: PathBuf,

    /// Path to the output DOCX file
    output: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = convert(&cli.input, &cli.output, cli.svg_only) {
        let msg = match e {
            Error::IoError => "An error occurred during I/O.",
            Error::ImageError => "Something went wrong while processing the images.",
            Error::InkscapeNotFound => "Inkscape not found. Consider installing inkscape?",
            Error::PDFInvalid => "Invalid PDF.",
            Error::TypstNotFound => "Typst not found. Consider installing typst?",
            Error::TypstInputInvalid => "Invalid Typst file or compilation failed.",
        };
        eprint!("{}", msg);
        exit(-1);
    }
}

fn convert(src: &Path, dst: &Path, svg_only: bool) -> dyw::Result<()> {
    let mut docx = dyw::Docx::new(svg_only)?;
    match src.extension().and_then(|e| e.to_str()) {
        Some("typ") => docx.convert_typst(src)?,
        _ => docx.convert_pdf(src)?,
    }
    println!("Done");
    print!("Generating the final result ... ");
    io::stdout().flush()?;
    docx.generate_docx(dst)?;
    println!("Done.");
    Ok(())
}
