import React from 'react'
import { Box, List, ListItem, ListItemText } from '@mui/material'
import { useResponsaveis } from '../api/admin'

export default function ResponsaveisPage(){
  const q = useResponsaveis()
  return (
    <Box>
      <h3>Responsáveis</h3>
      {q.isLoading ? <em>carregando...</em> : (
        <List>
          {(q.data || []).map((r: any)=>(<ListItem key={r.incdrespdepart}><ListItemText primary={r.nome || r.idp_user_id} secondary={`contato:${r.incdcontato} dept:${r.incddepartamento}`} /></ListItem>))}
        </List>
      )}
    </Box>
  )
}
