use std::fs::File;
use std::io::Write;
use tracing::{Event, field::{Field, Visit}};
use tracing_subscriber::{layer::Context, Registry, Layer};

struct CustomLogLayer;

impl<S> tracing_subscriber::Layer<S> for CustomLogLayer
where
    S: tracing::Subscriber,
{
    fn on_event(&self, event: &Event<'_>, _: Context<'_, S>) {
        let mut visitor = CustomVisitor{ user_id: None };
        event.record(&mut visitor);

        let user_id = visitor.user_id;

        // Your existing logic to write logs to different files based on user_id...
        // ...

        // For example, let's print the user_id to the console for demonstration:
        if let Some(id) = user_id {
            println!("User ID: {}", id);
            let file_path = format!("logs/user_{}.log", id);
            if let Ok(mut file) = File::create(file_path) {
                writeln!(file, "{:?}", event).ok();
            }
        } else {
            println!("No user ID found in the log event");
        }
    }
}

struct CustomVisitor {
    user_id: Option<i64>,
}

impl Visit for CustomVisitor {
    fn record_i64(&mut self, field: &Field, value: i64) {
        if field.name() == "user_id" {
            self.user_id = Some(value);
        }
    }

    fn record_debug(&mut self, _field: &Field, _value: &dyn std::fmt::Debug) {
        println!("record_debug");
    }
}

fn main() {
    // 创建 Registry，并将 CustomLogLayer 注册进去
    let subscriber = Registry::default();

    // 创建 CustomLogLayer
    let custom_layer = CustomLogLayer.with_subscriber(subscriber);

    // 初始化 tracing
    tracing::subscriber::set_global_default(custom_layer).unwrap();
    // let a = 123u64;

    // Example log event with a user_id field
    tracing::info!(user_id = 456, "This log message contains a user ID");
}