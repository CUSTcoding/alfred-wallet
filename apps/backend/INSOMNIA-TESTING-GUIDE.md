# 🎯 Guia Completo de Teste no Insomnia

## Arquivo Insomnia Atualizado

O arquivo `alfred-wallet-insomnia.json` já contém todas as rotas **corretas** pré-configuradas.

### Rotas Incluídas

```
✅ Health Check          → GET  /health
✅ Tor Status            → GET  /tor-status
✅ Create Wallet (12)    → POST /create-wallet
✅ Create Wallet (24)    → POST /create-wallet
✅ Restore Wallet        → POST /restore-wallet
✅ New Address (Taproot) → GET  /new-address
✅ Get Balance           → GET  /balance
✅ Send Transaction      → POST /send
```

## Como Importar no Insomnia

### Passo 1: Abrir Insomnia
```
Já está aberto em sua máquina
```

### Passo 2: Importar Collection
1. Menu → Import → From File
2. Selecione: `/home/custcoding/alfred-wallet/apps/backend/alfred-wallet-insomnia.json`
3. Clique em Import

### Passo 3: Verificar Ambiente
1. Canto direito superior → Select Environment
2. Escolha "Local"
3. Verificar se `base_url` está configurado como `http://127.0.0.1:8080`

## Executando os Testes

### Teste 1: Health Check ✅
```
1. Clique em "Health Check"
2. Pressione Send ou Ctrl+Enter
3. Resposta esperada: OK
```

### Teste 2: Tor Status ✅
```
1. Clique em "Tor Status"
2. Pressione Send
3. Resposta esperada: JSON com status do Tor
{
  "circuits_working": false,
  "tor_enabled": true,
  "tor_proxy": "socks5://127.0.0.1:9050"
}
```

### Teste 3: Create Wallet (12 words) ✅
```
1. Clique em "Create Wallet (12 words)"
2. Pressione Send
3. Resposta esperada: Mnemonic + Taproot Address
{
  "mnemonic": "word1 word2 word3 ...",
  "first_address": "bc1p..."
}
```

### Teste 4: Create Wallet (24 words) ✅
```
1. Clique em "Create Wallet (24 words)"
2. Pressione Send
3. Resposta esperada: Mnemonic (24 palavras) + Taproot Address
```

### Teste 5: Restore Wallet ✅
```
1. Clique em "Restore Wallet"
2. No Body, substitua o mnemonic:
   - Use o mnemonic gerado no Teste 3
   - Ou use qualquer BIP39 mnemonic válido
3. Pressione Send
4. Resposta esperada:
{
  "first_address": "bc1p..."
}
```

### Teste 6: New Address (Precisa Bitcoin Core) ⚠️
```
1. Clique em "New Address (Taproot)"
2. Pressione Send
3. Resposta:
   - Se Bitcoin Core está rodando: Novo endereço Taproot
   - Se não: Erro de conexão RPC (esperado)
{
  "error": "error sending request for url (http://127.0.0.1:18443/)"
}
```

### Teste 7: Get Balance (Precisa Bitcoin Core) ⚠️
```
1. Clique em "Get Balance"
2. Pressione Send
3. Resposta:
   - Se Bitcoin Core está rodando: Saldo em BTC
   - Se não: Erro de conexão RPC (esperado)
{
  "balance": 0.0
}
```

### Teste 8: Send Transaction (Precisa Bitcoin Core + Tor) ⚠️
```
1. Clique em "Send Transaction"
2. No Body, você pode manter o hex padrão ou usar outro
3. Pressione Send
4. Resposta:
   - Se Bitcoin Core está rodando: TXID da transação
   - Se não: Erro de conexão RPC (esperado)
{
  "error": "error sending request for url (http://127.0.0.1:18443/)"
}
```

## Configuração de Ambiente

### Base URL
```
Local: http://127.0.0.1:8080
```

Você pode criar ambientes adicionais:
- **Development**: http://127.0.0.1:8080
- **Production**: http://your-domain.com

## Dicas Úteis no Insomnia

### 1. Copiar Mnemonic do Teste Anterior
```
1. Execute "Create Wallet"
2. Na resposta, clique no mnemonic
3. Copie com Ctrl+C
4. Vá para "Restore Wallet"
5. Cole no campo mnemonic dentro do JSON
```

### 2. Formatar JSON
```
Insomnia formata automaticamente o JSON
Se não formatar, use o botão "Format" no lado direito
```

### 3. Ver Headers da Resposta
```
1. Execute uma requisição
2. Clique na aba "Headers"
3. Veja status HTTP, content-type, etc.
```

### 4. Debug com Logs
```
Enquanto testa no Insomnia:
1. Veja o terminal onde cargo run está executando
2. Procure pelos logs com emojis:
   🔍 = função foi chamada
   ✅ = sucesso
   ❌ = erro
   ⚠️  = aviso
```

## Teste Rápido (Tudo de Uma Vez)

1. Health Check ✅
2. Create Wallet ✅
3. Restore Wallet ✅

Todos funcionam **sem Bitcoin Core**!

## Se Houver Erros 404

**Problema:** Insomnia retorna "404 Not Found"

**Solução:**
- Verifique se a rota tem / ou -:
  - ❌ Errado: `/wallet/create`
  - ✅ Correto: `/create-wallet`
- A collection já tem as rotas corretas, não precisa mexer

## Se Houver Erro de Conexão

**Problema:** "Error: connect ECONNREFUSED 127.0.0.1:8080"

**Solução:**
```bash
# Verifique se o servidor está rodando
lsof -i :8080

# Se não estiver, inicie com:
cd /home/custcoding/alfred-wallet/apps/backend
cargo run
```

## Salvar Testes

Todos os testes são salvos automaticamente no Insomnia. Para exportar:
1. Menu → Export → HAR ou Insomnia Format
2. Salve em um local seguro

## Resumo

✅ **Funcionando sem Bitcoin Core:**
- Health Check
- Create Wallet
- Restore Wallet
- Tor Status

⚠️ **Requerem Bitcoin Core:**
- New Address
- Get Balance
- Send Transaction

---

**Próximo Passo:** Instalar Bitcoin Core para testar todos os endpoints
