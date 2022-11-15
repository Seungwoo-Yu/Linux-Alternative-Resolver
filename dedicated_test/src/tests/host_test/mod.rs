use linux_alternative_resolver_core::alternative_resolver::AlternativeResolver;
use linux_alternative_resolver_core::models::link_group::LinkGroup;
use linux_alternative_resolver_core::models::link_item::LinkItem;
use linux_alternative_resolver_core::models::link_path::LinkPath;
use linux_alternative_resolver_core::traits::alt_config_persistence::AltConfigPersistence;

#[test]
fn test1() {
    let resolver = AlternativeResolver {};
    // If you get error here, it's most likely due to issue related to settings of your (update-)alternative
    let mut config = resolver.resolve().unwrap();
    dbg!(serde_json::to_string(&config).unwrap());
    println!("Alternative loaded");

    // Sadly, core doesn't verify paths but only register does
    config.alternatives.push(LinkGroup {
        name: "lar-test".to_string(),
        selected: None,
        items: vec![
            LinkItem {
                family: None,
                priority: 1,
                paths: vec![
                    LinkPath {
                        name: "lar-test".to_string(), // Name of first item must be equal to name of parent link group!
                        target_path: "/there_is_probably_no_path_like_this/in_the_world".to_string(),
                        alternative_path: "/yes/you_are_right".to_string()
                    }
                ]
            }
        ]
    });

    // If you are stuck here, check if you get surely root permission.
    resolver.update(&config).unwrap();
    println!("Alternative updated");
    dbg!(serde_json::to_string(&config).unwrap());

    config.alternatives.pop();

    resolver.update(&config).unwrap();
    println!("Alternative restored");
    dbg!(serde_json::to_string(&config).unwrap());

    // This makes error when when it is about ot be updated
    // because there is no items while first link item must be declared and valid in the array!
    config.alternatives.push(LinkGroup {
        name: "this makes error".to_string(),
        selected: None,
        items: vec![],
    });

    assert!(resolver.update(&config).is_ok(), false);
    println!("error while updating Alternative");
}