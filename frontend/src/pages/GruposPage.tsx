import React from 'react'
import { Box, List, ListItem, ListItemText } from '@mui/material'
import { useGrupos, useGrupoMembros } from '../api/admin'

export default function GruposPage(){
  const g = useGrupos()
  const gm = useGrupoMembros()
  return (
    <Box>
      <h3>Grupos</h3>
      {g.isLoading ? <em>carregando...</em> : (
        <List>
          {(g.data || []).map((x: any)=>(<ListItem key={x.incdgrupo}><ListItemText primary={x.grupo} /></ListItem>))}
        </List>
      )}
      <h4>Membros</h4>
      {gm.isLoading ? <em>carregando...</em> : (
        <List>
          {(gm.data || []).map((m: any)=>(<ListItem key={`${m.incdgrupo}-${m.idp_user_id}`}><ListItemText primary={`${m.nome_exibicao || m.idp_user_id}`} secondary={`${m.grupo || m.incdgrupo}`} /></ListItem>))}
        </List>
      )}
    </Box>
  )
}
