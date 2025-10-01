# 📦 Guia de Publicação no GitHub

Este documento fornece instruções passo a passo para publicar o projeto SUT no GitHub.

## ✅ Pré-requisitos

Antes de publicar, certifique-se de que:

- [x] Projeto está limpo (sem artefatos de build)
- [x] Documentação está completa
- [x] Código está funcionando localmente
- [x] `.gitignore` está configurado corretamente
- [x] Arquivos sensíveis foram removidos
- [x] Você tem uma conta no GitHub

## 📝 Passos para Publicação

### 1️⃣ Configurar Git Local

Se ainda não configurou o Git globalmente:

```bash
git config --global user.name "Seu Nome"
git config --global user.email "seu-email@example.com"
```

### 2️⃣ Inicializar Repositório Git

Se ainda não inicializou o repositório:

```bash
# No diretório raiz do projeto
git init
```

### 3️⃣ Adicionar Arquivos ao Repositório

```bash
# Adicionar todos os arquivos
git add .

# Verificar o que será commitado
git status

# Criar commit inicial
git commit -m "feat: initial commit - SUT project with Clean Architecture"
```

### 4️⃣ Criar Repositório no GitHub

1. Acesse https://github.com/new
2. Configure o repositório:
   - **Repository name**: `sut-functional-pro` (ou nome de sua escolha)
   - **Description**: "Sistema Unificado de Telefonia - Full-stack application with Rust + React implementing Clean Architecture"
   - **Visibility**: Public ou Private (sua escolha)
   - ⚠️ **NÃO** inicialize com README, .gitignore ou LICENSE (já temos esses arquivos)
3. Clique em "Create repository"

### 5️⃣ Conectar Repositório Local ao GitHub

Copie os comandos fornecidos pelo GitHub (algo similar a):

```bash
# Adicionar remote
git remote add origin https://github.com/seu-usuario/sut-functional-pro.git

# ou usando SSH (recomendado)
git remote add origin git@github.com:seu-usuario/sut-functional-pro.git

# Renomear branch para main (se necessário)
git branch -M main

# Push inicial
git push -u origin main
```

### 6️⃣ Configurar Proteções de Branch (Opcional)

No GitHub, vá para:
1. Settings → Branches
2. Add branch protection rule
3. Configure:
   - Branch name pattern: `main`
   - ✅ Require pull request reviews before merging
   - ✅ Require status checks to pass before merging
   - ✅ Require conversation resolution before merging

### 7️⃣ Adicionar Topics ao Repositório

No GitHub, adicione topics relevantes:
- `rust`
- `typescript`
- `react`
- `clean-architecture`
- `ddd`
- `solid-principles`
- `keycloak`
- `postgresql`
- `docker`
- `full-stack`
- `axum`
- `vite`

### 8️⃣ Configurar GitHub Actions (Opcional)

Crie `.github/workflows/ci.yml`:

```yaml
name: CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test-backend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Check
        run: cd api && cargo check
      - name: Test
        run: cd api && cargo test

  test-frontend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '22'
      - name: Install
        run: cd frontend && npm install
      - name: Build
        run: cd frontend && npm run build
```

### 9️⃣ Criar Issues Templates (Opcional)

Crie `.github/ISSUE_TEMPLATE/bug_report.md`:

```markdown
---
name: Bug Report
about: Create a report to help us improve
title: '[BUG] '
labels: bug
assignees: ''
---

**Describe the bug**
A clear and concise description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Go to '...'
2. Click on '....'
3. See error

**Expected behavior**
A clear and concise description of what you expected to happen.

**Screenshots**
If applicable, add screenshots to help explain your problem.

**Environment:**
 - OS: [e.g. Ubuntu 22.04]
 - Docker version: [e.g. 24.0.7]
 - Browser [e.g. chrome, safari]

**Additional context**
Add any other context about the problem here.
```

### 🔟 Criar Pull Request Template (Opcional)

Crie `.github/pull_request_template.md`:

```markdown
## Description
<!-- Describe your changes in detail -->

## Type of change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## How Has This Been Tested?
<!-- Describe the tests you ran to verify your changes -->

## Checklist:
- [ ] My code follows the style guidelines of this project
- [ ] I have performed a self-review of my own code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have made corresponding changes to the documentation
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing unit tests pass locally with my changes
```

## 🎨 Personalizar README

Atualize o README.md com:
- Seu nome/organização
- Link correto do repositório
- Badges corretos
- Informações de contato

## 🏷️ Criar Release (Opcional)

Quando estiver pronto para a primeira release:

```bash
# Criar tag
git tag -a v1.0.0 -m "Release v1.0.0 - Initial release"

# Push da tag
git push origin v1.0.0
```

No GitHub:
1. Vá para "Releases"
2. Click "Draft a new release"
3. Selecione a tag `v1.0.0`
4. Preencha o título e descrição (use o CHANGELOG.md)
5. Publique a release

## 📣 Divulgação

Após publicar, considere:

1. **Twitter/X**: Compartilhe o link do repositório
2. **LinkedIn**: Faça um post sobre o projeto
3. **Reddit**: Poste em r/rust, r/reactjs, r/programming
4. **Dev.to**: Escreva um artigo sobre o projeto
5. **Hacker News**: Submeta o link

## 🔄 Workflow de Desenvolvimento

Para contribuições futuras:

```bash
# Criar nova branch
git checkout -b feature/nova-funcionalidade

# Fazer alterações e commits
git add .
git commit -m "feat: adiciona nova funcionalidade"

# Push da branch
git push origin feature/nova-funcionalidade

# Criar Pull Request no GitHub
```

## 🎯 Próximos Passos Após Publicação

1. ⭐ Adicione estrelas ao projeto para visibilidade
2. 📊 Configure GitHub Insights para analytics
3. 🤖 Configure Dependabot para atualizações de dependências
4. 📝 Mantenha o CHANGELOG.md atualizado
5. 🐛 Responda issues e pull requests regularmente
6. 📢 Promova o projeto nas redes sociais

## ⚠️ Importante

**NUNCA comite:**
- Arquivos `.env` com credenciais reais
- Tokens ou senhas
- Chaves privadas (`.pem`, `.key`)
- Dados sensíveis
- Backups de banco de dados com dados reais

**Sempre:**
- Use `env.example` como template
- Mantenha credenciais em variáveis de ambiente
- Use GitHub Secrets para CI/CD
- Revise antes de fazer push

---

**Pronto!** 🎉 Seu projeto está agora publicado no GitHub e pronto para receber contribuições!

## 📞 Suporte

Se tiver problemas durante a publicação:
- 📖 [Documentação do GitHub](https://docs.github.com)
- 💬 [GitHub Community](https://github.community)
- 🆘 [Stack Overflow](https://stackoverflow.com/questions/tagged/github)

