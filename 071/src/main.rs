mod am;
mod color;
mod de;
mod order;
mod util;

use de::from_reader;
use plotters::{
    coord::Shift,
    element::{Polygon, Rectangle, Text},
    prelude::{DrawingArea, DrawingBackend, IntoDrawingArea, SVGBackend},
    style::{
        text_anchor::{HPos, Pos, VPos},
        FontDesc, FontFamily, FontStyle, FontTransform, RGBColor, ShapeStyle,
        TextStyle,
    },
};
use std::{self, env, fs::File};
use util::{longest_contig_substr, PI_2};

const DARK_GREY: RGBColor = RGBColor(0x11, 0x13, 0x15);
const OFF_WHITE: RGBColor = RGBColor(0xf1, 0xf3, 0xf5);
const PADDING: i32 = 4;
const STROKE_WIDTH: i32 = 2;
const COL_WIDTH: i32 = 32;
const TOP_ROW_HEIGHT: i32 = 152;
const ROW_HEIGHT: i32 = 32;
const THIN_ROW_MARGIN: i32 = 12;
const TRIANGLE_HEIGHT: i32 = 8;
const DASHED_LINE_STRIDE: i32 = 8;
const FONT_SIZE: f64 = 24.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (input_file_name, output_file_name) = handle_args()?;

    let (vs, he_list, am) = {
        let f = File::open(input_file_name)?;

        from_reader(f)?
    };
    let order = order::guess_order(&am);

    let canvas_size = (
        vs.len() as i32 * (COL_WIDTH + STROKE_WIDTH) - STROKE_WIDTH,
        TOP_ROW_HEIGHT
            + STROKE_WIDTH
            + (he_list.len() as i32) * (PADDING + ROW_HEIGHT)
            + PADDING
            + STROKE_WIDTH
            + TOP_ROW_HEIGHT,
    );

    let backend = SVGBackend::new(
        &output_file_name,
        (canvas_size.0 as u32, canvas_size.1 as u32),
    )
    .into_drawing_area();

    backend.draw(&Rectangle::new(
        [(0, 0), canvas_size],
        ShapeStyle::from(DARK_GREY).filled().stroke_width(0),
    ))?;

    for (ix, o) in order.iter().enumerate() {
        draw_col(
            &backend,
            ix as i32,
            he_list.len() as i32,
            &vs[*o],
            color::sample_palette(ix as f32 / (vs.len() as f32)),
            canvas_size.1,
            ix + 1 != order.len(),
        )?;
    }

    let mut he_list: Vec<_> = he_list
        .into_iter()
        .map(|(he_name, he)| {
            let mut he_indices = Vec::new();
            for v_name in he {
                let v_id = vs.binary_search(&v_name).unwrap();
                let v_ix = order.iter().position(|id| id == &v_id).unwrap();

                he_indices.push(v_ix);
            }
            he_indices.sort_unstable();

            (he_name, he_indices)
        })
        .collect();
    he_list.sort_unstable_by(|(_, he_indices0), (_, he_indices1)| {
        let avg0 = he_indices0
            .iter()
            .map(|&ix| (ix as f32 / (vs.len() as f32) * PI_2).sin())
            .sum::<f32>()
            / (he_indices0.len() as f32);
        let avg1 = he_indices1
            .iter()
            .map(|&ix| (ix as f32 / (vs.len() as f32) * PI_2).sin())
            .sum::<f32>()
            / (he_indices1.len() as f32);

        avg0.partial_cmp(&avg1).unwrap()
    });
    for (i, (he_name, he_indices)) in he_list.into_iter().enumerate() {
        let he_color =
            color::mix(he_indices.iter().map(|&i| {
                color::sample_palette(i as f32 / (vs.len() as f32))
            }))
            .unwrap();
        let y_offset = i as i32 * (PADDING + ROW_HEIGHT);

        // Whether or not we've started and then stopped drawing a box, thus
        // leaving a rightwards gap.
        let mut stopped = None;
        for (j, he_ix) in he_indices.iter().enumerate() {
            let (stretch_l, stretch_r) = (
                {
                    let prev_he_ix = he_indices
                        [if j == 0 { he_indices.len() - 1 } else { j - 1 }];

                    (prev_he_ix + 1) % vs.len() == *he_ix
                },
                {
                    let next_he_ix = he_indices[(j + 1) % he_indices.len()];

                    (he_ix + 1) % vs.len() == next_he_ix
                },
            );

            let x_offset = *he_ix as i32 * (COL_WIDTH + STROKE_WIDTH);
            if let Some(gap_he_ix) = stopped {
                let gap_x_offset =
                    gap_he_ix as i32 * (COL_WIDTH + STROKE_WIDTH);

                backend.draw(&Rectangle::new(
                    [
                        (
                            gap_x_offset + PADDING + 1,
                            y_offset
                                + TOP_ROW_HEIGHT
                                + STROKE_WIDTH
                                + PADDING
                                + THIN_ROW_MARGIN,
                        ),
                        (
                            x_offset + COL_WIDTH - PADDING - 1,
                            y_offset
                                + TOP_ROW_HEIGHT
                                + STROKE_WIDTH
                                + PADDING
                                + ROW_HEIGHT
                                - THIN_ROW_MARGIN,
                        ),
                    ],
                    ShapeStyle::from(he_color).filled().stroke_width(0),
                ))?;

                backend.draw(&Polygon::new(
                    [
                        (
                            gap_x_offset + COL_WIDTH - PADDING - 1,
                            y_offset
                                + TOP_ROW_HEIGHT
                                + STROKE_WIDTH
                                + PADDING
                                + (THIN_ROW_MARGIN / 2),
                        ),
                        (
                            gap_x_offset + COL_WIDTH - PADDING - 1,
                            y_offset
                                + TOP_ROW_HEIGHT
                                + STROKE_WIDTH
                                + PADDING
                                + ROW_HEIGHT
                                - (THIN_ROW_MARGIN / 2),
                        ),
                        (
                            gap_x_offset + COL_WIDTH - PADDING
                                + TRIANGLE_HEIGHT,
                            y_offset
                                + TOP_ROW_HEIGHT
                                + STROKE_WIDTH
                                + PADDING
                                + (ROW_HEIGHT / 2),
                        ),
                    ],
                    ShapeStyle::from(he_color).filled().stroke_width(0),
                ))?;
                backend.draw(&Polygon::new(
                    [
                        (
                            x_offset + PADDING + 1,
                            y_offset
                                + TOP_ROW_HEIGHT
                                + STROKE_WIDTH
                                + PADDING
                                + (THIN_ROW_MARGIN / 2),
                        ),
                        (
                            x_offset + PADDING + 1,
                            y_offset
                                + TOP_ROW_HEIGHT
                                + STROKE_WIDTH
                                + PADDING
                                + ROW_HEIGHT
                                - (THIN_ROW_MARGIN / 2),
                        ),
                        (
                            x_offset + PADDING - TRIANGLE_HEIGHT,
                            y_offset
                                + TOP_ROW_HEIGHT
                                + STROKE_WIDTH
                                + PADDING
                                + (ROW_HEIGHT / 2),
                        ),
                    ],
                    ShapeStyle::from(he_color).filled().stroke_width(0),
                ))?;
            }
            if !stretch_r {
                stopped = Some(*he_ix);
            }

            backend.draw(&Rectangle::new(
                [
                    (
                        x_offset
                            + if stretch_l { -STROKE_WIDTH } else { PADDING },
                        y_offset + TOP_ROW_HEIGHT + STROKE_WIDTH + PADDING,
                    ),
                    (
                        x_offset
                            + COL_WIDTH
                            + if stretch_r { STROKE_WIDTH } else { -PADDING },
                        y_offset
                            + TOP_ROW_HEIGHT
                            + STROKE_WIDTH
                            + PADDING
                            + ROW_HEIGHT,
                    ),
                ],
                ShapeStyle::from(he_color).filled().stroke_width(0),
            ))?;
        }

        let text_lines: Vec<_> =
            he_name.split_inclusive(&['-', ' '][..]).collect();
        let (best_starting, best_streak) = longest_contig_substr(&he_indices);
        let text_color = if color::is_dark(he_color)
            || (best_streak < 2 && text_lines.len() > 1)
        {
            OFF_WHITE
        } else {
            DARK_GREY
        };
        let text_x_offset =
            he_indices[best_starting] as i32 * (COL_WIDTH + STROKE_WIDTH);
        for (j, text_line) in text_lines.iter().enumerate() {
            let text = Text::new(
                *text_line,
                (
                    text_x_offset + (2 * PADDING),
                    y_offset
                        + TOP_ROW_HEIGHT
                        + STROKE_WIDTH
                        + PADDING
                        + ((j + 1) as f32 * ROW_HEIGHT as f32
                            / (text_lines.len() + 1) as f32)
                            .round() as i32, // MASSIVE HACK
                ),
                TextStyle::from(FontDesc::new(
                    FontFamily::SansSerif,
                    FONT_SIZE / 2.0,
                    FontStyle::Normal,
                ))
                .color(&text_color)
                .pos(Pos::new(HPos::Left, VPos::Center)),
            );
            backend.draw(&text)?;
        }
    }

    Ok(())
}

fn draw_col<DB: DrawingBackend, S: AsRef<str>>(
    backend: &DrawingArea<DB, Shift>,
    i: i32,
    n_hes: i32,
    name: S,
    color: RGBColor,
    canvas_height: i32,
    sep: bool,
) -> Result<(), Box<dyn std::error::Error>>
where
    <DB as DrawingBackend>::ErrorType: 'static,
{
    let x_offset = i * (COL_WIDTH + STROKE_WIDTH);
    let y_offset = TOP_ROW_HEIGHT
        + STROKE_WIDTH
        + n_hes * (PADDING + ROW_HEIGHT)
        + PADDING;

    // Draw the coloured top & bottom rows.
    backend.draw(&Rectangle::new(
        [(x_offset, 0), (x_offset + COL_WIDTH, TOP_ROW_HEIGHT)],
        ShapeStyle::from(color).filled().stroke_width(0),
    ))?;
    backend.draw(&Rectangle::new(
        [
            (x_offset, y_offset + STROKE_WIDTH),
            (
                x_offset + COL_WIDTH,
                y_offset + STROKE_WIDTH + TOP_ROW_HEIGHT,
            ),
        ],
        ShapeStyle::from(color).filled().stroke_width(0),
    ))?;

    // Draw the name of the column.
    let text_color = if color::is_dark(color) {
        OFF_WHITE
    } else {
        DARK_GREY
    };
    let text = Text::new(
        name.as_ref(),
        (x_offset + (COL_WIDTH / 2) + 1, TOP_ROW_HEIGHT - PADDING),
        TextStyle::from(FontDesc::new(
            FontFamily::SansSerif,
            FONT_SIZE,
            FontStyle::Normal,
        ))
        .color(&text_color)
        .transform(FontTransform::Rotate90)
        .pos(Pos::new(HPos::Right, VPos::Center)),
    );
    backend.draw(&text)?;
    let text = Text::new(
        name.as_ref(),
        (
            x_offset + (COL_WIDTH / 2) + 1,
            y_offset + STROKE_WIDTH + PADDING,
        ),
        TextStyle::from(FontDesc::new(
            FontFamily::SansSerif,
            FONT_SIZE,
            FontStyle::Normal,
        ))
        .color(&text_color)
        .transform(FontTransform::Rotate90)
        .pos(Pos::new(HPos::Left, VPos::Center)),
    );
    backend.draw(&text)?;

    // Draw the solid white bottom border of the top row, and top border of the
    // bottom row.
    backend.draw(&Rectangle::new(
        [
            (x_offset, TOP_ROW_HEIGHT),
            (x_offset + COL_WIDTH, TOP_ROW_HEIGHT + STROKE_WIDTH),
        ],
        ShapeStyle::from(OFF_WHITE).filled().stroke_width(0),
    ))?;
    backend.draw(&Rectangle::new(
        [
            (x_offset, y_offset),
            (x_offset + COL_WIDTH, y_offset + STROKE_WIDTH),
        ],
        ShapeStyle::from(OFF_WHITE).filled().stroke_width(0),
    ))?;

    // Draw the dotted vertical line to separate the column from the column
    // immediately to its right.
    if sep {
        let n_dashes =
            (canvas_height - 2 * TOP_ROW_HEIGHT + 1) / DASHED_LINE_STRIDE;
        for i in 0..n_dashes {
            backend.draw(&Rectangle::new(
                [
                    (
                        x_offset + COL_WIDTH,
                        TOP_ROW_HEIGHT + i * DASHED_LINE_STRIDE,
                    ),
                    (
                        x_offset + COL_WIDTH + STROKE_WIDTH,
                        TOP_ROW_HEIGHT
                            + i * DASHED_LINE_STRIDE
                            + DASHED_LINE_STRIDE / 2,
                    ),
                ],
                ShapeStyle::from(OFF_WHITE).filled().stroke_width(0),
            ))?;
        }
    }

    Ok(())
}

fn handle_args() -> Result<(String, String), Box<dyn std::error::Error>> {
    static ERR_MSG: &str = "Expected exactly two arguments: \
the input RON file, and the output SVG file";

    let (mut a1, mut a2) = (String::new(), String::new());
    for (i, arg) in env::args().enumerate() {
        match i {
            0 => (),
            1 => a1 = arg,
            2 => a2 = arg,
            _ => Err(ERR_MSG)?,
        }
    }

    if a1.is_empty() || a2.is_empty() {
        Err(ERR_MSG)?
    }

    Ok((a1, a2))
}
