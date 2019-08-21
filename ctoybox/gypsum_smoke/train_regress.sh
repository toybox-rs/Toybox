#!/bin/bash

set -eu

for e in AmidarToyboxNoFrameskip-v4; do
  sbatch --mem=16000 --gres=gpu:1 -p 1080ti-short -o ${e}.timed.out -e ${e}.timed.err gypsum_smoke/train_single.sh $e 1e7 ${e}.timed.model
done
#for e in AmidarToyboxNoFrameskip-v4 SpaceInvadersToyboxNoFrameskip-v4 BreakoutToyboxNoFrameskip-v4; do
#  sbatch --mem=16000 --gres=gpu:1 -p 1080ti-short -o ${e}.timed.out -e ${e}.timed.err gypsum_smoke/train_single.sh $e 1e7 ${e}.timed.model
#done

#for e in AmidarToyboxNoFrameskip-v4 SpaceInvadersToyboxNoFrameskip-v4 BreakoutToyboxNoFrameskip-v4; do
#  sbatch --mem=16000 -p 1080ti-long -o ${e}.regress.out -e ${e}.regress.err gypsum_smoke/train_single.sh $e 1e7 ${e}.regress.model
#done
