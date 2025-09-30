#!/usr/bin/env node
const crypto = require('crypto')
if(process.argv.length<3){ console.log('usage: sign_webhook.js <json> <hex-secret>'); process.exit(1) }
const body = process.argv[2]
const secret = Buffer.from(process.argv[3] || 'deadbeef', 'hex')
const h = crypto.createHmac('sha256', secret).update(body).digest('hex')
console.log('sha256='+h)
