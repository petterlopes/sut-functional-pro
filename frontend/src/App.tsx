
import React from 'react'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import {
  AppBar, Toolbar, Typography, Button, Container, Box, Card, CardContent, TextField, IconButton, List, ListItem, ListItemText, ListItemSecondaryAction, Divider, MenuItem, Select, InputLabel, FormControl, CircularProgress, Paper, Chip,
  Dialog, DialogTitle, DialogContent, DialogContentText, DialogActions, Snackbar, Alert
} from '@mui/material'
import LogoutIcon from '@mui/icons-material/Logout'
import LoginIcon from '@mui/icons-material/Login'
import SettingsIcon from '@mui/icons-material/Settings'
import EditIcon from '@mui/icons-material/Edit'
import DeleteIcon from '@mui/icons-material/Delete'
import VisibilityIcon from '@mui/icons-material/Visibility'
import VisibilityOffIcon from '@mui/icons-material/VisibilityOff'
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
import LocalidadesPage from './pages/LocalidadesPage'
import DepartamentosPage from './pages/DepartamentosPage'
import LexicoPage from './pages/LexicoPage'
import GruposPage from './pages/GruposPage'
import ResponsaveisPage from './pages/ResponsaveisPage'
import SitesPage from './pages/SitesPage'

const qc = new QueryClient()

export default function App(){
  return (
    <QueryClientProvider client={qc}>
      <Home />
    </QueryClientProvider>
  )
}

function Home(){
  const { ready, token, login, logout } = useAuth()
  const [name, setName] = React.useState('')
  const [search, setSearch] = React.useState('')
  const [page, setPage] = React.useState<'home'|'localidades'|'departamentos'|'lexico'|'grupos'|'responsaveis'|'sites'>('home')
  const contactsQuery = useContacts(20)
  const create = useCreateContact()

  if (!ready) return (
    <Container maxWidth="sm" sx={{ mt: 8, textAlign: 'center' }}>
      <CircularProgress />
      <Typography variant="h6" sx={{ mt: 2 }}>carregando auth...</Typography>
    </Container>
  )

  return (
    <Box sx={{ minHeight: '100vh', bgcolor: 'background.default' }}>
      <AppBar position="static" color="primary">
        <Toolbar>
          <Typography variant="h6" sx={{ flexGrow: 1 }}>SUT Dashboard</Typography>
          <RuntimeStatus />
          <RuntimeConfigButton />
          {token ? (
            <Button color="inherit" startIcon={<LogoutIcon />} onClick={()=>logout()}>Logout</Button>
          ) : (
            <Button color="inherit" startIcon={<LoginIcon />} onClick={()=>login()}>Login</Button>
          )}
        </Toolbar>
      </AppBar>
      <Container maxWidth="lg" sx={{ py: 4 }}>
        {token ? (
          <>
            <ConnectivityBanner />
            <Box sx={{ mb: 2, display: 'flex', gap: 1, flexWrap: 'wrap' }}>
              <Button variant={page==='home' ? 'contained' : 'text'} onClick={()=>setPage('home')}>Home</Button>
              <Button variant={page==='localidades' ? 'contained' : 'text'} onClick={()=>setPage('localidades')}>Localidades</Button>
              <Button variant={page==='departamentos' ? 'contained' : 'text'} onClick={()=>setPage('departamentos')}>Departamentos</Button>
              <Button variant={page==='lexico' ? 'contained' : 'text'} onClick={()=>setPage('lexico')}>Léxico</Button>
              <Button variant={page==='grupos' ? 'contained' : 'text'} onClick={()=>setPage('grupos')}>Grupos</Button>
              <Button variant={page==='responsaveis' ? 'contained' : 'text'} onClick={()=>setPage('responsaveis')}>Responsáveis</Button>
              <Button variant={page==='sites' ? 'contained' : 'text'} onClick={()=>setPage('sites')}>Sites</Button>
            </Box>

            {page === 'home' && (
              <>
                <Card sx={{ mb: 3 }}>
                  <CardContent>
                    <Box sx={{ display: 'flex', flexDirection: { xs: 'column', md: 'row' }, gap: 2, alignItems: 'center' }}>
                      <Box sx={{ flex: 1 }}>
                        <TextField fullWidth label="Nome" value={name} onChange={e=>setName(e.target.value)} size="small" />
                      </Box>
                      <Box sx={{ width: { xs: '100%', md: 220 } }}>
                        <Button fullWidth variant="contained" color="primary"
                          onClick={()=> create.mutate({ fullName: name }, {
                            onSuccess: ()=>{
                              setName('')
                              contactsQuery.refetch()
                            }
                          })}
                          disabled={!name.trim() || create.isPending}
                        >
                          {create.isPending ? 'Criando...' : 'Criar'}
                        </Button>
                      </Box>
                    </Box>
                  </CardContent>
                </Card>

                <SearchBox value={search} onChange={setSearch} />

                <Paper sx={{ mt: 3, p: 2 }}>
                  {contactsQuery.isLoading ? (
                    <Box sx={{ display: 'flex', justifyContent: 'center', py: 4 }}><CircularProgress /></Box>
                  ) : (
                    <List>
                      {(contactsQuery.data?.items || []).map((contact: any)=>(
                        <ContactItem key={contact.id} contact={contact} />
                      ))}
                    </List>
                  )}
                </Paper>

                <Box sx={{ display: 'grid', gridTemplateColumns: { xs: '1fr', md: '1fr 1fr' }, gap: 3, mt: 3 }}>
                  <OrgView />
                  <PIIEditor />
                </Box>
              </>
            )}

            {page === 'localidades' && <LocalidadesPage />}
            {page === 'departamentos' && <DepartamentosPage />}
            {page === 'lexico' && <LexicoPage />}
            {page === 'grupos' && <GruposPage />}
            {page === 'responsaveis' && <ResponsaveisPage />}
            {page === 'sites' && <SitesPage />}
          </>
        ) : (
          <Paper sx={{ p: 4, textAlign: 'center' }}>
            <Typography variant="h6">Faça login para listar/criar contatos.</Typography>
          </Paper>
        )}
      </Container>
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

function RuntimeConfigButton(){
  const [open, setOpen] = React.useState(false)
  return (
    <>
      <IconButton color="inherit" onClick={()=>setOpen(true)} aria-label="settings">
        <SettingsIcon />
      </IconButton>
      <RuntimeConfig open={open} onClose={()=>setOpen(false)} />
    </>
  )
}

function RuntimeConfig({ open, onClose }:{ open: boolean, onClose: ()=>void }){
  const [apiUrl, setApiUrl] = React.useState('')
  const [kcUrl, setKcUrl] = React.useState('')
  const [probeResult, setProbeResult] = React.useState<string | null>(null)
  const [snackOpen, setSnackOpen] = React.useState(false)

  React.useEffect(()=>{
    try{ setApiUrl(localStorage.getItem('sut_api_base') || '') }catch(e){}
    try{ setKcUrl(localStorage.getItem('sut_kc_base') || '') }catch(e){}
  }, [open])

  function save(){
    try{ if (apiUrl) localStorage.setItem('sut_api_base', apiUrl); else localStorage.removeItem('sut_api_base') }catch(e){}
    try{ if (kcUrl) localStorage.setItem('sut_kc_base', kcUrl); else localStorage.removeItem('sut_kc_base') }catch(e){}
    // small delay so storage is written before closing
    setSnackOpen(true)
    setProbeResult('Saved overrides, reloading...')
    setTimeout(()=>{
      try{ window.location.reload() }catch(e){}
    }, 300)
  }

  async function probe(){
    setProbeResult('Probing...')
    try{
      const results: string[] = []
      if (apiUrl){
        try{ const r = await fetch(apiUrl.replace(/\/$/, '') + '/health'); results.push(`API: ${r.ok ? 'ok' : 'status:'+r.status}`) }catch(e){ results.push('API: unreachable') }
      }
      if (kcUrl){
        try{ const r = await fetch(kcUrl.replace(/\/$/, '') + '/realms/sut/.well-known/openid-configuration'); results.push(`Keycloak: ${r.ok ? 'ok' : 'status:'+r.status}`) }catch(e){ results.push('Keycloak: unreachable') }
      }
      setProbeResult(results.join(' • '))
    }catch(e){ setProbeResult('Probe failed') }
  }

  return (
    <>
      {open && (
        <Dialog open={open} onClose={onClose} fullWidth maxWidth="sm">
          <DialogTitle>Runtime configuration</DialogTitle>
          <DialogContent>
            <DialogContentText sx={{ mb: 2 }}>Override API and Keycloak base URLs for local development. Leave blank to use auto-detection.</DialogContentText>
            <TextField fullWidth label="API base (e.g. http://localhost:8080)" value={apiUrl} onChange={e=>setApiUrl(e.target.value)} size="small" sx={{ mb: 2 }} />
            <TextField fullWidth label="Keycloak base (e.g. http://localhost:8081)" value={kcUrl} onChange={e=>setKcUrl(e.target.value)} size="small" />
            {probeResult && <Typography variant="body2" sx={{ mt: 2 }}>{probeResult}</Typography>}
          </DialogContent>
          <DialogActions>
            <Button onClick={onClose}>Cancel</Button>
            <Button onClick={probe}>Probe</Button>
            <Button onClick={save} variant="contained">Save & reload</Button>
          </DialogActions>
        </Dialog>
      )}
      <Snackbar open={snackOpen} autoHideDuration={3000} onClose={()=>setSnackOpen(false)}>
        <Alert severity="success" sx={{ width: '100%' }}>Overrides saved</Alert>
      </Snackbar>
    </>
  )
}

function RuntimeStatus(){
  const [api, setApi] = React.useState<string | null>(null)
  const [kc, setKc] = React.useState<string | null>(null)

  React.useEffect(()=>{
    try{ setApi(localStorage.getItem('sut_api_base') || null) }catch(e){ setApi(null) }
    try{ setKc(localStorage.getItem('sut_kc_base') || null) }catch(e){ setKc(null) }
    // expose in console for quick debugging
    try{ console.info('[RuntimeStatus] sut_api_base=', localStorage.getItem('sut_api_base'), 'sut_kc_base=', localStorage.getItem('sut_kc_base')) }catch(e){}
  }, [])

  return (
    <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mr: 1 }}>
      <Typography variant="caption" sx={{ opacity: 0.9 }}>{api ? `API:${api}` : 'API:auto'}</Typography>
      <Typography variant="caption" sx={{ opacity: 0.9 }}>{kc ? `KC:${kc}` : 'KC:auto'}</Typography>
    </Box>
  )
}

function OrgView(){
  const units = useUnits()
  const [selected, setSelected] = React.useState<string | undefined>(undefined)
  const depts = useDepartments(selected)

  return (
    <Card>
      <CardContent>
        <Typography variant="h6" gutterBottom>Organização</Typography>
        {units.isLoading ? (
          <CircularProgress size={24} />
        ) : (
          <>
            <FormControl fullWidth size="small" sx={{ mb: 2 }}>
              <InputLabel>Unidade</InputLabel>
              <Select
                value={selected || ''}
                label="Unidade"
                onChange={e=>setSelected(e.target.value || undefined)}
              >
                <MenuItem value=''>-- selecione unidade --</MenuItem>
                {units.data?.map(u=>(<MenuItem key={u.id} value={u.id}>{u.name}</MenuItem>))}
              </Select>
            </FormControl>
            <List>
              {depts.isLoading ? <ListItem><CircularProgress size={20} /></ListItem> : (depts.data || []).map(d=>(<ListItem key={d.id}><ListItemText primary={d.name} /></ListItem>))}
            </List>
          </>
        )}
      </CardContent>
    </Card>
  )
}

function PIIEditor(){
  const contacts = useContacts(5)
  const [id, setId] = React.useState('')
  const [doc, setDoc] = React.useState('')
  const updateDocument = useUpdateContactDocument()

  async function save(){
    if (!id || !doc) return
    await updateDocument.mutateAsync({ id, document: doc })
    setDoc('')
  }

  return (
    <Card>
      <CardContent>
        <Typography variant="h6" gutterBottom>PII (documento)</Typography>
        <Box sx={{ display: 'flex', gap: 2, flexDirection: { xs: 'column', md: 'row' }, alignItems: 'center' }}>
          <Box sx={{ flex: 1 }}>
            <FormControl fullWidth size="small">
              <InputLabel>Contato</InputLabel>
              <Select value={id} label="Contato" onChange={e=>setId(e.target.value)}>
                <MenuItem value=''>-- selecione contato --</MenuItem>
                {(contacts.data?.items || []).map((c: any)=>(<MenuItem key={c.id} value={c.id}>{c.fullName}</MenuItem>))}
              </Select>
            </FormControl>
          </Box>
          <Box sx={{ width: { xs: '100%', md: '40%' } }}>
            <TextField fullWidth size="small" label="CPF/CNPJ..." value={doc} onChange={e=>setDoc(e.target.value)} />
          </Box>
          <Box sx={{ width: { xs: '100%', md: '20%' } }}>
            <Button fullWidth variant="contained" color="secondary" onClick={save} disabled={!id || !doc || updateDocument.isPending}>
              {updateDocument.isPending ? 'Salvando...' : 'Salvar PII'}
            </Button>
          </Box>
        </Box>
        {updateDocument.isError && <Typography color="error" sx={{ mt: 2 }}>Erro ao salvar documento.</Typography>}
      </CardContent>
    </Card>
  )
}

function SearchBox({ value, onChange }:{ value: string, onChange: (v: string)=>void }){
  const results = useSearch(value, true, 8)
  return (
    <Box sx={{ mt: 2 }}>
      <TextField
        fullWidth
        label="Buscar (autocomplete)..."
        value={value}
        onChange={e=>onChange(e.target.value)}
        size="small"
      />
      {value && (
        results.isLoading ? (
          <Box sx={{ display: 'flex', alignItems: 'center', mt: 1 }}><CircularProgress size={18} sx={{ mr: 1 }} /> <span>...</span></Box>
        ) : (
          <Paper sx={{ mt: 1, p: 1 }}>
            {(results.data?.items || []).length === 0 ? (
              <Typography variant="body2" color="text.secondary"><em>Nenhum resultado</em></Typography>
            ) : (
              <List dense>
                {results.data?.items?.map((item: any)=>(<ListItem key={item.id}><ListItemText primary={item.fullName} /></ListItem>))}
              </List>
            )}
          </Paper>
        )
      )}
    </Box>
  )
}

function ContactItem({ contact }:{ contact: any }){
  const [editing, setEditing] = React.useState(false)
  const [fullName, setFullName] = React.useState(contact.fullName)
  const update = useUpdateContact(contact.id)
  const remove = useDeleteContact(contact.id)
  const [showDoc, setShowDoc] = React.useState(false)
  const doc = useContactDocument(contact.id)

  return (
    <ListItem divider>
      {editing ? (
        <Box sx={{ display: 'flex', alignItems: 'center', width: '100%', gap: 1 }}>
          <TextField value={fullName} onChange={e=>setFullName(e.target.value)} size="small" sx={{ flex: 1 }} />
          <Button
            onClick={()=> update.mutate({ body: { fullName }, etag: contact.etag }, { onSuccess: ()=> setEditing(false) })}
            disabled={update.isPending}
            variant="contained"
            color="primary"
            size="small"
          >
            {update.isPending ? 'Salvando...' : 'Salvar'}
          </Button>
          <Button onClick={()=> setEditing(false)} size="small">Cancelar</Button>
        </Box>
      ) : (
        <Box sx={{ display: 'flex', alignItems: 'center', width: '100%' }}>
          <ListItemText
            primary={<><strong>{contact.fullName}</strong> <Chip label={`etag:${contact.etag}`} size="small" sx={{ ml: 1 }} /></>}
          />
          <ListItemSecondaryAction>
            <IconButton edge="end" onClick={()=> setEditing(true)}><EditIcon /></IconButton>
            <IconButton edge="end" onClick={()=> remove.mutate()} disabled={remove.isPending}>
              <DeleteIcon />
            </IconButton>
            <IconButton edge="end" onClick={()=> setShowDoc(v=>!v)}>
              {showDoc ? <VisibilityOffIcon /> : <VisibilityIcon />}
            </IconButton>
          </ListItemSecondaryAction>
        </Box>
      )}
      {showDoc && (
        <Box sx={{ mt: 1, width: '100%' }}>
          {doc.isLoading ? <CircularProgress size={16} /> : (doc.data?.document || <em>(sem documento)</em>)}
        </Box>
      )}
    </ListItem>
  )
}
