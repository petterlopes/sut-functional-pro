import React from 'react'
import {
  Box, Card, CardContent, Typography, Button, Dialog, DialogTitle, DialogContent,
  DialogActions, TextField, FormControl, InputLabel, Select, MenuItem, Alert, Snackbar,
  Chip, Avatar, Tooltip, IconButton, Tabs, Tab, Grid, Paper
} from '@mui/material'
import {
  Add as AddIcon, Edit as EditIcon, Delete as DeleteIcon, Person as PersonIcon,
  Email as EmailIcon, Phone as PhoneIcon, Business as BusinessIcon, Search as SearchIcon,
  Visibility as VisibilityIcon, VisibilityOff as VisibilityOffIcon, AttachFile as AttachFileIcon
} from '@mui/icons-material'
import { DataGrid, GridColDef, GridActionsCellItem, GridRowParams } from '@mui/x-data-grid'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import {
  useContacts,
  useCreateContact,
  useUpdateContact,
  useDeleteContact,
  useContactDocument,
  useUpdateContactDocument,
} from '../../api/hooks'

interface TabPanelProps {
  children?: React.ReactNode
  index: number
  value: number
}

function TabPanel(props: TabPanelProps) {
  const { children, value, index, ...other } = props

  return (
    <div
      role="tabpanel"
      hidden={value !== index}
      id={`contact-tabpanel-${index}`}
      aria-labelledby={`contact-tab-${index}`}
      {...other}
    >
      {value === index && (
        <Box sx={{ p: 3 }}>
          {children}
        </Box>
      )}
    </div>
  )
}

interface Contact {
  id: string
  fullName: string
  type: string
  document?: string
  unit_id?: string
  department_id?: string
  status: string
  email: string
  phones?: Array<{ e164: string; extension: string; type: string; is_primary: boolean }>
  emails?: Array<{ address: string; is_primary: boolean }>
  etag: string
  created_at: string
  updated_at: string
}

export default function ContatosPage() {
  const [openDialog, setOpenDialog] = React.useState(false)
  const [editingItem, setEditingItem] = React.useState<Contact | null>(null)
  const [formData, setFormData] = React.useState({
    fullName: '',
    type: 'PERSON',
    document: '',
    unit_id: '',
    department_id: '',
    status: 'ACTIVE',
    email: '',
    phone: '',
    phoneType: 'MOBILE'
  })
  const [snackbar, setSnackbar] = React.useState({ open: false, message: '', severity: 'success' as 'success' | 'error' })
  const [searchTerm, setSearchTerm] = React.useState('')
  const [tabValue, setTabValue] = React.useState(0)
  const [showPii, setShowPii] = React.useState<{ [key: string]: boolean }>({})

  const queryClient = useQueryClient()

  // Queries da API
  const { data: contactsData, isLoading } = useContacts(100)
  const contacts = contactsData?.items || []

  const createMutation = useCreateContact()
  const updateMutation = useUpdateContact('')
  const deleteMutation = useDeleteContact('')

  const handleOpenDialog = (item?: Contact) => {
    if (item) {
      setEditingItem(item)
      setFormData({
        fullName: item.fullName,
        type: item.type,
        document: item.document || '',
        unit_id: item.unit_id || '',
        department_id: item.department_id || '',
        status: item.status,
        email: item.email || '',
        phone: '',
        phoneType: 'MOBILE'
      })
    } else {
      setEditingItem(null)
      setFormData({
        fullName: '',
        type: 'PERSON',
        document: '',
        unit_id: '',
        department_id: '',
        status: 'ACTIVE',
        email: '',
        phone: '',
        phoneType: 'MOBILE'
      })
    }
    setOpenDialog(true)
  }

  const handleCloseDialog = () => {
    setOpenDialog(false)
    setEditingItem(null)
    setFormData({
      fullName: '',
      type: 'PERSON',
      document: '',
      unit_id: '',
      department_id: '',
      status: 'ACTIVE',
      email: '',
      phone: '',
      phoneType: 'MOBILE'
    })
  }

  const handleSubmit = () => {
    if (!formData.fullName.trim()) return

    const contactData = {
      fullName: formData.fullName,
      type: formData.type,
      document: formData.document || undefined,
      unit_id: formData.unit_id || undefined,
      department_id: formData.department_id || undefined,
      status: formData.status,
      email: formData.email || undefined
    }

    if (editingItem) {
      updateMutation.mutate(
        { body: contactData, etag: editingItem.etag },
        {
          onSuccess: () => {
            setSnackbar({ open: true, message: 'Contato atualizado com sucesso!', severity: 'success' })
            handleCloseDialog()
          },
          onError: (error: any) => {
            setSnackbar({ open: true, message: `Erro ao atualizar contato: ${error.message}`, severity: 'error' })
          }
        }
      )
    } else {
      createMutation.mutate(contactData, {
        onSuccess: () => {
          setSnackbar({ open: true, message: 'Contato criado com sucesso!', severity: 'success' })
          handleCloseDialog()
        },
        onError: (error: any) => {
          setSnackbar({ open: true, message: `Erro ao criar contato: ${error.message}`, severity: 'error' })
        }
      })
    }
  }

  const handleDelete = (id: string) => {
    if (window.confirm('Tem certeza que deseja excluir este contato?')) {
      deleteMutation.mutate(undefined, {
        onSuccess: () => {
          setSnackbar({ open: true, message: 'Contato excluído com sucesso!', severity: 'success' })
        },
        onError: (error: any) => {
          setSnackbar({ open: true, message: `Erro ao excluir contato: ${error.message}`, severity: 'error' })
        }
      })
    }
  }

  const handleTabChange = (event: React.SyntheticEvent, newValue: number) => {
    setTabValue(newValue)
  }

  const togglePiiVisibility = (contactId: string) => {
    setShowPii(prev => ({ ...prev, [contactId]: !prev[contactId] }))
  }

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'ACTIVE': return 'success'
      case 'INACTIVE': return 'error'
      case 'PENDING': return 'warning'
      default: return 'default'
    }
  }

  const getTypeColor = (type: string) => {
    switch (type) {
      case 'PERSON': return 'primary'
      case 'ORGANIZATION': return 'secondary'
      case 'DEPARTMENT': return 'success'
      default: return 'default'
    }
  }

  // Filtrar contatos
  const filteredContacts = contacts.filter(contact =>
    contact.fullName.toLowerCase().includes(searchTerm.toLowerCase()) ||
    contact.email?.toLowerCase().includes(searchTerm.toLowerCase()) ||
    contact.document?.toLowerCase().includes(searchTerm.toLowerCase())
  )

  const columns: GridColDef[] = [
    {
      field: 'fullName',
      headerName: 'Nome Completo',
      width: 250,
      renderCell: (params) => (
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <Avatar sx={{ width: 32, height: 32, bgcolor: 'primary.main' }}>
            <PersonIcon fontSize="small" />
          </Avatar>
          <Box>
            <Typography variant="body2" sx={{ fontWeight: 'medium' }}>
              {params.value}
            </Typography>
            {params.row.document && (
              <Typography variant="caption" color="text.secondary">
                Doc: {params.row.document}
              </Typography>
            )}
          </Box>
        </Box>
      )
    },
    {
      field: 'email',
      headerName: 'Email',
      width: 200,
      renderCell: (params) => (
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <EmailIcon fontSize="small" color="action" />
          <Typography variant="body2">
            {params.value || 'N/A'}
          </Typography>
        </Box>
      )
    },
    {
      field: 'type',
      headerName: 'Tipo',
      width: 120,
      renderCell: (params) => (
        <Chip 
          label={params.value === 'PERSON' ? 'Pessoa' : params.value === 'ORGANIZATION' ? 'Organização' : 'Departamento'} 
          color={getTypeColor(params.value) as any}
          size="small"
        />
      )
    },
    {
      field: 'status',
      headerName: 'Status',
      width: 100,
      renderCell: (params) => (
        <Chip 
          label={params.value === 'ACTIVE' ? 'Ativo' : params.value === 'INACTIVE' ? 'Inativo' : 'Pendente'} 
          color={getStatusColor(params.value) as any}
          size="small"
        />
      )
    },
    {
      field: 'created_at',
      headerName: 'Criado em',
      width: 150,
      renderCell: (params) => (
        <Typography variant="body2">
          {new Date(params.value).toLocaleDateString('pt-BR')}
        </Typography>
      )
    },
    {
      field: 'actions',
      type: 'actions',
      headerName: 'Ações',
      width: 150,
      getActions: (params: GridRowParams) => [
        <GridActionsCellItem
          icon={<EditIcon />}
          label="Editar"
          onClick={() => handleOpenDialog(params.row)}
        />,
        <GridActionsCellItem
          icon={showPii[params.row.id] ? <VisibilityOffIcon /> : <VisibilityIcon />}
          label="Ver PII"
          onClick={() => togglePiiVisibility(params.row.id)}
        />,
        <GridActionsCellItem
          icon={<DeleteIcon />}
          label="Excluir"
          onClick={() => handleDelete(params.row.id)}
        />
      ]
    }
  ]

  return (
    <Box sx={{ p: 3 }}>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
        <Box>
          <Typography variant="h4" sx={{ fontWeight: 'bold', mb: 1 }}>
            Contatos
          </Typography>
          <Typography variant="body1" color="text.secondary">
            Gerencie contatos pessoais e organizacionais
          </Typography>
        </Box>
        <Button
          variant="contained"
          startIcon={<AddIcon />}
          onClick={() => handleOpenDialog()}
          sx={{
            background: 'linear-gradient(83.81deg, #5D00FF 0%, #EE00FF 100%)',
            '&:hover': { opacity: 0.9 }
          }}
        >
          Novo Contato
        </Button>
      </Box>

      {/* Filtros */}
      <Card sx={{ mb: 3 }}>
        <CardContent>
          <Box sx={{ display: 'flex', gap: 2, alignItems: 'center' }}>
            <TextField
              placeholder="Buscar contatos..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              InputProps={{
                startAdornment: <SearchIcon sx={{ mr: 1, color: 'text.secondary' }} />
              }}
              sx={{ minWidth: 300 }}
            />
            <Button
              variant="outlined"
              onClick={() => setSearchTerm('')}
            >
              Limpar Filtros
            </Button>
          </Box>
        </CardContent>
      </Card>

      {/* Tabs para diferentes visualizações */}
      <Card>
        <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
          <Tabs value={tabValue} onChange={handleTabChange} aria-label="contact tabs">
            <Tab label="Lista" />
            <Tab label="Grid" />
            <Tab label="Estatísticas" />
          </Tabs>
        </Box>

        <TabPanel value={tabValue} index={0}>
          <DataGrid
            rows={filteredContacts}
            columns={columns}
            loading={isLoading}
            autoHeight
            disableRowSelectionOnClick
            pageSizeOptions={[5, 10, 25]}
            initialState={{
              pagination: { paginationModel: { pageSize: 10 } }
            }}
            sx={{
              border: 'none',
              '& .MuiDataGrid-cell': {
                borderBottom: '1px solid #f0f0f0'
              },
              '& .MuiDataGrid-columnHeaders': {
                backgroundColor: '#f8fafc',
                borderBottom: '2px solid #e2e8f0'
              }
            }}
          />
        </TabPanel>

        <TabPanel value={tabValue} index={1}>
          <Grid container spacing={3}>
            {filteredContacts.map((contact) => (
              <Grid item xs={12} sm={6} md={4} key={contact.id}>
                <ContactCard 
                  contact={contact} 
                  onEdit={() => handleOpenDialog(contact)}
                  onDelete={() => handleDelete(contact.id)}
                  showPii={showPii[contact.id]}
                  onTogglePii={() => togglePiiVisibility(contact.id)}
                />
              </Grid>
            ))}
          </Grid>
        </TabPanel>

        <TabPanel value={tabValue} index={2}>
          <ContactStatistics contacts={contacts} />
        </TabPanel>
      </Card>

      {/* Dialog para criar/editar */}
      <Dialog open={openDialog} onClose={handleCloseDialog} maxWidth="md" fullWidth>
        <DialogTitle>
          {editingItem ? 'Editar Contato' : 'Novo Contato'}
        </DialogTitle>
        <DialogContent>
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2, pt: 1 }}>
            <TextField
              label="Nome Completo"
              value={formData.fullName}
              onChange={(e) => setFormData({ ...formData, fullName: e.target.value })}
              fullWidth
              required
            />

            <Box sx={{ display: 'flex', gap: 2 }}>
              <FormControl sx={{ flex: 1 }}>
                <InputLabel>Tipo</InputLabel>
                <Select
                  value={formData.type}
                  onChange={(e) => setFormData({ ...formData, type: e.target.value })}
                  label="Tipo"
                >
                  <MenuItem value="PERSON">Pessoa</MenuItem>
                  <MenuItem value="ORGANIZATION">Organização</MenuItem>
                  <MenuItem value="DEPARTMENT">Departamento</MenuItem>
                </Select>
              </FormControl>

              <TextField
                label="Documento"
                value={formData.document}
                onChange={(e) => setFormData({ ...formData, document: e.target.value })}
                sx={{ flex: 1 }}
              />
            </Box>

            <TextField
              label="Email"
              type="email"
              value={formData.email}
              onChange={(e) => setFormData({ ...formData, email: e.target.value })}
              fullWidth
            />

            <Box sx={{ display: 'flex', gap: 2 }}>
              <TextField
                label="Telefone"
                value={formData.phone}
                onChange={(e) => setFormData({ ...formData, phone: e.target.value })}
                sx={{ flex: 1 }}
                placeholder="+55 11 99999-9999"
              />

              <FormControl sx={{ flex: 1 }}>
                <InputLabel>Tipo do Telefone</InputLabel>
                <Select
                  value={formData.phoneType}
                  onChange={(e) => setFormData({ ...formData, phoneType: e.target.value })}
                  label="Tipo do Telefone"
                >
                  <MenuItem value="MOBILE">Celular</MenuItem>
                  <MenuItem value="WORK">Trabalho</MenuItem>
                  <MenuItem value="HOME">Residencial</MenuItem>
                </Select>
              </FormControl>
            </Box>

            <FormControl>
              <InputLabel>Status</InputLabel>
              <Select
                value={formData.status}
                onChange={(e) => setFormData({ ...formData, status: e.target.value })}
                label="Status"
              >
                <MenuItem value="ACTIVE">Ativo</MenuItem>
                <MenuItem value="INACTIVE">Inativo</MenuItem>
                <MenuItem value="PENDING">Pendente</MenuItem>
              </Select>
            </FormControl>
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleCloseDialog}>Cancelar</Button>
          <Button
            onClick={handleSubmit}
            variant="contained"
            disabled={!formData.fullName.trim() || createMutation.isPending || updateMutation.isPending}
          >
            {editingItem ? 'Atualizar' : 'Criar'}
          </Button>
        </DialogActions>
      </Dialog>

      {/* Snackbar para feedback */}
      <Snackbar
        open={snackbar.open}
        autoHideDuration={6000}
        onClose={() => setSnackbar({ ...snackbar, open: false })}
      >
        <Alert
          onClose={() => setSnackbar({ ...snackbar, open: false })}
          severity={snackbar.severity}
          sx={{ width: '100%' }}
        >
          {snackbar.message}
        </Alert>
      </Snackbar>
    </Box>
  )
}

// Componente para Card de Contato
function ContactCard({ contact, onEdit, onDelete, showPii, onTogglePii }: {
  contact: Contact
  onEdit: () => void
  onDelete: () => void
  showPii: boolean
  onTogglePii: () => void
}) {
  const { data: piiDoc, isLoading: piiLoading } = useContactDocument(contact.id)

  return (
    <Card sx={{ height: '100%' }}>
      <CardContent>
        <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start', mb: 2 }}>
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
            <Avatar sx={{ bgcolor: 'primary.main' }}>
              <PersonIcon />
            </Avatar>
            <Box>
              <Typography variant="h6" sx={{ fontWeight: 'bold' }}>
                {contact.fullName}
              </Typography>
              <Chip 
                label={contact.type === 'PERSON' ? 'Pessoa' : 'Organização'} 
                size="small" 
                color="primary"
              />
            </Box>
          </Box>
          <Box>
            <IconButton size="small" onClick={onTogglePii}>
              {showPii ? <VisibilityOffIcon /> : <VisibilityIcon />}
            </IconButton>
            <IconButton size="small" onClick={onEdit}>
              <EditIcon />
            </IconButton>
            <IconButton size="small" onClick={onDelete}>
              <DeleteIcon />
            </IconButton>
          </Box>
        </Box>

        {contact.email && (
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mb: 1 }}>
            <EmailIcon fontSize="small" color="action" />
            <Typography variant="body2">{contact.email}</Typography>
          </Box>
        )}

        {contact.document && (
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mb: 1 }}>
            <AttachFileIcon fontSize="small" color="action" />
            <Typography variant="body2">Doc: {contact.document}</Typography>
          </Box>
        )}

        {showPii && (
          <Box sx={{ mt: 2, p: 2, bgcolor: 'grey.50', borderRadius: 1 }}>
            <Typography variant="subtitle2" sx={{ mb: 1 }}>Documento PII:</Typography>
            {piiLoading ? (
              <Typography variant="body2">Carregando...</Typography>
            ) : (
              <Typography variant="body2">
                {piiDoc?.document || 'Nenhum documento PII'}
              </Typography>
            )}
          </Box>
        )}

        <Box sx={{ mt: 2, display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
          <Chip 
            label={contact.status === 'ACTIVE' ? 'Ativo' : 'Inativo'} 
            color={contact.status === 'ACTIVE' ? 'success' : 'error'}
            size="small"
          />
          <Typography variant="caption" color="text.secondary">
            {new Date(contact.created_at).toLocaleDateString('pt-BR')}
          </Typography>
        </Box>
      </CardContent>
    </Card>
  )
}

// Componente para Estatísticas
function ContactStatistics({ contacts }: { contacts: Contact[] }) {
  const stats = {
    total: contacts.length,
    active: contacts.filter(c => c.status === 'ACTIVE').length,
    inactive: contacts.filter(c => c.status === 'INACTIVE').length,
    persons: contacts.filter(c => c.type === 'PERSON').length,
    organizations: contacts.filter(c => c.type === 'ORGANIZATION').length,
    withEmail: contacts.filter(c => c.email).length,
    withDocument: contacts.filter(c => c.document).length
  }

  return (
    <Grid container spacing={3}>
      <Grid item xs={12} sm={6} md={3}>
        <Paper sx={{ p: 2, textAlign: 'center' }}>
          <Typography variant="h4" color="primary" sx={{ fontWeight: 'bold' }}>
            {stats.total}
          </Typography>
          <Typography variant="body2" color="text.secondary">
            Total de Contatos
          </Typography>
        </Paper>
      </Grid>
      <Grid item xs={12} sm={6} md={3}>
        <Paper sx={{ p: 2, textAlign: 'center' }}>
          <Typography variant="h4" color="success.main" sx={{ fontWeight: 'bold' }}>
            {stats.active}
          </Typography>
          <Typography variant="body2" color="text.secondary">
            Contatos Ativos
          </Typography>
        </Paper>
      </Grid>
      <Grid item xs={12} sm={6} md={3}>
        <Paper sx={{ p: 2, textAlign: 'center' }}>
          <Typography variant="h4" color="primary.main" sx={{ fontWeight: 'bold' }}>
            {stats.persons}
          </Typography>
          <Typography variant="body2" color="text.secondary">
            Pessoas
          </Typography>
        </Paper>
      </Grid>
      <Grid item xs={12} sm={6} md={3}>
        <Paper sx={{ p: 2, textAlign: 'center' }}>
          <Typography variant="h4" color="secondary.main" sx={{ fontWeight: 'bold' }}>
            {stats.organizations}
          </Typography>
          <Typography variant="body2" color="text.secondary">
            Organizações
          </Typography>
        </Paper>
      </Grid>
    </Grid>
  )
}
