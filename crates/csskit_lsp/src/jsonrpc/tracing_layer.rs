use std::{
	collections::BTreeMap,
	sync::{Arc, RwLock},
};

use crossbeam_channel::Sender;
use lsp_types::{
	notification::{LogMessage, LogTrace, Notification as NotificationTrait},
	LogMessageParams, LogTraceParams, MessageType,
};
use serde::Serialize;
use serde_json::{to_string_pretty, to_value};
use tracing::{level_filters::LevelFilter, Event, Level, Subscriber};
use tracing_subscriber::{layer::Context, Layer};

use super::{Message, Notification};

pub struct TracingLayer {
	pub level: Arc<RwLock<LevelFilter>>,
	sender: Sender<Message>,
}

impl TracingLayer {
	pub fn new(level: Arc<RwLock<LevelFilter>>, sender: Sender<Message>) -> Self {
		Self { level, sender }
	}
}
// lsp_types::notification::LogTrace
// lsp_types::notification::LogMessage

#[derive(Default, Serialize)]
struct MessageVisitor(BTreeMap<String, String>);

impl MessageVisitor {
	pub fn insert(&mut self, str: &str, string: String) {
		self.0.insert(str.into(), string);
	}
}

impl tracing::field::Visit for MessageVisitor {
	fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
		self.insert(field.name(), format!("{:?}", value));
	}

	fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
		self.insert(field.name(), format!("{:?}", value));
	}

	fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
		self.insert(field.name(), format!("{:?}", value));
	}

	fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
		self.insert(field.name(), format!("{:?}", value));
	}

	fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
		self.insert(field.name(), format!("{:?}", value));
	}

	fn record_error(&mut self, field: &tracing::field::Field, value: &(dyn std::error::Error + 'static)) {
		self.insert(field.name(), format!("{:?}", value));
	}

	fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
		self.insert(field.name(), format!("{:?}", value));
	}
}

impl<S> Layer<S> for TracingLayer
where
	S: Subscriber,
{
	fn on_event(&self, event: &Event, _ctx: Context<'_, S>) {
		if event.metadata().level() > &(*self.level.read().unwrap()) {
			return;
		}
		let mut fields = MessageVisitor::default();
		fields.insert("name", event.metadata().name().into());
		fields.insert("level", event.metadata().level().as_str().into());
		event.record(&mut fields);
		let output = serde_json::json!(fields);

		let level = match event.metadata().level() {
			&Level::ERROR => MessageType::ERROR,
			&Level::WARN => MessageType::WARNING,
			&Level::INFO => MessageType::INFO,
			&Level::DEBUG | &Level::TRACE => {
				let message = Message::Notification(Notification {
					method: LogTrace::METHOD.into(),
					params: to_value(LogTraceParams {
						verbose: None,
						message: to_string_pretty(&output).ok().unwrap_or_default(),
					})
					.unwrap_or_default(),
				});
				self.sender.try_send(message).ok();
				return;
			}
		};

		let message = Message::Notification(Notification {
			method: LogMessage::METHOD.into(),
			params: to_value(LogMessageParams {
				typ: level,
				message: to_string_pretty(&output).ok().unwrap_or_default(),
			})
			.unwrap_or_default(),
		});

		self.sender.try_send(message).ok();
	}
}
