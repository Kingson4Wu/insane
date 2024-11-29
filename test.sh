#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to print test result
print_result() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✓ $2${NC}"
    else
        echo -e "${RED}✗ $2${NC}"
        exit 1
    fi
}

# Function to test an endpoint
test_endpoint() {
    local description=$1
    local method=$2
    local endpoint=$3
    local data=$4
    
    if [ "$method" = "GET" ]; then
        response=$(curl -s -X GET "http://localhost:3000$endpoint")
    else
        response=$(curl -s -X "$method" -H "Content-Type: application/json" -d "$data" "http://localhost:3000$endpoint")
    fi
    
    if [ $? -eq 0 ] && [ ! -z "$response" ]; then
        print_result 0 "$description - Success"
        echo "Response: $response"
        echo "----------------------------------------"
    else
        print_result 1 "$description - Failed"
    fi
}

echo "Starting API tests..."
echo "----------------------------------------"

# Test health endpoint
test_endpoint "Health Check" "GET" "/health"

# Test visit counter
test_endpoint "Visit Counter" "GET" "/visit"

# Test echo endpoint
test_endpoint "Echo Message" "POST" "/echo" '{"content":"Hello, Axum!"}'

echo "All tests completed successfully!"
