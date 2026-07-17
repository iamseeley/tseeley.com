use crate::{Config, build};
use axum::{
    Router,
    extract::State,
    http::{StatusCode, Uri, header},
    response::{
        IntoResponse, Response,
        sse::{Event, KeepAlive, Sse},
    },
    routing::get,
};
use notify::{Config as NotifyConfig, RecommendedWatcher, RecursiveMode, Watcher};
use std::convert::Infallible;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::sync::{Arc, mpsc};
use std::time::Duration;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::{Stream, StreamExt};

#[derive(Clone)]
struct AppState {
    reload_tx: broadcast::Sender<()>,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    build::run(&config)?;

    let config = Arc::new(config);
    let (reload_tx, _) = broadcast::channel::<()>(16);
    spawn_watcher(Arc::clone(&config), reload_tx.clone());

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    let dev_addr = config.dev_addr.clone();
    rt.block_on(async move {
        let state = AppState { reload_tx };
        let app = Router::new()
            .route("/__live", get(live_handler))
            .fallback(get(serve_file))
            .with_state(state);

        let listener = tokio::net::TcpListener::bind(&dev_addr).await?;
        println!("Serving http://{dev_addr}");
        axum::serve(listener, app).await?;
        Ok::<(), Box<dyn Error>>(())
    })?;

    Ok(())
}

fn spawn_watcher(config: Arc<Config>, reload_tx: broadcast::Sender<()>) {
    std::thread::spawn(move || {
        let (tx, rx) = mpsc::channel();
        let mut watcher = match RecommendedWatcher::new(tx, NotifyConfig::default()) {
            Ok(w) => w,
            Err(e) => {
                eprintln!("watcher: {e}");
                return;
            }
        };

        for (path, mode) in [
            ("content", RecursiveMode::Recursive),
            ("static", RecursiveMode::Recursive),
            ("config.toml", RecursiveMode::NonRecursive),
        ] {
            if let Err(e) = watcher.watch(Path::new(path), mode) {
                eprintln!("watching {path}: {e}");
            }
        }

        while let Ok(first) = rx.recv() {
            let mut paths: Vec<PathBuf> = first.ok().map(|e| e.paths).unwrap_or_default();
            while let Ok(next) = rx.recv_timeout(Duration::from_millis(150)) {
                if let Ok(e) = next {
                    paths.extend(e.paths);
                }
            }

            let kind = classify(&paths);
            let result = match kind {
                ChangeKind::StaticOnly => build::run_static_only(),
                ChangeKind::Full => build::run(&config),
            };
            let label = match kind {
                ChangeKind::StaticOnly => "static",
                ChangeKind::Full => "full",
            };
            match result {
                Ok(()) => {
                    println!("Rebuilt ({label})");
                    let _ = reload_tx.send(());
                }
                Err(e) => eprintln!("Build error: {e}"),
            }
        }
    });
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ChangeKind {
    StaticOnly,
    Full,
}

fn classify(paths: &[PathBuf]) -> ChangeKind {
    let touches_css = paths
        .iter()
        .any(|p| p.extension().is_some_and(|e| e == "css"));
    if touches_css {
        return ChangeKind::Full;
    }
    let only_static = !paths.is_empty()
        && paths
            .iter()
            .all(|p| p.components().any(|c| c.as_os_str() == "static"));
    if only_static {
        ChangeKind::StaticOnly
    } else {
        ChangeKind::Full
    }
}

async fn live_handler(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.reload_tx.subscribe();
    let s = BroadcastStream::new(rx).filter_map(|r| {
        r.ok()
            .map(|_| Ok::<_, Infallible>(Event::default().data("reload")))
    });
    Sse::new(s).keep_alive(KeepAlive::new().interval(Duration::from_secs(30)))
}

async fn serve_file(uri: Uri) -> Response {
    let mut path = PathBuf::from("public");
    path.push(uri.path().trim_start_matches('/'));
    if path.is_dir() {
        path.push("index.html");
    }

    match tokio::fs::read(&path).await {
        Ok(bytes) => {
            let ct = content_type(&path);
            ([(header::CONTENT_TYPE, ct)], bytes).into_response()
        }
        Err(_) => match tokio::fs::read("public/404.html").await {
            Ok(bytes) => (
                StatusCode::NOT_FOUND,
                [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
                bytes,
            )
                .into_response(),
            Err(_) => (StatusCode::NOT_FOUND, "404").into_response(),
        },
    }
}

fn content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "application/javascript",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("webp") => "image/webp",
        Some("ico") => "image/x-icon",
        Some("woff2") => "font/woff2",
        Some("xml") => "application/xml",
        _ => "application/octet-stream",
    }
}
