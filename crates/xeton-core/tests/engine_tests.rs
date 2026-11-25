use std::path::PathBuf;
use xeton_core::manager::DownloadManager;
use xeton_core::models::{
    DownloadProtocol, DownloadSettings, DownloadStatus, DuplicateStrategy, NewDownloadProps,
    RangedPart,
};

fn unique_temp_dir(label: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let dir = std::env::temp_dir().join(format!(
        "xeton_test_{}_{}_{}",
        label,
        std::process::id(),
        nanos
    ));
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

#[test]
fn ranged_part_arithmetic_invariants() {
    let mut part = RangedPart::new(0, Some(99), 0);
    assert_eq!(part.downloaded(), 0);
    assert_eq!(part.remaining(), Some(100));
    assert!(!part.is_completed());
    assert!(!part.is_blind());

    part.current = 50;
    assert_eq!(part.downloaded(), 50);
    assert_eq!(part.remaining(), Some(50));
    assert!(!part.is_completed());

    part.current = 100;
    assert_eq!(part.downloaded(), 100);
    assert_eq!(part.remaining(), Some(0));
    assert!(part.is_completed());

    let blind = RangedPart::new(0, None, 1024);
    assert!(blind.is_blind());
    assert!(!blind.is_completed());
    assert_eq!(blind.remaining(), None);
}

#[tokio::test]
async fn download_manager_add_list_pause_lifecycle() {
    let data_dir = unique_temp_dir("mgr_lifecycle");
    let out_dir = unique_temp_dir("mgr_out");

    let manager = DownloadManager::new(data_dir.clone(), DownloadSettings::default())
        .await
        .expect("init DownloadManager");

    let props = NewDownloadProps {
        link: "https://example.com/archive.tar.gz".to_string(),
        folder: out_dir.to_string_lossy().to_string(),
        name: "archive.tar.gz".to_string(),
        protocol: DownloadProtocol::Http,
        preferred_connections: Some(4),
        speed_limit: 0,
        on_duplicate: DuplicateStrategy::AddNumbered,
    };

    let id = manager.add_download(props).await.expect("add_download");
    assert!(id > 0);

    let list = manager
        .get_download_list()
        .await
        .expect("get_download_list");
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].name, "archive.tar.gz");
    assert_eq!(list[0].status, DownloadStatus::Added);

    manager.pause(id).await.expect("pause");

    let _ = std::fs::remove_dir_all(&data_dir);
    let _ = std::fs::remove_dir_all(&out_dir);
}

#[tokio::test]
async fn http_end_to_end_download_verifies_bytes() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .try_init();
    use axum::{
        Router,
        body::Body,
        http::{StatusCode, header},
        response::Response,
        routing::get,
    };

    let payload: Vec<u8> = (0..65_536u32).map(|i| (i % 251) as u8).collect();
    let expected = payload.clone();

    let app = Router::new().route(
        "/payload.bin",
        get(move |headers: axum::http::HeaderMap| {
            let bytes = payload.clone();
            async move {
                let total = bytes.len();
                if let Some(range_hdr) = headers.get(header::RANGE).and_then(|v| v.to_str().ok()) {
                    if let Some(spec) = range_hdr.strip_prefix("bytes=") {
                        let mut parts = spec.split('-');
                        let start: usize = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
                        let end: usize = parts
                            .next()
                            .and_then(|s| if s.is_empty() { None } else { s.parse().ok() })
                            .unwrap_or(total - 1)
                            .min(total - 1);
                        let slice = bytes[start..=end].to_vec();
                        return Response::builder()
                            .status(StatusCode::PARTIAL_CONTENT)
                            .header(header::CONTENT_LENGTH, slice.len().to_string())
                            .header(
                                header::CONTENT_RANGE,
                                format!("bytes {}-{}/{}", start, end, total),
                            )
                            .header(header::ACCEPT_RANGES, "bytes")
                            .body(Body::from(slice))
                            .unwrap();
                    }
                }
                Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_LENGTH, total.to_string())
                    .header(header::ACCEPT_RANGES, "bytes")
                    .body(Body::from(bytes))
                    .unwrap()
            }
        }),
    );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind local mock server");
    let addr = listener.local_addr().expect("local_addr");
    tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });

    let data_dir = unique_temp_dir("http_e2e_data");
    let out_dir = unique_temp_dir("http_e2e_out");

    let manager = DownloadManager::new(data_dir.clone(), DownloadSettings::default())
        .await
        .expect("init DownloadManager");

    let id = manager
        .add_download(NewDownloadProps {
            link: format!("http://{}/payload.bin", addr),
            folder: out_dir.to_string_lossy().to_string(),
            name: "payload.bin".to_string(),
            protocol: DownloadProtocol::Http,
            preferred_connections: Some(2),
            speed_limit: 0,
            on_duplicate: DuplicateStrategy::Override,
        })
        .await
        .expect("add_download");

    let resume_res = manager.resume(id).await;
    assert!(resume_res.is_ok(), "resume failed: {:?}", resume_res);

    let dest_file = out_dir.join("payload.bin");
    let mut completed = false;
    let mut last_status = None;
    for _ in 0..50 {
        if let Ok(list) = manager.get_download_list().await {
            if let Some(item) = list.into_iter().find(|i| i.numeric_id == id) {
                last_status = Some(item.status.clone());
                if item.status == DownloadStatus::Completed {
                    completed = true;
                    break;
                }
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    assert!(
        completed,
        "HTTP download did not reach Completed within 5s (last_status={:?})",
        last_status
    );
    let actual = std::fs::read(&dest_file).expect("read downloaded file");
    assert_eq!(
        actual.len(),
        expected.len(),
        "downloaded byte length mismatch"
    );
    assert_eq!(actual, expected, "downloaded file bytes corrupted");

    let _ = std::fs::remove_dir_all(&data_dir);
    let _ = std::fs::remove_dir_all(&out_dir);
}
