import React from 'react'
import { Box, Typography, Tabs, Tab } from '@mui/material'
import TiposPage from './TiposPage'
import OrigensPage from './OrigensPage'
import RefOrigemPage from './RefOrigemPage'

export default function LexicoPage(){
  const [tab, setTab] = React.useState(0)
  return (
    <Box>
      <Typography variant="h5" gutterBottom>Léxico de Contatos</Typography>
      <Tabs value={tab} onChange={(_,v)=>setTab(v)}>
        <Tab label="Tipos" />
        <Tab label="Origens" />
        <Tab label="Referências" />
      </Tabs>
      <Box sx={{ mt: 2 }}>
        {tab===0 && <TiposPage />}
        {tab===1 && <OrigensPage />}
        {tab===2 && <RefOrigemPage />}
      </Box>
    </Box>
  )
}
