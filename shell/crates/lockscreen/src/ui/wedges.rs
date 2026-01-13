use gpui::*;

// Left wedge dimensions: 540 x 106 (from wedge_left.svg viewBox)
pub const LEFT_WEDGE_WIDTH: f32 = 540.0;
pub const LEFT_WEDGE_HEIGHT: f32 = 106.0;

// Right wedge dimensions: 540 x 67 (from wedge_right.svg viewBox)
pub const RIGHT_WEDGE_WIDTH: f32 = 540.0;
pub const RIGHT_WEDGE_HEIGHT: f32 = 67.0;

// Left wedge - size 540 x 106, color #382000 (swapped)
pub fn left_wedge(content: impl IntoElement) -> impl IntoElement {
    let svg_color = rgba(0x382000FF);

    div()
        .absolute()
        .bottom_0()
        .left_0()
        .w(px(LEFT_WEDGE_WIDTH))
        .h(px(LEFT_WEDGE_HEIGHT))
        .child(
            svg()
                .path("icons/lockscreen/wedge_left.svg")
                .absolute()
                .inset_0()
                .w(px(LEFT_WEDGE_WIDTH))
                .h(px(LEFT_WEDGE_HEIGHT))
                .text_color(svg_color),
        )
        .child(
            div()
                .absolute()
                .inset_0()
                .flex()
                .items_center()
                .justify_center()
                .child(content),
        )
}

// Right wedge - size 540 x 67, color #1F1200 (swapped)
pub fn right_wedge(content: impl IntoElement) -> impl IntoElement {
    let svg_color = rgba(0x1F1200FF);

    div()
        .absolute()
        .bottom_0()
        .right_0()
        .w(px(RIGHT_WEDGE_WIDTH))
        .h(px(RIGHT_WEDGE_HEIGHT))
        .child(
            svg()
                .path("icons/lockscreen/wedge_right.svg")
                .absolute()
                .inset_0()
                .w(px(RIGHT_WEDGE_WIDTH))
                .h(px(RIGHT_WEDGE_HEIGHT))
                .text_color(svg_color),
        )
        .child(
            div()
                .absolute()
                .inset_0()
                .flex()
                .items_center()
                .justify_center()
                .child(content),
        )
}
