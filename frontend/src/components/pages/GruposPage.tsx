import React from 'react'
import {
  Box, Card, CardContent, Typography, Button, Dialog, DialogTitle, DialogContent,
  DialogActions, TextField, FormControl, InputLabel, Select, MenuItem, Alert, Snackbar,
  Chip, Avatar, Switch, FormControlLabel
} from '@mui/material'
import {
  Add as AddIcon, Edit as EditIcon, Delete as DeleteIcon, Group as GroupIcon,
  People as PeopleIcon, Security as SecurityIcon
} from '@mui/icons-material'
import { DataGrid, GridColDef, GridActionsCellItem, GridRowParams } from '@mui/x-data-grid'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'

// Mock data para demonstração
const mockGrupos = [
  { id: '1', name: 'Administradores', description: 'Grupo com acesso total ao sistema', permissions: ['read', 'write', 'delete', 'admin'], member_count: 5, is_active: true },
  { id: '2', name: 'Gerentes', description: 'Grupo para gerentes de departamento', permissions: ['read', 'write'], member_count: 12, is_active: true },
  { id: '3', name: 'Analistas', description: 'Grupo para analistas de dados', permissions: ['read'], member_count: 25, is_active: true },
  { id: '4', name: 'Visitantes', description: 'Grupo com acesso limitado', permissions: ['read'], member_count: 8, is_active: false },
  { id: '5', name: 'Desenvolvedores', description: 'Grupo para desenvolvedores', permissions: ['read', 'write', 'debug'], member_count: 15, is_active: true },
]

interface Grupo {
  id: string
  name: string
  description: string
  permissions: string[]
  member_count: number
  is_active: boolean
}

export default function GruposPage() {
  const [openDialog, setOpenDialog] = React.useState(false)
  const [editingItem, setEditingItem] = React.useState<Grupo | null>(null)
  const [formData, setFormData] = React.useState({
    name: '',
    description: '',
    permissions: [] as string[],
    is_active: true
  })
  const [snackbar, setSnackbar] = React.useState({ open: false, message: '', severity: 'success' as 'success' | 'error' })

  const queryClient = useQueryClient()

  const availablePermissions = [
    { value: 'read', label: 'Leitura' },
    { value: 'write', label: 'Escrita' },
    { value: 'delete', label: 'Exclusão' },
    { value: 'admin', label: 'Administração' },
    { value: 'debug', label: 'Debug' },
    { value: 'export', label: 'Exportação' },
    { value: 'import', label: 'Importação' }
  ]

  // Simular query da API
  const { data: grupos = [], isLoading } = useQuery({
    queryKey: ['grupos'],
    queryFn: async () => {
      await new Promise(resolve => setTimeout(resolve, 1000))
      return mockGrupos
    }
  })

  const createMutation = useMutation({
    mutationFn: async (data: Partial<Grupo>) => {
      await new Promise(resolve => setTimeout(resolve, 500))
      return { ...data, id: Date.now().toString(), member_count: 0 }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['grupos'] })
      setSnackbar({ open: true, message: 'Grupo criado com sucesso!', severity: 'success' })
      handleCloseDialog()
    },
    onError: () => {
      setSnackbar({ open: true, message: 'Erro ao criar grupo', severity: 'error' })
    }
  })

  const updateMutation = useMutation({
    mutationFn: async ({ id, data }: { id: string, data: Partial<Grupo> }) => {
      await new Promise(resolve => setTimeout(resolve, 500))
      return { ...data, id }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['grupos'] })
      setSnackbar({ open: true, message: 'Grupo atualizado com sucesso!', severity: 'success' })
      handleCloseDialog()
    },
    onError: () => {
      setSnackbar({ open: true, message: 'Erro ao atualizar grupo', severity: 'error' })
    }
  })

  const deleteMutation = useMutation({
    mutationFn: async (id: string) => {
      await new Promise(resolve => setTimeout(resolve, 500))
      return id
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['grupos'] })
      setSnackbar({ open: true, message: 'Grupo excluído com sucesso!', severity: 'success' })
    },
    onError: () => {
      setSnackbar({ open: true, message: 'Erro ao excluir grupo', severity: 'error' })
    }
  })

  const handleOpenDialog = (item?: Grupo) => {
    if (item) {
      setEditingItem(item)
      setFormData({
        name: item.name,
        description: item.description,
        permissions: item.permissions,
        is_active: item.is_active
      })
    } else {
      setEditingItem(null)
      setFormData({
        name: '',
        description: '',
        permissions: [],
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
      description: '',
      permissions: [],
      is_active: true
    })
  }

  const handleSubmit = () => {
    if (!formData.name.trim() || !formData.description.trim()) return

    const data = {
      name: formData.name,
      description: formData.description,
      permissions: formData.permissions,
      is_active: formData.is_active
    }

    if (editingItem) {
      updateMutation.mutate({ id: editingItem.id, data })
    } else {
      createMutation.mutate(data)
    }
  }

  const handleDelete = (id: string) => {
    if (window.confirm('Tem certeza que deseja excluir este grupo?')) {
      deleteMutation.mutate(id)
    }
  }

  const handlePermissionChange = (permission: string, checked: boolean) => {
    if (checked) {
      setFormData({ ...formData, permissions: [...formData.permissions, permission] })
    } else {
      setFormData({ ...formData, permissions: formData.permissions.filter(p => p !== permission) })
    }
  }

  const getPermissionColor = (permission: string) => {
    switch (permission) {
      case 'admin': return 'error'
      case 'write': return 'warning'
      case 'delete': return 'error'
      case 'read': return 'success'
      default: return 'default'
    }
  }

  const columns: GridColDef[] = [
    {
      field: 'name',
      headerName: 'Nome do Grupo',
      width: 200,
      renderCell: (params) => (
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <Avatar sx={{ width: 32, height: 32, bgcolor: 'primary.main' }}>
            <GroupIcon fontSize="small" />
          </Avatar>
          <Typography variant="body2" sx={{ fontWeight: 'medium' }}>
            {params.value}
          </Typography>
        </Box>
      )
    },
    {
      field: 'description',
      headerName: 'Descrição',
      width: 300,
      renderCell: (params) => (
        <Typography variant="body2" sx={{ 
          overflow: 'hidden',
          textOverflow: 'ellipsis',
          display: '-webkit-box',
          WebkitLineClamp: 2,
          WebkitBoxOrient: 'vertical'
        }}>
          {params.value}
        </Typography>
      )
    },
    {
      field: 'permissions',
      headerName: 'Permissões',
      width: 250,
      renderCell: (params) => (
        <Box sx={{ display: 'flex', gap: 0.5, flexWrap: 'wrap' }}>
          {params.value.map((permission: string) => (
            <Chip
              key={permission}
              label={permission}
              color={getPermissionColor(permission) as any}
              size="small"
            />
          ))}
        </Box>
      )
    },
    {
      field: 'member_count',
      headerName: 'Membros',
      width: 100,
      renderCell: (params) => (
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <PeopleIcon fontSize="small" color="action" />
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
            Grupos de Usuários
          </Typography>
          <Typography variant="body1" color="text.secondary">
            Gerencie grupos de usuários e suas permissões
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
          Novo Grupo
        </Button>
      </Box>

      <Card>
        <CardContent sx={{ p: 0 }}>
          <DataGrid
            rows={grupos}
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
          {editingItem ? 'Editar Grupo' : 'Novo Grupo'}
        </DialogTitle>
        <DialogContent>
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2, pt: 1 }}>
            <TextField
              label="Nome do Grupo"
              value={formData.name}
              onChange={(e) => setFormData({ ...formData, name: e.target.value })}
              fullWidth
              required
            />
            
            <TextField
              label="Descrição"
              value={formData.description}
              onChange={(e) => setFormData({ ...formData, description: e.target.value })}
              fullWidth
              multiline
              rows={3}
              required
            />

            <Box>
              <Typography variant="subtitle2" sx={{ mb: 1 }}>
                Permissões
              </Typography>
              <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
                {availablePermissions.map((permission) => (
                  <FormControlLabel
                    key={permission.value}
                    control={
                      <Switch
                        checked={formData.permissions.includes(permission.value)}
                        onChange={(e) => handlePermissionChange(permission.value, e.target.checked)}
                        color="primary"
                      />
                    }
                    label={
                      <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                        <SecurityIcon fontSize="small" />
                        <Typography variant="body2">
                          {permission.label}
                        </Typography>
                        <Chip
                          label={permission.value}
                          color={getPermissionColor(permission.value) as any}
                          size="small"
                        />
                      </Box>
                    }
                  />
                ))}
              </Box>
            </Box>

            <FormControlLabel
              control={
                <Switch
                  checked={formData.is_active}
                  onChange={(e) => setFormData({ ...formData, is_active: e.target.checked })}
                  color="primary"
                />
              }
              label="Grupo Ativo"
            />
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleCloseDialog}>Cancelar</Button>
          <Button
            onClick={handleSubmit}
            variant="contained"
            disabled={!formData.name.trim() || !formData.description.trim() || createMutation.isPending || updateMutation.isPending}
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
