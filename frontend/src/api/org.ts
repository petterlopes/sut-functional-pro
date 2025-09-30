import { useQuery } from '@tanstack/react-query'
import { useAuthedClient } from './client'

export function useUnits(){
  const api = useAuthedClient()
  return useQuery({
    queryKey: ['units'],
    queryFn: async ()=>{
      const r = await api.GET('/v1/org/units')
      if(r.error) throw new Error('api error')
      return (r.data as any).items as Array<{id:string,name:string,parent_id?:string|null}>
    }
  })
}

export function useDepartments(unitId?: string){
  const api = useAuthedClient()
  return useQuery({
    queryKey: ['departments', unitId],
    enabled: !!unitId,
    queryFn: async ()=>{
  const r = await api.GET('/v1/org/departments', { params: { query: { unitId } } } as any)
      if(r.error) throw new Error('api error')
      return (r.data as any).items as Array<{id:string,unit_id:string,name:string}>
    }
  })
}
