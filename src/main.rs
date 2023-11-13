use std::fs::File;
use std::io::Write;
use tracing::Event;
use tracing_subscriber::{layer::{Layer, Context}, Registry};

struct CustomLogLayer;

impl<S> Layer<S> for CustomLogLayer
where
    S: tracing::Subscriber,
{
    fn on_event(&self, event: &Event, _: Context<'_, S>) {
        // 在这里根据逻辑条件判断，选择将日志写入不同的文件
        let user_id = extract_user_id_from_event(event);

        // 假设你有一个函数根据日志事件提取用户ID的逻辑
        match user_id {
            Some(id) => {
                let file_path = format!("logs/user_{}.log", id);
                if let Ok(mut file) = File::create(file_path) {
                    writeln!(file, "{:?}", event).ok();
                }
            }
            None => {
                // 处理没有用户ID的情况，可能是默认文件
                let file_path = "logs/default.log";
                if let Ok(mut file) = File::create(file_path) {
                    writeln!(file, "{:?}", event).ok();
                }
            }
        }
    }
}

// 这是一个示例函数，你需要根据你的日志结构提取用户ID
fn extract_user_id_from_event(_event: &Event<'_>) -> Option<u64> {
    // 实现提取用户ID的逻辑
    // 如果日志中包含用户ID，返回Some(id)，否则返回None
    // 这里只是一个示例，请根据实际情况修改
    Some(123)
}

fn main() {
    // 创建 Registry，并将 CustomLogLayer 注册进去
    let subscriber = Registry::default();

    // 创建 CustomLogLayer
    let custom_layer = CustomLogLayer.with_subscriber(subscriber);

    // 初始化 tracing
    tracing::subscriber::set_global_default(custom_layer).unwrap();

    // 这里开始记录日志
    tracing::info!("[user:123] This log message will be routed to a specific file based on logic");
}