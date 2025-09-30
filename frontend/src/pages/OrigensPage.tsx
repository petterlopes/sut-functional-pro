import React from 'react'
import { Box, Typography, List, ListItem, ListItemText } from '@mui/material'
import { useOrigensContato } from '../api/admin'

export default function OrigensPage(){
  const q = useOrigensContato()
  return (
    <Box>
      {q.isLoading ? <em>carregando...</em> : (
        <List>
          {(q.data || []).map((o: any)=>(<ListItem key={o.incdorigemcontato}><ListItemText primary={o.tipo} secondary={o.origem} /></ListItem>))}
        </List>
      )}
    </Box>
  )
}
