# Webhooks

Endpoint: `POST /v1/ingestion/events`  
Headers:
- `Content-Type: application/json`
- `X-Signature: sha256=<hmac-hex>`

Payload:
```json
{
  "source": "crm-x",
  "sourceKey": "contact:123",
  "payload": { "event": "upsert", "contact": { "id": "123", "name": "Alice" } },
  "nonce": "random-uuid",
  "ts": 1710000000
}
```

Assinatura = `HMAC_SHA256(secret, JSON.stringify(body))`  
A janela de aceitação é de **5 minutos** (campo `ts`). Requisições duplicadas por `(source, nonce)` são rejeitadas como `status=duplicate`.
