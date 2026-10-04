#!/bin/bash
# Set up Tailscale Funnel for RuVector API on port 6333
# Run with: sudo bash ~/ruvector/scripts/setup-funnel.sh

set -e

# First, set operator so future commands don't need sudo
tailscale set --operator=markt

# Expose ruvector-server via funnel on port 443 path /ruvector
tailscale funnel --bg --set-path /ruvector 6333

echo ""
echo "RuVector API is now accessible at:"
echo "  https://spark-47d8.tail333a1d.ts.net/ruvector/"
echo ""
echo "Test with:"
echo "  curl https://spark-47d8.tail333a1d.ts.net/ruvector/health"
