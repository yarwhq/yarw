#[test]
fn test_create_dirs_all_vec() {
    use crate::fs::create_dirs_all_vec;

    let dirs = vec![
        std::path::Path::new("dir1"),
        std::path::Path::new("dir2"),
        std::path::Path::new("dir3"),
    ];
    
    create_dirs_all_vec(dirs.clone()).unwrap();
    
    for dir in &dirs {
        assert!(dir.exists());
    }

    for dir in &dirs {
        std::fs::remove_dir(dir).unwrap();
    }
}
