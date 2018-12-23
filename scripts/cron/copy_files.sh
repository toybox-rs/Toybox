#!/bin/bash 
# Script to be run as a cron job.
# Copies files from their target output directory.

gypsum_work1=/mnt/nfs/work1/jensen/etosch
swarm2_work1=/mnt/nfs/scratch1/etosch

if [ -z $1 ]; then
    from=$gypsum_work1
    to=$swarm2_work1
else
    from=$gypsum_work1/$1
    to=$swarm2_work1/$1
fi

echo "Transferring files from $from to $to..."
rsync -rv --stats --progress --partial $from/* swarm2:$to/*


