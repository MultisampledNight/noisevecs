# vectors and noise are friends

![Screenshot of the running sketch, showing lines with a grid as starting point and their ending points following a Simplex noise-ish pattern. Their hue and chroma is also controlled by noise. Their lightness is dependent on the length of a line, the longer a line, the lighter it is.](https://github.com/MultisampledNight/noisevecs/assets/80128916/de5ae0a2-1b47-4799-9f8b-ca455a38866f)

Random inefficient experiment with nannou which I probably shouldn't have done.

Note: This is not actual code I'd write or use in production.
Far too much lack of clarity of data flow,
very inefficient (resizing every frame; could be done in a shader instead),
and too much intrusivity (`Model` should really split its grid logic into a dedicated `Grid` struct).

## Running

Luckily not that involved, since Rust's toolchain is quite nice!

1. Install Rust: https://doc.rust-lang.org/stable/book/ch01-01-installation.html
2. `cargo install --git https://github.com/MultisampledNight/noisevecs`
3. `noisevecs`

## Hacking

1. Install
    - Rust: https://doc.rust-lang.org/stable/book/ch01-01-installation.html
    - Git: Using your system's package manager or from https://git-scm.com/
2. `git clone https://github.com/MultisampledNight/noisevecs`
3. `cd noisevecs`
4. Modify as you like!
5. `cargo run --release`
6. Go back to 4.
