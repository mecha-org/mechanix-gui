use dispatcher::Dispatcher;
use gpui::*;
use icons::prelude::Icons;
use settings::prelude::{Settings, ToastSettings};
use std::time::Duration;
use theme::{ActiveFonts, ActiveTheme};

// Toast dimensions
const TOAST_WIDTH: f32 = 358.0;
const TOAST_HEIGHT: f32 = 48.0;
const TOAST_INNER_WIDTH: f32 = 330.0;
const TOAST_INNER_HEIGHT: f32 = 20.0;
const TOAST_ICON_SIZE: f32 = 20.0;
const TOAST_CLOSE_SIZE: f32 = 10.0;
const TOAST_BORDER_RADIUS: f32 = 10.0;
const TOAST_GAP: f32 = 8.0;
const TOAST_TOP_PADDING: f32 = 20.0;
const TOAST_TEXT_SIZE: f32 = 16.0;

/// General toast UI component that shows a message with an icon.
pub struct Toast {
	message: Option<String>,
	icon_path: Option<String>,
	visible: bool,
	dismiss_task: Option<Task<()>>,
	settings: ToastSettings,
}

impl Toast {
	pub fn new(cx: &mut Context<Self>) -> Self {
		let settings = Settings::global(cx).toast.clone();

		Self {
			message: None,
			icon_path: None,
			visible: false,
			dismiss_task: None,
			settings,
		}
	}

	/// Show the toast with a message and icon, auto-dismisses after timeout
	pub fn show(&mut self, message: String, icon_path: String, cx: &mut Context<Self>) {
		// Skip if same message is already showing
		if self.visible && self.message.as_ref() == Some(&message) {
			return;
		}

		self.message = Some(message);
		self.icon_path = Some(icon_path);
		self.visible = true;
		self.dismiss_task.take();

		let timeout_ms = self.settings.timeout_ms;
		self.dismiss_task = Some(cx.spawn(async move |this, cx| {
			cx.background_executor()
				.timer(Duration::from_millis(timeout_ms))
				.await;
			let _ = this.update(cx, |this, cx| {
				this.hide(cx);
			});
		}));

		cx.notify();
	}

	/// Hide the toast immediately
	pub fn hide(&mut self, cx: &mut Context<Self>) {
		self.visible = false;
		self.dismiss_task.take();
		cx.notify();
	}

	/// Clear the toast completely
	pub fn clear(&mut self, cx: &mut Context<Self>) {
		self.message = None;
		self.icon_path = None;
		self.visible = false;
		self.dismiss_task.take();
		cx.notify();
	}

	/// Check if the toast is currently visible
	pub fn is_visible(&self) -> bool {
		self.visible
	}

	/// Get the current message
	pub fn current_message(&self) -> Option<&String> {
		self.message.as_ref()
	}
}

impl Render for Toast {
	fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
		if !self.visible {
			return div().into_any_element();
		}

		let Some(message) = self.message.clone() else {
			return div().into_any_element();
		};

		let Some(icon_path) = self.icon_path.clone() else {
			return div().into_any_element();
		};

		let colors = cx.theme().colors.clone();
		let primary_font = cx.fonts().primary.clone();

		let icon_close = Icons::global(cx)
			.toast
			.close
			.to_string_lossy()
			.to_string();

		div()
			.size_full()
			.flex()
			.justify_center()
			.pt(px(TOAST_TOP_PADDING))
			.child(
				div()
					.w(px(TOAST_WIDTH))
					.h(px(TOAST_HEIGHT))
					.flex()
					.items_center()
					.justify_center()
					.bg(colors.background_900)
					.border_1()
					.border_color(colors.background_700)
					.rounded(px(TOAST_BORDER_RADIUS))
					.shadow_md()
					// Inner content box
					.child(
						div()
							.w(px(TOAST_INNER_WIDTH))
							.h(px(TOAST_INNER_HEIGHT))
							.flex()
							.items_center()
							.gap(px(TOAST_GAP))
							// Icon
							.child(
								img(icon_path)
									.w(px(TOAST_ICON_SIZE))
									.h(px(TOAST_ICON_SIZE))
							)
							// Message text
							.child(
								div()
									.flex_1()
									.text_size(px(TOAST_TEXT_SIZE))
									.font_family(primary_font)
									.text_color(colors.foreground_100)
									.child(message),
							)
							// Close button
							.child(
								div()
									.w(px(TOAST_CLOSE_SIZE))
									.h(px(TOAST_CLOSE_SIZE))
									.flex()
									.items_center()
									.justify_center()
									.cursor_pointer()
									.on_mouse_down(MouseButton::Left, cx.listener(|this: &mut Toast, _, _, cx| {
										this.hide(cx);
									}))
									.child(
										img(icon_close.clone())
											.w(px(TOAST_CLOSE_SIZE))
											.h(px(TOAST_CLOSE_SIZE))
									),
							),
					),
			)
			.into_any_element()
	}
}

/// Listen for extension events and show toast notifications
pub fn listen_for_extensions(cx: &mut App, toast_entity: Entity<Toast>) {
	if !cx.has_global::<Dispatcher>() {
		return;
	}

	let icons = Icons::global(cx).toast.clone();

	let icon_attached = icons.extension_attached.to_string_lossy().to_string();
	let icon_detached = icons.extension_detached.to_string_lossy().to_string();

	let mut dispatcher_rx = Dispatcher::global(cx).channel().1.clone();

	// Track extension state as a tuple (name, detected) for robust state management
	let mut current_extension: Option<(String, bool)> = None;

	cx.spawn(async move |cx| {
		while let Ok(message) = dispatcher_rx.recv().await {
			match message {
				dispatcher::Message::SetExtensionName(name) => {
					// When extension name changes, check if it's a new extension
					let is_new_extension = current_extension
						.as_ref()
						.map(|(old_name, _)| old_name != &name)
						.unwrap_or(true);

					if is_new_extension {
						// Reset state for new extension - preserve name but clear detected state
						current_extension = Some((name, false));
					} else if let Some((_, detected)) = current_extension {
						// Same extension, keep the detected state
						current_extension = Some((name, detected));
					}
				}
				dispatcher::Message::SetExtensionDetected(detected) => {
					let Some((ref name, last_detected)) = current_extension else {
						// No extension name set yet, ignore
						continue;
					};

					// Skip if same state for the same extension
					if last_detected == detected {
						continue;
					}

					// Update state
					let ext_name = name.clone();
					current_extension = Some((ext_name.clone(), detected));

					// Clear existing toast before showing new one to prevent stale toasts
					let _ = cx.update(|cx| {
						let _ = toast_entity.update(cx, |toast, cx| {
							toast.clear(cx);
						});
					});

					// Small delay to ensure clear takes effect
					cx.background_executor()
						.timer(Duration::from_millis(50))
						.await;

					if detected {
						let toast_message = format!("{} was attached", ext_name);
						let icon = icon_attached.clone();
						let _ = cx.update(|cx| {
							let _ = toast_entity.update(cx, |toast, cx| {
								toast.show(toast_message, icon, cx);
							});
						});
					} else {
						let toast_message = format!("{} was detached", ext_name);
						let icon = icon_detached.clone();
						let _ = cx.update(|cx| {
							let _ = toast_entity.update(cx, |toast, cx| {
								toast.show(toast_message, icon, cx);
							});
						});
					}
				}
				_ => {}
			}
		}
	})
	.detach();
}
