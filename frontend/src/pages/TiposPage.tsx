import React from 'react'
import { Box, Typography, List, ListItem, ListItemText } from '@mui/material'
import { useTiposContato } from '../api/admin'

export default function TiposPage(){
  const q = useTiposContato()
  return (
    <Box>
      {q.isLoading ? <em>carregando...</em> : (
        <List>
          {(q.data || []).map((t: any)=>(<ListItem key={t.incdtipocontato}><ListItemText primary={t.tipo} secondary={t.descricao} /></ListItem>))}
        </List>
      )}
    </Box>
  )
}
