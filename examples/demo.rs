use imgui::*;
use imgui_knobs::*;
mod support;

fn main() {
    let system = support::init(file!());
    let mut values: Vec<f32> = vec![0.0; 7];
    let min = -6.0;
    let max = 6.0;
    let default = 0.0;
    let format = im_str!("%.2fdB");

    system.main_loop(move |_, ui| {
        Window::new(im_str!("Knob Demo"))
            .size([900.0, 400.0], Condition::FirstUseEver)
            .position([20.0, 20.0], Condition::Appearing)
            .build(ui, || {
                ui.set_window_font_scale(1.0);
                let t = ui.time();

                let mut colors = Vec::new();
                let dark_gray1 = [0.15, 0.15, 0.15, 1.0];
                let dark_gray2 = [0.15, 0.15, 0.15, 1.0];
                let dark_gray3 = [0.15, 0.15, 0.15, 1.0];
                colors.push(ui.push_style_color(StyleColor::FrameBg, dark_gray1));
                colors.push(ui.push_style_color(StyleColor::FrameBgHovered, dark_gray2));
                colors.push(ui.push_style_color(StyleColor::FrameBgActive, dark_gray3));

                let h = (t as f32 * 0.2).sin().abs();
                //ui.text(&ImString::new(format!("t {}", h)));
                let s = (t as f32 * 0.1).sin().abs() * 0.5 + 0.4;

                let highlight = ColorSet::new(
                    hsv2rgb([h, s, 0.75, 1.0]),
                    hsv2rgb([h, s, 0.95, 1.0]),
                    hsv2rgb([h, s, 1.0, 1.0]),
                );
                let base = ColorSet::new(
                    hsv2rgb([h, s, 0.5, 1.0]),
                    hsv2rgb([h, s, 0.6, 1.0]),
                    hsv2rgb([h, s, 0.7, 1.0]),
                );
                let lowlight = ColorSet::from(hsv2rgb([h, s, 0.2, 1.0]));

                ui.columns(7, im_str!("cols"), false);

                draw_wiper_knob(
                    &knob_with_drag(
                        ui,
                        im_str!("Knob1"),
                        im_str!("Gain1"),
                        &mut values[0],
                        min,
                        max,
                        default,
                        format,
                    ),
                    &base,
                    &highlight,
                    &lowlight,
                );

                ui.next_column();

                draw_wiper_dot_knob(
                    &knob_with_drag(
                        ui,
                        im_str!("Knob2"),
                        im_str!("Gain2"),
                        &mut values[1],
                        min,
                        max,
                        default,
                        format,
                    ),
                    &base,
                    &highlight,
                    &lowlight,
                );

                ui.next_column();

                draw_wiper_only_knob(
                    &knob_with_drag(
                        ui,
                        im_str!("Knob3"),
                        im_str!("Gain3"),
                        &mut values[2],
                        min,
                        max,
                        default,
                        format,
                    ),
                    &base,
                    &lowlight,
                );

                ui.next_column();

                draw_tick_knob(
                    &knob_with_drag(
                        ui,
                        im_str!("Knob4"),
                        im_str!("Gain4"),
                        &mut values[3],
                        min,
                        max,
                        default,
                        format,
                    ),
                    &base,
                    &highlight,
                );

                ui.next_column();

                draw_dot_knob(
                    &knob_with_drag(
                        ui,
                        im_str!("Knob5"),
                        im_str!("Gain5"),
                        &mut values[4],
                        min,
                        max,
                        default,
                        format,
                    ),
                    &base,
                    &highlight,
                );

                ui.next_column();

                draw_space_knob(
                    &knob_with_drag(
                        ui,
                        im_str!("Knob6"),
                        im_str!("Gain6"),
                        &mut values[5],
                        min,
                        max,
                        default,
                        format,
                    ),
                    &base,
                    &highlight,
                );

                ui.next_column();

                draw_stepped_knob(
                    &knob_with_drag(
                        ui,
                        im_str!("Knob7"),
                        im_str!("Gain7"),
                        &mut values[6],
                        min,
                        max,
                        default,
                        format,
                    ),
                    7,
                    &base,
                    &highlight,
                    &base,
                );

                ui.next_column();

                colors.into_iter().for_each(|color| color.pop(ui));
            });
    });
}
