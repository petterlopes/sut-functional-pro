import React from 'react'
import {
  Box, Card, CardContent, Typography, Button, Dialog, DialogTitle, DialogContent,
  DialogActions, TextField, FormControl, InputLabel, Select, MenuItem, Alert, Snackbar,
  Chip, Avatar, Tooltip
} from '@mui/material'
import {
  Add as AddIcon, Edit as EditIcon, Delete as DeleteIcon, Business as BusinessIcon,
  People as PeopleIcon, AttachMoney as AttachMoneyIcon
} from '@mui/icons-material'
import { DataGrid, GridColDef, GridActionsCellItem, GridRowParams } from '@mui/x-data-grid'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'

// Mock data para demonstração
const mockDepartamentos = [
  { id: '1', name: 'Recursos Humanos', unit_id: '1', unit_name: 'São Paulo', budget: 500000, employees: 25, status: 'ATIVO' },
  { id: '2', name: 'Tecnologia da Informação', unit_id: '1', unit_name: 'São Paulo', budget: 1200000, employees: 45, status: 'ATIVO' },
  { id: '3', name: 'Financeiro', unit_id: '1', unit_name: 'São Paulo', budget: 300000, employees: 15, status: 'ATIVO' },
  { id: '4', name: 'Marketing', unit_id: '2', unit_name: 'Rio de Janeiro', budget: 800000, employees: 20, status: 'ATIVO' },
  { id: '5', name: 'Vendas', unit_id: '2', unit_name: 'Rio de Janeiro', budget: 1500000, employees: 60, status: 'ATIVO' },
  { id: '6', name: 'Operações', unit_id: '3', unit_name: 'Brasília', budget: 2000000, employees: 80, status: 'ATIVO' },
]

const mockUnidades = [
  { id: '1', name: 'São Paulo' },
  { id: '2', name: 'Rio de Janeiro' },
  { id: '3', name: 'Brasília' },
]

interface Departamento {
  id: string
  name: string
  unit_id: string
  unit_name: string
  budget?: number
  employees?: number
  status: string
}

export default function DepartamentosPage() {
  const [openDialog, setOpenDialog] = React.useState(false)
  const [editingItem, setEditingItem] = React.useState<Departamento | null>(null)
  const [formData, setFormData] = React.useState({
    name: '',
    unit_id: '',
    budget: '',
    employees: '',
    status: 'ATIVO'
  })
  const [snackbar, setSnackbar] = React.useState({ open: false, message: '', severity: 'success' as 'success' | 'error' })

  const queryClient = useQueryClient()

  // Simular query da API
  const { data: departamentos = [], isLoading } = useQuery({
    queryKey: ['departamentos'],
    queryFn: async () => {
      await new Promise(resolve => setTimeout(resolve, 1000))
      return mockDepartamentos
    }
  })

  const { data: unidades = [] } = useQuery({
    queryKey: ['unidades'],
    queryFn: async () => {
      await new Promise(resolve => setTimeout(resolve, 500))
      return mockUnidades
    }
  })

  const createMutation = useMutation({
    mutationFn: async (data: Partial<Departamento>) => {
      await new Promise(resolve => setTimeout(resolve, 500))
      const unit = unidades.find(u => u.id === data.unit_id)
      return { ...data, id: Date.now().toString(), unit_name: unit?.name || '' }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['departamentos'] })
      setSnackbar({ open: true, message: 'Departamento criado com sucesso!', severity: 'success' })
      handleCloseDialog()
    },
    onError: () => {
      setSnackbar({ open: true, message: 'Erro ao criar departamento', severity: 'error' })
    }
  })

  const updateMutation = useMutation({
    mutationFn: async ({ id, data }: { id: string, data: Partial<Departamento> }) => {
      await new Promise(resolve => setTimeout(resolve, 500))
      const unit = unidades.find(u => u.id === data.unit_id)
      return { ...data, id, unit_name: unit?.name || '' }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['departamentos'] })
      setSnackbar({ open: true, message: 'Departamento atualizado com sucesso!', severity: 'success' })
      handleCloseDialog()
    },
    onError: () => {
      setSnackbar({ open: true, message: 'Erro ao atualizar departamento', severity: 'error' })
    }
  })

  const deleteMutation = useMutation({
    mutationFn: async (id: string) => {
      await new Promise(resolve => setTimeout(resolve, 500))
      return id
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['departamentos'] })
      setSnackbar({ open: true, message: 'Departamento excluído com sucesso!', severity: 'success' })
    },
    onError: () => {
      setSnackbar({ open: true, message: 'Erro ao excluir departamento', severity: 'error' })
    }
  })

  const handleOpenDialog = (item?: Departamento) => {
    if (item) {
      setEditingItem(item)
      setFormData({
        name: item.name,
        unit_id: item.unit_id,
        budget: item.budget?.toString() || '',
        employees: item.employees?.toString() || '',
        status: item.status
      })
    } else {
      setEditingItem(null)
      setFormData({
        name: '',
        unit_id: '',
        budget: '',
        employees: '',
        status: 'ATIVO'
      })
    }
    setOpenDialog(true)
  }

  const handleCloseDialog = () => {
    setOpenDialog(false)
    setEditingItem(null)
    setFormData({
      name: '',
      unit_id: '',
      budget: '',
      employees: '',
      status: 'ATIVO'
    })
  }

  const handleSubmit = () => {
    if (!formData.name.trim() || !formData.unit_id) return

    const data = {
      name: formData.name,
      unit_id: formData.unit_id,
      budget: formData.budget ? parseInt(formData.budget) : undefined,
      employees: formData.employees ? parseInt(formData.employees) : undefined,
      status: formData.status
    }

    if (editingItem) {
      updateMutation.mutate({ id: editingItem.id, data })
    } else {
      createMutation.mutate(data)
    }
  }

  const handleDelete = (id: string) => {
    if (window.confirm('Tem certeza que deseja excluir este departamento?')) {
      deleteMutation.mutate(id)
    }
  }

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'ATIVO': return 'success'
      case 'INATIVO': return 'error'
      case 'SUSPENSO': return 'warning'
      default: return 'default'
    }
  }

  const columns: GridColDef[] = [
    {
      field: 'name',
      headerName: 'Nome do Departamento',
      width: 250,
      renderCell: (params) => (
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <Avatar sx={{ width: 32, height: 32, bgcolor: 'primary.main' }}>
            <BusinessIcon fontSize="small" />
          </Avatar>
          <Typography variant="body2" sx={{ fontWeight: 'medium' }}>
            {params.value}
          </Typography>
        </Box>
      )
    },
    {
      field: 'unit_name',
      headerName: 'Unidade Organizacional',
      width: 200,
      renderCell: (params) => (
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <BusinessIcon fontSize="small" color="action" />
          <Typography variant="body2">
            {params.value}
          </Typography>
        </Box>
      )
    },
    {
      field: 'employees',
      headerName: 'Funcionários',
      width: 120,
      renderCell: (params) => (
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <PeopleIcon fontSize="small" color="action" />
          <Typography variant="body2">
            {params.value || 0}
          </Typography>
        </Box>
      )
    },
    {
      field: 'budget',
      headerName: 'Orçamento',
      width: 150,
      renderCell: (params) => (
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <AttachMoneyIcon fontSize="small" color="action" />
          <Typography variant="body2">
            {params.value ? `R$ ${params.value.toLocaleString()}` : 'N/A'}
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
            Departamentos
          </Typography>
          <Typography variant="body1" color="text.secondary">
            Gerencie departamentos e unidades organizacionais
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
          Novo Departamento
        </Button>
      </Box>

      <Card>
        <CardContent sx={{ p: 0 }}>
          <DataGrid
            rows={departamentos}
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
          {editingItem ? 'Editar Departamento' : 'Novo Departamento'}
        </DialogTitle>
        <DialogContent>
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2, pt: 1 }}>
            <TextField
              label="Nome do Departamento"
              value={formData.name}
              onChange={(e) => setFormData({ ...formData, name: e.target.value })}
              fullWidth
              required
            />
            
            <FormControl fullWidth required>
              <InputLabel>Unidade Organizacional</InputLabel>
              <Select
                value={formData.unit_id}
                onChange={(e) => setFormData({ ...formData, unit_id: e.target.value })}
                label="Unidade Organizacional"
              >
                {unidades.map((unidade) => (
                  <MenuItem key={unidade.id} value={unidade.id}>
                    {unidade.name}
                  </MenuItem>
                ))}
              </Select>
            </FormControl>

            <TextField
              label="Orçamento (R$)"
              type="number"
              value={formData.budget}
              onChange={(e) => setFormData({ ...formData, budget: e.target.value })}
              fullWidth
            />

            <TextField
              label="Número de Funcionários"
              type="number"
              value={formData.employees}
              onChange={(e) => setFormData({ ...formData, employees: e.target.value })}
              fullWidth
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
                <MenuItem value="SUSPENSO">Suspenso</MenuItem>
              </Select>
            </FormControl>
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleCloseDialog}>Cancelar</Button>
          <Button
            onClick={handleSubmit}
            variant="contained"
            disabled={!formData.name.trim() || !formData.unit_id || createMutation.isPending || updateMutation.isPending}
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
