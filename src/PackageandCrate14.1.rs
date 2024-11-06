//1
.
├── Cargo.toml
└── src
└── main.rs
//
.
├── Cargo.toml
└── src
└── lib.rs
//3
.
├── Cargo.toml
├── Cargo.lock
├── src
│   ├── main.rs          # Основной бинарный crate
│   ├── lib.rs           # Библиотечный crate
│   └── bin
│       ├── main1.rs     # Второй бинарный crate
│       └── main2.rs     # Третий бинарный crate
├── tests
│   └── some_integration_tests.rs
├── benches
│   └── simple_bench.rs
└── examples
└── simple_example.rs
