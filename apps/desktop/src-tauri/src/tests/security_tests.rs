use crate::utils::security::SecurityManager;
use tempfile::tempdir;

#[test]
fn test_security_init_and_unlock() {
    let dir = tempdir().unwrap();
    let security = SecurityManager::new(dir.path().to_path_buf());

    assert!(!security.is_configured());

    let password = "strong_password";
    security.init(password).unwrap();

    assert!(security.is_configured());
    assert!(security.get_master_key().is_some());

    // Lock simulation (new instance)
    let security2 = SecurityManager::new(dir.path().to_path_buf());
    assert!(security2.get_master_key().is_none());

    let unlocked = security2.unlock(password).unwrap();
    assert!(unlocked);
    assert!(security2.get_master_key().is_some());

    let failed = security2.unlock("wrong").unwrap();
    assert!(!failed); // Should return false, not panic
}
