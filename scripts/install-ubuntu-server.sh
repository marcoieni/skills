#!/usr/bin/env bash
# Set up Ubuntu Server: install the system packages I need and T3 Code's browser.

set -euo pipefail

sudo apt-get install -y libasound2t64

# Allow Chrome's sandbox and install the libraries T3 Code's headless browser needs.
t3 browser setup
