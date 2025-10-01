import React from 'react'
import {
  Box, Card, CardContent, Typography, Button, Dialog, DialogTitle, DialogContent,
  DialogActions, TextField, FormControl, InputLabel, Select, MenuItem, Alert, Snackbar,
  Chip, Avatar, Tooltip, IconButton
} from '@mui/material'
import {
  Add as AddIcon, Edit as EditIcon, Delete as DeleteIcon, Book as BookIcon,
  Search as SearchIcon, FilterList as FilterListIcon, Sort as SortIcon
} from '@mui/icons-material'
import { DataGrid, GridColDef, GridActionsCellItem, GridRowParams } from '@mui/x-data-grid'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'

// Mock data para demonstração
const mockLexico = [
  { id: '1', term: 'API', definition: 'Application Programming Interface', category: 'TECNOLOGIA', language: 'PT-BR', usage_count: 150, status: 'ATIVO' },
  { id: '2', term: 'CRUD', definition: 'Create, Read, Update, Delete', category: 'TECNOLOGIA', language: 'PT-BR', usage_count: 89, status: 'ATIVO' },
  { id: '3', term: 'Dashboard', definition: 'Painel de controle com métricas e indicadores', category: 'INTERFACE', language: 'PT-BR', usage_count: 45, status: 'ATIVO' },
  { id: '4', term: 'Authentication', definition: 'Processo de verificação de identidade', category: 'SEGURANCA', language: 'EN-US', usage_count: 67, status: 'ATIVO' },
  { id: '5', term: 'Database', definition: 'Base de dados estruturada', category: 'TECNOLOGIA', language: 'EN-US', usage_count: 123, status: 'ATIVO' },
  { id: '6', term: 'Middleware', definition: 'Software intermediário entre aplicações', category: 'ARQUITETURA', language: 'PT-BR', usage_count: 34, status: 'ATIVO' },
]

interface LexicoItem {
  id: string
  term: string
  definition: string
  category: string
  language: string
  usage_count: number
  status: string
  synonyms?: string[]
  related_terms?: string[]
}

export default function LexicoPage() {
  const [openDialog, setOpenDialog] = React.useState(false)
  const [editingItem, setEditingItem] = React.useState<LexicoItem | null>(null)
  const [formData, setFormData] = React.useState({
    term: '',
    definition: '',
    category: 'TECNOLOGIA',
    language: 'PT-BR',
    status: 'ATIVO',
    synonyms: '',
    related_terms: ''
  })
  const [snackbar, setSnackbar] = React.useState({ open: false, message: '', severity: 'success' as 'success' | 'error' })
  const [searchTerm, setSearchTerm] = React.useState('')
  const [filterCategory, setFilterCategory] = React.useState('')

  const queryClient = useQueryClient()

  // Simular query da API
  const { data: lexico = [], isLoading } = useQuery({
    queryKey: ['lexico'],
    queryFn: async () => {
      await new Promise(resolve => setTimeout(resolve, 1000))
      return mockLexico
    }
  })

  const createMutation = useMutation({
    mutationFn: async (data: Partial<LexicoItem>) => {
      await new Promise(resolve => setTimeout(resolve, 500))
      return { ...data, id: Date.now().toString(), usage_count: 0 }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['lexico'] })
      setSnackbar({ open: true, message: 'Termo criado com sucesso!', severity: 'success' })
      handleCloseDialog()
    },
    onError: () => {
      setSnackbar({ open: true, message: 'Erro ao criar termo', severity: 'error' })
    }
  })

  const updateMutation = useMutation({
    mutationFn: async ({ id, data }: { id: string, data: Partial<LexicoItem> }) => {
      await new Promise(resolve => setTimeout(resolve, 500))
      return { ...data, id }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['lexico'] })
      setSnackbar({ open: true, message: 'Termo atualizado com sucesso!', severity: 'success' })
      handleCloseDialog()
    },
    onError: () => {
      setSnackbar({ open: true, message: 'Erro ao atualizar termo', severity: 'error' })
    }
  })

  const deleteMutation = useMutation({
    mutationFn: async (id: string) => {
      await new Promise(resolve => setTimeout(resolve, 500))
      return id
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['lexico'] })
      setSnackbar({ open: true, message: 'Termo excluído com sucesso!', severity: 'success' })
    },
    onError: () => {
      setSnackbar({ open: true, message: 'Erro ao excluir termo', severity: 'error' })
    }
  })

  const handleOpenDialog = (item?: LexicoItem) => {
    if (item) {
      setEditingItem(item)
      setFormData({
        term: item.term,
        definition: item.definition,
        category: item.category,
        language: item.language,
        status: item.status,
        synonyms: item.synonyms?.join(', ') || '',
        related_terms: item.related_terms?.join(', ') || ''
      })
    } else {
      setEditingItem(null)
      setFormData({
        term: '',
        definition: '',
        category: 'TECNOLOGIA',
        language: 'PT-BR',
        status: 'ATIVO',
        synonyms: '',
        related_terms: ''
      })
    }
    setOpenDialog(true)
  }

  const handleCloseDialog = () => {
    setOpenDialog(false)
    setEditingItem(null)
    setFormData({
      term: '',
      definition: '',
      category: 'TECNOLOGIA',
      language: 'PT-BR',
      status: 'ATIVO',
      synonyms: '',
      related_terms: ''
    })
  }

  const handleSubmit = () => {
    if (!formData.term.trim() || !formData.definition.trim()) return

    const data = {
      term: formData.term,
      definition: formData.definition,
      category: formData.category,
      language: formData.language,
      status: formData.status,
      synonyms: formData.synonyms ? formData.synonyms.split(',').map(s => s.trim()).filter(s => s) : [],
      related_terms: formData.related_terms ? formData.related_terms.split(',').map(s => s.trim()).filter(s => s) : []
    }

    if (editingItem) {
      updateMutation.mutate({ id: editingItem.id, data })
    } else {
      createMutation.mutate(data)
    }
  }

  const handleDelete = (id: string) => {
    if (window.confirm('Tem certeza que deseja excluir este termo?')) {
      deleteMutation.mutate(id)
    }
  }

  const getCategoryColor = (category: string) => {
    switch (category) {
      case 'TECNOLOGIA': return 'primary'
      case 'INTERFACE': return 'secondary'
      case 'SEGURANCA': return 'error'
      case 'ARQUITETURA': return 'warning'
      case 'NEGOCIO': return 'success'
      default: return 'default'
    }
  }

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'ATIVO': return 'success'
      case 'INATIVO': return 'error'
      case 'PENDENTE': return 'warning'
      default: return 'default'
    }
  }

  const getLanguageFlag = (language: string) => {
    switch (language) {
      case 'PT-BR': return '🇧🇷'
      case 'EN-US': return '🇺🇸'
      case 'ES-ES': return '🇪🇸'
      default: return '🌐'
    }
  }

  // Filtrar dados
  const filteredData = lexico.filter(item => {
    const matchesSearch = !searchTerm || 
      item.term.toLowerCase().includes(searchTerm.toLowerCase()) ||
      item.definition.toLowerCase().includes(searchTerm.toLowerCase())
    const matchesCategory = !filterCategory || item.category === filterCategory
    return matchesSearch && matchesCategory
  })

  const columns: GridColDef[] = [
    {
      field: 'term',
      headerName: 'Termo',
      width: 200,
      renderCell: (params) => (
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <Avatar sx={{ width: 32, height: 32, bgcolor: 'primary.main' }}>
            <BookIcon fontSize="small" />
          </Avatar>
          <Typography variant="body2" sx={{ fontWeight: 'medium' }}>
            {params.value}
          </Typography>
        </Box>
      )
    },
    {
      field: 'definition',
      headerName: 'Definição',
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
      field: 'category',
      headerName: 'Categoria',
      width: 150,
      renderCell: (params) => (
        <Chip 
          label={params.value} 
          color={getCategoryColor(params.value) as any}
          size="small"
        />
      )
    },
    {
      field: 'language',
      headerName: 'Idioma',
      width: 100,
      renderCell: (params) => (
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <Typography variant="body2">
            {getLanguageFlag(params.value)}
          </Typography>
          <Typography variant="body2">
            {params.value}
          </Typography>
        </Box>
      )
    },
    {
      field: 'usage_count',
      headerName: 'Uso',
      width: 100,
      renderCell: (params) => (
        <Chip 
          label={params.value} 
          color={params.value > 100 ? 'success' : params.value > 50 ? 'warning' : 'default'}
          size="small"
        />
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
            Léxico Técnico
          </Typography>
          <Typography variant="body1" color="text.secondary">
            Gerencie termos técnicos, definições e glossário do sistema
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
          Novo Termo
        </Button>
      </Box>

      {/* Filtros */}
      <Card sx={{ mb: 3 }}>
        <CardContent>
          <Box sx={{ display: 'flex', gap: 2, alignItems: 'center' }}>
            <TextField
              placeholder="Buscar termos..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              InputProps={{
                startAdornment: <SearchIcon sx={{ mr: 1, color: 'text.secondary' }} />
              }}
              sx={{ minWidth: 300 }}
            />
            <FormControl sx={{ minWidth: 200 }}>
              <InputLabel>Categoria</InputLabel>
              <Select
                value={filterCategory}
                onChange={(e) => setFilterCategory(e.target.value)}
                label="Categoria"
              >
                <MenuItem value="">Todas</MenuItem>
                <MenuItem value="TECNOLOGIA">Tecnologia</MenuItem>
                <MenuItem value="INTERFACE">Interface</MenuItem>
                <MenuItem value="SEGURANCA">Segurança</MenuItem>
                <MenuItem value="ARQUITETURA">Arquitetura</MenuItem>
                <MenuItem value="NEGOCIO">Negócio</MenuItem>
              </Select>
            </FormControl>
            <Button
              variant="outlined"
              startIcon={<FilterListIcon />}
              onClick={() => {
                setSearchTerm('')
                setFilterCategory('')
              }}
            >
              Limpar Filtros
            </Button>
          </Box>
        </CardContent>
      </Card>

      <Card>
        <CardContent sx={{ p: 0 }}>
          <DataGrid
            rows={filteredData}
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
      <Dialog open={openDialog} onClose={handleCloseDialog} maxWidth="md" fullWidth>
        <DialogTitle>
          {editingItem ? 'Editar Termo' : 'Novo Termo'}
        </DialogTitle>
        <DialogContent>
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2, pt: 1 }}>
            <TextField
              label="Termo"
              value={formData.term}
              onChange={(e) => setFormData({ ...formData, term: e.target.value })}
              fullWidth
              required
            />
            
            <TextField
              label="Definição"
              value={formData.definition}
              onChange={(e) => setFormData({ ...formData, definition: e.target.value })}
              fullWidth
              multiline
              rows={3}
              required
            />

            <Box sx={{ display: 'flex', gap: 2 }}>
              <FormControl sx={{ flex: 1 }}>
                <InputLabel>Categoria</InputLabel>
                <Select
                  value={formData.category}
                  onChange={(e) => setFormData({ ...formData, category: e.target.value })}
                  label="Categoria"
                >
                  <MenuItem value="TECNOLOGIA">Tecnologia</MenuItem>
                  <MenuItem value="INTERFACE">Interface</MenuItem>
                  <MenuItem value="SEGURANCA">Segurança</MenuItem>
                  <MenuItem value="ARQUITETURA">Arquitetura</MenuItem>
                  <MenuItem value="NEGOCIO">Negócio</MenuItem>
                </Select>
              </FormControl>

              <FormControl sx={{ flex: 1 }}>
                <InputLabel>Idioma</InputLabel>
                <Select
                  value={formData.language}
                  onChange={(e) => setFormData({ ...formData, language: e.target.value })}
                  label="Idioma"
                >
                  <MenuItem value="PT-BR">🇧🇷 Português (BR)</MenuItem>
                  <MenuItem value="EN-US">🇺🇸 English (US)</MenuItem>
                  <MenuItem value="ES-ES">🇪🇸 Español (ES)</MenuItem>
                </Select>
              </FormControl>
            </Box>

            <TextField
              label="Sinônimos (separados por vírgula)"
              value={formData.synonyms}
              onChange={(e) => setFormData({ ...formData, synonyms: e.target.value })}
              fullWidth
              placeholder="ex: Interface, API, Endpoint"
            />

            <TextField
              label="Termos Relacionados (separados por vírgula)"
              value={formData.related_terms}
              onChange={(e) => setFormData({ ...formData, related_terms: e.target.value })}
              fullWidth
              placeholder="ex: REST, HTTP, JSON"
            />

            <FormControl>
              <InputLabel>Status</InputLabel>
              <Select
                value={formData.status}
                onChange={(e) => setFormData({ ...formData, status: e.target.value })}
                label="Status"
              >
                <MenuItem value="ATIVO">Ativo</MenuItem>
                <MenuItem value="INATIVO">Inativo</MenuItem>
                <MenuItem value="PENDENTE">Pendente</MenuItem>
              </Select>
            </FormControl>
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleCloseDialog}>Cancelar</Button>
          <Button
            onClick={handleSubmit}
            variant="contained"
            disabled={!formData.term.trim() || !formData.definition.trim() || createMutation.isPending || updateMutation.isPending}
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
