import React from 'react'
import Keycloak from 'keycloak-js'
type Ctx = { token: string | null, login: ()=>void, logout: ()=>void, ready: boolean }
const Ctx = React.createContext<Ctx>({ token: null, login: ()=>{}, logout: ()=>{}, ready: false })
export const useAuth = ()=> React.useContext(Ctx)
export function KeycloakProvider({ children }: { children: React.ReactNode }){
  const kc = React.useMemo(()=> {
    // Prefer explicit env (Vite injects VITE_* at build/dev time). When not set,
    // choose a host-friendly default for local browser development (localhost:8081).
    // Inside docker-compose the frontend will have VITE_KC_URL set to the service
    // name (http://keycloak:8080) so that the dev server in the container can reach it.
  const envUrl = (import.meta as any).env?.VITE_KC_URL
  // Allow a runtime override from localStorage (set by the RuntimeConfig UI)
  let runtimeOverride: string | null = null
  try{ runtimeOverride = localStorage.getItem('sut_kc_base') }catch(e){}
  // If VITE_KC_URL is provided (containerized runs) use it. Otherwise prefer
  // a host-friendly default (localhost:8081) so the browser doesn't attempt
  // to resolve the Docker service name `keycloak` which causes DNS errors
  // like DNS_PROBE_POSSIBLE when the frontend is opened from the host.
  // When a developer sets VITE_KC_URL to a Docker service hostname (e.g.
  // http://keycloak:8080) but the app is opened in a host browser, rewrite
  // that hostname to localhost:8081 so the browser can reach Keycloak on the
  // mapped host port. If you run entirely inside a container network where
  // service names resolve, set VITE_KC_URL accordingly in the compose env.
  const defaultHost = 'http://localhost:8081'
  let inferred = runtimeOverride || envUrl || defaultHost
  try{
    // If the inferred URL points at a Docker service name like `keycloak`
    // and we're running inside a host browser (window.location.hostname != 'keycloak'),
    // rewrite the host to localhost:8081 so the browser can reach Keycloak.
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
    clientId: env.VITE_KC_CLIENT || 'sut-frontend'
  })
  }, [])
  const [ready, setReady] = React.useState(false)
  const [token, setToken] = React.useState<string|null>(null)
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
          const auth = await kc.init({ onLoad: 'check-sso', pkceMethod: 'S256', silentCheckSsoRedirectUri: window.location.origin + '/silent-check-sso.html' })
          console.debug('[KeycloakProvider] kc.init resolved, authenticated=', auth)
          if (auth) setToken(kc.token!)
          // DEV helper: expose current token to window for quick debugging from the browser console
          try{ if (typeof window !== 'undefined') (window as any).__SUT_TOKEN = kc.token }catch(e){}
          setReady(true)
          const id = setInterval(async ()=>{
            try{
              const refreshed = await kc.updateToken(60)
              if (refreshed && kc.token) setToken(kc.token)
              try{ if (typeof window !== 'undefined') (window as any).__SUT_TOKEN = kc.token }catch(e){}
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
  return <Ctx.Provider value={{ token, login, logout: ()=>kc.logout({ redirectUri: window.location.origin }), ready }}>{children}</Ctx.Provider>
}
