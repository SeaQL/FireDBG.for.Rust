mod common;

pub use firedbg_rust_parser::*;
pub use pretty_assertions::assert_eq;

async fn run_workspace_parsing(
    root_dir: &str,
    workspace_expected: Workspace,
) -> anyhow::Result<Workspace> {
    let workspace = parse_workspace(root_dir)?;

    println!("{:#?}", workspace);

    let firedbg_dir = format!("{root_dir}/firedbg");

    std::fs::create_dir_all(&firedbg_dir)?;

    let bson_path = format!("{firedbg_dir}/workspace");

    serde::to_bson_file(&format!("{bson_path}.bson"), &workspace).await?;
    serde::to_json_file(&format!("{bson_path}.json"), &workspace).await?;

    assert_eq!(
        workspace_expected,
        serde::from_bson_file(&format!("{}.bson", bson_path)).await?,
    );

    assert_eq!(workspace_expected, workspace);

    Ok(workspace)
}

#[tokio::test]
async fn parse_example_workspace() -> anyhow::Result<()> {
    let workspace = run_workspace_parsing(
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/example-workspace"),
        Workspace {
            packages: vec![
                Package {
                    name: "main-one".into(),
                    version: "0.1.0".into(),
                    root_dir: concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/tests/example-workspace/main-one"
                    )
                    .into(),
                    dependencies: vec![Dependency {
                        name: "quick-sort".into(),
                        default_features: true,
                        features: vec![],
                        root_dir: concat!(
                            env!("CARGO_MANIFEST_DIR"),
                            "/tests/example-workspace/quick-sort"
                        )
                        .into(),
                    }],
                    binaries: vec![
                        Binary {
                            name: "main-one".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/example-workspace/main-one/src/main.rs"
                            )
                            .into(),
                        },
                        Binary {
                            name: "another_main".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/example-workspace/main-one/src/bin/another_main.rs"
                            )
                            .into(),
                        },
                    ],
                    tests: vec![Test {
                        name: "simple_tests".into(),
                        src_path: concat!(
                            env!("CARGO_MANIFEST_DIR"),
                            "/tests/example-workspace/main-one/tests/simple_tests.rs"
                        )
                        .into(),
                    }],
                    examples: vec![Example {
                        name: "demo".into(),
                        src_path: concat!(
                            env!("CARGO_MANIFEST_DIR"),
                            "/tests/example-workspace/main-one/examples/demo.rs"
                        )
                        .into(),
                    }],
                    has_lib: true,
                },
                Package {
                    name: "main-two".into(),
                    version: "0.1.0".into(),
                    root_dir: concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/tests/example-workspace/main-two"
                    )
                    .into(),
                    dependencies: vec![],
                    binaries: vec![
                        Binary {
                            name: "renamed_main".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/example-workspace/main-two/src/renamed-main.rs"
                            )
                            .into(),
                        },
                        Binary {
                            name: "hyphenated-main".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/example-workspace/main-two/src/bin/hyphenated-main.rs"
                            )
                            .into(),
                        },
                    ],
                    tests: vec![],
                    examples: vec![],
                    has_lib: false,
                },
                Package {
                    name: "quick-sort".into(),
                    version: "0.1.0".into(),
                    root_dir: concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/tests/example-workspace/quick-sort"
                    )
                    .into(),
                    dependencies: vec![],
                    binaries: vec![Binary {
                        name: "quick-sort".into(),
                        src_path: concat!(
                            env!("CARGO_MANIFEST_DIR"),
                            "/tests/example-workspace/quick-sort/src/main.rs"
                        )
                        .into(),
                    }],
                    tests: vec![],
                    examples: vec![],
                    has_lib: true,
                },
            ],
            target_dir: concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/example-workspace/target"
            )
            .into(),
            root_dir: concat!(env!("CARGO_MANIFEST_DIR"), "/tests/example-workspace").into(),
        },
    )
    .await?;

    let main_one_package = &workspace.packages[0];
    let main_one = &main_one_package.binaries[0];
    let simple_tests = &main_one_package.tests[0];
    let demo = &main_one_package.examples[0];

    // /Applications/MAMP/htdocs/FireDBG.for.Rust.Internal/parser/tests/example-workspace/target/debug/main-one
    println!("binary_path {}", main_one.get_binary_path(&workspace));
    // /Applications/MAMP/htdocs/FireDBG.for.Rust.Internal/parser/tests/example-workspace/target/debug/deps/simple_tests-eb63f31f8208c8a4
    println!("test_path {}", simple_tests.get_test_path(&workspace)?);
    // /Applications/MAMP/htdocs/FireDBG.for.Rust.Internal/parser/tests/example-workspace/target/debug/examples/demo
    println!("example_path {}", demo.get_example_path(&workspace));

    Ok(())
}

#[tokio::test]
async fn parse_example_without_workspace() -> anyhow::Result<()> {
    run_workspace_parsing(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/example-without-workspace"
        ),
        Workspace {
            packages: vec![Package {
                name: "example-without-workspace".into(),
                version: "0.1.0".into(),
                root_dir: concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/tests/example-without-workspace"
                )
                .into(),
                dependencies: vec![],
                binaries: vec![Binary {
                    name: "example-without-workspace".into(),
                    src_path: concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/tests/example-without-workspace/src/main.rs"
                    )
                    .into(),
                }],
                tests: vec![],
                examples: vec![],
                has_lib: false,
            }],
            target_dir: concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/example-without-workspace/target"
            )
            .into(),
            root_dir: concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/example-without-workspace"
            )
            .into(),
        },
    )
    .await?;
    Ok(())
}

#[tokio::test]
async fn parse_sea_streamer() -> anyhow::Result<()> {
    run_workspace_parsing(
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/sea-streamer"),
        Workspace {
            packages: vec![
                Package {
                    name: "sea-streamer".into(),
                    version: "0.3.2".into(),
                    root_dir: concat!(env!("CARGO_MANIFEST_DIR"), "/tests/sea-streamer").into(),
                    dependencies: vec![
                        Dependency {
                            name: "sea-streamer-file".into(),
                            default_features: true,
                            features: vec![],
                            root_dir: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-file"
                            )
                            .into(),
                        },
                        Dependency {
                            name: "sea-streamer-kafka".into(),
                            default_features: true,
                            features: vec![],
                            root_dir: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-kafka"
                            )
                            .into(),
                        },
                        Dependency {
                            name: "sea-streamer-redis".into(),
                            default_features: true,
                            features: vec![],
                            root_dir: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-redis"
                            )
                            .into(),
                        },
                        Dependency {
                            name: "sea-streamer-runtime".into(),
                            default_features: true,
                            features: vec![],
                            root_dir: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-runtime"
                            )
                            .into(),
                        },
                        Dependency {
                            name: "sea-streamer-socket".into(),
                            default_features: true,
                            features: vec![],
                            root_dir: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-socket"
                            )
                            .into(),
                        },
                        Dependency {
                            name: "sea-streamer-stdio".into(),
                            default_features: true,
                            features: vec![],
                            root_dir: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-stdio"
                            )
                            .into(),
                        },
                        Dependency {
                            name: "sea-streamer-types".into(),
                            default_features: true,
                            features: vec![],
                            root_dir: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-types"
                            )
                            .into(),
                        },
                    ],
                    binaries: vec![],
                    tests: vec![],
                    examples: vec![],
                    has_lib: true,
                },
                Package {
                    name: "sea-streamer-benchmark".into(),
                    version: "0.3.0".into(),
                    root_dir: concat!(env!("CARGO_MANIFEST_DIR"), "/tests/sea-streamer/benchmark")
                        .into(),
                    dependencies: vec![Dependency {
                        name: "sea-streamer".into(),
                        default_features: true,
                        features: vec![
                            "kafka".into(),
                            "redis".into(),
                            "stdio".into(),
                            "file".into(),
                            "socket".into(),
                        ],
                        root_dir: concat!(env!("CARGO_MANIFEST_DIR"), "/tests/sea-streamer").into(),
                    }],
                    binaries: vec![
                        Binary {
                            name: "producer".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/benchmark/src/bin/producer.rs"
                            )
                            .into(),
                        },
                        Binary {
                            name: "relay".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/benchmark/src/bin/relay.rs"
                            )
                            .into(),
                        },
                        Binary {
                            name: "consumer".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/benchmark/src/bin/consumer.rs"
                            )
                            .into(),
                        },
                    ],
                    tests: vec![],
                    examples: vec![],
                    has_lib: false,
                },
                Package {
                    name: "sea-streamer-examples".into(),
                    version: "0.3.0".into(),
                    root_dir: concat!(env!("CARGO_MANIFEST_DIR"), "/tests/sea-streamer/examples")
                        .into(),
                    dependencies: vec![Dependency {
                        name: "sea-streamer".into(),
                        default_features: true,
                        features: vec![
                            "kafka".into(),
                            "redis".into(),
                            "stdio".into(),
                            "file".into(),
                            "socket".into(),
                        ],
                        root_dir: concat!(env!("CARGO_MANIFEST_DIR"), "/tests/sea-streamer").into(),
                    }],
                    binaries: vec![
                        Binary {
                            name: "resumable".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/examples/src/bin/resumable.rs"
                            )
                            .into(),
                        },
                        Binary {
                            name: "processor".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/examples/src/bin/processor.rs"
                            )
                            .into(),
                        },
                        Binary {
                            name: "producer".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/examples/src/bin/producer.rs"
                            )
                            .into(),
                        },
                        Binary {
                            name: "buffered".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/examples/src/bin/buffered.rs"
                            )
                            .into(),
                        },
                        Binary {
                            name: "blocking".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/examples/src/bin/blocking.rs"
                            )
                            .into(),
                        },
                        Binary {
                            name: "consumer".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/examples/src/bin/consumer.rs"
                            )
                            .into(),
                        },
                    ],
                    tests: vec![],
                    examples: vec![],
                    has_lib: false,
                },
                Package {
                    name: "sea-streamer-file".into(),
                    version: "0.3.3".into(),
                    root_dir: concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/tests/sea-streamer/sea-streamer-file"
                    )
                    .into(),
                    dependencies: vec![
                        Dependency {
                            name: "sea-streamer-runtime".into(),
                            default_features: true,
                            features: vec!["file".into()],
                            root_dir: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-runtime"
                            )
                            .into(),
                        },
                        Dependency {
                            name: "sea-streamer-types".into(),
                            default_features: true,
                            features: vec![],
                            root_dir: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-types"
                            )
                            .into(),
                        },
                    ],
                    binaries: vec![
                        Binary {
                            name: "clock".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-file/src/bin/clock.rs"
                            )
                            .into(),
                        },
                        Binary {
                            name: "decoder".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-file/src/bin/decoder.rs"
                            )
                            .into(),
                        },
                        Binary {
                            name: "sink".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-file/src/bin/sink.rs"
                            )
                            .into(),
                        },
                        Binary {
                            name: "tail".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-file/src/bin/tail.rs"
                            )
                            .into(),
                        },
                    ],
                    tests: vec![
                        Test {
                            name: "loopback".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-file/tests/loopback.rs"
                            )
                            .into(),
                        },
                        Test {
                            name: "producer".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-file/tests/producer.rs"
                            )
                            .into(),
                        },
                        Test {
                            name: "util".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-file/tests/util.rs"
                            )
                            .into(),
                        },
                        Test {
                            name: "surveyor".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-file/tests/surveyor.rs"
                            )
                            .into(),
                        },
                        Test {
                            name: "consumer".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-file/tests/consumer.rs"
                            )
                            .into(),
                        },
                    ],
                    examples: vec![],
                    has_lib: true,
                },
                Package {
                    name: "sea-streamer-kafka".into(),
                    version: "0.3.1".into(),
                    root_dir: concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/tests/sea-streamer/sea-streamer-kafka"
                    )
                    .into(),
                    dependencies: vec![
                        Dependency {
                            name: "sea-streamer-runtime".into(),
                            default_features: true,
                            features: vec![],
                            root_dir: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-runtime"
                            )
                            .into(),
                        },
                        Dependency {
                            name: "sea-streamer-types".into(),
                            default_features: true,
                            features: vec![],
                            root_dir: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-types"
                            )
                            .into(),
                        },
                    ],
                    binaries: vec![
                        Binary {
                            name: "consumer".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-kafka/src/bin/consumer.rs"
                            )
                            .into(),
                        },
                        Binary {
                            name: "producer".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-kafka/src/bin/producer.rs"
                            )
                            .into(),
                        },
                    ],
                    tests: vec![Test {
                        name: "consumer".into(),
                        src_path: concat!(
                            env!("CARGO_MANIFEST_DIR"),
                            "/tests/sea-streamer/sea-streamer-kafka/tests/consumer.rs"
                        )
                        .into(),
                    }],
                    examples: vec![],
                    has_lib: true,
                },
                Package {
                    name: "sea-streamer-redis".into(),
                    version: "0.3.2".into(),
                    root_dir: concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/tests/sea-streamer/sea-streamer-redis"
                    )
                    .into(),
                    dependencies: vec![
                        Dependency {
                            name: "sea-streamer-runtime".into(),
                            default_features: true,
                            features: vec![],
                            root_dir: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-runtime"
                            )
                            .into(),
                        },
                        Dependency {
                            name: "sea-streamer-types".into(),
                            default_features: true,
                            features: vec![],
                            root_dir: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-types"
                            )
                            .into(),
                        },
                    ],
                    binaries: vec![
                        Binary {
                            name: "consumer".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-redis/src/bin/consumer.rs"
                            )
                            .into(),
                        },
                        Binary {
                            name: "producer".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-redis/src/bin/producer.rs"
                            )
                            .into(),
                        },
                    ],
                    tests: vec![
                        Test {
                            name: "resumable".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-redis/tests/resumable.rs"
                            )
                            .into(),
                        },
                        Test {
                            name: "seek-rewind".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-redis/tests/seek-rewind.rs"
                            )
                            .into(),
                        },
                        Test {
                            name: "consumer-group".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-redis/tests/consumer-group.rs"
                            )
                            .into(),
                        },
                        Test {
                            name: "util".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-redis/tests/util.rs"
                            )
                            .into(),
                        },
                        Test {
                            name: "realtime".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-redis/tests/realtime.rs"
                            )
                            .into(),
                        },
                        Test {
                            name: "sharding".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-redis/tests/sharding.rs"
                            )
                            .into(),
                        },
                        Test {
                            name: "load-balanced".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-redis/tests/load-balanced.rs"
                            )
                            .into(),
                        },
                    ],
                    examples: vec![],
                    has_lib: true,
                },
                Package {
                    name: "sea-streamer-runtime".into(),
                    version: "0.3.2".into(),
                    root_dir: concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/tests/sea-streamer/sea-streamer-runtime"
                    )
                    .into(),
                    dependencies: vec![],
                    binaries: vec![],
                    tests: vec![],
                    examples: vec![],
                    has_lib: true,
                },
                Package {
                    name: "sea-streamer-socket".into(),
                    version: "0.3.0".into(),
                    root_dir: concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/tests/sea-streamer/sea-streamer-socket"
                    )
                    .into(),
                    dependencies: vec![
                        Dependency {
                            name: "sea-streamer-file".into(),
                            default_features: true,
                            features: vec![],
                            root_dir: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-file"
                            )
                            .into(),
                        },
                        Dependency {
                            name: "sea-streamer-kafka".into(),
                            default_features: true,
                            features: vec![],
                            root_dir: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-kafka"
                            )
                            .into(),
                        },
                        Dependency {
                            name: "sea-streamer-redis".into(),
                            default_features: true,
                            features: vec![],
                            root_dir: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-redis"
                            )
                            .into(),
                        },
                        Dependency {
                            name: "sea-streamer-stdio".into(),
                            default_features: true,
                            features: vec![],
                            root_dir: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-stdio"
                            )
                            .into(),
                        },
                        Dependency {
                            name: "sea-streamer-types".into(),
                            default_features: true,
                            features: vec![],
                            root_dir: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-types"
                            )
                            .into(),
                        },
                    ],
                    binaries: vec![Binary {
                        name: "relay".into(),
                        src_path: concat!(
                            env!("CARGO_MANIFEST_DIR"),
                            "/tests/sea-streamer/sea-streamer-socket/src/bin/relay.rs"
                        )
                        .into(),
                    }],
                    tests: vec![],
                    examples: vec![],
                    has_lib: true,
                },
                Package {
                    name: "sea-streamer-stdio".into(),
                    version: "0.3.0".into(),
                    root_dir: concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/tests/sea-streamer/sea-streamer-stdio"
                    )
                    .into(),
                    dependencies: vec![
                        Dependency {
                            name: "sea-streamer-runtime".into(),
                            default_features: true,
                            features: vec![],
                            root_dir: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-runtime"
                            )
                            .into(),
                        },
                        Dependency {
                            name: "sea-streamer-types".into(),
                            default_features: true,
                            features: vec![],
                            root_dir: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-types"
                            )
                            .into(),
                        },
                    ],
                    binaries: vec![
                        Binary {
                            name: "clock".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-stdio/src/bin/clock.rs"
                            )
                            .into(),
                        },
                        Binary {
                            name: "complex".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-stdio/src/bin/complex.rs"
                            )
                            .into(),
                        },
                        Binary {
                            name: "relay".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-stdio/src/bin/relay.rs"
                            )
                            .into(),
                        },
                    ],
                    tests: vec![
                        Test {
                            name: "loopback".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-stdio/tests/loopback.rs"
                            )
                            .into(),
                        },
                        Test {
                            name: "group".into(),
                            src_path: concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/tests/sea-streamer/sea-streamer-stdio/tests/group.rs"
                            )
                            .into(),
                        },
                    ],
                    examples: vec![],
                    has_lib: true,
                },
                Package {
                    name: "sea-streamer-types".into(),
                    version: "0.3.0".into(),
                    root_dir: concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/tests/sea-streamer/sea-streamer-types"
                    )
                    .into(),
                    dependencies: vec![],
                    binaries: vec![],
                    tests: vec![],
                    examples: vec![],
                    has_lib: true,
                },
            ],
            target_dir: concat!(env!("CARGO_MANIFEST_DIR"), "/tests/sea-streamer/target").into(),
            root_dir: concat!(env!("CARGO_MANIFEST_DIR"), "/tests/sea-streamer").into(),
        },
    )
    .await?;
    Ok(())
}
