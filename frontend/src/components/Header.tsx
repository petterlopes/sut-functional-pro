import React from 'react'
import {
  AppBar, Toolbar, Typography, IconButton, Box, Avatar, Badge,
  TextField, InputAdornment, Button, Menu, MenuItem, Divider
} from '@mui/material'
import {
  Menu as MenuIcon, Search, Notifications, Settings, AccountCircle,
  Logout, Login, Person, Help
} from '@mui/icons-material'

interface HeaderProps {
  onMenuClick: () => void
  onLogin: () => void
  onLogout: () => void
  isAuthenticated: boolean
  userName?: string
}

export default function Header({ onMenuClick, onLogin, onLogout, isAuthenticated, userName }: HeaderProps) {
  const [anchorEl, setAnchorEl] = React.useState<null | HTMLElement>(null)
  const [searchValue, setSearchValue] = React.useState('')

  const handleProfileMenuOpen = (event: React.MouseEvent<HTMLElement>) => {
    setAnchorEl(event.currentTarget)
  }

  const handleMenuClose = () => {
    setAnchorEl(null)
  }

  const handleLogout = () => {
    handleMenuClose()
    onLogout()
  }

  return (
    <AppBar 
      position="fixed" 
      sx={{ 
        backgroundColor: 'white',
        color: '#1a202c',
        boxShadow: '0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06)',
        borderBottom: '1px solid #e2e8f0',
        zIndex: (theme) => theme.zIndex.drawer + 1
      }}
    >
      <Toolbar sx={{ px: 3 }}>
        {/* Menu Button */}
        <IconButton
          edge="start"
          color="inherit"
          aria-label="menu"
          onClick={onMenuClick}
          sx={{ mr: 2 }}
        >
          <MenuIcon />
        </IconButton>

        {/* Breadcrumb */}
        <Box sx={{ flexGrow: 1 }}>
          <Typography variant="body2" color="text.secondary" sx={{ fontSize: '0.875rem' }}>
            Pages / Dashboard
          </Typography>
          <Typography variant="h5" sx={{ fontWeight: 'bold', color: '#1a202c' }}>
            Dashboard
          </Typography>
        </Box>

        {/* Search Bar */}
        <Box sx={{ flexGrow: 1, maxWidth: 400, mx: 3 }}>
          <TextField
            fullWidth
            size="small"
            placeholder="Type here..."
            value={searchValue}
            onChange={(e) => setSearchValue(e.target.value)}
            InputProps={{
              startAdornment: (
                <InputAdornment position="start">
                  <Search sx={{ color: '#64748b' }} />
                </InputAdornment>
              ),
            }}
            sx={{
              '& .MuiOutlinedInput-root': {
                backgroundColor: '#f8fafc',
                borderRadius: 2,
                '& fieldset': {
                  borderColor: '#e2e8f0',
                },
                '&:hover fieldset': {
                  borderColor: '#cbd5e1',
                },
                '&.Mui-focused fieldset': {
                  borderColor: '#667eea',
                },
              },
            }}
          />
        </Box>

        {/* Right Side Actions */}
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          {/* Notifications */}
          <IconButton color="inherit">
            <Badge badgeContent={3} color="error">
              <Notifications />
            </Badge>
          </IconButton>

          {/* Settings */}
          <IconButton color="inherit">
            <Settings />
          </IconButton>

          {/* User Menu */}
          {isAuthenticated ? (
            <>
              <IconButton
                onClick={handleProfileMenuOpen}
                color="inherit"
                sx={{ ml: 1 }}
              >
                <Avatar sx={{ width: 32, height: 32, bgcolor: '#667eea' }}>
                  {userName ? userName.charAt(0).toUpperCase() : <Person />}
                </Avatar>
              </IconButton>
              <Menu
                anchorEl={anchorEl}
                open={Boolean(anchorEl)}
                onClose={handleMenuClose}
                PaperProps={{
                  sx: {
                    mt: 1,
                    minWidth: 200,
                    boxShadow: '0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)',
                    border: '1px solid #e2e8f0',
                  }
                }}
              >
                <MenuItem onClick={handleMenuClose}>
                  <Person sx={{ mr: 2 }} />
                  Profile
                </MenuItem>
                <MenuItem onClick={handleMenuClose}>
                  <Settings sx={{ mr: 2 }} />
                  Settings
                </MenuItem>
                <MenuItem onClick={handleMenuClose}>
                  <Help sx={{ mr: 2 }} />
                  Help
                </MenuItem>
                <Divider />
                <MenuItem onClick={handleLogout}>
                  <Logout sx={{ mr: 2 }} />
                  Logout
                </MenuItem>
              </Menu>
            </>
          ) : (
            <Button
              variant="contained"
              startIcon={<Login />}
              onClick={onLogin}
              sx={{
                background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
                '&:hover': {
                  background: 'linear-gradient(135deg, #5a6fd8 0%, #6a4190 100%)',
                },
                textTransform: 'none',
                borderRadius: 2,
                px: 3,
              }}
            >
              Sign In
            </Button>
          )}
        </Box>
      </Toolbar>
    </AppBar>
  )
}
