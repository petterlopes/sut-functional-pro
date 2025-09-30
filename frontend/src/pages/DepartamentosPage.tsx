import React from 'react'
import { Box, Typography, TextField, Button, List, ListItem, ListItemText, IconButton } from '@mui/material'
import DeleteIcon from '@mui/icons-material/Delete'
import EditIcon from '@mui/icons-material/Edit'
import { useDepartamentos, useCreateDepartamento } from '../api/admin'
import { useAuthFetch } from '../api/client'

export default function DepartamentosPage(){
  const q = useDepartamentos()
  const create = useCreateDepartamento()
  const fetcher = useAuthFetch()
  const [name, setName] = React.useState('')

  return (
    <Box>
      <Typography variant="h5" gutterBottom>Departamentos</Typography>
      <Box sx={{ display: 'flex', gap: 1, mb: 2 }}>
        <TextField label="Nome" size="small" value={name} onChange={e=>setName(e.target.value)} />
        <Button variant="contained" onClick={()=>{ if(!name) return; create.mutate({ departamento: name }); setName('') }}>Criar</Button>
      </Box>
      {q.isLoading ? <em>carregando...</em> : (
        <List>
          {(q.data || []).map((d: any)=>(
            <ListItem key={d.incddepartamento} secondaryAction={<>
              <IconButton edge="end" onClick={async ()=>{
                const newName = prompt('Novo nome', d.departamento)
                if(!newName) return
                try{ await fetcher(`/v1/departamentos/${d.incddepartamento}`, { method: 'PATCH', body: JSON.stringify({ departamento: newName }) }) }catch(e){ }
                q.refetch()
              }}><EditIcon /></IconButton>
              <IconButton edge="end" onClick={async ()=>{
                if(!confirm(`Apagar departamento ${d.departamento}?`)) return
                try{ await fetcher(`/v1/departamentos/${d.incddepartamento}`, { method: 'DELETE' }) }catch(e){ }
                q.refetch()
              }}><DeleteIcon /></IconButton>
            </>}>
              <ListItemText primary={d.departamento} secondary={`id: ${d.incddepartamento}`} />
            </ListItem>
          ))}
        </List>
      )}
    </Box>
  )
}
