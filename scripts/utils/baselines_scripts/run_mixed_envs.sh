module load cudnn/7.3-cuda_9.0

algs="acktr a2c ppo2"
work1=/mnt/nfs/work1/jensen/kclary/NeurIPS_2019
logs=$work1/logs

mkdir -p $logs

partition="titanx-long"

envs="AmidarNoFrameskip-v4 BreakoutNoFrameskip-v4 SpaceInvadersNoFrameskip-v4"
timesteps="1e8"
stepincr="1e7"
weights="[0.1,0.9] [0.2,0.8] [0.3,0.7] [0.4,0.6] [0.5,0.5] [0.6,0.4] [0.7,0.3] [0.8,0.2] [0.9,0.1]"
seeds=`cat training_seeds`

for env in $envs; do
    for seed in $seeds; do
     for alg in $algs; do
        for steps in $timesteps; do
            for weight in $weights; do
                echo $weight
                if [[ "$weights" = "0" ]]; then
                    uid=$env.$alg.$steps.$seed
                    wflg=""
                else
                    wname=`python3 -c "print('$weight'.replace('[','').replace(']','').replace(',','').replace('.',''))"`
                    echo $wname
                    uid=$env.$wname.$alg.$steps.$seed
                    wflg="--weights=$weight"
                fi
                echo "Processing model $uid"
                model=$work1/$uid.`date -I`
                dest=mixed_env_scripts/run_cmd_$uid.sh
                logdir=$logs/$uid
                mkdir -p $logdir

                echo "Running on $partition. Command saved to $dest."

                cmd="#!/bin/bash
#
#SBATCH --job-name=$uid
#SBATCH --output=$uid.out
#SBATCH -e $uid.err
#SBATCH --mem=16g

unset PYTHONPATH
source gypsum.sh

OPENAI_LOGDIR=$logdir
OPENAI_FORMAT=csv 
./start_python -m baselines.run --alg=$alg --seed=$seed --env=$env --num_timesteps=$stepincr --save_path=$model.$stepincr.model $wflg
./start_python -m baselines.run --alg=$alg --seed=$seed --env=$env --num_timesteps=$stepincr --load_path=$model.$stepincr.model --save_path=$model.2e7.model $wflg
./start_python -m baselines.run --alg=$alg --seed=$seed --env=$env --num_timesteps=$stepincr --load_path=$model.2e7.model --save_path=$model.3e7.model $wflg
./start_python -m baselines.run --alg=$alg --seed=$seed --env=$env --num_timesteps=$stepincr --load_path=$model.3e7.model --save_path=$model.4e7.model $wflg
./start_python -m baselines.run --alg=$alg --seed=$seed --env=$env --num_timesteps=$stepincr --load_path=$model.4e7.model --save_path=$model.5e7.model $wflg
./start_python -m baselines.run --alg=$alg --seed=$seed --env=$env --num_timesteps=$stepincr --load_path=$model.5e7.model --save_path=$model.6e7.model $wflg
./start_python -m baselines.run --alg=$alg --seed=$seed --env=$env --num_timesteps=$stepincr --load_path=$model.6e7.model --save_path=$model.7e7.model $wflg
./start_python -m baselines.run --alg=$alg --seed=$seed --env=$env --num_timesteps=$stepincr --load_path=$model.7e7.model --save_path=$model.8e7.model $wflg
./start_python -m baselines.run --alg=$alg --seed=$seed --env=$env --num_timesteps=$stepincr --load_path=$model.8e7.model --save_path=$model.9e7.model $wflg
./start_python -m baselines.run --alg=$alg --seed=$seed --env=$env --num_timesteps=$stepincr --load_path=$model.9e7.model --save_path=$model.10e7.model $wflg"
                echo "$cmd"
                echo "$cmd" > $dest
                #sbatch -p $partition --gres=gpu:1 $dest
            done;
        done;
        done;
    done;
done
