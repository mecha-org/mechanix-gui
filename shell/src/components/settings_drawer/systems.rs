pub fn control_click_system(control_name: String) -> impl Fn() + 'static {
    move || {
        println!("{:?} clicked!", control_name);
    }
}
