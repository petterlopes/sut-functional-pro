# 🚀 Guia de Início Rápido - SUT

Este guia irá ajudá-lo a configurar e executar o projeto SUT em poucos minutos.

## 📋 Pré-requisitos

Certifique-se de ter instalado:

- [Docker](https://www.docker.com/) (versão 20.10+)
- [Docker Compose](https://docs.docker.com/compose/) (versão 2.0+)
- [Git](https://git-scm.com/) (para clonar o repositório)

**Opcional** (apenas se não usar Docker):
- [Rust](https://www.rust-lang.org/) (versão 1.84+)
- [Node.js](https://nodejs.org/) (versão 22+)
- [PostgreSQL](https://www.postgresql.org/) (versão 16+)

## 🎯 Início Rápido com Docker (Recomendado)

### 1️⃣ Clone o Repositório

```bash
git clone https://github.com/seu-usuario/sut-functional-pro.git
cd sut-functional-pro
```

### 2️⃣ Inicie os Serviços

```bash
cd deploy
docker compose -f docker-compose.dev.yml up -d
```

Este comando irá:
- ✅ Baixar e construir todas as imagens Docker necessárias
- ✅ Iniciar PostgreSQL, Keycloak, Vault, Prometheus, Grafana
- ✅ Compilar e executar a API Rust
- ✅ Instalar dependências e executar o frontend React
- ✅ Configurar automaticamente usuários e realms no Keycloak

### 3️⃣ Aguarde a Inicialização

A primeira execução pode levar alguns minutos (5-10 min) para:
- Baixar imagens Docker
- Compilar o código Rust
- Instalar dependências do Node.js

**Acompanhe os logs:**
```bash
docker compose -f docker-compose.dev.yml logs -f
```

### 4️⃣ Acesse o Sistema

Quando todos os serviços estiverem prontos, acesse:

| Serviço | URL | Credenciais |
|---------|-----|-------------|
| **Frontend** | http://localhost:5173 | - |
| **API** | http://localhost:8080 | - |
| **Keycloak** | http://localhost:8081 | admin / Admin@Keycloak2024! |
| **Grafana** | http://localhost:3000 | admin / admin |
| **Prometheus** | http://localhost:9090 | - |
| **Vault** | http://localhost:8200 | Token: dev-root-token |

> **Dica**: o endpoint `/metrics` exige `X-Metrics-Token: dev-metrics-token`. Webhooks locais devem enviar `X-Webhook-Token: dev-shared-webhook-token`. Configure `CORS_ALLOWED_ORIGINS` antes de qualquer deploy real.

> Para que a API popular usuários de exemplo, habilite o GUC `app.enable_demo_users` antes da primeira execução das migrações (por exemplo, via `ALTER DATABASE sut SET app.enable_demo_users = 'on';`).

### 5️⃣ Faça Login

No frontend (http://localhost:5173), clique em **"Fazer Login"** e use:

**Usuários de Teste (Keycloak):**

| Usuário | Senha | Papel |
|---------|-------|-------|
| admin | admin123 (temporário) | Administrador |
| manager | manager123 (temporário) | Gestor |
| analyst | analyst123 (temporário) | Analista |

> O Keycloak exige troca imediata de senha para essas contas. O primeiro login redireciona para a tela de atualização.

### 6️⃣ Explore o Sistema

Agora você pode:
- ✅ Navegar pelo dashboard moderno
- ✅ Gerenciar contatos
- ✅ Visualizar unidades organizacionais
- ✅ Gerenciar departamentos
- ✅ Ver métricas no Grafana

## 🛑 Parar os Serviços

```bash
cd deploy
docker compose -f docker-compose.dev.yml down
```

**Para remover também os volumes (dados):**
```bash
docker compose -f docker-compose.dev.yml down -v
```

## 🔧 Desenvolvimento Local (Sem Docker)

Se preferir executar sem Docker:

### Backend (API Rust)

```bash
# 1. Configure o PostgreSQL (crie um banco chamado 'sut')
createdb sut

# 2. Configure as variáveis de ambiente
export PG_DSN="postgres://seu_usuario:sua_senha@localhost:5432/sut"
export KEYCLOAK_JWKS="http://localhost:8081/realms/sut/protocol/openid-connect/certs"

# 3. Execute as migrations
cd api
sqlx migrate run

# 4. Execute a API
cargo run --release
```

### Frontend (React)

```bash
cd frontend

# 1. Instale as dependências
npm install

# 2. Configure as variáveis de ambiente
export VITE_API_BASE=http://localhost:8080
export VITE_KC_URL=http://localhost:8081
export VITE_KC_REALM=sut
export VITE_KC_CLIENT=sut-frontend

# 3. Execute o servidor de desenvolvimento
npm run dev
```

## 📚 Próximos Passos

- 📖 Leia a [Documentação Completa](README.md)
- 🏗️ Entenda a [Arquitetura](docs/ARCHITECTURE.md)
- 🔌 Explore a [API](docs/API.md)
- 💻 Aprenda sobre o [Frontend](docs/FRONTEND.md)
- 🚀 Configure para [Produção](docs/DEPLOYMENT.md)

## 🆘 Problemas Comuns

### Porta já em uso

Se alguma porta já estiver em uso, você pode:

1. Parar o serviço que está usando a porta
2. Ou modificar as portas no `docker-compose.dev.yml`

### Erro de permissão no Docker

No Linux, você pode precisar adicionar seu usuário ao grupo docker:

```bash
sudo usermod -aG docker $USER
newgrp docker
```

### API não inicia

Verifique se o PostgreSQL está rodando:

```bash
docker compose -f docker-compose.dev.yml ps postgres
```

### Frontend não carrega

1. Verifique se o serviço está rodando:
   ```bash
   docker compose -f docker-compose.dev.yml ps frontend
   ```

2. Veja os logs:
   ```bash
   docker compose -f docker-compose.dev.yml logs frontend
   ```

## 📞 Suporte

- 🐛 [Reporte bugs](https://github.com/seu-usuario/sut-functional-pro/issues)
- 💬 [Discussões](https://github.com/seu-usuario/sut-functional-pro/discussions)
- 📧 [Email de suporte](mailto:seu-email@example.com)

---

**Pronto!** 🎉 Você está agora executando o SUT localmente e pode começar a desenvolver!

