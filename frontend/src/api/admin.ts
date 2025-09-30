import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { useAuthedClient } from './client'
import type { paths } from './schema'

// Localidades
export function useLocalidades(){
  const api = useAuthedClient()
  return useQuery({ queryKey: ['localidades'], queryFn: async ()=>{
    const r = await api.GET('/v1/localidades', {}) as any
    if (r.error) throw new Error('api error')
    return (r.data as any).items
  }})
}

export function useCreateLocalidade(){
  const api = useAuthedClient(); const qc = useQueryClient()
  return useMutation({ mutationFn: async (body: any)=>{
    const r = await api.POST('/v1/localidades', { body })
    if (r.error) throw new Error('api error')
    return r.data
  }, onSuccess: ()=> qc.invalidateQueries({ queryKey: ['localidades'] }) })
}

export function useUpdateLocalidade(id: number){
  const api = useAuthedClient(); const qc = useQueryClient()
  return useMutation({ mutationFn: async (body: any)=>{
    const r = await api.PATCH('/v1/localidades/{id}', { params: { path: { id } }, body })
    if (r.error) throw new Error('api error')
    return r.data
  }, onSuccess: ()=> qc.invalidateQueries({ queryKey: ['localidades'] }) })
}

export function useDeleteLocalidade(id: number){
  const api = useAuthedClient(); const qc = useQueryClient()
  return useMutation({ mutationFn: async ()=>{
    const r = await api.DELETE('/v1/localidades/{id}', { params: { path: { id } } })
    if (r.error) throw new Error('api error')
    return r.data
  }, onSuccess: ()=> qc.invalidateQueries({ queryKey: ['localidades'] }) })
}

// Departamentos
export function useDepartamentos(){
  const api = useAuthedClient()
  return useQuery({ queryKey: ['departamentos'], queryFn: async ()=>{
    const r = await api.GET('/v1/departamentos', {}) as any
    if (r.error) throw new Error('api error')
    return (r.data as any).items
  }})
}

export function useCreateDepartamento(){
  const api = useAuthedClient(); const qc = useQueryClient()
  return useMutation({ mutationFn: async (body: any)=>{
    const r = await api.POST('/v1/departamentos', { body })
    if (r.error) throw new Error('api error')
    return r.data
  }, onSuccess: ()=> qc.invalidateQueries({ queryKey: ['departamentos'] }) })
}

export function useUpdateDepartamento(id: number){
  const api = useAuthedClient(); const qc = useQueryClient()
  return useMutation({ mutationFn: async (body: any)=>{
    const r = await api.PATCH('/v1/departamentos/{id}', { params: { path: { id } }, body })
    if (r.error) throw new Error('api error')
    return r.data
  }, onSuccess: ()=> qc.invalidateQueries({ queryKey: ['departamentos'] }) })
}

export function useDeleteDepartamento(id: number){
  const api = useAuthedClient(); const qc = useQueryClient()
  return useMutation({ mutationFn: async ()=>{
    const r = await api.DELETE('/v1/departamentos/{id}', { params: { path: { id } } })
    if (r.error) throw new Error('api error')
    return r.data
  }, onSuccess: ()=> qc.invalidateQueries({ queryKey: ['departamentos'] }) })
}

// Tipos / Origem / Ref - simple endpoints
export function useTiposContato(){
  const api = useAuthedClient()
  return useQuery({ queryKey: ['tipos-contato'], queryFn: async ()=>{
    const r = await api.GET('/v1/tipos-contato', {}) as any
    if (r.error) throw new Error('api error')
    return (r.data as any).items
  }})
}

export function useOrigensContato(){
  const api = useAuthedClient()
  return useQuery({ queryKey: ['origens-contato'], queryFn: async ()=>{
    const r = await api.GET('/v1/origens-contato', {}) as any
    if (r.error) throw new Error('api error')
    return (r.data as any).items
  }})
}

export function useRefOrigemContato(){
  const api = useAuthedClient()
  return useQuery({ queryKey: ['ref-origem'], queryFn: async ()=>{
    const r = await api.GET('/v1/ref-origem-contato', {}) as any
    if (r.error) throw new Error('api error')
    return (r.data as any).items
  }})
}

// Grupos e membros
export function useGrupos(){
  const api = useAuthedClient()
  return useQuery({ queryKey: ['grupos'], queryFn: async ()=>{
    const r = await api.GET('/v1/grupos', {}) as any
    if (r.error) throw new Error('api error')
    return (r.data as any).items
  }})
}

export function useGrupoMembros(){
  const api = useAuthedClient()
  return useQuery({ queryKey: ['grupo-membros'], queryFn: async ()=>{
    const r = await api.GET('/v1/grupo-membros', {}) as any
    if (r.error) throw new Error('api error')
    return (r.data as any).items
  }})
}

// Responsáveis
export function useResponsaveis(){
  const api = useAuthedClient()
  return useQuery({ queryKey: ['responsaveis'], queryFn: async ()=>{
    const r = await api.GET('/v1/responsaveis', {}) as any
    if (r.error) throw new Error('api error')
    return (r.data as any).items
  }})
}

// Sites de busca
export function useSites(){
  const api = useAuthedClient()
  return useQuery({ queryKey: ['sites'], queryFn: async ()=>{
    const r = await api.GET('/v1/sites', {}) as any
    if (r.error) throw new Error('api error')
    return (r.data as any).items
  }})
}
