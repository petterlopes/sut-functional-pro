import React from 'react'
import { Box, Typography, TextField, Button, List, ListItem, ListItemText, IconButton } from '@mui/material'
import DeleteIcon from '@mui/icons-material/Delete'
import EditIcon from '@mui/icons-material/Edit'
import { useLocalidades, useCreateLocalidade } from '../api/admin'
import { useAuthFetch } from '../api/client'

export default function LocalidadesPage(){
  const q = useLocalidades()
  const create = useCreateLocalidade()
  const fetcher = useAuthFetch()
  const [name, setName] = React.useState('')


  return (
    <Box>
      <Typography variant="h5" gutterBottom>Localidades</Typography>
      <Box sx={{ display: 'flex', gap: 1, mb: 2 }}>
        <TextField label="Nome" size="small" value={name} onChange={e=>setName(e.target.value)} />
        <Button variant="contained" onClick={()=>{ if(!name) return; create.mutate({ txnomelocalidade: name }); setName('') }}>Criar</Button>
      </Box>
      {q.isLoading ? <em>carregando...</em> : (
        <List>
          {(q.data || []).map((l: any)=>(
            <ListItem key={l.incdlocalidade} secondaryAction={<>
              <IconButton edge="end" onClick={async ()=>{
                if(!confirm(`Apagar localidade ${l.txnomelocalidade}?`)) return
                try{ await fetcher(`/v1/localidades/${l.incdlocalidade}`, { method: 'DELETE' }) }catch(e){ }
                q.refetch()
              }}><DeleteIcon /></IconButton>
            </>}>
              <ListItemText primary={l.txnomelocalidade} secondary={`id: ${l.incdlocalidade}`} />
            </ListItem>
          ))}
        </List>
      )}
    </Box>
  )
}
