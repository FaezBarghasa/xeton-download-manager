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
