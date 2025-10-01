import createClient from 'openapi-fetch'
import type { paths } from './schema'
import { useAuth } from '../auth/KeycloakProvider'

// Runtime API base resolver: tries a list of candidate base URLs and picks
// the first that responds to /health. Results are cached in localStorage so
// repeated probes are fast.
const STORAGE_KEY = 'sut_api_base'
let resolvedBasePromise: Promise<string> | null = null

async function probeUrl(base: string, timeout = 2000){
  try{
    const controller = new AbortController()
    const id = setTimeout(()=> controller.abort(), timeout)
    // Prefer readiness checks; try several common paths so we work with both
    // the main-level /ready and presentation's /readyz, /health, /healthz.
    const baseNoSlash = base.replace(/\/$/, '')
    const candidates = [
      baseNoSlash + '/ready',
      baseNoSlash + '/readyz',
      baseNoSlash + '/health',
      baseNoSlash + '/healthz',
    ]
    let res = null
    for(const u of candidates){
      res = await fetch(u, { signal: controller.signal }).catch(()=> null)
      if (res) break
    }
    clearTimeout(id)
    return res?.ok ?? false
  }catch(e){
    return false
  }
}

async function resolveApiBase(){
  if (resolvedBasePromise) return resolvedBasePromise
  resolvedBasePromise = (async ()=>{
    const env: any = (import.meta as any).env || {}
    const candidates: string[] = []
    if (env.VITE_API_BASE) candidates.push(env.VITE_API_BASE)
    // common host and container addresses
    candidates.push('http://localhost:8080')
    candidates.push('http://api:8080')
    // also try origin with port 8080 (useful if dev server is proxied)
    try{ if (typeof window !== 'undefined' && window.location) candidates.push(window.location.protocol + '//' + window.location.hostname + ':8080') }catch(e){}
    // include any previously cached value first, but avoid Docker-internal
    // hostnames (like http://api:8080) when running in a host browser.
    try{
      const cached = localStorage.getItem(STORAGE_KEY)
      if (cached) {
        const lower = cached.toLowerCase()
        const isDockerName = /:\/\/(api|keycloak|deploy-)/.test(lower) || /:\/\/[a-z0-9_-]+:\d+/.test(lower) && !lower.includes('localhost')
        if (!isDockerName) candidates.unshift(cached)
        else console.debug('[resolveApiBase] ignoring cached docker-internal base', cached)
      }
    }catch(e){/* ignore storage errors */}

    // Remove duplicates while preserving order
    const seen = new Set<string>()
    const unique = candidates.filter(c => c && !seen.has(c) && (seen.add(c), true))

    for(const c of unique){
      try{
        const ok = await probeUrl(c, 1500)
        console.debug('[resolveApiBase] probe', c, 'ok=', ok)
        if (ok){
          try{ localStorage.setItem(STORAGE_KEY, c) }catch(e){}
          const chosen = c.replace(/\/$/, '')
          console.info('[resolveApiBase] resolved api base ->', chosen)
          return chosen
        }
      }catch(e){
        // ignore probe errors
      }
    }
    // fallback
    return (env.VITE_API_BASE) ?? 'http://localhost:8080'
  })()
  return resolvedBasePromise
}

export function useAuthedClient(){
  const { token } = useAuth()
  // Start resolving the API base (async). The fetch wrapper will await it.
  const _ = resolveApiBase()

  return createClient<paths>({
    baseUrl: 'http://localhost:8080', // placeholder, actual base is rewritten in fetch
    fetch: async (url, init={})=>{
      const headers = new Headers(init && (init as any).headers || {})
      if (token) headers.set('Authorization', `Bearer ${token}`)
      
      // Fallback: add X-Dev-User header for development when JWT validation fails
      if (token) headers.set('X-Dev-User', 'admin')

      // Ensure we have a resolved base URL (with a short timeout)
      let base = 'http://localhost:8080'
      try{
        base = await Promise.race([resolveApiBase(), new Promise<string>(r=>setTimeout(()=>r(base), 1500))])
      }catch(e){/* ignore */}

      try{
        // Normalize the incoming url which may be a string, URL, or a Request
        const incoming: any = url
        let urlStr: string
        if (typeof incoming === 'string') urlStr = incoming
        else if (typeof Request !== 'undefined' && incoming instanceof (Request as any)) urlStr = incoming.url
        else if (typeof URL !== 'undefined' && incoming instanceof (URL as any)) urlStr = incoming.toString()
        else urlStr = (incoming && typeof incoming.toString === 'function') ? incoming.toString() : String(incoming)

        // If the URL is absolute, replace its origin with the resolved base origin
        const urlObj = new URL(urlStr)
        const baseObj = new URL(base)
        urlObj.protocol = baseObj.protocol
        urlObj.hostname = baseObj.hostname
        urlObj.port = baseObj.port
        return fetch(urlObj.toString(), { ...init, headers })
      }catch(e){
        // If URL parsing fails, fallback to joining base + path
        const path = (url && typeof (url as any).toString === 'function') ? (url as any).toString() : String(url)
        const target = base.replace(/\/$/, '') + '/' + path.replace(/^\//, '')
        return fetch(target, { ...init, headers })
      }
    }
  })
}

// Expose helpers for UI to read/clear resolved base and force re-resolve
export async function getResolvedApiBase(timeout = 1500){
  try{
    return await Promise.race([resolveApiBase(), new Promise<string>(r=>setTimeout(()=>r(localStorage.getItem(STORAGE_KEY)||'http://localhost:8080'), timeout))])
  }catch(e){ return localStorage.getItem(STORAGE_KEY) || 'http://localhost:8080' }
}

export function clearResolvedApiBase(){
  try{ localStorage.removeItem(STORAGE_KEY) }catch(e){}
  resolvedBasePromise = null
}

// Lightweight untyped fetch helper (useful for admin endpoints not present in OpenAPI)
export function useAuthFetch(){
  const { token } = useAuth()
  const _ = resolveApiBase()
  return async (path: string, opts: RequestInit = {}) => {
    const headers = new Headers(opts.headers || {})
    if (token) headers.set('Authorization', `Bearer ${token}`)
    if (!headers.has('Content-Type') && opts.body) headers.set('Content-Type', 'application/json')

    let base = 'http://localhost:8080'
    try{ base = await Promise.race([resolveApiBase(), new Promise<string>(r=>setTimeout(()=>r(base), 1500))]) }catch(e){}
    const res = await fetch(base.replace(/\/$/, '') + path, { ...opts, headers })
    const text = await res.text()
    try { return { ok: res.ok, status: res.status, data: text ? JSON.parse(text) : null } }
    catch { return { ok: res.ok, status: res.status, data: text } }
  }
}
