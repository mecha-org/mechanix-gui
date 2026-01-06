use gpui::*;

struct DispatcherExample;

impl DispatcherExample {
    pub fn new(cx: &mut Context<Self>) -> Self {
        //Start listening
        let sender = dispatcher::Dispatcher::global(cx).0.clone();
        let receiver = dispatcher::Dispatcher::global(cx).1.clone();

        for i in 0..2 {
            cx.spawn({
                let mut receiver = receiver.clone();
                async move |_this, _cx| {
                    while let Ok(message) = receiver.recv().await {
                        println!("Received message in thread {}: {:?}", i, message);
                    }
                }
            })
            .detach();
        }

        //Send message
        let executor = cx.background_executor().clone();
        cx.background_executor()
            .spawn(async move {
                println!("sending message ");
                executor.timer(std::time::Duration::from_secs(3)).await;
                _ = sender
                    .broadcast(dispatcher::Message::ShowPowerOptions(true))
                    .await;
            })
            .detach();

        Self {}
    }
}

impl Render for DispatcherExample {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(gpui::black())
            .flex()
            .items_center()
            .justify_center()
            .text_color(gpui::white())
            .child("Check logs")
    }
}

fn main() {
    let application = Application::new();
    application.run(|cx| {
        dispatcher::init(cx);

        let _ = cx.open_window(WindowOptions::default(), |_window, cx| {
            cx.new(|cx| DispatcherExample::new(cx))
        });

        cx.activate(true);
    });
}
