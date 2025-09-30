import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { useAuthedClient } from './client'

export function useContacts(limit = 20){
  const api = useAuthedClient()
  return useQuery({
    queryKey: ['contacts', limit],
    queryFn: async ()=>{
  const r = await api.GET('/v1/contacts', { params: { query: { limit } } })
      if(r.error) throw new Error('api error')
      return (r.data as any)
    }
  })
}

export function useCreateContact(){
  const api = useAuthedClient()
  const qc = useQueryClient()
  return useMutation({
    mutationFn: async (body: any)=>{
      const r = await api.POST('/v1/contacts', { body })
      if(r.error) throw new Error('api error')
      return r.data
    },
    onSuccess: ()=> qc.invalidateQueries({ queryKey: ['contacts'] })
  })
}

export function useUpdateContact(id: string){
  const api = useAuthedClient()
  const qc = useQueryClient()
  return useMutation({
    mutationFn: async ({ body, etag }: { body: any, etag: string })=>{
  const r = await api.PATCH('/v1/contacts/{id}', { params: { path: { id } }, body, headers: { 'If-Match': etag } })
      if(r.error) throw new Error('api error')
      return r.data
    },
    onSuccess: ()=> qc.invalidateQueries({ queryKey: ['contacts'] })
  })
}

export function useDeleteContact(id: string){
  const api = useAuthedClient()
  const qc = useQueryClient()
  return useMutation({
    mutationFn: async ()=>{
  const r = await api.DELETE('/v1/contacts/{id}', { params: { path: { id } } })
      if(r.error) throw new Error('api error')
      return r.data
    },
    onSuccess: ()=> qc.invalidateQueries({ queryKey: ['contacts'] })
  })
}

export function useUpdateContactDocument(){
  const api = useAuthedClient()
  const qc = useQueryClient()
  return useMutation({
    mutationFn: async ({ id, document }: { id: string, document: string })=>{
  const r = await api.PATCH('/v1/contacts/{id}/document', { params: { path: { id } }, body: { document } })
      if(r.error) throw new Error('api error')
      return r.data
    },
    onSuccess: (_, vars)=>{
      qc.invalidateQueries({ queryKey: ['contact-doc', vars.id] })
      qc.invalidateQueries({ queryKey: ['contacts'] })
    }
  })
}

export function useSearch(query: string, autocomplete = true, limit = 8){
  const api = useAuthedClient()
  return useQuery({
    queryKey: ['search', query, autocomplete, limit],
    enabled: query.trim().length>0,
    queryFn: async ()=>{
  const r = await api.GET('/v1/search', { params: { query: { q: query, autocomplete, limit } } })
      if(r.error) throw new Error('api error')
      return r.data as any
    }
  })
}

export function useContactDocument(id: string){
  const api = useAuthedClient()
  return useQuery({
    queryKey: ['contact-doc', id],
    enabled: !!id,
    queryFn: async ()=>{
  const r = await api.GET('/v1/contacts/{id}/document', { params: { path: { id } } })
      if(r.error) throw new Error('api error')
      return r.data as any
    }
  })
}
