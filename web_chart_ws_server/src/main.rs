use std::sync::Arc;
use tokio::sync::{Mutex, mpsc };
use warp::Filter;
use warp::ws::{ Message, WebSocket };
use futures_util::StreamExt;
use futures_util::sink::SinkExt;
use rand::Rng;
use log::{info, warn, error, debug};

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    // 创建用于WebSocket endpoint的路由
    let websocket_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::any().map(move || Arc::new(Mutex::new(()))))
        .map(|ws: warp::ws::Ws, _state| {
            // 使用handle_socket函数来处理
            ws.on_upgrade(move |socket| handle_socket(socket))
        });

    // 开启Warp服务
    info!("Warp starting at ws://127.0.0.1:8000/ws");
    warp::serve(websocket_route)
        .run(([127, 0, 0, 1], 8000))
        .await;
}

async fn handle_socket(socket: WebSocket) {
    // 对socket变量新建原子引用计数和互斥锁(保证线程安全)
    let socket = Arc::new(Mutex::new(socket));

    // 新建mpsc通道
    let (tx, mut rx) = mpsc::channel::<Message>(32);

    // Spawn一个发送消息的异步任务
    let send_socket = Arc::clone(&socket);
    tokio::spawn(async move {
        // 如果mpsc通道中有消息则取出消息并发送
        while let Some(msg) = rx.recv().await {
            if let Ok(text) = msg.to_str() {
                debug!("Sending message: {}", text);
            } else {
                warn!("Sending binary message or invalid UTF-8");
            }
            let mut send_socket = send_socket.lock().await;
            if send_socket.send(msg).await.is_err() {
                error!("Error while sending message...");
                break;
            }
        }
    });

    // Spawn一个接收消息的异步任务
    let recv_socket = Arc::clone(&socket);
    let receiver_tx = tx.clone();
    tokio::spawn(async move {
        loop {
            let mut recv_socket = recv_socket.lock().await;
            // 开始接收消息，如果超过100毫秒无消息则释放锁
            tokio::select! {
                result = recv_socket.next() => {
                    if let Some(Ok(msg)) = result {
                        if let Ok(text) = msg.to_str() {
                            debug!("Received message: {}", text);
                        } else {
                            warn!("Received message is binary or including invalid UTF-8");
                        }
                        // 发送回复信息至mpsc通道等待发送
                        if receiver_tx.send(msg).await.is_err() {
                            error!("Error forwarding received message to mpsc channel...");
                            break;
                        }
                    }
                }
                _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {}
            }
        }
    });

    // Spawn一个定时生成随机数的异步任务
    let countdown_tx = tx.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
        loop {
            interval.tick().await;
            let random_number: u8 = rand::thread_rng().gen_range(0..=100);
            let msg = Message::text(random_number.to_string());
            // 发送随机数信息至mpsc通道等待发送
            if countdown_tx.send(msg).await.is_err() {
                error!("Error sending random number to mpsc channel...");
                break;
            }
        }
    });
}
