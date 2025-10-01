import React from 'react'
import {
  Drawer, List, ListItem, ListItemButton, ListItemIcon, ListItemText,
  Typography, Box, Avatar, Divider, Chip, Button
} from '@mui/material'
import {
  Dashboard, TableChart, Receipt, Language, Person, Login, PersonAdd,
  Help, Description, Star
} from '@mui/icons-material'

const drawerWidth = 280

interface SidebarProps {
  open: boolean
  onClose: () => void
  currentPage: string
  onPageChange: (page: string) => void
}

const menuItems = [
  { id: 'home', label: 'Dashboard', icon: <Dashboard /> },
  { id: 'contatos', label: 'Contatos', icon: <Person /> },
  { id: 'contatos-clean', label: 'Contatos (Clean)', icon: <Person /> },
  { id: 'localidades', label: 'Localidades', icon: <TableChart /> },
  { id: 'departamentos', label: 'Departamentos', icon: <TableChart /> },
  { id: 'lexico', label: 'Léxico', icon: <TableChart /> },
  { id: 'grupos', label: 'Grupos', icon: <TableChart /> },
  { id: 'responsaveis', label: 'Responsáveis', icon: <TableChart /> },
  { id: 'sites', label: 'Sites', icon: <TableChart /> },
]

const accountPages = [
  { id: 'profile', label: 'Profile', icon: <Person /> },
  { id: 'signin', label: 'Sign In', icon: <Login /> },
  { id: 'signup', label: 'Sign Up', icon: <PersonAdd /> },
]

export default function Sidebar({ open, onClose, currentPage, onPageChange }: SidebarProps) {
  return (
    <Drawer
      variant="persistent"
      anchor="left"
      open={open}
      sx={{
        width: drawerWidth,
        flexShrink: 0,
        '& .MuiDrawer-paper': {
          width: drawerWidth,
          boxSizing: 'border-box',
          backgroundColor: '#f8fafc',
          borderRight: '1px solid #e2e8f0',
        },
      }}
    >
      {/* Logo Section */}
      <Box sx={{ 
        p: 3, 
        background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
        color: 'white',
        textAlign: 'center'
      }}>
        <Avatar sx={{ 
          width: 48, 
          height: 48, 
          mx: 'auto', 
          mb: 2,
          bgcolor: 'rgba(255,255,255,0.2)',
          backdropFilter: 'blur(10px)'
        }}>
          <Dashboard />
        </Avatar>
        <Typography variant="h6" sx={{ fontWeight: 'bold' }}>
          argon | chakra
        </Typography>
      </Box>

      {/* Main Navigation */}
      <Box sx={{ flexGrow: 1, py: 2 }}>
        <List>
          {menuItems.map((item) => (
            <ListItem key={item.id} disablePadding sx={{ px: 2, mb: 0.5 }}>
              <ListItemButton
                selected={currentPage === item.id}
                onClick={() => onPageChange(item.id)}
                sx={{
                  borderRadius: 2,
                  '&.Mui-selected': {
                    backgroundColor: '#667eea',
                    color: 'white',
                    '&:hover': {
                      backgroundColor: '#5a6fd8',
                    },
                    '& .MuiListItemIcon-root': {
                      color: 'white',
                    },
                  },
                  '&:hover': {
                    backgroundColor: '#f1f5f9',
                  },
                }}
              >
                <ListItemIcon sx={{ minWidth: 40 }}>
                  {item.icon}
                </ListItemIcon>
                <ListItemText 
                  primary={item.label}
                  primaryTypographyProps={{
                    fontSize: '0.875rem',
                    fontWeight: currentPage === item.id ? 600 : 400,
                  }}
                />
              </ListItemButton>
            </ListItem>
          ))}
        </List>

        <Divider sx={{ mx: 2, my: 3 }} />

        {/* Account Pages */}
        <Box sx={{ px: 2, mb: 2 }}>
          <Typography 
            variant="caption" 
            sx={{ 
              color: '#64748b', 
              fontWeight: 600, 
              textTransform: 'uppercase',
              letterSpacing: '0.05em',
              fontSize: '0.75rem'
            }}
          >
            Account Pages
          </Typography>
        </Box>
        
        <List>
          {accountPages.map((item) => (
            <ListItem key={item.id} disablePadding sx={{ px: 2, mb: 0.5 }}>
              <ListItemButton
                sx={{
                  borderRadius: 2,
                  '&:hover': {
                    backgroundColor: '#f1f5f9',
                  },
                }}
              >
                <ListItemIcon sx={{ minWidth: 40, color: '#64748b' }}>
                  {item.icon}
                </ListItemIcon>
                <ListItemText 
                  primary={item.label}
                  primaryTypographyProps={{
                    fontSize: '0.875rem',
                    color: '#64748b',
                  }}
                />
              </ListItemButton>
            </ListItem>
          ))}
        </List>
      </Box>

      {/* Help Section */}
      <Box sx={{ p: 3, borderTop: '1px solid #e2e8f0' }}>
        <Box sx={{ 
          p: 2, 
          backgroundColor: 'white', 
          borderRadius: 2, 
          border: '1px solid #e2e8f0',
          textAlign: 'center'
        }}>
          <Avatar sx={{ 
            width: 40, 
            height: 40, 
            mx: 'auto', 
            mb: 2,
            bgcolor: '#f1f5f9',
            color: '#64748b'
          }}>
            <Description />
          </Avatar>
          <Typography variant="body2" sx={{ fontWeight: 600, mb: 1 }}>
            Need help?
          </Typography>
          <Typography variant="caption" color="text.secondary" sx={{ mb: 2, display: 'block' }}>
            Please check our docs.
          </Typography>
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
            <Button 
              size="small" 
              variant="outlined" 
              sx={{ 
                fontSize: '0.75rem',
                textTransform: 'none',
                borderColor: '#e2e8f0',
                color: '#64748b',
                '&:hover': {
                  borderColor: '#cbd5e1',
                  backgroundColor: '#f8fafc',
                }
              }}
            >
              Documentation
            </Button>
            <Button 
              size="small" 
              variant="contained" 
              sx={{ 
                fontSize: '0.75rem',
                textTransform: 'none',
                background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
                '&:hover': {
                  background: 'linear-gradient(135deg, #5a6fd8 0%, #6a4190 100%)',
                }
              }}
            >
              Upgrade to Pro
            </Button>
          </Box>
        </Box>
      </Box>
    </Drawer>
  )
}
