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
import { Contact } from '../../domain/entities/Contact'
import { useContacts, useCreateContact, useUpdateContact, useDeleteContact, useContactStatistics } from '../../presentation/hooks/useContactUseCases'

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

export default function ContatosPageClean() {
  const [openDialog, setOpenDialog] = React.useState(false)
  const [editingContact, setEditingContact] = React.useState<Contact | null>(null)
  const [formData, setFormData] = React.useState({
    fullName: '',
    type: 'PERSON' as 'PERSON' | 'ORGANIZATION' | 'DEPARTMENT',
    status: 'ACTIVE' as 'ACTIVE' | 'INACTIVE' | 'PENDING',
    document: '',
    unitId: '',
    departmentId: '',
    email: '',
    phone: '',
    phoneType: 'MOBILE' as 'MOBILE' | 'WORK' | 'HOME'
  })
  const [snackbar, setSnackbar] = React.useState({ open: false, message: '', severity: 'success' as 'success' | 'error' })
  const [searchTerm, setSearchTerm] = React.useState('')
  const [tabValue, setTabValue] = React.useState(0)
  const [showPii, setShowPii] = React.useState<{ [key: string]: boolean }>({})

  // Clean Architecture hooks
  const { data: contactsData, isLoading } = useContacts({ 
    searchTerm: searchTerm || undefined,
    limit: 100 
  })
  const { data: statisticsData } = useContactStatistics()
  
  const createMutation = useCreateContact()
  const updateMutation = useUpdateContact()
  const deleteMutation = useDeleteContact()

  const contacts = contactsData?.contacts || []
  const statistics = statisticsData?.statistics

  const handleOpenDialog = (contact?: Contact) => {
    if (contact) {
      setEditingContact(contact)
      setFormData({
        fullName: contact.fullName,
        type: contact.type.value,
        status: contact.status.value,
        document: contact.document?.value || '',
        unitId: contact.unitId || '',
        departmentId: contact.departmentId || '',
        email: contact.emails.find(e => e.isPrimary)?.value || '',
        phone: contact.phones.find(p => p.isPrimary)?.e164 || '',
        phoneType: contact.phones.find(p => p.isPrimary)?.type || 'MOBILE'
      })
    } else {
      setEditingContact(null)
      setFormData({
        fullName: '',
        type: 'PERSON',
        status: 'ACTIVE',
        document: '',
        unitId: '',
        departmentId: '',
        email: '',
        phone: '',
        phoneType: 'MOBILE'
      })
    }
    setOpenDialog(true)
  }

  const handleCloseDialog = () => {
    setOpenDialog(false)
    setEditingContact(null)
    setFormData({
      fullName: '',
      type: 'PERSON',
      status: 'ACTIVE',
      document: '',
      unitId: '',
      departmentId: '',
      email: '',
      phone: '',
      phoneType: 'MOBILE'
    })
  }

  const handleSubmit = () => {
    if (!formData.fullName.trim()) return

    if (editingContact) {
      updateMutation.mutate({
        id: editingContact.id.value,
        fullName: formData.fullName,
        status: formData.status,
        document: formData.document || undefined,
        unitId: formData.unitId || undefined,
        departmentId: formData.departmentId || undefined,
        etag: editingContact.etag
      }, {
        onSuccess: () => {
          setSnackbar({ open: true, message: 'Contato atualizado com sucesso!', severity: 'success' })
          handleCloseDialog()
        },
        onError: (error: any) => {
          setSnackbar({ open: true, message: `Erro ao atualizar contato: ${error.message}`, severity: 'error' })
        }
      })
    } else {
      createMutation.mutate({
        fullName: formData.fullName,
        type: formData.type,
        status: formData.status,
        document: formData.document || undefined,
        unitId: formData.unitId || undefined,
        departmentId: formData.departmentId || undefined,
        email: formData.email || undefined,
        phone: formData.phone || undefined,
        phoneType: formData.phoneType
      }, {
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

  const handleDelete = (contact: Contact) => {
    if (window.confirm(`Tem certeza que deseja excluir o contato "${contact.fullName}"?`)) {
      deleteMutation.mutate({
        id: contact.id.value
      }, {
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
            {params.row.document?.value && (
              <Typography variant="caption" color="text.secondary">
                Doc: {params.row.document.value}
              </Typography>
            )}
          </Box>
        </Box>
      )
    },
    {
      field: 'emails',
      headerName: 'Email',
      width: 200,
      renderCell: (params) => {
        const primaryEmail = params.value.find((e: any) => e.isPrimary)
        return (
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
            <EmailIcon fontSize="small" color="action" />
            <Typography variant="body2">
              {primaryEmail?.value || 'N/A'}
            </Typography>
          </Box>
        )
      }
    },
    {
      field: 'type',
      headerName: 'Tipo',
      width: 120,
      renderCell: (params) => (
        <Chip 
          label={params.value.value === 'PERSON' ? 'Pessoa' : params.value.value === 'ORGANIZATION' ? 'Organização' : 'Departamento'} 
          color={getTypeColor(params.value.value) as any}
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
          label={params.value.value === 'ACTIVE' ? 'Ativo' : params.value.value === 'INACTIVE' ? 'Inativo' : 'Pendente'} 
          color={getStatusColor(params.value.value) as any}
          size="small"
        />
      )
    },
    {
      field: 'createdAt',
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
          icon={showPii[params.row.id.value] ? <VisibilityOffIcon /> : <VisibilityIcon />}
          label="Ver PII"
          onClick={() => togglePiiVisibility(params.row.id.value)}
        />,
        <GridActionsCellItem
          icon={<DeleteIcon />}
          label="Excluir"
          onClick={() => handleDelete(params.row)}
        />
      ]
    }
  ]

  return (
    <Box sx={{ p: 3 }}>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
        <Box>
          <Typography variant="h4" sx={{ fontWeight: 'bold', mb: 1 }}>
            Contatos (Clean Architecture)
          </Typography>
          <Typography variant="body1" color="text.secondary">
            Gerencie contatos pessoais e organizacionais usando Clean Architecture
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
            rows={contacts}
            columns={columns}
            loading={isLoading}
            autoHeight
            disableRowSelectionOnClick
            pageSizeOptions={[5, 10, 25]}
            getRowId={(row) => row.id.value}
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
            {contacts.map((contact) => (
              <Grid item xs={12} sm={6} md={4} key={contact.id.value}>
                <ContactCard 
                  contact={contact} 
                  onEdit={() => handleOpenDialog(contact)}
                  onDelete={() => handleDelete(contact)}
                  showPii={showPii[contact.id.value]}
                  onTogglePii={() => togglePiiVisibility(contact.id.value)}
                />
              </Grid>
            ))}
          </Grid>
        </TabPanel>

        <TabPanel value={tabValue} index={2}>
          <ContactStatistics statistics={statistics} />
        </TabPanel>
      </Card>

      {/* Dialog para criar/editar */}
      <Dialog open={openDialog} onClose={handleCloseDialog} maxWidth="md" fullWidth>
        <DialogTitle>
          {editingContact ? 'Editar Contato' : 'Novo Contato'}
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
                  onChange={(e) => setFormData({ ...formData, type: e.target.value as any })}
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
                  onChange={(e) => setFormData({ ...formData, phoneType: e.target.value as any })}
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
                onChange={(e) => setFormData({ ...formData, status: e.target.value as any })}
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
            {editingContact ? 'Atualizar' : 'Criar'}
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
  const primaryEmail = contact.emails.find(e => e.isPrimary)
  const primaryPhone = contact.phones.find(p => p.isPrimary)

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
                label={contact.type.value === 'PERSON' ? 'Pessoa' : 'Organização'} 
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

        {primaryEmail && (
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mb: 1 }}>
            <EmailIcon fontSize="small" color="action" />
            <Typography variant="body2">{primaryEmail.value}</Typography>
          </Box>
        )}

        {primaryPhone && (
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mb: 1 }}>
            <PhoneIcon fontSize="small" color="action" />
            <Typography variant="body2">{primaryPhone.e164}</Typography>
          </Box>
        )}

        {contact.document && (
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mb: 1 }}>
            <AttachFileIcon fontSize="small" color="action" />
            <Typography variant="body2">Doc: {contact.document.value}</Typography>
          </Box>
        )}

        {showPii && (
          <Box sx={{ mt: 2, p: 2, bgcolor: 'grey.50', borderRadius: 1 }}>
            <Typography variant="subtitle2" sx={{ mb: 1 }}>Documento PII:</Typography>
            <Typography variant="body2">
              Documento PII será carregado da API
            </Typography>
          </Box>
        )}

        <Box sx={{ mt: 2, display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
          <Chip 
            label={contact.status.value === 'ACTIVE' ? 'Ativo' : 'Inativo'} 
            color={contact.status.value === 'ACTIVE' ? 'success' : 'error'}
            size="small"
          />
          <Typography variant="caption" color="text.secondary">
            {contact.createdAt.toLocaleDateString('pt-BR')}
          </Typography>
        </Box>
      </CardContent>
    </Card>
  )
}

// Componente para Estatísticas
function ContactStatistics({ statistics }: { statistics?: any }) {
  if (!statistics) {
    return <Typography>Carregando estatísticas...</Typography>
  }

  return (
    <Grid container spacing={3}>
      <Grid item xs={12} sm={6} md={3}>
        <Paper sx={{ p: 2, textAlign: 'center' }}>
          <Typography variant="h4" color="primary" sx={{ fontWeight: 'bold' }}>
            {statistics.total}
          </Typography>
          <Typography variant="body2" color="text.secondary">
            Total de Contatos
          </Typography>
        </Paper>
      </Grid>
      <Grid item xs={12} sm={6} md={3}>
        <Paper sx={{ p: 2, textAlign: 'center' }}>
          <Typography variant="h4" color="success.main" sx={{ fontWeight: 'bold' }}>
            {statistics.active}
          </Typography>
          <Typography variant="body2" color="text.secondary">
            Contatos Ativos
          </Typography>
        </Paper>
      </Grid>
      <Grid item xs={12} sm={6} md={3}>
        <Paper sx={{ p: 2, textAlign: 'center' }}>
          <Typography variant="h4" color="primary.main" sx={{ fontWeight: 'bold' }}>
            {statistics.persons}
          </Typography>
          <Typography variant="body2" color="text.secondary">
            Pessoas
          </Typography>
        </Paper>
      </Grid>
      <Grid item xs={12} sm={6} md={3}>
        <Paper sx={{ p: 2, textAlign: 'center' }}>
          <Typography variant="h4" color="secondary.main" sx={{ fontWeight: 'bold' }}>
            {statistics.organizations}
          </Typography>
          <Typography variant="body2" color="text.secondary">
            Organizações
          </Typography>
        </Paper>
      </Grid>
    </Grid>
  )
}
