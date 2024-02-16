# Split RGB
Create 3 new images from the original image, one for each color channel.
In each image, only a single channel is active, and the other two are set to 0.

## Build
``` cargo build --release ```


## Run
 ``` ./target/release/split_rgb <input_image> <output_image> ```