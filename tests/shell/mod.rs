use nu_test_support::{nu, pipeline};

mod pipeline;

#[test]
fn plugins_are_declared_with_wix() {
    let actual = nu!(
        cwd: ".", pipeline(
        r#"
            echo $(open wix/main.wxs --raw | from xml
                | get Wix.children.Product.children.0.Directory.children.0
                | where Directory.attributes.Id == "$(var.PlatformProgramFilesFolder)"
                | get Directory.children.Directory.children.0 | last
                | get Directory.children.Component.children
                | each { echo $it | first }
                | skip
                | where File.attributes.Name =~ "nu_plugin"
                | str substring [_, -4] File.attributes.Name
                | get File.attributes.Name
                | sort-by
                | wrap wix) | merge {
                    open Cargo.toml |
                    get bin.name |
                    drop |
                    sort-by |
                    wrap cargo
                }
            | if $it.wix != $it.cargo { = 1 } { = 0 }
            | math sum
            "#
    ));

    assert_eq!(actual.out, "0");
}
