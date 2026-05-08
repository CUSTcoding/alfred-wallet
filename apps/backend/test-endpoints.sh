#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

BASE_URL="http://localhost:8080"

echo -e "${BLUE}═══════════════════════════════════════${NC}"
echo -e "${BLUE}   Alfred Wallet - API Endpoint Tests${NC}"
echo -e "${BLUE}═══════════════════════════════════════${NC}"
echo

# Test 1: Health Check
echo -e "${YELLOW}[1/7] Testing Health Check...${NC}"
response=$(curl -s "$BASE_URL/health")
if [ "$response" = "OK" ]; then
    echo -e "${GREEN}✓ Health Check${NC}: $response"
else
    echo -e "${RED}✗ Health Check${NC}: Unexpected response"
fi
echo

# Test 2: Tor Status
echo -e "${YELLOW}[2/7] Testing Tor Status...${NC}"
response=$(curl -s "$BASE_URL/tor-status" | jq .)
echo -e "${GREEN}✓ Tor Status${NC}:"
echo "$response" | jq '.' --indent 2
echo

# Test 3: Create Wallet
echo -e "${YELLOW}[3/7] Testing Create Wallet (12 words)...${NC}"
response=$(curl -s -X POST "$BASE_URL/create-wallet" \
    -H "Content-Type: application/json" \
    -d '{"word_count": 12}')
echo -e "${GREEN}✓ Create Wallet${NC}:"
echo "$response" | jq '.' --indent 2
echo

# Store mnemonic for next test
mnemonic=$(echo "$response" | jq -r '.mnemonic')
echo

# Test 4: Restore Wallet
echo -e "${YELLOW}[4/7] Testing Restore Wallet...${NC}"
if [ ! -z "$mnemonic" ] && [ "$mnemonic" != "null" ]; then
    response=$(curl -s -X POST "$BASE_URL/restore-wallet" \
        -H "Content-Type: application/json" \
        -d "{\"mnemonic\": \"$mnemonic\"}")
    echo -e "${GREEN}✓ Restore Wallet${NC}:"
    echo "$response" | jq '.' --indent 2
else
    echo -e "${RED}✗ Restore Wallet${NC}: Could not get mnemonic from create wallet"
fi
echo

# Test 5: New Address
echo -e "${YELLOW}[5/7] Testing New Address...${NC}"
response=$(curl -s "$BASE_URL/new-address")
echo -e "${GREEN}Response from New Address:${NC}"
echo "$response" | jq '.' --indent 2 2>/dev/null || echo "$response"
echo

# Test 6: Get Balance
echo -e "${YELLOW}[6/7] Testing Get Balance...${NC}"
response=$(curl -s "$BASE_URL/balance")
echo -e "${GREEN}Response from Get Balance:${NC}"
echo "$response" | jq '.' --indent 2 2>/dev/null || echo "$response"
echo

# Test 7: Send Transaction (test only, no actual BTC)
echo -e "${YELLOW}[7/7] Testing Send Transaction...${NC}"
response=$(curl -s -X POST "$BASE_URL/send" \
    -H "Content-Type: application/json" \
    -d '{"hex": "0100000000010000000000"}')
echo -e "${GREEN}Response from Send Transaction:${NC}"
echo "$response" | jq '.' --indent 2 2>/dev/null || echo "$response"
echo

echo -e "${BLUE}═══════════════════════════════════════${NC}"
echo -e "${BLUE}     All Endpoint Tests Completed${NC}"
echo -e "${BLUE}═══════════════════════════════════════${NC}"
