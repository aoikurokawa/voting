use std::thread::sleep;

use chrono::Utc;

use crate::TestSetup;

#[test]
fn test_create_governance() {
    let setup = TestSetup::new();

    let name = "superteam";

    // Success pattern
    let success_res = setup.create_governance(name);
    assert!(success_res.is_ok());

    // Fail pattern (Same name)
    let fail_res = setup.create_governance(name);
    assert!(fail_res.is_err());
}

#[test]
fn test_join() {
    let setup = TestSetup::new();
    let name = "superteam1";

    let _ = setup.create_governance(name);

    // Success pattern
    let success_res = setup.join(name);
    assert!(success_res.is_ok());

    // Fail pattern (already joined)
    let fail_res = setup.join(name);
    assert!(fail_res.is_err());
}

#[test]
fn test_create_proposal() {
    let setup = TestSetup::new();
    let name = "superteam2";
    let title = "Hello World";

    let _ = setup.create_governance(name);
    let _ = setup.join(name);

    // Success pattern
    let success_res = setup.create_proposal(name, title);
    assert!(success_res.is_ok());

    // Fail pattern (Already proposed)
    let fail_res = setup.create_proposal(name, title);
    assert!(fail_res.is_err());
}

#[test]
fn test_start_vote() {
    let setup = TestSetup::new();
    let name = "superteam3";
    let title = "Hello World1";
    let end = Utc::now() + chrono::Duration::days(1);

    let _ = setup.create_governance(name);
    let _ = setup.join(name);
    let _ = setup.create_proposal(name, title);

    // Success pattern
    let success_res = setup.start_vote(name, title, end.timestamp());
    assert!(success_res.is_ok());

    // Fail pattern (Does not exist the governance)
    let fail_res = setup.start_vote("fake name", title, end.timestamp());
    assert!(fail_res.is_err());
}

#[test]
fn test_commit_vote() {
    let setup = TestSetup::new();
    let name = "superteam4";
    let title = "Hello World2";
    let end = Utc::now() + chrono::Duration::days(1);
    let vote = 1; // Yes
    let salt = "salt";

    let _ = setup.create_governance(name);
    let _ = setup.join(name);
    let _ = setup.create_proposal(name, title);
    let _ = setup.start_vote(name, title, end.timestamp());

    // Success pattern
    let success_res = setup.commit_vote(name, title, vote, salt);
    assert!(success_res.is_ok());

    // Fail pattern (Already committed)
    let fail_res = setup.commit_vote(name, title, vote, salt);
    assert!(fail_res.is_err());
}

#[test]
fn test_reveal_vote() {
    let setup = TestSetup::new();
    let name = "superteam5";
    let title = "Hello World3";
    let end = Utc::now().timestamp() + 1;
    let vote = 1; // Yes
    let salt = "salt";

    let _ = setup.create_governance(name);
    let _ = setup.join(name);
    let _ = setup.create_proposal(name, title);
    let _ = setup.start_vote(name, title, end);
    let _ = setup.commit_vote(name, title, vote, salt);

    sleep(std::time::Duration::new(1, 0));

    // Success pattern
    let success_res = setup.reveal_vote(name, title, vote, salt);
    assert!(success_res.is_ok());

    // Fail pattern (Governance does not exist)
    let fail_res = setup.reveal_vote("fake name", title, vote, salt);
    assert!(fail_res.is_err());
}
