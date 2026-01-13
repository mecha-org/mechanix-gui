use gpui::*;

// This is of the size 540 x 107
fn left_wedge(content: impl IntoElement) -> impl IntoElement {
    div()
        .relative()
        .child(
            img("icons/lockscreen/wedge_right.svg")
                .absolute()
                .inset_0()
                .size_full()
                .object_fit(ObjectFit::Fill),
        )
        .child(
            div()
                .relative()
                .flex()
                .items_center()
                .justify_center()
                .size_full()
                .child(content),
        )
}

// This has a size 540 x 67
fn right_wedge(content: impl IntoElement) -> impl IntoElement {
    div()
        .relative()
        .child(
            img("icons/lockscreen/wedge_left.svg")
                .absolute()
                .inset_0()
                .size_full()
                .object_fit(ObjectFit::Fill),
        )
        .child(
            div()
                .relative()
                .flex()
                .items_center()
                .justify_center()
                .size_full()
                .child(content),
        )
}
