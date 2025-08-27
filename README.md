.
├── Cargo.lock
├── Cargo.toml
├── bruno_requests
│   ├── bruno.json
│   ├── create transaccao.bru
│   ├── delete transacoes.bru
│   ├── get estatistica.bru
│   ├── get estatisticas.bru
│   └── get transacoes.bru
├── docker-compose.yml
├── Dockerfile
├── LICENSE
├── README.md
├── src
│   ├── application
│   │   ├── dto
│   │   ├── estatistica_service.rs
│   │   ├── mod.rs
│   │   └── transacao_service.rs
│   ├── domain
│   │   ├── mod.rs
│   │   └── transacao.rs
│   ├── infrastructure
│   │   ├── db
│   │   ├── error.rs
│   │   ├── http
│   │   └── mod.rs
│   └── main.rs
└── target
