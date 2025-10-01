import React from 'react'
import {
  Card, CardContent, Typography, Box, List, ListItem, ListItemText,
  ListItemAvatar, Avatar, Chip, IconButton, Button, Grid
} from '@mui/material'
import {
  Person, Add, Edit, Delete
} from '@mui/icons-material'
import { useContactsClean, useCreateContactClean, useUpdateContactClean, useDeleteContactClean } from '../presentation/hooks/useContactUseCasesClean'

interface ContactsOverviewProps {
  limit?: number
}

export default function ContactsOverview({ limit = 5 }: ContactsOverviewProps) {
  const contactsQuery = useContactsClean({ limit })
  const create = useCreateContactClean()
  const [newContactName, setNewContactName] = React.useState('')

  const handleCreateContact = () => {
    if (newContactName.trim()) {
      create.mutate({ 
        full_name: newContactName,
        contact_type: 'PERSON',
        status: 'ACTIVE',
        document: null,
        unit_id: null,
        department_id: null,
        emails: [],
        phones: []
      }, {
        onSuccess: () => {
          setNewContactName('')
          contactsQuery.refetch()
        }
      })
    }
  }

  return (
    <Card sx={{ height: '100%' }}>
      <CardContent>
        <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
          <Typography variant="h6" sx={{ fontWeight: 'bold' }}>
            Recent Contacts
          </Typography>
          <Chip 
            label={`${contactsQuery.data?.items?.length || 0} contacts`} 
            size="small" 
            color="primary"
            variant="outlined"
          />
        </Box>

        {/* Quick Add Contact */}
        <Box sx={{ mb: 3, p: 2, backgroundColor: '#f8fafc', borderRadius: 2, border: '1px solid #e2e8f0' }}>
          <Typography variant="body2" sx={{ fontWeight: 600, mb: 2, color: '#64748b' }}>
            Quick Add Contact
          </Typography>
          <Box sx={{ display: 'flex', gap: 1 }}>
            <input
              type="text"
              placeholder="Enter contact name..."
              value={newContactName}
              onChange={(e) => setNewContactName(e.target.value)}
              onKeyPress={(e) => e.key === 'Enter' && handleCreateContact()}
              style={{
                flex: 1,
                padding: '8px 12px',
                border: '1px solid #e2e8f0',
                borderRadius: '8px',
                fontSize: '14px',
                outline: 'none',
                backgroundColor: 'white'
              }}
            />
            <Button
              variant="contained"
              size="small"
              onClick={handleCreateContact}
              disabled={!newContactName.trim() || create.isPending}
              sx={{
                background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
                '&:hover': {
                  background: 'linear-gradient(135deg, #5a6fd8 0%, #6a4190 100%)',
                },
                minWidth: 'auto',
                px: 2
              }}
            >
              <Add />
            </Button>
          </Box>
        </Box>

        {/* Contacts List */}
        {contactsQuery.isLoading ? (
          <Box sx={{ display: 'flex', justifyContent: 'center', py: 4 }}>
            <Typography variant="body2" color="text.secondary">
              Loading contacts...
            </Typography>
          </Box>
        ) : (
          <List>
            {(contactsQuery.data?.items || []).map((contact: any, index: number) => (
              <ContactItem key={contact.id} contact={contact} isLast={index === (contactsQuery.data?.items?.length || 0) - 1} />
            ))}
            {(!contactsQuery.data?.items || contactsQuery.data.items.length === 0) && (
              <Box sx={{ textAlign: 'center', py: 4 }}>
                <Person sx={{ fontSize: 48, color: '#e2e8f0', mb: 2 }} />
                <Typography variant="body2" color="text.secondary">
                  No contacts found. Create your first contact above!
                </Typography>
              </Box>
            )}
          </List>
        )}
      </CardContent>
    </Card>
  )
}

function ContactItem({ contact, isLast }: { contact: any, isLast: boolean }) {
  const [editing, setEditing] = React.useState(false)
  const [fullName, setFullName] = React.useState(contact.fullName)
  const update = useUpdateContactClean()
  const remove = useDeleteContactClean()

  const handleSave = () => {
    update.mutate({ 
      id: contact.id,
      full_name: fullName,
      etag: contact.etag
    }, { 
      onSuccess: () => setEditing(false) 
    })
  }

  const handleCancel = () => {
    setFullName(contact.fullName)
    setEditing(false)
  }

  return (
    <ListItem 
      sx={{ 
        px: 0, 
        py: 2,
        borderBottom: isLast ? 'none' : '1px solid #f1f5f9'
      }}
    >
      <ListItemAvatar>
        <Avatar sx={{ 
          bgcolor: '#667eea',
          width: 40,
          height: 40,
          fontSize: '0.875rem'
        }}>
          {contact.fullName.charAt(0).toUpperCase()}
        </Avatar>
      </ListItemAvatar>
      
      <ListItemText
        primary={
          editing ? (
            <input
              type="text"
              value={fullName}
              onChange={(e) => setFullName(e.target.value)}
              onKeyPress={(e) => e.key === 'Enter' && handleSave()}
              style={{
                width: '100%',
                padding: '4px 8px',
                border: '1px solid #e2e8f0',
                borderRadius: '4px',
                fontSize: '14px',
                outline: 'none'
              }}
            />
          ) : (
            <Typography variant="body1" sx={{ fontWeight: 500 }}>
              {contact.fullName}
            </Typography>
          )
        }
        secondary={
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mt: 0.5 }}>
            <Chip 
              label={`ETag: ${contact.etag}`} 
              size="small" 
              variant="outlined"
              sx={{ fontSize: '0.75rem', height: 20 }}
            />
            {contact.status && (
              <Chip 
                label={contact.status} 
                size="small" 
                color={contact.status === 'ACTIVE' ? 'success' : 'default'}
                sx={{ fontSize: '0.75rem', height: 20 }}
              />
            )}
          </Box>
        }
      />

      <Box sx={{ display: 'flex', gap: 0.5 }}>
        {editing ? (
          <>
            <IconButton size="small" onClick={handleSave} disabled={update.isPending}>
              <Edit sx={{ fontSize: 16 }} />
            </IconButton>
            <IconButton size="small" onClick={handleCancel}>
              <Delete sx={{ fontSize: 16 }} />
            </IconButton>
          </>
        ) : (
          <>
            <IconButton size="small" onClick={() => setEditing(true)}>
              <Edit sx={{ fontSize: 16, color: '#64748b' }} />
            </IconButton>
            <IconButton 
              size="small" 
              onClick={() => remove.mutate({ id: contact.id })} 
              disabled={remove.isPending}
            >
              <Delete sx={{ fontSize: 16, color: '#ef4444' }} />
            </IconButton>
          </>
        )}
      </Box>

    </ListItem>
  )
}
