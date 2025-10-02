# Guia de Deployment - SUT

## 🚀 Visão Geral

Este documento descreve como fazer o deployment do Sistema Único de Telefonia (SUT) em diferentes ambientes, desde desenvolvimento local até produção.

## 🏗️ Arquitetura de Deployment

```
┌─────────────────────────────────────────────────────────────┐
│                    Load Balancer (Nginx)                    │
└─────────────────────────────────────────────────────────────┘
                                │
                ┌───────────────┼───────────────┐
                │               │               │
┌───────────────▼──┐  ┌────────▼──┐  ┌────────▼──┐
│   Frontend       │  │   API      │  │   API      │
│   (React SPA)    │  │   (Rust)   │  │   (Rust)   │
└──────────────────┘  └───────────┘  └───────────┘
                                │
                ┌───────────────┼───────────────┐
                │               │               │
┌───────────────▼──┐  ┌────────▼──┐  ┌────────▼──┐
│   PostgreSQL     │  │  Keycloak  │  │   Vault    │
│   (Primary)      │  │            │  │            │
└──────────────────┘  └───────────┘  └───────────┘
                │
┌───────────────▼──┐
│   PostgreSQL     │
│   (Replica)      │
└──────────────────┘
```

## 🛠️ Pré-requisitos

### Desenvolvimento
- Docker & Docker Compose v2
- Node.js 18+
- Rust (cargo)
- Git

### Produção
- Docker & Docker Compose v2
- Nginx (opcional, para load balancing)
- SSL/TLS certificates
- Domain name
- Monitoring tools (Prometheus, Grafana)

## 🏠 Desenvolvimento Local

### 1. Clone do Repositório
```bash
git clone <repository-url>
cd sut-functional-pro
```

### 2. Configuração de Ambiente
```bash
# Copiar arquivos de exemplo
cp .env.example .env.local

# Editar variáveis de ambiente
nano .env.local
```

### 3. Subir Infraestrutura
```bash
cd deploy
docker compose -f docker-compose.dev.yml up --build
```

### 4. Verificar Serviços
```bash
# Verificar status dos containers
docker compose -f docker-compose.dev.yml ps

# Verificar logs
docker compose -f docker-compose.dev.yml logs -f
```

### 5. Acessos
- **Frontend**: http://localhost:5173
- **API**: http://localhost:8080
- **Keycloak**: http://localhost:8081
- **Grafana**: http://localhost:3000
- **Prometheus**: http://localhost:9090
- **Vault**: http://localhost:8200

### 6. ⚠️ Notas Importantes sobre Correções

#### Função `unaccent` Removida
- O projeto foi refatorado para não depender mais da função PostgreSQL `unaccent`
- Agora usa uma função `normalize_text` customizada: `LOWER(TRIM(input_text))`
- Todas as migrações foram atualizadas para usar a nova função

#### Autenticação de Métricas
- O endpoint `/metrics` requer autenticação Basic Auth
- Credenciais: `metrics:dev-metrics-token`
- Prometheus configurado automaticamente para usar Basic Auth

#### Dependências do Frontend Corrigidas
- Todas as versões incompatíveis foram corrigidas
- Frontend agora instala e executa corretamente

## 🧪 Ambiente de Testes

### 1. Configuração
```bash
# Criar arquivo de ambiente para testes
cp .env.example .env.test

# Configurar variáveis específicas para testes
export NODE_ENV=test
export DATABASE_URL=postgresql://test:test@localhost:5433/sut_test
```

### 2. Deploy
```bash
cd deploy
docker compose -f docker-compose.test.yml up --build
```

### 3. Executar Testes
```bash
# Backend
cd api
cargo test

# Frontend
cd frontend
npm test

# Testes de integração
npm run test:integration
```

## 🏭 Ambiente de Produção

### 1. Preparação do Servidor

#### Instalar Docker
```bash
# Ubuntu/Debian
curl -fsSL https://get.docker.com -o get-docker.sh
sh get-docker.sh

# CentOS/RHEL
yum install -y docker
systemctl enable docker
systemctl start docker
```

#### Instalar Docker Compose
```bash
# Download da versão mais recente
curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
chmod +x /usr/local/bin/docker-compose
```

### 2. Configuração de Ambiente

#### Criar Arquivo de Ambiente
```bash
# /opt/sut/.env.prod
# Database
POSTGRES_DB=sut_prod
POSTGRES_USER=sut_user
POSTGRES_PASSWORD=<strong_password>

# API
PG_DSN=postgresql://sut_user:<strong_password>@postgres:5432/sut_prod
KEYCLOAK_ISSUER=https://auth.example.com/realms/sut
KEYCLOAK_JWKS=https://auth.example.com/realms/sut/protocol/openid-connect/certs
KEYCLOAK_AUDIENCE=sut-api
JWT_LEEWAY_SECS=30

# Vault
VAULT_ADDR=https://vault.example.com
VAULT_TOKEN=<vault_token>

# Security
METRICS_TOKEN=<metrics_token>
CORS_ALLOWED_ORIGINS=https://app.example.com

# Frontend
VITE_API_BASE=https://api.example.com
VITE_KC_URL=https://auth.example.com
VITE_KC_REALM=sut
VITE_KC_CLIENT=sut-frontend
```

### 3. Deploy com Docker Compose

#### Arquivo de Produção
```yaml
# docker-compose.prod.yml
version: '3.8'

services:
  postgres:
    image: postgres:15-alpine
    environment:
      POSTGRES_DB: ${POSTGRES_DB}
      POSTGRES_USER: ${POSTGRES_USER}
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./init.sql:/docker-entrypoint-initdb.d/init.sql
    networks:
      - sut-network
    restart: unless-stopped

  api:
    build:
      context: ..
      dockerfile: api/Dockerfile
    environment:
      PG_DSN: ${PG_DSN}
      KEYCLOAK_ISSUER: ${KEYCLOAK_ISSUER}
      KEYCLOAK_JWKS: ${KEYCLOAK_JWKS}
      KEYCLOAK_AUDIENCE: ${KEYCLOAK_AUDIENCE}
      JWT_LEEWAY_SECS: ${JWT_LEEWAY_SECS}
      VAULT_ADDR: ${VAULT_ADDR}
      VAULT_TOKEN: ${VAULT_TOKEN}
      METRICS_TOKEN: ${METRICS_TOKEN}
      CORS_ALLOWED_ORIGINS: ${CORS_ALLOWED_ORIGINS}
    depends_on:
      - postgres
    networks:
      - sut-network
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  frontend:
    build:
      context: ..
      dockerfile: frontend/Dockerfile
    environment:
      VITE_API_BASE: ${VITE_API_BASE}
      VITE_KC_URL: ${VITE_KC_URL}
      VITE_KC_REALM: ${VITE_KC_REALM}
      VITE_KC_CLIENT: ${VITE_KC_CLIENT}
    depends_on:
      - api
    networks:
      - sut-network
    restart: unless-stopped

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
      - ./ssl:/etc/nginx/ssl
    depends_on:
      - frontend
      - api
    networks:
      - sut-network
    restart: unless-stopped

volumes:
  postgres_data:

networks:
  sut-network:
    driver: bridge
```

#### Deploy
```bash
# Fazer deploy
docker compose -f docker-compose.prod.yml up -d

# Verificar status
docker compose -f docker-compose.prod.yml ps

# Verificar logs
docker compose -f docker-compose.prod.yml logs -f
```

### 4. Configuração do Nginx

#### Nginx Configuration
```nginx
# nginx.conf
events {
    worker_connections 1024;
}

http {
    upstream api_backend {
        server api:8080;
    }

    upstream frontend_backend {
        server frontend:80;
    }

    server {
        listen 80;
        server_name app.example.com;
        return 301 https://$server_name$request_uri;
    }

    server {
        listen 443 ssl http2;
        server_name app.example.com;

        ssl_certificate /etc/nginx/ssl/cert.pem;
        ssl_certificate_key /etc/nginx/ssl/key.pem;
        ssl_protocols TLSv1.2 TLSv1.3;
        ssl_ciphers ECDHE-RSA-AES256-GCM-SHA512:DHE-RSA-AES256-GCM-SHA512:ECDHE-RSA-AES256-GCM-SHA384:DHE-RSA-AES256-GCM-SHA384;
        ssl_prefer_server_ciphers off;

        # Frontend
        location / {
            proxy_pass http://frontend_backend;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }

        # API
        location /v1/ {
            proxy_pass http://api_backend;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }

        # Health check
        location /health {
            proxy_pass http://api_backend;
            access_log off;
        }

        # Metrics (protegido)
        location /metrics {
            proxy_pass http://api_backend;
            allow 10.0.0.0/8;
            allow 172.16.0.0/12;
            allow 192.168.0.0/16;
            deny all;
        }
    }
}
```

## 🔄 CI/CD Pipeline

### GitHub Actions

#### Workflow de Build e Test
```yaml
# .github/workflows/build.yml
name: Build and Test

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  test-backend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          components: rustfmt, clippy
      
      - name: Cache cargo registry
        uses: actions/cache@v3
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Cache cargo index
        uses: actions/cache@v3
        with:
          path: ~/.cargo/git
          key: ${{ runner.os }}-cargo-index-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Cache cargo build
        uses: actions/cache@v3
        with:
          path: target
          key: ${{ runner.os }}-cargo-build-target-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Run tests
        run: cargo test --locked
      
      - name: Run clippy
        run: cargo clippy -- -D warnings
      
      - name: Run fmt check
        run: cargo fmt -- --check

  test-frontend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '18'
          cache: 'npm'
          cache-dependency-path: frontend/package-lock.json
      
      - name: Install dependencies
        run: |
          cd frontend
          npm ci
      
      - name: Run tests
        run: |
          cd frontend
          npm test
      
      - name: Run lint
        run: |
          cd frontend
          npm run lint
      
      - name: Build
        run: |
          cd frontend
          npm run build
```

#### Workflow de Deploy
```yaml
# .github/workflows/deploy.yml
name: Deploy

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    environment: production
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Deploy to production
        run: |
          # SSH para o servidor de produção
          ssh user@production-server << 'EOF'
            cd /opt/sut
            git pull origin main
            docker compose -f docker-compose.prod.yml down
            docker compose -f docker-compose.prod.yml up -d --build
            docker system prune -f
          EOF
```

### Jenkins Pipeline

#### Jenkinsfile
```groovy
pipeline {
    agent any
    
    environment {
        DOCKER_REGISTRY = 'registry.example.com'
        IMAGE_TAG = "${BUILD_NUMBER}"
    }
    
    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }
        
        stage('Build Backend') {
            steps {
                dir('api') {
                    sh 'cargo build --release'
                    sh 'cargo test'
                }
            }
        }
        
        stage('Build Frontend') {
            steps {
                dir('frontend') {
                    sh 'npm ci'
                    sh 'npm test'
                    sh 'npm run build'
                }
            }
        }
        
        stage('Build Docker Images') {
            steps {
                sh 'docker build -f api/Dockerfile -t ${DOCKER_REGISTRY}/sut-api:${IMAGE_TAG} .'
                sh 'docker build -f frontend/Dockerfile -t ${DOCKER_REGISTRY}/sut-frontend:${IMAGE_TAG} .'
            }
        }
        
        stage('Push Images') {
            steps {
                sh 'docker push ${DOCKER_REGISTRY}/sut-api:${IMAGE_TAG}'
                sh 'docker push ${DOCKER_REGISTRY}/sut-frontend:${IMAGE_TAG}'
            }
        }
        
        stage('Deploy') {
            when {
                branch 'main'
            }
            steps {
                sh 'docker compose -f docker-compose.prod.yml up -d'
            }
        }
    }
    
    post {
        always {
            cleanWs()
        }
        failure {
            emailext (
                subject: "Build Failed: ${env.JOB_NAME} - ${env.BUILD_NUMBER}",
                body: "Build failed. Check console output at ${env.BUILD_URL}",
                to: "devops@example.com"
            )
        }
    }
}
```

## 📊 Monitoramento

### Prometheus Configuration
```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'sut-api'
    static_configs:
      - targets: ['api:8080']
    metrics_path: '/metrics'
    basic_auth:
      username: 'metrics'
      password: 'dev-metrics-token'
    scrape_interval: 5s

  - job_name: 'postgres'
    static_configs:
      - targets: ['postgres:5432']

  - job_name: 'nginx'
    static_configs:
      - targets: ['nginx:80']
```

**Nota**: O Prometheus agora usa Basic Auth para acessar o endpoint `/metrics` da API, garantindo que apenas sistemas autorizados possam coletar métricas.

### Grafana Dashboards
```json
{
  "dashboard": {
    "title": "SUT System Overview",
    "panels": [
      {
        "title": "API Requests",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(http_requests_total[5m])",
            "legendFormat": "{{method}} {{status}}"
          }
        ]
      },
      {
        "title": "Response Time",
        "type": "graph",
        "targets": [
          {
            "expr": "histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m]))",
            "legendFormat": "95th percentile"
          }
        ]
      }
    ]
  }
}
```

## 🔒 Segurança

### SSL/TLS
```bash
# Gerar certificado SSL
openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
  -keyout /etc/nginx/ssl/key.pem \
  -out /etc/nginx/ssl/cert.pem

# Ou usar Let's Encrypt
certbot --nginx -d app.example.com
```

### Firewall
```bash
# Configurar UFW
ufw allow 22/tcp
ufw allow 80/tcp
ufw allow 443/tcp
ufw enable
```

### Secrets Management
```bash
# Usar Docker Secrets
echo "mysecretpassword" | docker secret create postgres_password -

# Ou usar Vault
vault kv put secret/sut/database password=mysecretpassword
```

## 🔄 Backup e Recovery

### Backup do Banco de Dados
```bash
#!/bin/bash
# backup.sh

DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_DIR="/opt/backups"
DB_NAME="sut_prod"

# Criar backup
docker exec postgres pg_dump -U sut_user $DB_NAME > $BACKUP_DIR/sut_$DATE.sql

# Comprimir
gzip $BACKUP_DIR/sut_$DATE.sql

# Remover backups antigos (manter últimos 30 dias)
find $BACKUP_DIR -name "sut_*.sql.gz" -mtime +30 -delete
```

### Restore do Banco de Dados
```bash
#!/bin/bash
# restore.sh

BACKUP_FILE=$1
DB_NAME="sut_prod"

if [ -z "$BACKUP_FILE" ]; then
    echo "Usage: $0 <backup_file>"
    exit 1
fi

# Restaurar backup
gunzip -c $BACKUP_FILE | docker exec -i postgres psql -U sut_user -d $DB_NAME
```

### Cron Job para Backup
```bash
# Adicionar ao crontab
0 2 * * * /opt/sut/scripts/backup.sh
```

## 🚨 Troubleshooting

### Problemas Comuns

#### Container não inicia
```bash
# Verificar logs
docker logs <container_name>

# Verificar recursos
docker stats

# Verificar espaço em disco
df -h
```

#### Problemas de conectividade
```bash
# Verificar rede
docker network ls
docker network inspect sut-network

# Testar conectividade
docker exec -it api ping postgres
```

#### Problemas de performance
```bash
# Verificar uso de CPU e memória
docker stats

# Verificar logs de aplicação
docker logs api | grep ERROR

# Verificar métricas (com autenticação)
curl -u metrics:dev-metrics-token http://localhost:8080/metrics
```

#### Problemas com função `unaccent`
```bash
# Se houver erro relacionado à função unaccent, o projeto foi refatorado
# Verificar se a migração foi executada corretamente
docker logs deploy-api-1 | grep migration

# A função normalize_text substitui o unaccent
docker exec deploy-postgres-1 psql -U sut -d sut -c "SELECT normalize_text('Teste');"
```

#### Problemas de dependências do frontend
```bash
# Verificar logs do frontend
docker logs deploy-frontend-1

# Se houver problemas de versão, todas foram corrigidas
# Verificar se o container está rodando
docker ps | grep frontend
```

### Logs Centralizados
```yaml
# docker-compose.prod.yml
services:
  api:
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"
  
  frontend:
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"
```

## 📈 Escalabilidade

### Horizontal Scaling
```yaml
# docker-compose.scale.yml
services:
  api:
    deploy:
      replicas: 3
    environment:
      - PG_DSN=postgresql://sut_user:password@postgres:5432/sut_prod

  nginx:
    volumes:
      - ./nginx-load-balancer.conf:/etc/nginx/nginx.conf
```

### Load Balancer Configuration
```nginx
# nginx-load-balancer.conf
upstream api_backend {
    server api_1:8080;
    server api_2:8080;
    server api_3:8080;
}
```

### Database Scaling
```yaml
# docker-compose.db-cluster.yml
services:
  postgres-primary:
    image: postgres:15-alpine
    environment:
      POSTGRES_REPLICATION_MODE: master
      POSTGRES_REPLICATION_USER: replicator
      POSTGRES_REPLICATION_PASSWORD: replicator_password

  postgres-replica:
    image: postgres:15-alpine
    environment:
      POSTGRES_REPLICATION_MODE: slave
      POSTGRES_MASTER_HOST: postgres-primary
      POSTGRES_REPLICATION_USER: replicator
      POSTGRES_REPLICATION_PASSWORD: replicator_password
```

## 🔧 Manutenção

### Atualizações
```bash
# Atualizar aplicação
git pull origin main
docker compose -f docker-compose.prod.yml down
docker compose -f docker-compose.prod.yml up -d --build

# Atualizar dependências
docker compose -f docker-compose.prod.yml pull
docker compose -f docker-compose.prod.yml up -d
```

### Limpeza
```bash
# Limpar containers parados
docker container prune -f

# Limpar imagens não utilizadas
docker image prune -f

# Limpar volumes não utilizados
docker volume prune -f

# Limpeza completa
docker system prune -a -f
```

### Health Checks
```bash
#!/bin/bash
# health-check.sh

# Verificar API
curl -f http://localhost:8080/health || exit 1

# Verificar banco de dados
docker exec postgres pg_isready -U sut_user || exit 1

# Verificar frontend
curl -f http://localhost:80 || exit 1

echo "All services are healthy"
```

---

Este guia cobre todos os aspectos do deployment do SUT. Para mais detalhes específicos, consulte a documentação de cada componente.
