import React from 'react'
import {
  Box, Card, CardContent, Typography, Button, IconButton, Dialog, DialogTitle, DialogContent,
  DialogActions, TextField, FormControl, InputLabel, Select, MenuItem, Alert, Snackbar,
  Chip, Avatar, Tooltip
} from '@mui/material'
import {
  Add as AddIcon, Edit as EditIcon, Delete as DeleteIcon, Visibility as VisibilityIcon,
  LocationOn as LocationOnIcon, Business as BusinessIcon
} from '@mui/icons-material'
import { DataGrid, GridColDef, GridActionsCellItem, GridRowParams } from '@mui/x-data-grid'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'

// Mock data para demonstração - em produção viria da API
const mockLocalidades = [
  { id: '1', name: 'São Paulo', parent_id: null, type: 'ESTADO', population: 12396372, area: 1521.11 },
  { id: '2', name: 'Rio de Janeiro', parent_id: null, type: 'ESTADO', population: 6775561, area: 1200.27 },
  { id: '3', name: 'Brasília', parent_id: null, type: 'DISTRITO_FEDERAL', population: 3015268, area: 5779.999 },
  { id: '4', name: 'Centro', parent_id: '1', type: 'BAIRRO', population: 150000, area: 5.2 },
  { id: '5', name: 'Vila Madalena', parent_id: '1', type: 'BAIRRO', population: 45000, area: 8.1 },
  { id: '6', name: 'Copacabana', parent_id: '2', type: 'BAIRRO', population: 146392, area: 4.1 },
]

interface Localidade {
  id: string
  name: string
  parent_id: string | null
  type: string
  population?: number
  area?: number
}

export default function LocalidadesPage() {
  const [openDialog, setOpenDialog] = React.useState(false)
  const [editingItem, setEditingItem] = React.useState<Localidade | null>(null)
  const [formData, setFormData] = React.useState({
    name: '',
    parent_id: '',
    type: 'BAIRRO',
    population: '',
    area: ''
  })
  const [snackbar, setSnackbar] = React.useState({ open: false, message: '', severity: 'success' as 'success' | 'error' })

  const queryClient = useQueryClient()

  // Simular query da API
  const { data: localidades = [], isLoading } = useQuery({
    queryKey: ['localidades'],
    queryFn: async () => {
      // Simular delay da API
      await new Promise(resolve => setTimeout(resolve, 1000))
      return mockLocalidades
    }
  })

  const createMutation = useMutation({
    mutationFn: async (data: Partial<Localidade>) => {
      // Simular criação
      await new Promise(resolve => setTimeout(resolve, 500))
      return { ...data, id: Date.now().toString() }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['localidades'] })
      setSnackbar({ open: true, message: 'Localidade criada com sucesso!', severity: 'success' })
      handleCloseDialog()
    },
    onError: () => {
      setSnackbar({ open: true, message: 'Erro ao criar localidade', severity: 'error' })
    }
  })

  const updateMutation = useMutation({
    mutationFn: async ({ id, data }: { id: string, data: Partial<Localidade> }) => {
      // Simular atualização
      await new Promise(resolve => setTimeout(resolve, 500))
      return { ...data, id }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['localidades'] })
      setSnackbar({ open: true, message: 'Localidade atualizada com sucesso!', severity: 'success' })
      handleCloseDialog()
    },
    onError: () => {
      setSnackbar({ open: true, message: 'Erro ao atualizar localidade', severity: 'error' })
    }
  })

  const deleteMutation = useMutation({
    mutationFn: async (id: string) => {
      // Simular exclusão
      await new Promise(resolve => setTimeout(resolve, 500))
      return id
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['localidades'] })
      setSnackbar({ open: true, message: 'Localidade excluída com sucesso!', severity: 'success' })
    },
    onError: () => {
      setSnackbar({ open: true, message: 'Erro ao excluir localidade', severity: 'error' })
    }
  })

  const handleOpenDialog = (item?: Localidade) => {
    if (item) {
      setEditingItem(item)
      setFormData({
        name: item.name,
        parent_id: item.parent_id || '',
        type: item.type,
        population: item.population?.toString() || '',
        area: item.area?.toString() || ''
      })
    } else {
      setEditingItem(null)
      setFormData({
        name: '',
        parent_id: '',
        type: 'BAIRRO',
        population: '',
        area: ''
      })
    }
    setOpenDialog(true)
  }

  const handleCloseDialog = () => {
    setOpenDialog(false)
    setEditingItem(null)
    setFormData({
      name: '',
      parent_id: '',
      type: 'BAIRRO',
      population: '',
      area: ''
    })
  }

  const handleSubmit = () => {
    if (!formData.name.trim()) return

    const data = {
      name: formData.name,
      parent_id: formData.parent_id || null,
      type: formData.type,
      population: formData.population ? parseInt(formData.population) : undefined,
      area: formData.area ? parseFloat(formData.area) : undefined
    }

    if (editingItem) {
      updateMutation.mutate({ id: editingItem.id, data })
    } else {
      createMutation.mutate(data)
    }
  }

  const handleDelete = (id: string) => {
    if (window.confirm('Tem certeza que deseja excluir esta localidade?')) {
      deleteMutation.mutate(id)
    }
  }

  const getParentName = (parentId: string | null) => {
    if (!parentId) return 'N/A'
    const parent = localidades.find(l => l.id === parentId)
    return parent?.name || 'N/A'
  }

  const getTypeColor = (type: string) => {
    switch (type) {
      case 'ESTADO': return 'primary'
      case 'DISTRITO_FEDERAL': return 'secondary'
      case 'BAIRRO': return 'success'
      default: return 'default'
    }
  }

  const columns: GridColDef[] = [
    {
      field: 'name',
      headerName: 'Nome',
      width: 200,
      renderCell: (params) => (
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <Avatar sx={{ width: 32, height: 32, bgcolor: 'primary.main' }}>
            <LocationOnIcon fontSize="small" />
          </Avatar>
          <Typography variant="body2" sx={{ fontWeight: 'medium' }}>
            {params.value}
          </Typography>
        </Box>
      )
    },
    {
      field: 'type',
      headerName: 'Tipo',
      width: 150,
      renderCell: (params) => (
        <Chip 
          label={params.value} 
          color={getTypeColor(params.value) as any}
          size="small"
        />
      )
    },
    {
      field: 'parent_id',
      headerName: 'Localização Pai',
      width: 180,
      renderCell: (params) => (
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <BusinessIcon fontSize="small" color="action" />
          <Typography variant="body2">
            {getParentName(params.value)}
          </Typography>
        </Box>
      )
    },
    {
      field: 'population',
      headerName: 'População',
      width: 120,
      renderCell: (params) => (
        <Typography variant="body2">
          {params.value ? params.value.toLocaleString() : 'N/A'}
        </Typography>
      )
    },
    {
      field: 'area',
      headerName: 'Área (km²)',
      width: 120,
      renderCell: (params) => (
        <Typography variant="body2">
          {params.value ? params.value.toFixed(2) : 'N/A'}
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
            Localidades
          </Typography>
          <Typography variant="body1" color="text.secondary">
            Gerencie localidades, estados, cidades e bairros
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
          Nova Localidade
        </Button>
      </Box>

      <Card>
        <CardContent sx={{ p: 0 }}>
          <DataGrid
            rows={localidades}
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
          {editingItem ? 'Editar Localidade' : 'Nova Localidade'}
        </DialogTitle>
        <DialogContent>
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2, pt: 1 }}>
            <TextField
              label="Nome"
              value={formData.name}
              onChange={(e) => setFormData({ ...formData, name: e.target.value })}
              fullWidth
              required
            />
            
            <FormControl fullWidth>
              <InputLabel>Tipo</InputLabel>
              <Select
                value={formData.type}
                onChange={(e) => setFormData({ ...formData, type: e.target.value })}
                label="Tipo"
              >
                <MenuItem value="ESTADO">Estado</MenuItem>
                <MenuItem value="DISTRITO_FEDERAL">Distrito Federal</MenuItem>
                <MenuItem value="CIDADE">Cidade</MenuItem>
                <MenuItem value="BAIRRO">Bairro</MenuItem>
              </Select>
            </FormControl>

            <FormControl fullWidth>
              <InputLabel>Localização Pai</InputLabel>
              <Select
                value={formData.parent_id}
                onChange={(e) => setFormData({ ...formData, parent_id: e.target.value })}
                label="Localização Pai"
              >
                <MenuItem value="">Nenhuma</MenuItem>
                {localidades
                  .filter(l => l.type === 'ESTADO' || l.type === 'DISTRITO_FEDERAL')
                  .map((localidade) => (
                    <MenuItem key={localidade.id} value={localidade.id}>
                      {localidade.name}
                    </MenuItem>
                  ))}
              </Select>
            </FormControl>

            <TextField
              label="População"
              type="number"
              value={formData.population}
              onChange={(e) => setFormData({ ...formData, population: e.target.value })}
              fullWidth
            />

            <TextField
              label="Área (km²)"
              type="number"
              step="0.01"
              value={formData.area}
              onChange={(e) => setFormData({ ...formData, area: e.target.value })}
              fullWidth
            />
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleCloseDialog}>Cancelar</Button>
          <Button
            onClick={handleSubmit}
            variant="contained"
            disabled={!formData.name.trim() || createMutation.isPending || updateMutation.isPending}
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
