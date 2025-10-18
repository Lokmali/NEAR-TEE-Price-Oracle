#!/bin/bash

# NEAR TEE Oracle - Monitoring Script
# Monitor oracle system health and send alerts

set -e

# Configuration
CONTRACT_ID=${1:-oracle.near}
NETWORK=${2:-mainnet}
ALERT_EMAIL=${3:-""}
SLACK_WEBHOOK=${4:-""}

# Thresholds
MAX_PRICE_AGE=300  # 5 minutes in seconds
MIN_ACTIVE_NODES=3
MIN_CONFIDENCE=90

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')

echo "======================================"
echo "NEAR TEE Oracle - Health Check"
echo "Time: $TIMESTAMP"
echo "Contract: $CONTRACT_ID"
echo "======================================"
echo ""

# Function to send alert
send_alert() {
    local level=$1
    local message=$2
    
    echo -e "${RED}[ALERT]${NC} $message"
    
    # Send email if configured
    if [ -n "$ALERT_EMAIL" ]; then
        echo "$message" | mail -s "Oracle Alert: $level" $ALERT_EMAIL
    fi
    
    # Send Slack notification if configured
    if [ -n "$SLACK_WEBHOOK" ]; then
        curl -X POST -H 'Content-type: application/json' \
            --data "{\"text\":\"🚨 Oracle Alert: $message\"}" \
            $SLACK_WEBHOOK
    fi
}

# Check active nodes
echo "Checking active nodes..."
NODES=$(near view $CONTRACT_ID get_active_nodes '{}' --networkId $NETWORK 2>/dev/null)

if [ -z "$NODES" ]; then
    send_alert "CRITICAL" "Unable to query oracle contract"
    exit 1
fi

NODE_COUNT=$(echo $NODES | jq '. | length')
echo "Active nodes: $NODE_COUNT"

if [ $NODE_COUNT -lt $MIN_ACTIVE_NODES ]; then
    send_alert "HIGH" "Only $NODE_COUNT active nodes (minimum: $MIN_ACTIVE_NODES)"
fi

# Check each node's last update
echo ""
echo "Checking node update times..."
CURRENT_TIME=$(date +%s)

echo $NODES | jq -r '.[] | "\(.account_id) \(.last_update)"' | while read node_id last_update; do
    # Convert nanoseconds to seconds
    last_update_sec=$((last_update / 1000000000))
    age=$((CURRENT_TIME - last_update_sec))
    
    if [ $age -gt $MAX_PRICE_AGE ]; then
        echo -e "${YELLOW}[WARNING]${NC} $node_id: Last update ${age}s ago"
        send_alert "MEDIUM" "Node $node_id hasn't updated in ${age}s"
    else
        echo -e "${GREEN}[OK]${NC} $node_id: Updated ${age}s ago"
    fi
done

# Check asset prices
echo ""
echo "Checking asset prices..."
ASSETS=$(near view $CONTRACT_ID get_supported_assets '{}' --networkId $NETWORK 2>/dev/null)

if [ -z "$ASSETS" ]; then
    send_alert "HIGH" "Unable to fetch supported assets"
else
    echo "Supported assets: $(echo $ASSETS | jq -r '. | join(", ")')"
    
    # Check a few key assets
    for asset in "BTC" "ETH" "NEAR"; do
        PRICE=$(near view $CONTRACT_ID get_price "{\"symbol\": \"$asset\"}" --networkId $NETWORK 2>/dev/null)
        
        if [ -z "$PRICE" ] || [ "$PRICE" == "null" ]; then
            echo -e "${RED}[ERROR]${NC} $asset: Price not available"
            send_alert "HIGH" "$asset price not available"
            continue
        fi
        
        CONFIDENCE=$(echo $PRICE | jq -r '.confidence')
        TIMESTAMP_NS=$(echo $PRICE | jq -r '.timestamp')
        TIMESTAMP_S=$((TIMESTAMP_NS / 1000000000))
        AGE=$((CURRENT_TIME - TIMESTAMP_S))
        
        if [ $CONFIDENCE -lt $MIN_CONFIDENCE ]; then
            echo -e "${YELLOW}[WARNING]${NC} $asset: Low confidence ($CONFIDENCE%)"
            send_alert "MEDIUM" "$asset price has low confidence ($CONFIDENCE%)"
        fi
        
        if [ $AGE -gt $MAX_PRICE_AGE ]; then
            echo -e "${YELLOW}[WARNING]${NC} $asset: Stale price (${AGE}s old)"
            send_alert "MEDIUM" "$asset price is stale (${AGE}s old)"
        else
            PRICE_VALUE=$(echo $PRICE | jq -r '.price')
            PRICE_USD=$(echo "scale=2; $PRICE_VALUE / 100000000" | bc)
            echo -e "${GREEN}[OK]${NC} $asset: \$$PRICE_USD (confidence: $CONFIDENCE%, age: ${AGE}s)"
        fi
    done
fi

# Check contract state
echo ""
echo "Checking contract state..."
STATE=$(near state $CONTRACT_ID --networkId $NETWORK 2>/dev/null)

if [ -z "$STATE" ]; then
    send_alert "CRITICAL" "Unable to query contract state"
else
    BALANCE=$(echo $STATE | grep -oP 'amount: \K[0-9]+')
    BALANCE_NEAR=$(echo "scale=2; $BALANCE / 1000000000000000000000000" | bc)
    echo "Contract balance: $BALANCE_NEAR NEAR"
    
    if (( $(echo "$BALANCE_NEAR < 1" | bc -l) )); then
        send_alert "MEDIUM" "Contract balance low: $BALANCE_NEAR NEAR"
    fi
fi

# System resource check (if running on node server)
if command -v free &> /dev/null; then
    echo ""
    echo "System Resources:"
    
    # Memory usage
    MEM_USAGE=$(free | grep Mem | awk '{printf("%.0f", $3/$2 * 100)}')
    echo "Memory usage: $MEM_USAGE%"
    if [ $MEM_USAGE -gt 90 ]; then
        send_alert "MEDIUM" "High memory usage: $MEM_USAGE%"
    fi
    
    # Disk usage
    DISK_USAGE=$(df -h / | awk 'NR==2 {print $5}' | sed 's/%//')
    echo "Disk usage: $DISK_USAGE%"
    if [ $DISK_USAGE -gt 90 ]; then
        send_alert "HIGH" "High disk usage: $DISK_USAGE%"
    fi
fi

echo ""
echo "======================================"
echo "Health check complete"
echo "======================================"

