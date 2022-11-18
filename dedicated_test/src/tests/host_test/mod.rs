use indexmap::IndexSet;
use linux_alternative_resolver::alternative_resolver::AlternativeResolver;
use linux_alternative_resolver::traits::alt_config_persistence::AltConfigPersistence;
use linux_alternative_resolver_shared::common_models::models::alt_config::AltConfig;
use linux_alternative_resolver_shared::common_models::models::link_group::LinkGroup;
use linux_alternative_resolver_shared::common_models::models::link_item::LinkItem;
use linux_alternative_resolver_shared::common_models::models::link_path::LinkPath;

#[test]
fn test1() {
    let resolver = AlternativeResolver {};
    // If you get error here, it's most likely due to issue related to settings of your (update-)alternative
    let config = resolver.resolve().unwrap();
    dbg!(serde_json::to_string(&config).unwrap());
    println!("Alternative loaded");

    let mut new_config = AltConfig { alternatives: Default::default() };

    // Sadly, core doesn't verify paths but only register does
    (&mut new_config).alternatives.insert(LinkGroup {
        name: "lar-test".to_string(),
        filename: "lar-test".to_string(),
        selected: None,
        items: IndexSet::from([
            LinkItem {
                family: None,
                priority: 1,
                paths: IndexSet::from([
                    LinkPath {
                        name: "lar-test".to_string(), // Name of first item must be equal to name of parent link group!
                        target_path: "/there_is_probably_no_path_like_this/in_the_world".to_string(),
                        alternative_path: "/yes/you_are_right".to_string()
                    }
                ])
            }
        ])
    });

    // If you are stuck here, check if you get surely root permission.
    resolver.update(&new_config).unwrap();
    println!("Alternative updated");

    (&mut new_config).alternatives.pop();

    resolver.update(&new_config).unwrap();
    println!("Alternative restored");

    // This makes error when when it is about ot be updated
    // because there is no items while first link item must be declared and valid in the array!
    new_config.alternatives.insert(LinkGroup {
        name: "this makes error".to_string(),
        filename: "make_error".to_string(),
        selected: None,
        items: IndexSet::new(),
    });

    assert_eq!(resolver.update(&config).is_ok(), false);
    println!("error while updating Alternative");
}