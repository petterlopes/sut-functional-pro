# Documentação do Frontend SUT

## 🌐 Visão Geral

O frontend do Sistema Único de Telefonia (SUT) é uma Single Page Application (SPA) construída em React com TypeScript, seguindo os princípios de Clean Architecture. A aplicação utiliza Material-UI para interface, React Query para gerenciamento de estado e Keycloak para autenticação.

## 🏗️ Arquitetura

### Clean Architecture Frontend

```
frontend/src/
├── domain/                   # Camada de Domínio
│   ├── entities/            # Entidades TypeScript
│   └── repositories/        # Interfaces de repositório
├── application/             # Camada de Aplicação
│   ├── use-cases/          # Casos de uso
│   └── services/           # Serviços de aplicação
├── infrastructure/          # Camada de Infraestrutura
│   ├── api/                # Clientes HTTP
│   ├── repositories/       # Implementações de repositório
│   └── di/                 # Injeção de dependência
└── presentation/           # Camada de Apresentação
    ├── components/         # Componentes React
    ├── pages/              # Páginas
    └── hooks/              # Hooks customizados
```

## 🚀 Início Rápido

### Pré-requisitos
- Node.js 18+
- npm ou pnpm

### Instalação
```bash
cd frontend
npm install
```

### Desenvolvimento
```bash
npm run dev
```

### Build
```bash
npm run build
```

### Preview
```bash
npm run preview
```

## 🔧 Configuração

### Variáveis de Ambiente

Crie um arquivo `.env.local`:

```bash
# Keycloak
VITE_KC_URL=http://localhost:8081
VITE_KC_REALM=sut
VITE_KC_CLIENT=sut-frontend

# API
VITE_API_BASE=http://localhost:8080
```

### Configuração de Desenvolvimento

Para desenvolvimento local com serviços Docker:

```bash
# .env.local
VITE_KC_URL=http://localhost:8081
VITE_API_BASE=http://localhost:8080
```

Para desenvolvimento com frontend local e serviços Docker:

```bash
# .env.local
VITE_KC_URL=http://localhost:8081
VITE_API_BASE=http://localhost:8080
```

## 🏛️ Estrutura de Camadas

### Domain Layer

#### Entidades
```typescript
// src/domain/entities/Contact.ts
export interface Contact {
  id: string;
  fullName: string;
  contactType: 'PERSON' | 'COMPANY';
  status: 'ACTIVE' | 'INACTIVE';
  document?: string;
  unitId?: string;
  departmentId?: string;
  etag: string;
  createdAt: string;
  updatedAt: string;
}

export interface ContactSearchCriteria {
  fullName?: string;
  contactType?: string;
  status?: string;
  unitId?: string;
  departmentId?: string;
  limit?: number;
  offset?: number;
}

export interface ContactSearchResponse {
  items: Contact[];
  total: number;
}
```

#### Interfaces de Repositório
```typescript
// src/domain/repositories/IContactRepository.ts
export interface IContactRepository {
  findById(id: string): Promise<Contact | null>;
  findAll(criteria: ContactSearchCriteria): Promise<ContactSearchResponse>;
  create(contact: CreateContactRequest): Promise<Contact>;
  update(id: string, contact: UpdateContactRequest, etag: string): Promise<Contact>;
  delete(id: string): Promise<void>;
  getStatistics(): Promise<ContactStatistics>;
}
```

### Application Layer

#### Casos de Uso
```typescript
// src/application/use-cases/contact/CreateContactUseCase.ts
export class CreateContactUseCase {
  constructor(private contactRepository: IContactRepository) {}

  async execute(request: CreateContactRequest): Promise<Contact> {
    // Validação de entrada
    if (!request.fullName) {
      throw new Error('Nome completo é obrigatório');
    }

    // Criação do contato
    const contact = await this.contactRepository.create(request);
    
    return contact;
  }
}
```

#### Serviços de Aplicação
```typescript
// src/application/services/ContactService.ts
export class ContactService {
  constructor(
    private createContactUseCase: CreateContactUseCase,
    private updateContactUseCase: UpdateContactUseCase,
    private deleteContactUseCase: DeleteContactUseCase,
    private getContactsUseCase: GetContactsUseCase,
    private getContactStatisticsUseCase: GetContactStatisticsUseCase
  ) {}

  async createContact(request: CreateContactRequest): Promise<Contact> {
    return this.createContactUseCase.execute(request);
  }

  async updateContact(id: string, request: UpdateContactRequest, etag: string): Promise<Contact> {
    return this.updateContactUseCase.execute(id, request, etag);
  }

  async deleteContact(id: string): Promise<void> {
    return this.deleteContactUseCase.execute(id);
  }

  async getContacts(criteria: ContactSearchCriteria): Promise<ContactSearchResponse> {
    return this.getContactsUseCase.execute(criteria);
  }

  async getContactStatistics(): Promise<ContactStatistics> {
    return this.getContactStatisticsUseCase.execute();
  }
}
```

### Infrastructure Layer

#### Cliente HTTP
```typescript
// src/infrastructure/api/AxiosApiClient.ts
export class AxiosApiClient implements IApiClient {
  private client: AxiosInstance;

  constructor(baseURL: string, token?: string) {
    this.client = axios.create({
      baseURL,
      timeout: 10000,
      headers: {
        'Content-Type': 'application/json',
        ...(token && { Authorization: `Bearer ${token}` })
      }
    });

    this.setupInterceptors();
  }

  private setupInterceptors(): void {
    // Request interceptor
    this.client.interceptors.request.use(
      (config) => config,
      (error) => Promise.reject(error)
    );

    // Response interceptor
    this.client.interceptors.response.use(
      (response) => response,
      (error) => {
        const apiError = {
          message: error.response?.data?.message || error.message,
          status: error.response?.status || 500,
          data: error.response?.data
        };
        return Promise.reject(apiError);
      }
    );
  }

  async get<T>(url: string, config?: ApiRequestConfig): Promise<ApiResponse<T>> {
    const response = await this.client.get<T>(url, {
      headers: config?.headers,
      params: config?.params
    });
    return this.transformResponse(response);
  }

  async post<T>(url: string, data?: any, config?: ApiRequestConfig): Promise<ApiResponse<T>> {
    const response = await this.client.post<T>(url, data, {
      headers: config?.headers,
      params: config?.params
    });
    return this.transformResponse(response);
  }

  async put<T>(url: string, data?: any, config?: ApiRequestConfig): Promise<ApiResponse<T>> {
    const response = await this.client.put<T>(url, data, {
      headers: config?.headers,
      params: config?.params
    });
    return this.transformResponse(response);
  }

  async patch<T>(url: string, data?: any, config?: ApiRequestConfig): Promise<ApiResponse<T>> {
    const response = await this.client.patch<T>(url, data, {
      headers: config?.headers,
      params: config?.params
    });
    return this.transformResponse(response);
  }

  async delete<T>(url: string, config?: ApiRequestConfig): Promise<ApiResponse<T>> {
    const response = await this.client.delete<T>(url, {
      headers: config?.headers,
      params: config?.params
    });
    return this.transformResponse(response);
  }

  private transformResponse<T>(response: AxiosResponse<T>): ApiResponse<T> {
    return {
      data: response.data,
      status: response.status,
      headers: response.headers as Record<string, string>
    };
  }

  updateToken(token: string): void {
    this.client.defaults.headers.Authorization = `Bearer ${token}`;
  }

  removeToken(): void {
    delete this.client.defaults.headers.Authorization;
  }
}
```

#### Implementação de Repositório
```typescript
// src/infrastructure/repositories/ContactRepository.ts
export class ContactRepository implements IContactRepository {
  constructor(private apiClient: IApiClient) {}

  async findById(id: string): Promise<Contact | null> {
    try {
      const response = await this.apiClient.get<Contact>(`/v1/contacts/${id}`);
      return response.data;
    } catch (error: any) {
      if (error.status === 404) {
        return null;
      }
      throw error;
    }
  }

  async findAll(criteria: ContactSearchCriteria): Promise<ContactSearchResponse> {
    const params = new URLSearchParams();
    
    if (criteria.fullName) params.append('full_name', criteria.fullName);
    if (criteria.contactType) params.append('contact_type', criteria.contactType);
    if (criteria.status) params.append('status', criteria.status);
    if (criteria.unitId) params.append('unit_id', criteria.unitId);
    if (criteria.departmentId) params.append('department_id', criteria.departmentId);
    if (criteria.limit) params.append('limit', criteria.limit.toString());
    if (criteria.offset) params.append('offset', criteria.offset.toString());

    const response = await this.apiClient.get<ContactSearchResponse>(`/v1/contacts?${params}`);
    return response.data;
  }

  async create(request: CreateContactRequest): Promise<Contact> {
    const response = await this.apiClient.post<Contact>('/v1/contacts', request);
    return response.data;
  }

  async update(id: string, request: UpdateContactRequest, etag: string): Promise<Contact> {
    const response = await this.apiClient.patch<Contact>(`/v1/contacts/${id}`, request, {
      headers: { 'If-Match': etag }
    });
    return response.data;
  }

  async delete(id: string): Promise<void> {
    await this.apiClient.delete(`/v1/contacts/${id}`);
  }

  async getStatistics(): Promise<ContactStatistics> {
    const response = await this.apiClient.get<ContactStatistics>('/v1/contacts/statistics');
    return response.data;
  }
}
```

#### Injeção de Dependência
```typescript
// src/infrastructure/di/ServiceRegistry.ts
export class ServiceRegistry {
  private static instance: ServiceRegistry;
  private services: Map<string, any> = new Map();

  public static getInstance(): ServiceRegistry {
    if (!ServiceRegistry.instance) {
      ServiceRegistry.instance = new ServiceRegistry();
    }
    return ServiceRegistry.instance;
  }

  public register<T>(key: string, service: T): void {
    this.services.set(key, service);
  }

  public get<T>(key: string): T {
    const service = this.services.get(key);
    if (!service) {
      throw new Error(`Service ${key} not found`);
    }
    return service as T;
  }

  public has(key: string): boolean {
    return this.services.has(key);
  }

  public clear(): void {
    this.services.clear();
  }
}

// Configuração de serviços
export function configureServices(): void {
  const registry = ServiceRegistry.getInstance();

  // API Client
  const apiClient = new AxiosApiClient(
    import.meta.env.VITE_API_BASE || 'http://localhost:8080'
  );
  registry.register('apiClient', apiClient);

  // Repositories
  const contactRepository = new ContactRepository(apiClient);
  registry.register('contactRepository', contactRepository);

  // Use Cases
  const createContactUseCase = new CreateContactUseCase(contactRepository);
  const updateContactUseCase = new UpdateContactUseCase(contactRepository);
  const deleteContactUseCase = new DeleteContactUseCase(contactRepository);
  const getContactsUseCase = new GetContactsUseCase(contactRepository);
  const getContactStatisticsUseCase = new GetContactStatisticsUseCase(contactRepository);

  registry.register('createContactUseCase', createContactUseCase);
  registry.register('updateContactUseCase', updateContactUseCase);
  registry.register('deleteContactUseCase', deleteContactUseCase);
  registry.register('getContactsUseCase', getContactsUseCase);
  registry.register('getContactStatisticsUseCase', getContactStatisticsUseCase);

  // Services
  const contactService = new ContactService(
    createContactUseCase,
    updateContactUseCase,
    deleteContactUseCase,
    getContactsUseCase,
    getContactStatisticsUseCase
  );
  registry.register('contactService', contactService);
}
```

### Presentation Layer

#### Hooks Customizados
```typescript
// src/presentation/hooks/useContactUseCasesClean.ts
export function useContactUseCasesClean() {
  const registry = ServiceRegistry.getInstance();

  const createContactUseCase = registry.get<CreateContactUseCase>('createContactUseCase');
  const updateContactUseCase = registry.get<UpdateContactUseCase>('updateContactUseCase');
  const deleteContactUseCase = registry.get<DeleteContactUseCase>('deleteContactUseCase');
  const getContactsUseCase = registry.get<GetContactsUseCase>('getContactsUseCase');
  const getContactStatisticsUseCase = registry.get<GetContactStatisticsUseCase>('getContactStatisticsUseCase');

  return {
    createContactUseCase,
    updateContactUseCase,
    deleteContactUseCase,
    getContactsUseCase,
    getContactStatisticsUseCase
  };
}

// Hook com React Query
export function useContacts(criteria: ContactSearchCriteria) {
  const { getContactsUseCase } = useContactUseCasesClean();

  return useQuery({
    queryKey: ['contacts', criteria],
    queryFn: () => getContactsUseCase.execute(criteria),
    staleTime: 5 * 60 * 1000, // 5 minutos
  });
}

export function useContact(id: string) {
  const { getContactsUseCase } = useContactUseCasesClean();

  return useQuery({
    queryKey: ['contact', id],
    queryFn: async () => {
      const response = await getContactsUseCase.execute({ limit: 1, offset: 0 });
      return response.items.find(contact => contact.id === id) || null;
    },
    enabled: !!id,
  });
}

export function useCreateContact() {
  const { createContactUseCase } = useContactUseCasesClean();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (request: CreateContactRequest) => createContactUseCase.execute(request),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['contacts'] });
    },
  });
}

export function useUpdateContact() {
  const { updateContactUseCase } = useContactUseCasesClean();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, request, etag }: { id: string; request: UpdateContactRequest; etag: string }) =>
      updateContactUseCase.execute(id, request, etag),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: ['contacts'] });
      queryClient.invalidateQueries({ queryKey: ['contact', id] });
    },
  });
}

export function useDeleteContact() {
  const { deleteContactUseCase } = useContactUseCasesClean();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: string) => deleteContactUseCase.execute(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['contacts'] });
    },
  });
}
```

#### Componentes
```typescript
// src/presentation/components/ContactGrid.tsx
import { DataGrid, GridColDef, GridActionsCellItem } from '@mui/x-data-grid';
import { Edit, Delete } from '@mui/icons-material';
import { useContacts, useDeleteContact } from '../hooks/useContactUseCasesClean';

interface ContactGridProps {
  criteria: ContactSearchCriteria;
  onEdit: (contact: Contact) => void;
}

export function ContactGrid({ criteria, onEdit }: ContactGridProps) {
  const { data, isLoading, error } = useContacts(criteria);
  const deleteContact = useDeleteContact();

  const handleDelete = async (id: string) => {
    if (window.confirm('Tem certeza que deseja excluir este contato?')) {
      try {
        await deleteContact.mutateAsync(id);
      } catch (error) {
        console.error('Erro ao excluir contato:', error);
      }
    }
  };

  const columns: GridColDef[] = [
    { field: 'fullName', headerName: 'Nome Completo', width: 200 },
    { field: 'contactType', headerName: 'Tipo', width: 120 },
    { field: 'status', headerName: 'Status', width: 120 },
    { field: 'document', headerName: 'Documento', width: 150 },
    { field: 'createdAt', headerName: 'Criado em', width: 180 },
    {
      field: 'actions',
      type: 'actions',
      headerName: 'Ações',
      width: 100,
      getActions: (params) => [
        <GridActionsCellItem
          icon={<Edit />}
          label="Editar"
          onClick={() => onEdit(params.row)}
        />,
        <GridActionsCellItem
          icon={<Delete />}
          label="Excluir"
          onClick={() => handleDelete(params.row.id)}
        />,
      ],
    },
  ];

  if (error) {
    return <div>Erro ao carregar contatos: {error.message}</div>;
  }

  return (
    <DataGrid
      rows={data?.items || []}
      columns={columns}
      loading={isLoading}
      pageSizeOptions={[10, 25, 50]}
      initialState={{
        pagination: {
          paginationModel: { page: 0, pageSize: 25 },
        },
      }}
      disableRowSelectionOnClick
    />
  );
}
```

#### Páginas
```typescript
// src/presentation/pages/ContactsPage.tsx
import { useState } from 'react';
import { Box, Button, Paper, Typography } from '@mui/material';
import { Add } from '@mui/icons-material';
import { ContactGrid } from '../components/ContactGrid';
import { ContactForm } from '../components/ContactForm';
import { ContactSearchCriteria } from '../../domain/entities/Contact';

export function ContactsPage() {
  const [criteria, setCriteria] = useState<ContactSearchCriteria>({
    limit: 25,
    offset: 0,
  });
  const [editingContact, setEditingContact] = useState<Contact | null>(null);
  const [showForm, setShowForm] = useState(false);

  const handleEdit = (contact: Contact) => {
    setEditingContact(contact);
    setShowForm(true);
  };

  const handleCreate = () => {
    setEditingContact(null);
    setShowForm(true);
  };

  const handleFormClose = () => {
    setShowForm(false);
    setEditingContact(null);
  };

  return (
    <Box sx={{ p: 3 }}>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
        <Typography variant="h4">Contatos</Typography>
        <Button
          variant="contained"
          startIcon={<Add />}
          onClick={handleCreate}
        >
          Novo Contato
        </Button>
      </Box>

      <Paper sx={{ p: 2, mb: 2 }}>
        <ContactGrid criteria={criteria} onEdit={handleEdit} />
      </Paper>

      {showForm && (
        <ContactForm
          contact={editingContact}
          onClose={handleFormClose}
        />
      )}
    </Box>
  );
}
```

## 🔐 Autenticação

### Configuração do Keycloak
```typescript
// src/infrastructure/auth/KeycloakService.ts
import Keycloak from 'keycloak-js';

const keycloakConfig = {
  url: import.meta.env.VITE_KC_URL || 'http://localhost:8081',
  realm: import.meta.env.VITE_KC_REALM || 'sut',
  clientId: import.meta.env.VITE_KC_CLIENT || 'sut-frontend',
};

export const keycloak = new Keycloak(keycloakConfig);

export async function initializeKeycloak(): Promise<boolean> {
  try {
    const authenticated = await keycloak.init({
      onLoad: 'check-sso',
      silentCheckSsoRedirectUri: window.location.origin + '/silent-check-sso.html',
    });

    if (authenticated) {
      // Atualizar token do cliente API
      const apiClient = ServiceRegistry.getInstance().get<AxiosApiClient>('apiClient');
      apiClient.updateToken(keycloak.token!);
    }

    return authenticated;
  } catch (error) {
    console.error('Erro ao inicializar Keycloak:', error);
    return false;
  }
}
```

### Hook de Autenticação
```typescript
// src/presentation/hooks/useAuth.ts
import { useState, useEffect } from 'react';
import { keycloak, initializeKeycloak } from '../../infrastructure/auth/KeycloakService';

export function useAuth() {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [user, setUser] = useState<any>(null);

  useEffect(() => {
    const init = async () => {
      try {
        const authenticated = await initializeKeycloak();
        setIsAuthenticated(authenticated);
        
        if (authenticated) {
          setUser({
            id: keycloak.subject,
            username: keycloak.tokenParsed?.preferred_username,
            email: keycloak.tokenParsed?.email,
            roles: keycloak.tokenParsed?.realm_access?.roles || [],
          });
        }
      } catch (error) {
        console.error('Erro na autenticação:', error);
      } finally {
        setIsLoading(false);
      }
    };

    init();
  }, []);

  const login = () => {
    keycloak.login();
  };

  const logout = () => {
    keycloak.logout();
  };

  const hasRole = (role: string) => {
    return keycloak.hasRealmRole(role);
  };

  return {
    isAuthenticated,
    isLoading,
    user,
    login,
    logout,
    hasRole,
  };
}
```

## 🎨 UI/UX

### Material-UI
A aplicação utiliza Material-UI para componentes de interface:

```typescript
// src/presentation/theme/theme.ts
import { createTheme } from '@mui/material/styles';

export const theme = createTheme({
  palette: {
    primary: {
      main: '#1976d2',
    },
    secondary: {
      main: '#dc004e',
    },
  },
  typography: {
    fontFamily: '"Roboto", "Helvetica", "Arial", sans-serif',
  },
  components: {
    MuiButton: {
      styleOverrides: {
        root: {
          textTransform: 'none',
        },
      },
    },
  },
});
```

### Layout Principal
```typescript
// src/presentation/components/Layout.tsx
import { AppBar, Toolbar, Typography, Box, Container } from '@mui/material';
import { useAuth } from '../hooks/useAuth';

interface LayoutProps {
  children: React.ReactNode;
}

export function Layout({ children }: LayoutProps) {
  const { user, logout } = useAuth();

  return (
    <Box sx={{ display: 'flex', flexDirection: 'column', minHeight: '100vh' }}>
      <AppBar position="static">
        <Toolbar>
          <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
            SUT - Sistema Único de Telefonia
          </Typography>
          {user && (
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
              <Typography variant="body2">
                Olá, {user.username}
              </Typography>
              <Button color="inherit" onClick={logout}>
                Sair
              </Button>
            </Box>
          )}
        </Toolbar>
      </AppBar>
      
      <Container component="main" sx={{ flexGrow: 1, py: 3 }}>
        {children}
      </Container>
    </Box>
  );
}
```

## 📊 Gerenciamento de Estado

### React Query
```typescript
// src/presentation/providers/QueryProvider.tsx
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: 1,
      refetchOnWindowFocus: false,
      staleTime: 5 * 60 * 1000, // 5 minutos
    },
  },
});

interface QueryProviderProps {
  children: React.ReactNode;
}

export function QueryProvider({ children }: QueryProviderProps) {
  return (
    <QueryClientProvider client={queryClient}>
      {children}
      <ReactQueryDevtools initialIsOpen={false} />
    </QueryClientProvider>
  );
}
```

## 🧪 Testes

### Testes de Componentes
```typescript
// src/presentation/components/__tests__/ContactGrid.test.tsx
import { render, screen } from '@testing-library/react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { ContactGrid } from '../ContactGrid';

const createTestQueryClient = () => new QueryClient({
  defaultOptions: {
    queries: { retry: false },
    mutations: { retry: false },
  },
});

const TestWrapper = ({ children }: { children: React.ReactNode }) => {
  const queryClient = createTestQueryClient();
  return (
    <QueryClientProvider client={queryClient}>
      {children}
    </QueryClientProvider>
  );
};

describe('ContactGrid', () => {
  it('renders contact grid', () => {
    render(
      <TestWrapper>
        <ContactGrid criteria={{}} onEdit={jest.fn()} />
      </TestWrapper>
    );
    
    expect(screen.getByRole('grid')).toBeInTheDocument();
  });
});
```

### Testes de Hooks
```typescript
// src/presentation/hooks/__tests__/useContacts.test.ts
import { renderHook, waitFor } from '@testing-library/react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { useContacts } from '../useContacts';

const createTestQueryClient = () => new QueryClient({
  defaultOptions: {
    queries: { retry: false },
  },
});

const wrapper = ({ children }: { children: React.ReactNode }) => {
  const queryClient = createTestQueryClient();
  return (
    <QueryClientProvider client={queryClient}>
      {children}
    </QueryClientProvider>
  );
};

describe('useContacts', () => {
  it('fetches contacts', async () => {
    const { result } = renderHook(() => useContacts({}), { wrapper });

    await waitFor(() => {
      expect(result.current.isSuccess).toBe(true);
    });
  });
});
```

## 🚀 Build e Deploy

### Build de Produção
```bash
npm run build
```

### Preview Local
```bash
npm run preview
```

### Docker
```dockerfile
# Dockerfile
FROM node:18-alpine AS build

WORKDIR /app
COPY package*.json ./
RUN npm ci --only=production

COPY . .
RUN npm run build

FROM nginx:alpine
COPY --from=build /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/nginx.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
```

### Docker Compose
```yaml
# docker-compose.yml
version: '3.8'

services:
  frontend:
    build: .
    ports:
      - "80:80"
    environment:
      - VITE_API_BASE=http://api:8080
      - VITE_KC_URL=http://keycloak:8080
    depends_on:
      - api
      - keycloak
```

## 📈 Performance

### Otimizações
- **Code Splitting**: Carregamento sob demanda de componentes
- **Lazy Loading**: Carregamento lazy de rotas
- **Memoização**: Uso de `useMemo` e `useCallback`
- **Virtual Scrolling**: Para listas grandes
- **Image Optimization**: Otimização de imagens

### Bundle Analysis
```bash
npm run build
npx vite-bundle-analyzer dist
```

## 🔧 Desenvolvimento

### Scripts Disponíveis
```json
{
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview",
    "test": "vitest",
    "test:ui": "vitest --ui",
    "lint": "eslint src --ext ts,tsx",
    "lint:fix": "eslint src --ext ts,tsx --fix",
    "type-check": "tsc --noEmit"
  }
}
```

### Hot Reload
O Vite fornece hot reload instantâneo para desenvolvimento.

### Debug
```bash
# Executar com logs detalhados
DEBUG=vite:* npm run dev
```

## 🐛 Troubleshooting

### Problemas Comuns

#### Erro de CORS
```bash
# Verificar se a API está configurada para aceitar requisições do frontend
# Verificar VITE_API_BASE
```

#### Erro de Autenticação
```bash
# Verificar configuração do Keycloak
# Verificar VITE_KC_URL, VITE_KC_REALM, VITE_KC_CLIENT
```

#### Erro de Build
```bash
# Limpar cache
rm -rf node_modules package-lock.json
npm install
```

### Logs de Debug
```typescript
// Habilitar logs detalhados
localStorage.setItem('debug', 'sut:*');
```

## 📚 Referências

- [React Documentation](https://react.dev/)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/)
- [Material-UI Documentation](https://mui.com/)
- [React Query Documentation](https://tanstack.com/query/latest)
- [Vite Documentation](https://vitejs.dev/)
- [Keycloak JS Adapter](https://www.keycloak.org/docs/latest/securing_apps/#_javascript_adapter)

---

Esta documentação cobre todos os aspectos do frontend SUT. Para mais detalhes, consulte o código fonte e os comentários inline.
