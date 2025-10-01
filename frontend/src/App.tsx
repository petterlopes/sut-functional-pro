
import React from 'react'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import {
  Box, Container, CircularProgress, Paper, Typography, CssBaseline, Alert, Button
} from '@mui/material'
import {
  useContacts,
  useCreateContact,
  useUpdateContact,
  useDeleteContact,
  useSearch,
  useContactDocument,
  useUpdateContactDocument,
} from './api/hooks'
import { useUnits, useDepartments } from './api/org'
import { useAuth } from './auth/KeycloakProvider'
import { getResolvedApiBase, clearResolvedApiBase } from './api/client'
import Dashboard from './components/Dashboard'
import Sidebar from './components/Sidebar'
import Header from './components/Header'
import ContatosPage from './components/pages/ContatosPage'
import ContatosPageClean from './components/pages/ContatosPageClean'
import LocalidadesPage from './components/pages/LocalidadesPage'
import DepartamentosPage from './components/pages/DepartamentosPage'
import LexicoPage from './components/pages/LexicoPage'
import GruposPage from './components/pages/GruposPage'
import ResponsaveisPage from './components/pages/ResponsaveisPage'
import SitesPage from './components/pages/SitesPage'
import { AppInitializer } from './infrastructure/AppInitializer'

const qc = new QueryClient()

export default function App(){
  return (
    <QueryClientProvider client={qc}>
      <Home />
    </QueryClientProvider>
  )
}

function Home(){
  const { ready, token, login, logout, user } = useAuth()
  const [page, setPage] = React.useState<'home'|'contatos'|'contatos-clean'|'localidades'|'departamentos'|'lexico'|'grupos'|'responsaveis'|'sites'>('home')
  const [sidebarOpen, setSidebarOpen] = React.useState(true)

  // Initialize Clean Architecture when ready
  React.useEffect(() => {
    if (ready) {
      getResolvedApiBase().then(apiBase => {
        AppInitializer.initialize(apiBase, token)
      }).catch(() => {
        // Fallback to default base if resolution fails
        AppInitializer.initialize('http://localhost:8080', token)
      })
    }
  }, [token, ready])

  if (!ready) return (
    <Box sx={{ 
      display: 'flex', 
      justifyContent: 'center', 
      alignItems: 'center', 
      minHeight: '100vh',
      backgroundColor: '#f8fafc'
    }}>
      <Box sx={{ textAlign: 'center' }}>
        <CircularProgress size={60} sx={{ color: '#667eea', mb: 2 }} />
        <Typography variant="h6" sx={{ color: '#64748b' }}>
          Carregando autenticação...
        </Typography>
      </Box>
    </Box>
  )

  const handlePageChange = (newPage: string) => {
    setPage(newPage as any)
  }

  const handleMenuClick = () => {
    setSidebarOpen(!sidebarOpen)
  }

  return (
    <Box sx={{ display: 'flex', minHeight: '100vh', backgroundColor: '#f8fafc' }}>
      <CssBaseline />
      
      {/* Header */}
      <Header 
        onMenuClick={handleMenuClick}
        onLogin={login}
        onLogout={logout}
        isAuthenticated={!!token}
        userName={user?.username}
      />

      {/* Sidebar */}
      <Sidebar 
        open={sidebarOpen}
        onClose={() => setSidebarOpen(false)}
        currentPage={page}
        onPageChange={handlePageChange}
      />

      {/* Main Content */}
      <Box 
        component="main" 
        sx={{ 
          flexGrow: 1, 
          pt: 8, // Account for header height
          ml: sidebarOpen ? '280px' : 0,
          transition: 'margin-left 0.3s ease',
          minHeight: '100vh'
        }}
      >
            {token ? (
              <>
                <ConnectivityBanner />
                {page === 'home' && <Dashboard />}
                {page === 'contatos' && <ContatosPage />}
                {page === 'contatos-clean' && <ContatosPageClean />}
                {page === 'localidades' && <LocalidadesPage />}
                {page === 'departamentos' && <DepartamentosPage />}
                {page === 'lexico' && <LexicoPage />}
                {page === 'grupos' && <GruposPage />}
                {page === 'responsaveis' && <ResponsaveisPage />}
                {page === 'sites' && <SitesPage />}
              </>
            ) : (
          <Container maxWidth="md" sx={{ py: 8 }}>
            <Paper sx={{ 
              p: 6, 
              textAlign: 'center',
              background: 'linear-gradient(135deg, #f8fafc 0%, #e2e8f0 100%)',
              border: '1px solid #e2e8f0',
              borderRadius: 3
            }}>
              <Typography variant="h4" sx={{ fontWeight: 'bold', mb: 2, color: '#1a202c' }}>
                Bem-vindo ao SUT Dashboard
              </Typography>
              <Typography variant="body1" sx={{ color: '#64748b', mb: 4 }}>
                Faça login para acessar o sistema de gerenciamento de contatos e dados organizacionais.
              </Typography>
          </Paper>
          </Container>
        )}
      </Box>
    </Box>
  )
}

function ConnectivityBanner(){
  const [base, setBase] = React.useState<string | null>(null)
  const [checking, setChecking] = React.useState(false)

  React.useEffect(()=>{ let mounted = true; getResolvedApiBase().then(b=>{ if(mounted) setBase(b) }).catch(()=>{}); return ()=>{ mounted=false } }, [])

  async function retry(){
    setChecking(true)
    clearResolvedApiBase()
    try{
      const b = await getResolvedApiBase(3000)
      setBase(b)
    }catch(e){ setBase(null) }
    setChecking(false)
  }

  if (base) return null
  return (
    <Alert severity="warning" sx={{ mb: 2 }} action={(
      <Button color="inherit" size="small" onClick={retry} disabled={checking}>{checking ? 'Probing...' : 'Retry'}</Button>
    )}>
      API base not resolved automatically — click Retry to probe (or open settings to override).
    </Alert>
  )
}

// Removed old components - now using modern Dashboard, Sidebar, and Header components
