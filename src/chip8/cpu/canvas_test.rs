#[cfg(test)]
use speculate::speculate;

use crate::{assert_all_elems_eq, flatten};
use super::*;

speculate! {
    describe "new" {
        test "create an empty canvas" {
            let canvas = Canvas::new();

            assert_all_elems_eq!(flatten!(canvas.pixels), false);
            assert_eq!(canvas.have_pixels_changed, false);
        }
    }

    describe "clean" {
        test "set all pixels to false" {
            let mut canvas = build_canvas(true);

            canvas.clean();

            assert_all_elems_eq!(flatten!(canvas.pixels), false);
        }

        test "set a flag to true if any pixels have changed" {
            let mut canvas = build_canvas(false);
            canvas.pixels[0][0] = true;

            canvas.clean();

            assert_eq!(canvas.have_pixels_changed, true);
        }

        test "set flag to false if no pixels have changed" {
            let mut canvas = build_canvas(false);

            canvas.clean();

            assert_eq!(canvas.have_pixels_changed, false);
        }
    }

    describe "draw_sprite" {
        // ********    11111111    0xFF
        // *......*    10000001    0x81
        // *......*    10000001    0x81
        // *......*    10000001    0x81
        // ********    11111111    0xFF
        const SPRITE: [u8; 5] = [0xFF, 0x81, 0x81, 0x81, 0xFF];

        test "draw a sprite on the canvas" {
            let (x_left, y_top, x_right, y_bottom) = (2, 4, 9, 8);
            let mut canvas = build_canvas(false);

            canvas.draw_sprite(x_left, y_top, &SPRITE);

            let canvas_pixels = canvas_pixels_slice(&canvas, x_left, y_top, x_right, y_bottom);
            let expected_pixels = [
                [true, true,  true,  true,  true,  true,  true,  true],
                [true, false, false, false, false, false, false, true],
                [true, false, false, false, false, false, false, true],
                [true, false, false, false, false, false, false, true],
                [true, true,  true,  true,  true,  true,  true,  true],
            ];
            assert_eq!(canvas_pixels, expected_pixels);
        }

        test "wrap a sprite on the opposite side of canvas" {
            // wraps both horizontally and vertically drawing the sprite in 4 corners
            let (x_left, y_top) = (CHIP8_WIDTH - 4, CHIP8_HEIGHT - 2);
            let (x_left_wrapped, y_top_wrapped) = (0, 0);
            let (x_right, y_bottom) = (CHIP8_WIDTH - 1, CHIP8_HEIGHT - 1);
            let (x_right_wrapped, y_bottom_wrapped) = (3, 2);
            let mut canvas = build_canvas(false);

            canvas.draw_sprite(x_left, y_top, &SPRITE);

            // bottom right corner
            let canvas_pixels = canvas_pixels_slice(&canvas, x_left, y_top, x_right, y_bottom);
            let expected_pixels = [
                [true, true,  true,  true],
                [true, false, false, false],
            ];
            assert_eq!(canvas_pixels, expected_pixels);

            // top right corner (wrapped on y-axis)
            let canvas_pixels = canvas_pixels_slice(&canvas, x_left, y_top_wrapped, x_right, y_bottom_wrapped);
            let expected_pixels = [
                [true, false, false, false],
                [true, false, false, false],
                [true, true,  true,  true],
            ];
            assert_eq!(canvas_pixels, expected_pixels);

            // bottom left corner (wrapped on x-axis)
            let canvas_pixels = canvas_pixels_slice(&canvas, x_left_wrapped, y_top, x_right_wrapped, y_bottom);
            let expected_pixels = [
                [true,  true,  true,  true],
                [false, false, false, true],
            ];
            assert_eq!(canvas_pixels, expected_pixels);

            // top left corner (wrapped on x-axis and y-axis)
            let canvas_pixels = canvas_pixels_slice(&canvas, x_left_wrapped, y_top_wrapped, x_right_wrapped, y_bottom_wrapped);
            let expected_pixels = [
                [false, false, false, true],
                [false, false, false, true],
                [true,  true,  true,  true],
            ];
            assert_eq!(canvas_pixels, expected_pixels);
        }

        test "erase pixels when drawing over existing pixels (XOR)" {
            let (x_left, y_top) = (2, 4);
            let mut canvas = build_canvas(false);

            canvas.draw_sprite(x_left, y_top, &SPRITE);
            canvas.draw_sprite(x_left, y_top, &SPRITE);

            assert_all_elems_eq!(flatten!(canvas.pixels), false);
        }

        test "return true if any previous pixels were erased" {
            let (x_left, y_top) = (2, 4);
            let mut canvas = build_canvas(true);

            let any_pixel_erased = canvas.draw_sprite(x_left, y_top, &SPRITE);

            assert_eq!(any_pixel_erased, true);
        }

        test "return false if no previous pixels were erased" {
            let (x_left, y_top) = (2, 4);
            let mut canvas = build_canvas(false);

            let any_pixel_erased = canvas.draw_sprite(x_left, y_top, &SPRITE);

            assert_eq!(any_pixel_erased, false);
        }
    }

    describe "reset_pixels_changed" {
        test "set the flag to false" {
            let mut canvas = build_canvas(true);

            canvas.clean();
            assert_eq!(canvas.have_pixels_changed, true);

            canvas.reset_pixels_changed();
            assert_eq!(canvas.have_pixels_changed, false);
        }
    }

    fn build_canvas(value: bool) -> Canvas {
        let mut canvas = Canvas::new();
        for pixel in canvas.pixels.iter_mut().flatten() {
            *pixel = value;
        }
        canvas
    }

    fn canvas_pixels_slice(canvas: &Canvas, x_left: usize, y_top: usize, x_right: usize, y_bottom: usize) -> Vec<&[bool]> {
        canvas.pixels[y_top..=y_bottom]
            .iter()
            .map(|row| &row[x_left..=x_right])
            .collect()
    }
}
