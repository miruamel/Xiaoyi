use xiaoyi::AsyncConfigSource;

#[test]
fn test_debug_file_source() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("test.toml");
    std::fs::write(&file_path, r#"
        [server]
        port = 8080
        host = "localhost"
        [database]
        url = "postgres://..."
    "#).unwrap();

    let source = xiaoyi::FileSource::new(file_path.to_str().unwrap());
    let data = tokio_test::block_on(source.load()).unwrap();
    println!("data = {:?}", data);
    
    let port = data.get("server.port").and_then(|v| v.as_i64());
    println!("port = {:?}", port);
}
