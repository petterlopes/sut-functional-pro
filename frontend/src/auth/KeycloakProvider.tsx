import React from 'react'
import Keycloak from 'keycloak-js'

// Tipos de contexto de autenticação
type AuthContext = { 
  token: string | null
  login: () => void
  logout: () => void
  ready: boolean
  authenticated: boolean
  user: KeycloakUser | null
  roles: string[]
  hasRole: (role: string) => boolean
  hasAnyRole: (roles: string[]) => boolean
  isAdmin: () => boolean
}

type KeycloakUser = {
  id: string
  username: string
  email?: string
  firstName?: string
  lastName?: string
  roles: string[]
}

const AuthContext = React.createContext<AuthContext>({ 
  token: null, 
  login: () => {}, 
  logout: () => {}, 
  ready: false,
  authenticated: false,
  user: null,
  roles: [],
  hasRole: () => false,
  hasAnyRole: () => false,
  isAdmin: () => false
})

export const useAuth = () => React.useContext(AuthContext)
export function KeycloakProvider({ children }: { children: React.ReactNode }){
  const kc = React.useMemo(() => {
    // Configuração de segurança do Keycloak
    const envUrl = (import.meta as any).env?.VITE_KC_URL
    let runtimeOverride: string | null = null
    try { 
      runtimeOverride = localStorage.getItem('sut_kc_base') 
    } catch(e) {
      console.warn('Failed to read runtime override from localStorage:', e)
    }
    
    const defaultHost = 'http://localhost:8081'
    let inferred = runtimeOverride || envUrl || defaultHost
    
    try {
      // Rewrite Docker service names to localhost for host browser access
    if (inferred.includes('keycloak') && typeof window !== 'undefined' && window.location && window.location.hostname !== 'keycloak'){
      const urlObj = new URL(inferred)
      urlObj.hostname = 'localhost'
      // map container port 8080 to host 8081 (dev compose maps 8081:8080)
      urlObj.port = '8081'
      inferred = urlObj.toString().replace(/\/$/, '')
    }
  }catch(e){
    // if URL parsing fails, fall back to the env/default inferred value
  }
  const env: any = (import.meta as any).env || {}
  console.debug('[KeycloakProvider] kc original env url=', envUrl, ' -> using=', inferred)
  return new Keycloak({
    url: inferred,
    realm: env.VITE_KC_REALM || 'sut',
    clientId: env.VITE_KC_CLIENT || 'sut-frontend',
    // Configurações de segurança
    enableLogging: process.env.NODE_ENV === 'development',
    checkLoginIframe: false, // Desabilitar para melhor performance
    checkLoginIframeInterval: 5, // Verificar a cada 5 segundos se necessário
    onLoad: 'check-sso', // Verificar SSO automaticamente
    silentCheckSsoRedirectUri: window.location.origin + '/silent-check-sso.html',
    pkceMethod: 'S256', // Usar PKCE com SHA256
    flow: 'standard', // Usar Authorization Code Flow
    responseMode: 'fragment', // Usar fragment mode para segurança
    scope: 'openid profile email', // Escopo mínimo necessário
  })
  }, [])
  const [ready, setReady] = React.useState(false)
  const [token, setToken] = React.useState<string|null>(null)
  const [authenticated, setAuthenticated] = React.useState(false)
  const [user, setUser] = React.useState<KeycloakUser | null>(null)
  const [roles, setRoles] = React.useState<string[]>([])
  React.useEffect(()=>{
    console.debug('[KeycloakProvider] initializing Keycloak with url=', kc.authServerUrl)
    let mounted = true
    ;(async ()=>{
      const maxAttempts = 4
      let attempt = 0
      let delay = 500
      while(mounted && attempt < maxAttempts){
        attempt++
        try{
          const auth = await kc.init({ 
            onLoad: 'check-sso', 
            pkceMethod: 'S256', 
            silentCheckSsoRedirectUri: window.location.origin + '/silent-check-sso.html',
            checkLoginIframe: false,
            enableLogging: process.env.NODE_ENV === 'development'
          })
          console.debug('[KeycloakProvider] kc.init resolved, authenticated=', auth)
          
          if (auth && kc.authenticated) {
            setToken(kc.token!)
            setAuthenticated(true)
            
            // Extrair informações do usuário
            if (kc.tokenParsed) {
              const userRoles = kc.tokenParsed.realm_access?.roles || []
              const userInfo: KeycloakUser = {
                id: kc.tokenParsed.sub || '',
                username: kc.tokenParsed.preferred_username || kc.tokenParsed.name || '',
                email: kc.tokenParsed.email,
                firstName: kc.tokenParsed.given_name,
                lastName: kc.tokenParsed.family_name,
                roles: userRoles
              }
              setUser(userInfo)
              setRoles(userRoles)
            }
          } else {
            setAuthenticated(false)
            setUser(null)
            setRoles([])
          }
          
          // DEV helper: expose current token to window for quick debugging from the browser console
          setReady(true)
          const id = setInterval(async ()=>{
            try{
              const refreshed = await kc.updateToken(60)
              if (refreshed && kc.token) {
                setToken(kc.token)
                setAuthenticated(true)
                
                // Atualizar informações do usuário se necessário
                if (kc.tokenParsed) {
                  const userRoles = kc.tokenParsed.realm_access?.roles || []
                  setRoles(userRoles)
                  if (user) {
                    setUser({
                      ...user,
                      roles: userRoles
                    })
                  }
                }
              }
                }catch(e){
              // ignore refresh errors
            }
          }, 30000)
          return ()=> clearInterval(id)
        }catch(err){
          console.warn(`[KeycloakProvider] kc.init attempt ${attempt} failed`, err)
          if (attempt >= maxAttempts){
            console.error('[KeycloakProvider] kc.init failed after retries', err)
            // expose login button so user can attempt full login
            setReady(true)
            return
          }
          await new Promise(r=>setTimeout(r, delay))
          delay = delay * 2
          continue
        }
      }
    })()
    return ()=>{ mounted = false }
  }, [kc])
  const login = ()=>{
    try{
      const loginUrl = (kc as any).createLoginUrl ? (kc as any).createLoginUrl({ redirectUri: window.location.href }) : undefined
      console.debug('[KeycloakProvider] initiating login, loginUrl=', loginUrl, ' authServerUrl=', kc.authServerUrl)
    }catch(e){
      console.debug('[KeycloakProvider] error building loginUrl', e)
    }
    return kc.login({ redirectUri: window.location.href })
  }
  const logout = () => {
    setToken(null)
    setAuthenticated(false)
    setUser(null)
    setRoles([])
    return kc.logout({ redirectUri: window.location.origin })
  }

  // Funções auxiliares para verificação de roles
  const hasRole = React.useCallback((role: string) => {
    return roles.includes(role)
  }, [roles])

  const hasAnyRole = React.useCallback((requiredRoles: string[]) => {
    return requiredRoles.some(role => roles.includes(role))
  }, [roles])

  const isAdmin = React.useCallback(() => {
    return hasRole('admin')
  }, [hasRole])

  return (
    <AuthContext.Provider value={{ 
      token, 
      login, 
      logout, 
      ready, 
      authenticated,
      user,
      roles,
      hasRole,
      hasAnyRole,
      isAdmin
    }}>
      {children}
    </AuthContext.Provider>
  )
}
