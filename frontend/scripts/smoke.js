#!/usr/bin/env node
const http = require('http')
const https = require('https')

function fetch(url, opts = {}){
  return new Promise((resolve, reject)=>{
    const lib = url.startsWith('https') ? https : http
    const req = lib.get(url, opts, res=>{
      let data = ''
      res.on('data', chunk=>data+=chunk)
      res.on('end', ()=> resolve({ status: res.statusCode, body: data }))
    })
    req.on('error', reject)
    req.end()
  })
}

async function main(){
  const apiCandidates = [process.env.VITE_API_BASE || 'http://localhost:8080', 'http://api:8080']
  const kcCandidates = [process.env.VITE_KC_URL || 'http://localhost:8081', 'http://keycloak:8080']

  console.log('Probing API candidates:')
  for(const c of apiCandidates){
    try{
      const r = await fetch(c.replace(/\/$/, '') + '/health')
      console.log(` - ${c} -> ${r.status}`)
    }catch(e){ console.log(` - ${c} -> error (${e.message})`) }
  }

  console.log('Probing Keycloak candidates:')
  for(const c of kcCandidates){
    try{
      const r = await fetch(c.replace(/\/$/, '') + '/realms/sut/.well-known/openid-configuration')
      console.log(` - ${c} -> ${r.status}`)
    }catch(e){ console.log(` - ${c} -> error (${e.message})`) }
  }

  if (process.env.SMOKE_TOKEN){
    const api = (process.env.VITE_API_BASE || 'http://localhost:8080').replace(/\/$/, '')
    try{
      const lib = api.startsWith('https') ? https : http
      const req = lib.get(api + '/v1/localidades', { headers: { Authorization: 'Bearer ' + process.env.SMOKE_TOKEN } }, res=>{
        console.log('Auth call to /v1/localidades ->', res.statusCode)
        res.resume()
      })
      req.on('error', e=> console.log('auth call error', e.message))
    }catch(e){ console.log('auth call exception', e.message) }
  }else{
    console.log('SMOKE_TOKEN not set; skipping authenticated API call')
  }
}

main().catch(e=>{ console.error(e); process.exit(1) })
