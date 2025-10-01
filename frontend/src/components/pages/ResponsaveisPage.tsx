import React from 'react'
import {
  Box, Card, CardContent, Typography, Button, Dialog, DialogTitle, DialogContent,
  DialogActions, TextField, FormControl, InputLabel, Select, MenuItem, Alert, Snackbar,
  Chip, Avatar, Tooltip
} from '@mui/material'
import {
  Add as AddIcon, Edit as EditIcon, Delete as DeleteIcon, Person as PersonIcon,
  Email as EmailIcon, Phone as PhoneIcon, Business as BusinessIcon
} from '@mui/icons-material'
import { DataGrid, GridColDef, GridActionsCellItem, GridRowParams } from '@mui/x-data-grid'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'

// Mock data para demonstração
const mockResponsaveis = [
  { id: '1', name: 'João Silva', email: 'joao.silva@empresa.com', phone: '+55 11 99999-9999', department: 'TI', position: 'Gerente', is_active: true },
  { id: '2', name: 'Maria Santos', email: 'maria.santos@empresa.com', phone: '+55 11 88888-8888', department: 'RH', position: 'Coordenadora', is_active: true },
  { id: '3', name: 'Pedro Costa', email: 'pedro.costa@empresa.com', phone: '+55 11 77777-7777', department: 'Financeiro', position: 'Analista', is_active: true },
  { id: '4', name: 'Ana Oliveira', email: 'ana.oliveira@empresa.com', phone: '+55 11 66666-6666', department: 'Marketing', position: 'Supervisora', is_active: false },
  { id: '5', name: 'Carlos Ferreira', email: 'carlos.ferreira@empresa.com', phone: '+55 11 55555-5555', department: 'Vendas', position: 'Diretor', is_active: true },
]

const mockDepartamentos = [
  'TI', 'RH', 'Financeiro', 'Marketing', 'Vendas', 'Operações', 'Jurídico'
]

const mockPosicoes = [
  'Diretor', 'Gerente', 'Coordenador', 'Supervisor', 'Analista', 'Assistente', 'Estagiário'
]

interface Responsavel {
  id: string
  name: string
  email: string
  phone: string
  department: string
  position: string
  is_active: boolean
}

export default function ResponsaveisPage() {
  const [openDialog, setOpenDialog] = React.useState(false)
  const [editingItem, setEditingItem] = React.useState<Responsavel | null>(null)
  const [formData, setFormData] = React.useState({
    name: '',
    email: '',
    phone: '',
    department: '',
    position: '',
    is_active: true
  })
  const [snackbar, setSnackbar] = React.useState({ open: false, message: '', severity: 'success' as 'success' | 'error' })

  const queryClient = useQueryClient()

  // Simular query da API
  const { data: responsaveis = [], isLoading } = useQuery({
    queryKey: ['responsaveis'],
    queryFn: async () => {
      await new Promise(resolve => setTimeout(resolve, 1000))
      return mockResponsaveis
    }
  })

  const createMutation = useMutation({
    mutationFn: async (data: Partial<Responsavel>) => {
      await new Promise(resolve => setTimeout(resolve, 500))
      return { ...data, id: Date.now().toString() }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['responsaveis'] })
      setSnackbar({ open: true, message: 'Responsável criado com sucesso!', severity: 'success' })
      handleCloseDialog()
    },
    onError: () => {
      setSnackbar({ open: true, message: 'Erro ao criar responsável', severity: 'error' })
    }
  })

  const updateMutation = useMutation({
    mutationFn: async ({ id, data }: { id: string, data: Partial<Responsavel> }) => {
      await new Promise(resolve => setTimeout(resolve, 500))
      return { ...data, id }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['responsaveis'] })
      setSnackbar({ open: true, message: 'Responsável atualizado com sucesso!', severity: 'success' })
      handleCloseDialog()
    },
    onError: () => {
      setSnackbar({ open: true, message: 'Erro ao atualizar responsável', severity: 'error' })
    }
  })

  const deleteMutation = useMutation({
    mutationFn: async (id: string) => {
      await new Promise(resolve => setTimeout(resolve, 500))
      return id
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['responsaveis'] })
      setSnackbar({ open: true, message: 'Responsável excluído com sucesso!', severity: 'success' })
    },
    onError: () => {
      setSnackbar({ open: true, message: 'Erro ao excluir responsável', severity: 'error' })
    }
  })

  const handleOpenDialog = (item?: Responsavel) => {
    if (item) {
      setEditingItem(item)
      setFormData({
        name: item.name,
        email: item.email,
        phone: item.phone,
        department: item.department,
        position: item.position,
        is_active: item.is_active
      })
    } else {
      setEditingItem(null)
      setFormData({
        name: '',
        email: '',
        phone: '',
        department: '',
        position: '',
        is_active: true
      })
    }
    setOpenDialog(true)
  }

  const handleCloseDialog = () => {
    setOpenDialog(false)
    setEditingItem(null)
    setFormData({
      name: '',
      email: '',
      phone: '',
      department: '',
      position: '',
      is_active: true
    })
  }

  const handleSubmit = () => {
    if (!formData.name.trim() || !formData.email.trim()) return

    const data = {
      name: formData.name,
      email: formData.email,
      phone: formData.phone,
      department: formData.department,
      position: formData.position,
      is_active: formData.is_active
    }

    if (editingItem) {
      updateMutation.mutate({ id: editingItem.id, data })
    } else {
      createMutation.mutate(data)
    }
  }

  const handleDelete = (id: string) => {
    if (window.confirm('Tem certeza que deseja excluir este responsável?')) {
      deleteMutation.mutate(id)
    }
  }

  const getDepartmentColor = (department: string) => {
    const colors = {
      'TI': 'primary',
      'RH': 'secondary',
      'Financeiro': 'success',
      'Marketing': 'warning',
      'Vendas': 'error',
      'Operações': 'info',
      'Jurídico': 'default'
    }
    return colors[department as keyof typeof colors] || 'default'
  }

  const columns: GridColDef[] = [
    {
      field: 'name',
      headerName: 'Nome',
      width: 200,
      renderCell: (params) => (
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <Avatar sx={{ width: 32, height: 32, bgcolor: 'primary.main' }}>
            <PersonIcon fontSize="small" />
          </Avatar>
          <Typography variant="body2" sx={{ fontWeight: 'medium' }}>
            {params.value}
          </Typography>
        </Box>
      )
    },
    {
      field: 'email',
      headerName: 'Email',
      width: 250,
      renderCell: (params) => (
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <EmailIcon fontSize="small" color="action" />
          <Typography variant="body2">
            {params.value}
          </Typography>
        </Box>
      )
    },
    {
      field: 'phone',
      headerName: 'Telefone',
      width: 150,
      renderCell: (params) => (
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <PhoneIcon fontSize="small" color="action" />
          <Typography variant="body2">
            {params.value}
          </Typography>
        </Box>
      )
    },
    {
      field: 'department',
      headerName: 'Departamento',
      width: 150,
      renderCell: (params) => (
        <Chip 
          label={params.value} 
          color={getDepartmentColor(params.value) as any}
          size="small"
        />
      )
    },
    {
      field: 'position',
      headerName: 'Cargo',
      width: 150,
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
      field: 'is_active',
      headerName: 'Status',
      width: 100,
      renderCell: (params) => (
        <Chip 
          label={params.value ? 'Ativo' : 'Inativo'} 
          color={params.value ? 'success' : 'error'}
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
            Responsáveis
          </Typography>
          <Typography variant="body1" color="text.secondary">
            Gerencie responsáveis e contatos organizacionais
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
          Novo Responsável
        </Button>
      </Box>

      <Card>
        <CardContent sx={{ p: 0 }}>
          <DataGrid
            rows={responsaveis}
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
          {editingItem ? 'Editar Responsável' : 'Novo Responsável'}
        </DialogTitle>
        <DialogContent>
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2, pt: 1 }}>
            <TextField
              label="Nome Completo"
              value={formData.name}
              onChange={(e) => setFormData({ ...formData, name: e.target.value })}
              fullWidth
              required
            />
            
            <TextField
              label="Email"
              type="email"
              value={formData.email}
              onChange={(e) => setFormData({ ...formData, email: e.target.value })}
              fullWidth
              required
            />

            <TextField
              label="Telefone"
              value={formData.phone}
              onChange={(e) => setFormData({ ...formData, phone: e.target.value })}
              fullWidth
              placeholder="+55 11 99999-9999"
            />

            <Box sx={{ display: 'flex', gap: 2 }}>
              <FormControl sx={{ flex: 1 }}>
                <InputLabel>Departamento</InputLabel>
                <Select
                  value={formData.department}
                  onChange={(e) => setFormData({ ...formData, department: e.target.value })}
                  label="Departamento"
                >
                  {mockDepartamentos.map((dept) => (
                    <MenuItem key={dept} value={dept}>
                      {dept}
                    </MenuItem>
                  ))}
                </Select>
              </FormControl>

              <FormControl sx={{ flex: 1 }}>
                <InputLabel>Cargo</InputLabel>
                <Select
                  value={formData.position}
                  onChange={(e) => setFormData({ ...formData, position: e.target.value })}
                  label="Cargo"
                >
                  {mockPosicoes.map((pos) => (
                    <MenuItem key={pos} value={pos}>
                      {pos}
                    </MenuItem>
                  ))}
                </Select>
              </FormControl>
            </Box>
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleCloseDialog}>Cancelar</Button>
          <Button
            onClick={handleSubmit}
            variant="contained"
            disabled={!formData.name.trim() || !formData.email.trim() || createMutation.isPending || updateMutation.isPending}
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
