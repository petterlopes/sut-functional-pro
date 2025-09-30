import React from 'react'
import { Box, List, ListItem, ListItemText } from '@mui/material'
import { useRefOrigemContato } from '../api/admin'

export default function RefOrigemPage(){
  const q = useRefOrigemContato()
  return (
    <Box>
      {q.isLoading ? <em>carregando...</em> : (
        <List>
          {(q.data || []).map((r: any)=>(<ListItem key={r.incdreforigcont}><ListItemText primary={r.descricao} secondary={r.formato} /></ListItem>))}
        </List>
      )}
    </Box>
  )
}
