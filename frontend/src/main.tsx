import React from 'react'
import { createRoot } from 'react-dom/client'

import App from './App'
import { KeycloakProvider } from './auth/KeycloakProvider'
import { ThemeProvider, CssBaseline } from '@mui/material'
import theme from './theme'

createRoot(document.getElementById('root')!).render(
  <ThemeProvider theme={theme}>
    <CssBaseline />
    <KeycloakProvider>
      <App />
    </KeycloakProvider>
  </ThemeProvider>
)
