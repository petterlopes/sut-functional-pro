import React from 'react'
import {
  Box, Card, CardContent, Typography, Grid, Paper, List, ListItem, ListItemText,
  LinearProgress, Chip, IconButton, Avatar, Divider
} from '@mui/material'
import {
  TrendingUp, TrendingDown, People, AttachMoney, ShoppingCart, Public,
  Facebook, Google, Twitter, LinkedIn
} from '@mui/icons-material'
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, BarChart, Bar } from 'recharts'
import ContactsOverview from './ContactsOverview'

// Mock data for charts
const salesData = [
  { month: 'Jan', mobile: 200, websites: 180 },
  { month: 'Feb', mobile: 220, websites: 200 },
  { month: 'Mar', mobile: 180, websites: 220 },
  { month: 'Apr', mobile: 250, websites: 240 },
  { month: 'May', mobile: 280, websites: 260 },
  { month: 'Jun', mobile: 250, websites: 290 },
  { month: 'Jul', mobile: 300, websites: 320 },
  { month: 'Aug', mobile: 320, websites: 300 },
  { month: 'Sep', mobile: 350, websites: 340 }
]

const performanceData = [
  { month: 'Jul', orders: 12 },
  { month: 'Aug', orders: 19 },
  { month: 'Sep', orders: 15 },
  { month: 'Oct', orders: 25 },
  { month: 'Nov', orders: 22 },
  { month: 'Dec', orders: 28 }
]

const pageVisitsData = [
  { page: '/argon/', visitors: 4569, uniqueUsers: 340, bounceRate: 46.53 },
  { page: '/argon/index.html', visitors: 3985, uniqueUsers: 315, bounceRate: 46.53 },
  { page: '/argon/charts.html', visitors: 3513, uniqueUsers: 294, bounceRate: 36.49 }
]

const socialTrafficData = [
  { platform: 'Facebook', visitors: 1480, percentage: 60 },
  { platform: 'Google', visitors: 4807, percentage: 80 },
  { platform: 'Twitter', visitors: 3200, percentage: 70 },
  { platform: 'LinkedIn', visitors: 2100, percentage: 50 }
]

interface MetricCardProps {
  title: string
  value: string
  change: number
  changeLabel: string
  icon: React.ReactNode
  color: string
}

function MetricCard({ title, value, change, changeLabel, icon, color }: MetricCardProps) {
  const isPositive = change >= 0
  const ChangeIcon = isPositive ? TrendingUp : TrendingDown
  
  return (
    <Card sx={{ 
      height: '100%', 
      background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
      color: 'white',
      position: 'relative',
      overflow: 'hidden'
    }}>
      <CardContent>
        <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start' }}>
          <Box>
            <Typography variant="body2" sx={{ opacity: 0.8, mb: 1 }}>
              {title}
            </Typography>
            <Typography variant="h4" sx={{ fontWeight: 'bold', mb: 1 }}>
              {value}
            </Typography>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.5 }}>
              <ChangeIcon sx={{ fontSize: 16, color: isPositive ? '#4caf50' : '#f44336' }} />
              <Typography 
                variant="body2" 
                sx={{ 
                  color: isPositive ? '#4caf50' : '#f44336',
                  fontWeight: 'medium'
                }}
              >
                {change > 0 ? '+' : ''}{change}% {changeLabel}
              </Typography>
            </Box>
          </Box>
          <Avatar sx={{ 
            bgcolor: 'rgba(255,255,255,0.2)', 
            width: 56, 
            height: 56,
            backdropFilter: 'blur(10px)'
          }}>
            {icon}
          </Avatar>
        </Box>
      </CardContent>
    </Card>
  )
}

function SalesOverview() {
  return (
    <Card sx={{ height: '100%' }}>
      <CardContent>
        <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
          <Box>
            <Typography variant="h6" sx={{ fontWeight: 'bold' }}>
              Sales Overview
            </Typography>
            <Typography variant="body2" color="text.secondary">
              (+5) more in 2022
            </Typography>
          </Box>
        </Box>
        <Box sx={{ height: 300 }}>
          <ResponsiveContainer width="100%" height="100%">
            <LineChart data={salesData}>
              <CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
              <XAxis dataKey="month" stroke="#666" />
              <YAxis stroke="#666" />
              <Tooltip 
                contentStyle={{ 
                  backgroundColor: 'white', 
                  border: '1px solid #e0e0e0',
                  borderRadius: '8px',
                  boxShadow: '0 4px 12px rgba(0,0,0,0.1)'
                }} 
              />
              <Line 
                type="monotone" 
                dataKey="mobile" 
                stroke="#9c27b0" 
                strokeWidth={3}
                dot={{ fill: '#9c27b0', strokeWidth: 2, r: 4 }}
                name="Mobile apps"
              />
              <Line 
                type="monotone" 
                dataKey="websites" 
                stroke="#2196f3" 
                strokeWidth={3}
                dot={{ fill: '#2196f3', strokeWidth: 2, r: 4 }}
                name="Websites"
              />
            </LineChart>
          </ResponsiveContainer>
        </Box>
      </CardContent>
    </Card>
  )
}

function PerformanceChart() {
  return (
    <Card sx={{ height: '100%' }}>
      <CardContent>
        <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
          <Box>
            <Typography variant="h6" sx={{ fontWeight: 'bold' }}>
              PERFORMANCE
            </Typography>
            <Typography variant="body2" color="text.secondary">
              Total orders
            </Typography>
          </Box>
        </Box>
        <Box sx={{ height: 300 }}>
          <ResponsiveContainer width="100%" height="100%">
            <BarChart data={performanceData}>
              <CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
              <XAxis dataKey="month" stroke="#666" />
              <YAxis stroke="#666" />
              <Tooltip 
                contentStyle={{ 
                  backgroundColor: 'white', 
                  border: '1px solid #e0e0e0',
                  borderRadius: '8px',
                  boxShadow: '0 4px 12px rgba(0,0,0,0.1)'
                }} 
              />
              <Bar dataKey="orders" fill="#ff9800" radius={[4, 4, 0, 0]} />
            </BarChart>
          </ResponsiveContainer>
        </Box>
      </CardContent>
    </Card>
  )
}

function PageVisits() {
  return (
    <Card sx={{ height: '100%' }}>
      <CardContent>
        <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
          <Typography variant="h6" sx={{ fontWeight: 'bold' }}>
            Page visits
          </Typography>
          <Chip label="SEE ALL" size="small" clickable />
        </Box>
        <List>
          {pageVisitsData.map((item, index) => (
            <React.Fragment key={index}>
              <ListItem sx={{ px: 0 }}>
                <ListItemText
                  primary={
                    <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                      <Typography variant="body1" component="span" sx={{ fontWeight: 'medium' }}>
                        {item.page}
                      </Typography>
                      <Typography variant="body2" component="span" color="text.secondary">
                        {item.bounceRate}%
                      </Typography>
                    </Box>
                  }
                  secondary={
                    <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mt: 1 }}>
                      <Typography variant="body2" component="span" color="text.secondary">
                        {item.visitors.toLocaleString()} visitors
                      </Typography>
                      <Typography variant="body2" component="span" color="text.secondary">
                        {item.uniqueUsers} unique users
                      </Typography>
                    </Box>
                  }
                />
              </ListItem>
              {index < pageVisitsData.length - 1 && <Divider />}
            </React.Fragment>
          ))}
        </List>
      </CardContent>
    </Card>
  )
}

function SocialTraffic() {
  const getPlatformIcon = (platform: string) => {
    switch (platform) {
      case 'Facebook': return <Facebook sx={{ color: '#1877f2' }} />
      case 'Google': return <Google sx={{ color: '#ea4335' }} />
      case 'Twitter': return <Twitter sx={{ color: '#1da1f2' }} />
      case 'LinkedIn': return <LinkedIn sx={{ color: '#0077b5' }} />
      default: return <Public />
    }
  }

  const getProgressColor = (percentage: number) => {
    if (percentage >= 80) return '#4caf50'
    if (percentage >= 60) return '#ff9800'
    return '#f44336'
  }

  return (
    <Card sx={{ height: '100%' }}>
      <CardContent>
        <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
          <Typography variant="h6" sx={{ fontWeight: 'bold' }}>
            Social traffic
          </Typography>
          <Chip label="SEE ALL" size="small" clickable />
        </Box>
        <List>
          {socialTrafficData.map((item, index) => (
            <React.Fragment key={index}>
              <ListItem sx={{ px: 0 }}>
                <ListItemText
                  primary={
                    <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                      <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                        {getPlatformIcon(item.platform)}
                        <Typography variant="body1" component="span" sx={{ fontWeight: 'medium' }}>
                          {item.platform}
                        </Typography>
                      </Box>
                      <Typography variant="body2" component="span" color="text.secondary">
                        {item.visitors.toLocaleString()}
                      </Typography>
                    </Box>
                  }
                  secondary={
                    <Box sx={{ mt: 1 }}>
                      <LinearProgress 
                        variant="determinate" 
                        value={item.percentage} 
                        sx={{ 
                          height: 6, 
                          borderRadius: 3,
                          backgroundColor: '#e0e0e0',
                          '& .MuiLinearProgress-bar': {
                            backgroundColor: getProgressColor(item.percentage),
                            borderRadius: 3
                          }
                        }} 
                      />
                    </Box>
                  }
                />
              </ListItem>
              {index < socialTrafficData.length - 1 && <Divider />}
            </React.Fragment>
          ))}
        </List>
      </CardContent>
    </Card>
  )
}

export default function Dashboard() {
  return (
    <Box sx={{ p: 3 }}>
      {/* Header */}
      <Box sx={{ mb: 4 }}>
        <Typography variant="h4" sx={{ fontWeight: 'bold', mb: 1 }}>
          Dashboard
        </Typography>
        <Typography variant="body1" color="text.secondary">
          Welcome to your modern dashboard
        </Typography>
      </Box>

      {/* Metrics Cards */}
      <Grid container spacing={3} sx={{ mb: 4 }}>
        <Grid size={{ xs: 12, sm: 6, md: 3 }}>
          <MetricCard
            title="TODAY'S MONEY"
            value="$53,897"
            change={3.48}
            changeLabel="Since last month"
            icon={<AttachMoney />}
            color="#667eea"
          />
        </Grid>
        <Grid size={{ xs: 12, sm: 6, md: 3 }}>
          <MetricCard
            title="TODAY'S USERS"
            value="$3,200"
            change={5.2}
            changeLabel="Since last month"
            icon={<People />}
            color="#764ba2"
          />
        </Grid>
        <Grid size={{ xs: 12, sm: 6, md: 3 }}>
          <MetricCard
            title="NEW CLIENTS"
            value="+2,503"
            change={-2.82}
            changeLabel="Since last month"
            icon={<Public />}
            color="#f093fb"
          />
        </Grid>
        <Grid size={{ xs: 12, sm: 6, md: 3 }}>
          <MetricCard
            title="TOTAL SALES"
            value="$173,000"
            change={0.12}
            changeLabel="Since last month"
            icon={<ShoppingCart />}
            color="#4facfe"
          />
        </Grid>
      </Grid>

      {/* Charts Row */}
      <Grid container spacing={3} sx={{ mb: 4 }}>
        <Grid size={{ xs: 12, lg: 8 }}>
          <SalesOverview />
        </Grid>
        <Grid size={{ xs: 12, lg: 4 }}>
          <PerformanceChart />
        </Grid>
      </Grid>

      {/* Tables Row */}
      <Grid container spacing={3}>
        <Grid size={{ xs: 12, lg: 4 }}>
          <ContactsOverview limit={5} />
        </Grid>
        <Grid size={{ xs: 12, lg: 4 }}>
          <PageVisits />
        </Grid>
        <Grid size={{ xs: 12, lg: 4 }}>
          <SocialTraffic />
        </Grid>
      </Grid>
    </Box>
  )
}
