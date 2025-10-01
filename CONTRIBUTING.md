# Guia de Contribuição - SUT

## 🤝 Como Contribuir

Obrigado por considerar contribuir para o Sistema Único de Telefonia (SUT)! Este documento fornece diretrizes e informações sobre como contribuir de forma eficaz.

## 📋 Índice

- [Código de Conduta](#código-de-conduta)
- [Como Contribuir](#como-contribuir)
- [Configuração do Ambiente](#configuração-do-ambiente)
- [Padrões de Código](#padrões-de-código)
- [Processo de Pull Request](#processo-de-pull-request)
- [Reportando Bugs](#reportando-bugs)
- [Sugerindo Melhorias](#sugerindo-melhorias)

## 📜 Código de Conduta

Este projeto segue um código de conduta para garantir um ambiente acolhedor e inclusivo para todos os contribuidores. Ao participar, você concorda em:

- Ser respeitoso e inclusivo
- Aceitar críticas construtivas
- Focar no que é melhor para a comunidade
- Mostrar empatia com outros membros da comunidade

## 🚀 Como Contribuir

### Tipos de Contribuição

1. **🐛 Reportar Bugs**
2. **💡 Sugerir Melhorias**
3. **📝 Melhorar Documentação**
4. **🔧 Implementar Features**
5. **🧪 Adicionar Testes**
6. **🎨 Melhorar UI/UX**

### Processo de Contribuição

1. **Fork** o repositório
2. **Clone** seu fork localmente
3. **Crie** uma branch para sua feature
4. **Implemente** suas mudanças
5. **Teste** suas mudanças
6. **Commit** suas mudanças
7. **Push** para sua branch
8. **Abra** um Pull Request

## 🛠️ Configuração do Ambiente

### Pré-requisitos

- Docker & Docker Compose v2
- Node.js 18+
- Rust (cargo)
- Git

### Configuração Inicial

```bash
# 1. Fork e clone o repositório
git clone https://github.com/SEU_USUARIO/sut-functional-pro.git
cd sut-functional-pro

# 2. Adicionar upstream
git remote add upstream https://github.com/ORIGINAL_REPO/sut-functional-pro.git

# 3. Configurar ambiente de desenvolvimento
cp .env.example .env.local

# 4. Subir infraestrutura
cd deploy
docker compose -f docker-compose.dev.yml up --build

# 5. Verificar se tudo está funcionando
curl http://localhost:8080/health
curl http://localhost:5173
```

## 📝 Padrões de Código

### Backend (Rust)

#### Formatação e Linting
```bash
# Usar rustfmt
cargo fmt

# Usar clippy
cargo clippy -- -D warnings

# Executar testes
cargo test
```

#### Convenções de Código

1. **Nomenclatura**:
   - `snake_case` para funções e variáveis
   - `PascalCase` para tipos e estruturas
   - `SCREAMING_SNAKE_CASE` para constantes

2. **Documentação**:
   ```rust
   /// Cria um novo contato com os dados fornecidos.
   /// 
   /// # Argumentos
   /// * `request` - Dados do contato a ser criado
   /// 
   /// # Retorna
   /// * `Result<Contact, DomainError>` - Contato criado ou erro
   pub async fn create_contact(request: CreateContactRequest) -> Result<Contact, DomainError> {
       // implementação
   }
   ```

3. **Clean Architecture**:
   - Mantenha as camadas separadas
   - Use traits para abstrações
   - Injeção de dependência via construtor

### Frontend (TypeScript/React)

#### Formatação e Linting
```bash
# ESLint
npm run lint
npm run lint:fix

# TypeScript check
npm run type-check

# Testes
npm test
```

#### Convenções de Código

1. **Nomenclatura**:
   - `camelCase` para funções e variáveis
   - `PascalCase` para componentes e tipos
   - `SCREAMING_SNAKE_CASE` para constantes

2. **Componentes**:
   ```typescript
   interface ContactGridProps {
     criteria: ContactSearchCriteria;
     onEdit: (contact: Contact) => void;
   }
   
   export function ContactGrid({ criteria, onEdit }: ContactGridProps) {
     // implementação
   }
   ```

3. **Clean Architecture**:
   - Mantenha as camadas separadas
   - Use interfaces para abstrações
   - Injeção de dependência via ServiceRegistry

## 🔄 Processo de Pull Request

### Antes de Abrir um PR

1. **Sincronize** com upstream:
   ```bash
   git fetch upstream
   git checkout main
   git merge upstream/main
   ```

2. **Crie** uma branch descritiva:
   ```bash
   git checkout -b feature/add-contact-validation
   git checkout -b fix/contact-search-bug
   git checkout -b docs/update-api-documentation
   ```

3. **Implemente** suas mudanças seguindo os padrões

4. **Teste** suas mudanças:
   ```bash
   # Backend
   cargo test
   cargo clippy
   cargo fmt -- --check
   
   # Frontend
   npm test
   npm run lint
   npm run type-check
   ```

5. **Commit** com mensagem descritiva:
   ```bash
   git add .
   git commit -m "feat: add contact validation rules"
   git commit -m "fix: resolve contact search pagination bug"
   git commit -m "docs: update API endpoint documentation"
   ```

### Convenções de Commit

Use o padrão [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` nova funcionalidade
- `fix:` correção de bug
- `docs:` mudanças na documentação
- `style:` formatação, ponto e vírgula, etc.
- `refactor:` refatoração de código
- `test:` adição ou correção de testes
- `chore:` mudanças em ferramentas, configurações, etc.

### Template de Pull Request

```markdown
## Descrição
Breve descrição das mudanças implementadas.

## Tipo de Mudança
- [ ] Bug fix (mudança que corrige um problema)
- [ ] Nova feature (mudança que adiciona funcionalidade)
- [ ] Breaking change (mudança que quebra compatibilidade)
- [ ] Documentação (mudanças apenas na documentação)

## Como Testar
1. Passo 1
2. Passo 2
3. Passo 3

## Checklist
- [ ] Meu código segue os padrões do projeto
- [ ] Realizei uma auto-revisão do meu código
- [ ] Comentei código complexo
- [ ] Minhas mudanças não geram warnings
- [ ] Adicionei testes que provam que minha correção é eficaz
- [ ] Testes novos e existentes passam localmente
- [ ] Qualquer mudança dependente foi documentada
```

## 🐛 Reportando Bugs

### Template de Bug Report

```markdown
## Descrição do Bug
Descrição clara e concisa do bug.

## Passos para Reproduzir
1. Vá para '...'
2. Clique em '...'
3. Role até '...'
4. Veja o erro

## Comportamento Esperado
Descrição clara do que deveria acontecer.

## Ambiente
- OS: [ex: Windows 10, macOS 12.0, Ubuntu 20.04]
- Browser: [ex: Chrome 91, Firefox 89]
- Versão: [ex: 1.0.0]

## Logs
```
Cole logs relevantes aqui
```
```

## 💡 Sugerindo Melhorias

### Template de Feature Request

```markdown
## Resumo da Feature
Descrição clara e concisa da feature desejada.

## Problema que Resolve
Qual problema esta feature resolve?

## Solução Proposta
Descrição clara da solução que você gostaria.

## Alternativas Consideradas
Descrição de soluções alternativas consideradas.
```

## 🧪 Testes

### Backend (Rust)

#### Testes Unitários
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_contact_success() {
        // Arrange
        let request = CreateContactRequest { /* ... */ };
        
        // Act
        let result = create_contact(request).await;
        
        // Assert
        assert!(result.is_ok());
    }
}
```

### Frontend (TypeScript/React)

#### Testes de Componentes
```typescript
import { render, screen, fireEvent } from '@testing-library/react';
import { ContactForm } from '../ContactForm';

describe('ContactForm', () => {
  it('renders form fields', () => {
    render(<ContactForm onSave={jest.fn()} />);
    
    expect(screen.getByLabelText(/nome completo/i)).toBeInTheDocument();
  });
});
```

## 🔍 Code Review

### Checklist de Review

- [ ] Código segue os padrões do projeto
- [ ] Testes foram adicionados/atualizados
- [ ] Documentação foi atualizada
- [ ] Não há código duplicado
- [ ] Performance não foi impactada negativamente
- [ ] Segurança foi considerada

## 📞 Suporte

### Canais de Comunicação

- **Issues**: Para bugs e feature requests
- **Discussions**: Para perguntas e discussões
- **Email**: dev@sut.example.com

---

Obrigado por contribuir para o SUT! 🚀