# canvas

Draw a pixel grid to a window. This can be used for basic games, visualizations, drawings, etc. 

This is essentially a reimplementation of <https://crates.io/crates/pixels> for learning purposes.

The drawing is done using wgpu. A single quad that covers the screen is created and the pixels are drawn onto it as a texture. The implementation was purposely made as simple as possible, so some features from the pixels crate, like mouse coordinate conversion, are not implemented.
