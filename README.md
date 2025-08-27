# 🏦 Itaú Unibanco - Desafio de Programação

Este repositório contém a minha resolução do **Desafio de Programação** proposto pelo Itaú para vaga de desenvolvedor júnior.

> 💡 Não sou júnior, mas o desafio me chamou a atenção. Resolvi implementá-lo em **Rust** com **Actix Web**, aplicando **Clean Architecture** e expondo uma API **REST**.

O projeto permite **inserir múltiplas transações** em memória (via `HashMap`) e **calcular estatísticas** das transações realizadas nos últimos `60 segundos`.

📌 [Repositório Original do Desafio](https://github.com/rafaellins-itau/desafio-itau-vaga-99-junior)

---

## 🚀 Tecnologias

* 🦀 [Rust](https://www.rust-lang.org/)
* ⚡ [Actix Web](https://actix.rs/)
* 🧱 Clean Architecture

---

## 📡 Endpoints

🔗 **Base URL:** `http://localhost:8080`

| Método    | Endpoint       | Descrição             | Corpo (JSON)   | Parâmetros                                |
| --------- | -------------- | --------------------- | -------------- | ----------------------------------------- |
| ➕ POST    | `/transacao`   | Receber Transações    | `TransacaoDto` | -                                         |
| 🗑 DELETE | `/transacao`   | Limpar Transações     | -              | -                                         |
| 📊 GET    | `/estatistica` | Calcular Estatísticas | -              | `segundos` (**Opcional**, Default: `60s`) |

---

### 📦 Exemplos de DTOs

**TransacaoDto**

```json
{
  "valor": 123.45,
  "dataHora": "2025-08-15T09:31:54.072-03:00"
}
```

**EstatisticaDto**

```json
{
  "count": 10,
  "sum": 1234.56,
  "avg": 123.456,
  "min": 12.34,
  "max": 123.56
}
```

---

## 🛠 Como Executar

### 🔹 1. Clonar o repositório

```bash
git clone https://github.com/seu-usuario/itau-desafio-programacao.git
cd itau-desafio-programacao
```

### 🔹 2. Rodar com Cargo

```bash
cargo run
```

### 🔹 3. Rodar com Docker (opcional)

**Build da imagem:**

```bash
docker build -t transacoes-api .
```

**Rodar o container:**

```bash
docker run -p 8080:8080 -p 8081:8081 transacoes-api
```

---

## 📂 Estrutura do Projeto

```
.
├── Cargo.lock
├── Cargo.toml
├── DesafioItau
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
```

---

## ✨ Autor

👤 **Gabriel Victor de Paula**
