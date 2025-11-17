#!/bin/bash

echo "=== IoT Fleet Management Demo ==="
echo "Starting tmux session with asciinema recording..."
echo ""

# Clean up old sockets
rm -f /tmp/supervisor-us-west.sock
rm -f /tmp/supervisor-eu-west.sock

# Check if asciinema is installed
if ! command -v asciinema &> /dev/null; then
    echo "asciinema not found. Install with: sudo apt install asciinema"
    echo "Continuing without recording..."
    RECORD_CMD=""
else
    TIMESTAMP=$(date +%Y%m%d_%H%M%S)
    RECORD_FILE="iot-fleet-demo-${TIMESTAMP}.cast"
    echo "Recording to: ${RECORD_FILE}"
    RECORD_CMD="asciinema rec -c"
fi

# Create new tmux session named "iot-fleet"
if [ -n "$RECORD_CMD" ]; then
    asciinema rec "$RECORD_FILE" -c "tmux new-session -s iot-fleet \\; \
        send-keys 'htop -p \$(pgrep -d, -f \"regional-supervisor|cluster_control\" || echo \$\$)' C-m \\; \
        split-window -v -p 75 \\; \
        send-keys 'cargo run --bin regional-supervisor-us-west' C-m \\; \
        split-window -h \\; \
        send-keys 'sleep 3 && cargo run --bin regional-supervisor-eu-west' C-m \\; \
        split-window -v \\; \
        send-keys 'sleep 6 && cargo run --example cluster_control' C-m \\; \
        select-pane -t 1 \\; \
        send-keys 'sleep 8 && tmux send-keys -t iot-fleet:0.0 q htop Space -p Space \$(pgrep -d, -f \"regional-supervisor|cluster_control\") Enter' C-m \\; \
        select-pane -t 0"
else
    tmux new-session -d -s iot-fleet -n "Fleet Management"
    
    # Top pane: htop (will update after processes start)
    tmux send-keys -t iot-fleet "htop -p \$(pgrep -d, -f 'regional-supervisor|cluster_control' || echo \$\$)" C-m
    
    # Split bottom portion (75% of screen)
    tmux split-window -v -p 75 -t iot-fleet
    
    # Bottom-left: US-West supervisor
    tmux send-keys -t iot-fleet "cargo run --bin regional-supervisor-us-west" C-m
    
    # Split horizontally for EU-West
    tmux split-window -h -t iot-fleet
    tmux send-keys -t iot-fleet "sleep 3 && cargo run --bin regional-supervisor-eu-west" C-m
    
    # Split the right pane vertically for cluster control
    tmux split-window -v -t iot-fleet
    tmux send-keys -t iot-fleet "sleep 6 && cargo run --example cluster_control" C-m
    
    # Create a temporary pane to refresh htop with actual PIDs after processes start
    tmux split-window -v -t iot-fleet:0.1
    tmux send-keys -t iot-fleet "sleep 8 && tmux send-keys -t iot-fleet:0.0 q 'htop -p \$(pgrep -d, -f \"regional-supervisor|cluster_control\")' Enter && tmux kill-pane" C-m
    
    # Select the top pane (htop)
    tmux select-pane -t iot-fleet:0.0
    
    # Attach to the session
    tmux attach-session -t iot-fleet
fi

echo ""
echo "Demo finished!"
