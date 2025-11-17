#!/bin/bash

# Ash Flare Supervisor Tree Demo with Asciinema Recording
# Creates a tmux dashboard showing supervisor tree and logs

SESSION_NAME="ash-flare-demo"
RECORDING_FILE="ash-flare-demo-$(date +%Y%m%d_%H%M%S).cast"

# Colors for better presentation
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${CYAN}🎥 Starting Ash Flare Demo Recording${NC}"
echo -e "${YELLOW}📊 Setting up tmux session: $SESSION_NAME${NC}"

# Check for asciinema
if ! command -v asciinema &> /dev/null; then
    echo -e "${RED}❌ asciinema not found${NC}"
    echo -e "${BLUE}Install with: sudo apt install asciinema${NC}"
    exit 1
fi

# Kill existing session if it exists
tmux kill-session -t $SESSION_NAME 2>/dev/null

# Build the project first
echo -e "${BLUE}🔨 Building project...${NC}"
cargo build --example tmux_demo --release --quiet

if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Build failed${NC}"
    exit 1
fi

echo -e "${GREEN}🎬 Starting recording: $RECORDING_FILE${NC}"
echo -e "${CYAN}💡 The demo will run automatically for 30 seconds${NC}"
echo -e "${CYAN}💡 Press Ctrl+D to stop early${NC}"
echo ""

# Function to cleanup on exit
cleanup() {
    echo -e "\n${RED}🛑 Cleaning up...${NC}"
    tmux kill-session -t $SESSION_NAME 2>/dev/null || true
    echo -e "${GREEN}✅ Cleanup complete${NC}"
}

# Set trap for cleanup on script exit
trap cleanup EXIT

# Start asciinema recording
asciinema rec "$RECORDING_FILE" \
    --overwrite \
    --title "Ash Flare Supervisor Tree Demo" \
    --command "tmux new-session -s $SESSION_NAME \
        'cargo run --example tmux_demo --release tree' \; \
        split-window -h 'cargo run --example tmux_demo --release logs' \; \
        select-pane -t 0 -T 'Supervisor Tree' \; \
        select-pane -t 1 -T 'Live Logs' \; \
        select-pane -t 0"

echo ""
echo -e "${GREEN}✅ Recording complete!${NC}"
echo ""
echo -e "${PURPLE}📁 File: $RECORDING_FILE${NC}"
echo ""
echo -e "${CYAN}▶️  Play locally:${NC}"
echo -e "   asciinema play $RECORDING_FILE"
echo ""
echo -e "${CYAN}☁️  Upload to share:${NC}"
echo -e "   asciinema upload $RECORDING_FILE"
echo ""
