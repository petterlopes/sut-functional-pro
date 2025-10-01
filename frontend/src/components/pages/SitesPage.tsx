import React from 'react'
import {
  Box, Card, CardContent, Typography, Button, Dialog, DialogTitle, DialogContent,
  DialogActions, TextField, FormControl, InputLabel, Select, MenuItem, Alert, Snackbar,
  Chip, Avatar, Tooltip, Switch, FormControlLabel
} from '@mui/material'
import {
  Add as AddIcon, Edit as EditIcon, Delete as DeleteIcon, Public as PublicIcon,
  Language as LanguageIcon, Security as SecurityIcon, Speed as SpeedIcon
} from '@mui/icons-material'
import { DataGrid, GridColDef, GridActionsCellItem, GridRowParams } from '@mui/x-data-grid'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'

// Mock data para demonstração
const mockSites = [
  { id: '1', name: 'Site Principal', url: 'https://www.empresa.com', status: 'ATIVO', ssl_enabled: true, response_time: 120, uptime: 99.9, last_check: '2024-01-15T10:30:00Z' },
  { id: '2', name: 'Portal do Cliente', url: 'https://portal.empresa.com', status: 'ATIVO', ssl_enabled: true, response_time: 95, uptime: 99.8, last_check: '2024-01-15T10:29:00Z' },
  { id: '3', name: 'API Gateway', url: 'https://api.empresa.com', status: 'ATIVO', ssl_enabled: true, response_time: 45, uptime: 99.95, last_check: '2024-01-15T10:28:00Z' },
  { id: '4', name: 'Blog Corporativo', url: 'https://blog.empresa.com', status: 'MANUTENCAO', ssl_enabled: true, response_time: 0, uptime: 98.5, last_check: '2024-01-15T09:15:00Z' },
  { id: '5', name: 'Intranet', url: 'https://intranet.empresa.com', status: 'ATIVO', ssl_enabled: false, response_time: 200, uptime: 99.2, last_check: '2024-01-15T10:25:00Z' },
]

interface Site {
  id: string
  name: string
  url: string
  status: string
  ssl_enabled: boolean
  response_time: number
  uptime: number
  last_check: string
}

export default function SitesPage() {
  const [openDialog, setOpenDialog] = React.useState(false)
  const [editingItem, setEditingItem] = React.useState<Site | null>(null)
  const [formData, setFormData] = React.useState({
    name: '',
    url: '',
    status: 'ATIVO',
    ssl_enabled: true
  })
  const [snackbar, setSnackbar] = React.useState({ open: false, message: '', severity: 'success' as 'success' | 'error' })

  const queryClient = useQueryClient()

  // Simular query da API
  const { data: sites = [], isLoading } = useQuery({
    queryKey: ['sites'],
    queryFn: async () => {
      await new Promise(resolve => setTimeout(resolve, 1000))
      return mockSites
    }
  })

  const createMutation = useMutation({
    mutationFn: async (data: Partial<Site>) => {
      await new Promise(resolve => setTimeout(resolve, 500))
      return { 
        ...data, 
        id: Date.now().toString(), 
        response_time: 0, 
        uptime: 0, 
        last_check: new Date().toISOString() 
      }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['sites'] })
      setSnackbar({ open: true, message: 'Site criado com sucesso!', severity: 'success' })
      handleCloseDialog()
    },
    onError: () => {
      setSnackbar({ open: true, message: 'Erro ao criar site', severity: 'error' })
    }
  })

  const updateMutation = useMutation({
    mutationFn: async ({ id, data }: { id: string, data: Partial<Site> }) => {
      await new Promise(resolve => setTimeout(resolve, 500))
      return { ...data, id }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['sites'] })
      setSnackbar({ open: true, message: 'Site atualizado com sucesso!', severity: 'success' })
      handleCloseDialog()
    },
    onError: () => {
      setSnackbar({ open: true, message: 'Erro ao atualizar site', severity: 'error' })
    }
  })

  const deleteMutation = useMutation({
    mutationFn: async (id: string) => {
      await new Promise(resolve => setTimeout(resolve, 500))
      return id
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['sites'] })
      setSnackbar({ open: true, message: 'Site excluído com sucesso!', severity: 'success' })
    },
    onError: () => {
      setSnackbar({ open: true, message: 'Erro ao excluir site', severity: 'error' })
    }
  })

  const handleOpenDialog = (item?: Site) => {
    if (item) {
      setEditingItem(item)
      setFormData({
        name: item.name,
        url: item.url,
        status: item.status,
        ssl_enabled: item.ssl_enabled
      })
    } else {
      setEditingItem(null)
      setFormData({
        name: '',
        url: '',
        status: 'ATIVO',
        ssl_enabled: true
      })
    }
    setOpenDialog(true)
  }

  const handleCloseDialog = () => {
    setOpenDialog(false)
    setEditingItem(null)
    setFormData({
      name: '',
      url: '',
      status: 'ATIVO',
      ssl_enabled: true
    })
  }

  const handleSubmit = () => {
    if (!formData.name.trim() || !formData.url.trim()) return

    const data = {
      name: formData.name,
      url: formData.url,
      status: formData.status,
      ssl_enabled: formData.ssl_enabled
    }

    if (editingItem) {
      updateMutation.mutate({ id: editingItem.id, data })
    } else {
      createMutation.mutate(data)
    }
  }

  const handleDelete = (id: string) => {
    if (window.confirm('Tem certeza que deseja excluir este site?')) {
      deleteMutation.mutate(id)
    }
  }

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'ATIVO': return 'success'
      case 'INATIVO': return 'error'
      case 'MANUTENCAO': return 'warning'
      case 'ERRO': return 'error'
      default: return 'default'
    }
  }

  const getResponseTimeColor = (time: number) => {
    if (time === 0) return 'error'
    if (time < 100) return 'success'
    if (time < 300) return 'warning'
    return 'error'
  }

  const getUptimeColor = (uptime: number) => {
    if (uptime >= 99.9) return 'success'
    if (uptime >= 99.0) return 'warning'
    return 'error'
  }

  const formatLastCheck = (dateString: string) => {
    const date = new Date(dateString)
    return date.toLocaleString('pt-BR')
  }

  const columns: GridColDef[] = [
    {
      field: 'name',
      headerName: 'Nome do Site',
      width: 200,
      renderCell: (params) => (
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <Avatar sx={{ width: 32, height: 32, bgcolor: 'primary.main' }}>
            <PublicIcon fontSize="small" />
          </Avatar>
          <Typography variant="body2" sx={{ fontWeight: 'medium' }}>
            {params.value}
          </Typography>
        </Box>
      )
    },
    {
      field: 'url',
      headerName: 'URL',
      width: 250,
      renderCell: (params) => (
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <LanguageIcon fontSize="small" color="action" />
          <Typography 
            variant="body2" 
            sx={{ 
              color: 'primary.main', 
              cursor: 'pointer',
              textDecoration: 'underline'
            }}
            onClick={() => window.open(params.value, '_blank')}
          >
            {params.value}
          </Typography>
        </Box>
      )
    },
    {
      field: 'status',
      headerName: 'Status',
      width: 120,
      renderCell: (params) => (
        <Chip 
          label={params.value} 
          color={getStatusColor(params.value) as any}
          size="small"
        />
      )
    },
    {
      field: 'ssl_enabled',
      headerName: 'SSL',
      width: 80,
      renderCell: (params) => (
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <SecurityIcon 
            fontSize="small" 
            color={params.value ? 'success' : 'error'} 
          />
          <Typography variant="body2">
            {params.value ? 'Sim' : 'Não'}
          </Typography>
        </Box>
      )
    },
    {
      field: 'response_time',
      headerName: 'Tempo Resposta (ms)',
      width: 150,
      renderCell: (params) => (
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <SpeedIcon fontSize="small" color="action" />
          <Chip 
            label={params.value === 0 ? 'N/A' : `${params.value}ms`}
            color={getResponseTimeColor(params.value) as any}
            size="small"
          />
        </Box>
      )
    },
    {
      field: 'uptime',
      headerName: 'Uptime (%)',
      width: 120,
      renderCell: (params) => (
        <Chip 
          label={`${params.value}%`}
          color={getUptimeColor(params.value) as any}
          size="small"
        />
      )
    },
    {
      field: 'last_check',
      headerName: 'Última Verificação',
      width: 180,
      renderCell: (params) => (
        <Typography variant="body2">
          {formatLastCheck(params.value)}
        </Typography>
      )
    },
    {
      field: 'actions',
      type: 'actions',
      headerName: 'Ações',
      width: 120,
      getActions: (params: GridRowParams) => [
        <GridActionsCellItem
          icon={<EditIcon />}
          label="Editar"
          onClick={() => handleOpenDialog(params.row)}
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
            Sites e URLs
          </Typography>
          <Typography variant="body1" color="text.secondary">
            Gerencie sites, URLs e monitoramento de disponibilidade
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
          Novo Site
        </Button>
      </Box>

      <Card>
        <CardContent sx={{ p: 0 }}>
          <DataGrid
            rows={sites}
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
        </CardContent>
      </Card>

      {/* Dialog para criar/editar */}
      <Dialog open={openDialog} onClose={handleCloseDialog} maxWidth="sm" fullWidth>
        <DialogTitle>
          {editingItem ? 'Editar Site' : 'Novo Site'}
        </DialogTitle>
        <DialogContent>
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2, pt: 1 }}>
            <TextField
              label="Nome do Site"
              value={formData.name}
              onChange={(e) => setFormData({ ...formData, name: e.target.value })}
              fullWidth
              required
            />
            
            <TextField
              label="URL"
              value={formData.url}
              onChange={(e) => setFormData({ ...formData, url: e.target.value })}
              fullWidth
              required
              placeholder="https://exemplo.com"
            />

            <FormControl fullWidth>
              <InputLabel>Status</InputLabel>
              <Select
                value={formData.status}
                onChange={(e) => setFormData({ ...formData, status: e.target.value })}
                label="Status"
              >
                <MenuItem value="ATIVO">Ativo</MenuItem>
                <MenuItem value="INATIVO">Inativo</MenuItem>
                <MenuItem value="MANUTENCAO">Manutenção</MenuItem>
                <MenuItem value="ERRO">Erro</MenuItem>
              </Select>
            </FormControl>

            <FormControlLabel
              control={
                <Switch
                  checked={formData.ssl_enabled}
                  onChange={(e) => setFormData({ ...formData, ssl_enabled: e.target.checked })}
                  color="primary"
                />
              }
              label="SSL Habilitado"
            />
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleCloseDialog}>Cancelar</Button>
          <Button
            onClick={handleSubmit}
            variant="contained"
            disabled={!formData.name.trim() || !formData.url.trim() || createMutation.isPending || updateMutation.isPending}
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
