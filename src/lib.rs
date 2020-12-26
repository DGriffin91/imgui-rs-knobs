//TODO stepped increments
//TODO make something similar to drag control but with single click (doesn't need to support drag)

use imgui::*;
use std::f32::consts::PI;
pub fn bezier_arc(center: [f32; 2], start: [f32; 2], end: [f32; 2]) -> ([f32; 2], [f32; 2]) {
    let ax = start[0] - center[0];
    let ay = start[1] - center[1];
    let bx = end[0] - center[0];
    let by = end[1] - center[1];
    let q1 = ax * ax + ay * ay;
    let q2 = q1 + ax * bx + ay * by;
    let k2 = (4.0 / 3.0) * ((2.0 * q1 * q2).sqrt() - q2) / (ax * by - ay * bx);

    (
        [center[0] + ax - k2 * ay, center[1] + ay + k2 * ax],
        [center[0] + bx + k2 * by, center[1] + by - k2 * bx],
    )
}

pub fn draw_arc1(
    draw_list: &WindowDrawList,
    center: [f32; 2],
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    thickness: f32,
    color: [f32; 4],
    num_segments: u32,
) {
    let start = [
        center[0] + start_angle.cos() * radius,
        center[1] + start_angle.sin() * radius,
    ];

    let end = [
        center[0] + end_angle.cos() * radius,
        center[1] + end_angle.sin() * radius,
    ];

    let (c1, c2) = bezier_arc(center, start, end);

    draw_list
        .add_bezier_curve(start, c1, c2, end, color)
        .thickness(thickness)
        .num_segments(num_segments)
        .build();
}

pub fn draw_arc(
    draw_list: &WindowDrawList,
    center: [f32; 2],
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    thickness: f32,
    color: [f32; 4],
    num_segments: u32,
    bezier_count: u8,
) {
    //Overlap & angle of ends of bezier curves needs work, only looks good when not transperant
    let overlap = thickness * radius * 0.00001 * PI;
    let delta = end_angle - start_angle;
    let bez_step = 1.0 / bezier_count as f32;
    let mut mid_angle = start_angle + overlap;
    for _ in 1..bezier_count {
        let mid_angle2 = delta * bez_step + mid_angle;
        draw_arc1(
            draw_list,
            center,
            radius,
            mid_angle - overlap,
            mid_angle2 + overlap,
            thickness,
            color,
            num_segments,
        );
        mid_angle = mid_angle2;
    }
    draw_arc1(
        draw_list,
        center,
        radius,
        mid_angle - overlap,
        end_angle,
        thickness,
        color,
        num_segments,
    );
}

pub fn knob_control(
    ui: &Ui,
    id: &ImStr,
    p_value: &mut f32,
    v_min: f32,
    v_max: f32,
    v_default: f32,
    radius: f32,
) -> bool {
    ui.invisible_button(id, [radius * 2.0, radius * 2.0]);

    let mut value_changed = false;

    let is_active = ui.is_item_active();
    let delta = ui.mouse_drag_delta_with_threshold(MouseButton::Left, 0.0001);

    if ui.is_mouse_double_clicked(MouseButton::Left) && is_active {
        *p_value = v_default;
        value_changed = true;
    } else if is_active && delta[1] != 0.0 {
        let step = (v_max - v_min) / 200.0;
        *p_value -= delta[1] * step;
        if *p_value < v_min {
            *p_value = v_min;
        }
        if *p_value > v_max {
            *p_value = v_max;
        }
        value_changed = true;

        //There may be a way to do this without using this
        ui.reset_mouse_drag_delta(MouseButton::Left);
    }

    value_changed
}

pub struct ColorSet {
    pub base: [f32; 4],
    pub hovered: [f32; 4],
    pub active: [f32; 4],
}

impl ColorSet {
    pub fn new(base: [f32; 4], hovered: [f32; 4], active: [f32; 4]) -> ColorSet {
        ColorSet {
            base,
            hovered,
            active,
        }
    }
}

impl From<[f32; 4]> for ColorSet {
    fn from(color: [f32; 4]) -> ColorSet {
        ColorSet {
            base: color,
            hovered: color,
            active: color,
        }
    }
}

pub struct Knob<'a> {
    pub ui: &'a Ui<'a>,
    pub label: &'a ImStr,
    pub p_value: &'a mut f32,
    pub v_min: f32,
    pub v_max: f32,
    pub v_default: f32,
    pub radius: f32,
    pub screen_pos: [f32; 2],
    pub value_changed: bool,
    pub center: [f32; 2],
    pub draw_list: WindowDrawList<'a>,
    pub is_active: bool,
    pub is_hovered: bool,
    pub angle_min: f32,
    pub angle_max: f32,
    pub t: f32,
    pub angle: f32,
    pub angle_cos: f32,
    pub angle_sin: f32,
}

impl<'a> Knob<'a> {
    pub fn new(
        ui: &'a Ui,
        label: &'a ImStr,
        p_value: &'a mut f32,
        v_min: f32,
        v_max: f32,
        v_default: f32,
        radius: f32,
    ) -> Knob<'a> {
        let angle_min = PI * 0.75;
        let angle_max = PI * 2.25;
        let t = (*p_value - v_min) / (v_max - v_min);
        let angle = angle_min + (angle_max - angle_min) * t;
        let screen_pos = ui.cursor_screen_pos();
        let value_changed = knob_control(ui, label, p_value, v_min, v_max, v_default, radius);
        Knob {
            ui,
            label,
            p_value,
            v_min,
            v_max,
            v_default,
            radius,
            screen_pos,
            value_changed,
            center: [screen_pos[0] + radius, screen_pos[1] + radius],
            draw_list: ui.get_window_draw_list(),
            is_active: ui.is_item_active(),
            is_hovered: ui.is_item_hovered(),
            angle_min,
            angle_max,
            t,
            angle,
            angle_cos: angle.cos(),
            angle_sin: angle.sin(),
        }
    }

    pub fn draw_dot(
        &self,
        size: f32,
        radius: f32,
        angle: f32,
        color: &ColorSet,
        filled: bool,
        segments: u32,
    ) {
        let dot_size = size * self.radius;
        let dot_radius = radius * self.radius;
        self.draw_list
            .add_circle(
                [
                    self.center[0] + angle.cos() * dot_radius,
                    self.center[1] + angle.sin() * dot_radius,
                ],
                dot_size,
                if self.is_active {
                    color.active
                } else if self.is_hovered {
                    color.hovered
                } else {
                    color.base
                },
            )
            .filled(filled)
            .num_segments(segments)
            .build();
    }

    pub fn draw_tick(&self, start: f32, end: f32, width: f32, angle: f32, color: &ColorSet) {
        let tick_start = start * self.radius;
        let tick_end = end * self.radius;
        let angle_cos = angle.cos();
        let angle_sin = angle.sin();
        self.draw_list
            .add_line(
                [
                    self.center[0] + angle_cos * tick_end,
                    self.center[1] + angle_sin * tick_end,
                ],
                [
                    self.center[0] + angle_cos * tick_start,
                    self.center[1] + angle_sin * tick_start,
                ],
                if self.is_active {
                    color.active
                } else if self.is_hovered {
                    color.hovered
                } else {
                    color.base
                },
            )
            .thickness(width * self.radius)
            .build();
    }

    pub fn draw_circle(&self, size: f32, color: &ColorSet, filled: bool, segments: u32) {
        let circle_radius = size * self.radius;

        self.draw_list
            .add_circle(
                self.center,
                circle_radius,
                if self.is_active {
                    color.active
                } else if self.is_hovered {
                    color.hovered
                } else {
                    color.base
                },
            )
            .filled(filled)
            .num_segments(segments)
            .build();
    }

    pub fn draw_arc(
        &self,
        radius: f32,
        size: f32,
        start_angle: f32,
        end_angle: f32,
        color: &ColorSet,
        segments: u32,
        bezier_count: u8,
    ) {
        let track_radius = radius * self.radius;
        let track_size = size * self.radius * 0.5 + 0.0001;
        draw_arc(
            &self.draw_list,
            self.center,
            track_radius,
            start_angle,
            end_angle,
            track_size,
            if self.is_active {
                color.active
            } else if self.is_hovered {
                color.hovered
            } else {
                color.base
            },
            segments,
            bezier_count,
        );
    }
}

pub fn draw_wiper_knob(
    knob: &Knob,
    circle_color: &ColorSet,
    wiper_color: &ColorSet,
    track_color: &ColorSet,
) {
    knob.draw_circle(0.7, circle_color, true, 32);
    knob.draw_arc(
        0.8,
        0.41,
        knob.angle_min,
        knob.angle_max,
        track_color,
        16,
        2,
    );
    if knob.t > 0.01 {
        knob.draw_arc(0.8, 0.43, knob.angle_min, knob.angle, wiper_color, 16, 2);
    }
}

pub fn draw_wiper_only_knob(knob: &Knob, wiper_color: &ColorSet, track_color: &ColorSet) {
    knob.draw_arc(
        0.8,
        0.41,
        knob.angle_min,
        knob.angle_max,
        track_color,
        32,
        2,
    );
    if knob.t > 0.01 {
        knob.draw_arc(0.8, 0.43, knob.angle_min, knob.angle, wiper_color, 16, 2);
    }
}

pub fn draw_wiper_dot_knob(
    knob: &Knob,
    circle_color: &ColorSet,
    dot_color: &ColorSet,
    track_color: &ColorSet,
) {
    knob.draw_circle(0.6, circle_color, true, 32);
    knob.draw_arc(
        0.85,
        0.41,
        knob.angle_min,
        knob.angle_max,
        track_color,
        16,
        2,
    );
    knob.draw_dot(0.1, 0.85, knob.angle, dot_color, true, 12);
}

pub fn draw_tick_knob(knob: &Knob, circle_color: &ColorSet, tick_color: &ColorSet) {
    knob.draw_circle(0.7, circle_color, true, 32);
    knob.draw_tick(0.4, 0.7, 0.08, knob.angle, tick_color);
}

pub fn draw_dot_knob(knob: &Knob, circle_color: &ColorSet, dot_color: &ColorSet) {
    knob.draw_circle(0.85, circle_color, true, 32);
    knob.draw_dot(0.12, 0.6, knob.angle, dot_color, true, 12);
}

pub fn draw_space_knob(knob: &Knob, circle_color: &ColorSet, wiper_color: &ColorSet) {
    knob.draw_circle(0.3 - knob.t * 0.1, circle_color, true, 16);
    if knob.t > 0.01 {
        knob.draw_arc(
            0.4,
            0.15,
            knob.angle_min - 1.0,
            knob.angle - 1.0,
            wiper_color,
            16,
            2,
        );

        knob.draw_arc(
            0.6,
            0.15,
            knob.angle_min + 1.0,
            knob.angle + 1.0,
            wiper_color,
            16,
            2,
        );

        knob.draw_arc(
            0.8,
            0.15,
            knob.angle_min + 3.0,
            knob.angle + 3.0,
            wiper_color,
            16,
            2,
        );
    }
}

pub fn draw_stepped_knob(
    knob: &Knob,
    steps: u32,
    circle_color: &ColorSet,
    dot_color: &ColorSet,
    step_color: &ColorSet,
) {
    for n in 0..steps {
        let a = n as f32 / (steps - 1) as f32;
        let angle = knob.angle_min + (knob.angle_max - knob.angle_min) * a;
        knob.draw_tick(0.7, 0.9, 0.04, angle, step_color);
    }
    knob.draw_circle(0.6, circle_color, true, 32);
    knob.draw_dot(0.12, 0.4, knob.angle, dot_color, true, 12);
}

pub fn knob_title(ui: &Ui, label: &ImStr, width: f32) {
    let size = ui.calc_text_size(label, false, width);
    let old_cursor_pos = ui.cursor_pos();
    ui.set_cursor_pos([
        old_cursor_pos[0] + (width - size[0]) * 0.5,
        old_cursor_pos[1],
    ]);
    ui.text(label);

    ui.set_cursor_pos([old_cursor_pos[0], ui.cursor_pos()[1]]);
}

pub fn knob_with_drag<'a>(
    ui: &'a Ui,
    id: &'a ImStr,
    title: &ImStr,
    p_value: &'a mut f32,
    v_min: f32,
    v_max: f32,
    v_default: f32,
    format: &ImStr,
) -> Knob<'a> {
    let width = ui.text_line_height() * 4.0;
    let w = ui.push_item_width(width);

    knob_title(ui, title, width);

    let knob = Knob::new(ui, id, p_value, v_min, v_max, v_default, width * 0.5);

    Drag::new(&ImString::new(format!(
        "###{}_KNOB_DRAG_CONTORL_",
        id.to_str()
    )))
    .range(v_min..=v_max)
    .display_format(format)
    .speed((v_max - v_min) / 1000.0)
    .build(ui, knob.p_value);

    w.pop(ui);
    knob
}

pub fn hsv2rgb(hsva: [f32; 4]) -> [f32; 4] {
    let mut hsva = [
        hsva[0].max(0.0).min(1.0),
        hsva[1].max(0.0).min(1.0),
        hsva[2].max(0.0).min(1.0),
        hsva[3],
    ];

    if hsva[0] == 1.0 {
        hsva[0] = 0.0;
    }
    let h = hsva[0] * 6.0;
    let fract = h - h.floor();
    let p = hsva[2] * (1.0 - hsva[1]);
    let q = hsva[2] * (1.0 - (hsva[1] * fract));
    let t = hsva[2] * (1.0 - (hsva[1] * (1.0 - fract)));

    match h.floor() as i32 {
        0 => [hsva[2], t, p, hsva[3]],
        1 => [q, hsva[2], p, hsva[3]],
        2 => [p, hsva[2], t, hsva[3]],
        3 => [p, q, hsva[2], hsva[3]],
        4 => [t, p, hsva[2], hsva[3]],
        5 => [hsva[2], p, q, hsva[3]],
        _ => [0.0, 0.0, 0.0, hsva[3]],
    }
}
