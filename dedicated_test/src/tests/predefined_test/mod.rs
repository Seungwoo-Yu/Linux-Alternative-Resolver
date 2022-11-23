use linux_alternative_resolver::impls::alternative_resolver_impl::convert_strings_to_alt_config;

#[test]
fn test1() {
    // One of well-known complicated data from Ubuntu 22.04 (/var/lib/dpkg/alternatives/editor)
    let predefined_data = [
        "auto",
        "/usr/bin/editor",
        "editor.1.gz",
        "/usr/share/man/man1/editor.1.gz",
        "editor.da.1.gz",
        "/usr/share/man/da/man1/editor.1.gz",
        "editor.de.1.gz",
        "/usr/share/man/de/man1/editor.1.gz",
        "editor.fr.1.gz",
        "/usr/share/man/fr/man1/editor.1.gz",
        "editor.it.1.gz",
        "/usr/share/man/it/man1/editor.1.gz",
        "editor.ja.1.gz",
        "/usr/share/man/ja/man1/editor.1.gz",
        "editor.pl.1.gz",
        "/usr/share/man/pl/man1/editor.1.gz",
        "editor.ru.1.gz",
        "/usr/share/man/ru/man1/editor.1.gz",
        "",
        "/bin/ed",
        "-100",
        "/usr/share/man/man1/ed.1.gz",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "/bin/nano",
        "40",
        "/usr/share/man/man1/nano.1.gz",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "/usr/bin/vim.basic",
        "30",
        "/usr/share/man/man1/vim.1.gz",
        "/usr/share/man/da/man1/vim.1.gz",
        "/usr/share/man/de/man1/vim.1.gz",
        "/usr/share/man/fr/man1/vim.1.gz",
        "/usr/share/man/it/man1/vim.1.gz",
        "/usr/share/man/ja/man1/vim.1.gz",
        "/usr/share/man/pl/man1/vim.1.gz",
        "/usr/share/man/ru/man1/vim.1.gz",
        "/usr/bin/vim.tiny",
        "15",
        "/usr/share/man/man1/vim.1.gz",
        "/usr/share/man/da/man1/vim.1.gz",
        "/usr/share/man/de/man1/vim.1.gz",
        "/usr/share/man/fr/man1/vim.1.gz",
        "/usr/share/man/it/man1/vim.1.gz",
        "/usr/share/man/ja/man1/vim.1.gz",
        "/usr/share/man/pl/man1/vim.1.gz",
        "/usr/share/man/ru/man1/vim.1.gz",
        "",
    ].join("\n");

    let config = convert_strings_to_alt_config(&vec![
        (format!("editor"), predefined_data)
    ]).unwrap();
    assert_eq!(config.alternatives[0].items[0].paths[0].name, "editor");
    assert_eq!(
        config.alternatives[0].items[2].paths.last().map(| value | &value.alternative_path),
        Some(&"/usr/share/man/ru/man1/vim.1.gz".to_string()),
    );
}