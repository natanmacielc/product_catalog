# 🦀 Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore API

API REST para gerenciamento de produtos construída com Rust, utilizando:

* **Axum** (framework web)
* **SQLx** (acesso ao banco)
* **PostgreSQL** (persistência)
* **Redis** (cache)
* Arquitetura em camadas: **Repository + UseCase**

---

# 📦 Funcionalidades

* Criar produto
* Listar produtos
* Buscar produto por ID
* Buscar produtos por filtros (name, brand, category)
* Cache com Redis para otimizar leitura

---

# 🧠 Arquitetura

```
src/
  domain/
    entity/
      product.rs

  application/
    dto/
      create_product_request.rs
    usecase/
      product_catalog_use_case.rs

  infrastructure/
    database/
      product_repository.rs
    cache/
      redis_cache.rs
      product_cache.rs

  main.rs
```

### Responsabilidades

* **Domain** → entidades puras
* **Repository** → acesso ao Postgres
* **UseCase** → regras de negócio + cache
* **Cache (Redis)** → leitura otimizada

---

# 🚀 Como executar o projeto

## 1. Pré-requisitos

* Docker
* Docker Compose
* Rust (opcional, se quiser rodar local sem container)

---

## 2. Configurar `.env`

Crie um arquivo `.env` na raiz:

```env
APP_HOST=0.0.0.0
APP_PORT=3000

POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres
POSTGRES_DB=product_catalog

DATABASE_URL=postgres://postgres:postgres@postgres:5432/product_catalog
REDIS_URL=redis://redis:6379/
```

---

## 3. Subir tudo com Docker

```bash
docker compose up --build
```

Isso irá subir:

* API → http://localhost:3000
* Postgres → porta 5432
* Redis → porta 6379

---

## 4. Verificar se está rodando

```bash
curl http://localhost:3000/health
```

Resposta:

```text
ok
```

---

# 📡 Endpoints

## 🔹 Criar produto

```http
POST /products
```

### Body:

```json
{
  "name": "iPhone 15",
  "brand": "Apple",
  "category": "Smartphone",
  "price_cents": 500000
}
```

---

## 🔹 Listar produtos

```http
GET /products
```

---

## 🔹 Buscar por ID

```http
GET /products/{id}
```

---

## 🔹 Busca customizada

```http
GET /products/search?name=iphone&brand=apple&category=smartphone
```

### Parâmetros opcionais:

| Parâmetro | Tipo   | Descrição                   |
| --------- | ------ | --------------------------- |
| name      | string | busca parcial por nome      |
| brand     | string | busca parcial por marca     |
| category  | string | busca parcial por categoria |

---

# ⚡ Cache com Redis

## Estratégia

* `GET /products` → cache: `products:all`
* `GET /products/{id}` → cache: `products:{id}`

## Invalidação

* `POST /products` → limpa:

  * `products:all`
  * `products:{id}`

## TTL

* 60 segundos

---

# 🧪 Rodando testes

```bash
cargo test
```

Com output:

```bash
cargo test -- --nocapture
```

---

# 🧱 Banco de dados

Tabela esperada:

```sql
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    brand TEXT NOT NULL,
    category TEXT NOT NULL,
    price_cents BIGINT NOT NULL
);
```

---

# 🐳 Docker

## Serviços

* **app** → API Rust
* **postgres** → banco
* **redis** → cache

## Comunicação interna

Dentro dos containers:

* Postgres → `postgres:5432`
* Redis → `redis:6379`

---

# ⚠️ Observações importantes

### ❌ Não use localhost dentro do container

Use sempre:

```env
postgres
redis
```

---

### ⚠️ Cache não é crítico

Se o Redis cair:

* API continua funcionando
* apenas perde performance
