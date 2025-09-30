import React from 'react'
import { Box, List, ListItem, ListItemText } from '@mui/material'
import { useSites } from '../api/admin'

export default function SitesPage(){
  const q = useSites()
  return (
    <Box>
      <h3>Sites de Busca</h3>
      {q.isLoading ? <em>carregando...</em> : (
        <List>
          {(q.data || []).map((s: any)=>(<ListItem key={s.incdsite}><ListItemText primary={s.site} secondary={s.url} /></ListItem>))}
        </List>
      )}
    </Box>
  )
}
