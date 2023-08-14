#!/bin/bash

USER="root"
IP_ADDRESS="104.192.0.90"

# Step 1: SSH into the given IP address
ssh -tt $USER@$IP_ADDRESS /bin/bash << SSH_EOF

# Step 2: Run commands as the postgres user
sudo -u postgres bash;

# Step 3: Change directory to the home directory
cd ~;

# Step 4: Dump the schema using pg_dump
pg_dump --exclude-table='scan_[0-9]+' --exclude-table='scan_[0-9]+_errors' --no-tablespaces --no-owner --no-privileges --schema-only rocketsource > schema.sql;

# Step 5: Exit the postgres user shell
exit;

# Step 6: Exit the SSH session
exit;

SSH_EOF

# Step 7: Copy the schema.sql file to the tests/ directory
scp $USER@$IP_ADDRESS:/var/lib/postgresql/schema.sql tests/
