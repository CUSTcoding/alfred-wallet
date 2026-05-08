# Alfred Wallet Backend - Insomnia Tests

## 📋 Como Importar e Usar no Insomnia

### 1. Importar o Arquivo
1. Abra o Insomnia
2. Clique em "Create" → "Import/Export"
3. Selecione "Import Data" → "From File"
4. Escolha o arquivo `alfred-wallet-insomnia.json`
5. Selecione o workspace "Alfred Wallet Backend API"

### 2. Configurar Environment
- Vá para "Manage Environments" (ícone de olho)
- Selecione "Local"
- Verifique se `base_url` está como `http://127.0.0.1:8080`

### 3. Ordem Recomendada de Testes

#### ✅ 1. Health Check
- **Request**: `GET /health`
- **Expected**: `OK`
- **Purpose**: Verificar se o servidor está rodando

#### ✅ 2. Tor Status
- **Request**: `GET /tor-status`
- **Expected**: Status do Tor e circuitos
- **Purpose**: Verificar integração Tor

#### ✅ 3. Criar Wallet
- **Request**: `POST /create-wallet` com `{"word_count": 12}` ou `24`
- **Expected**: Mnemonic de 12/24 palavras + endereço Taproot
- **Purpose**: Gerar nova wallet HD com seed BIP39

#### ✅ 4. Novo Endereço
- **Request**: `GET /new-address`
- **Expected**: Endereço Taproot (bc1p...)
- **Purpose**: Gerar endereço P2TR via Bitcoin Core

#### ✅ 5. Verificar Saldo
- **Request**: `GET /balance`
- **Expected**: Saldo em BTC (provavelmente 0.00000000 inicialmente)
- **Purpose**: Consultar saldo da wallet

#### ✅ 6. Enviar Transação (Apenas se tiver fundos)
- **Request**: `POST /send` com `{"hex": "0200000001..."}`
- **Expected**: TXID da transação
- **Purpose**: Enviar BTC com novo circuito Tor
- **⚠️ Warning**: Requer transação raw hex válida

#### ✅ 7. Restore Wallet (Opcional)
- **Request**: `POST /restore-wallet` com mnemonic
- **Expected**: Primeiro endereço da wallet restaurada
- **Purpose**: Recuperar wallet existente

## 🔧 Configuração Necessária

### Backend
```bash
cd apps/backend
cargo run
```

### Bitcoin Core (para testes reais)
- Deve estar rodando com RPC habilitado
- ZMQ configurado: `zmqpubrawtx=tcp://127.0.0.1:28332`

### Tor (opcional para anonimato)
- Tor rodando na porta 9050 (SOCKS5)
- Control Port 9051 habilitado

## 📊 Fluxo Completo de Teste

1. **Health** ✅ → Servidor OK
2. **Tor Status** ✅ → Tor funcionando
3. **Create Wallet** ✅ → Wallet criada
4. **New Address** ✅ → Endereço Taproot gerado
5. **Balance** ✅ → Saldo consultado
6. **Send** ⚠️ → Apenas com fundos reais

## 🐛 Troubleshooting

- **404 Not Found**: Servidor não está rodando
- **500 Internal Server Error**: Bitcoin Core não configurado
- **Connection Refused**: Porta errada ou firewall
- **Tor Errors**: Tor não está rodando ou mal configurado

## 📝 Notas
- Todos os requests são para `http://127.0.0.1:8080`
- Content-Type: `application/json` para POST requests
- Responses em JSON (exceto health que retorna texto)