work1=/mnt/nfs/work1/jensen/kclary/ijcai20
logs=$work1/logs

mkdir -p $logs

partition="titanx-long"

algs="acktr a2c ppo2"
env="AmidarToyboxNoFrameskip-v4"
timesteps="1e7 5e7"
#seeds=`cat training_seeds`
seeds="2202933152"

# make sure we have all the pip dependencies we want installed
pip3 install 'gym==0.10.5' --user
pip3 install 'tensorboard<1.12.0,>=1.11.0' --user
pip3 uninstall atari-py --user
pip3 install 'atari-py>=0.1.1,<0.1.2' --user
pip3 install tensorflow-gpu --user
curl https://sh.rustup.rs -sSf > install_rust.sh
chmod +x install_rust.sh
./install_rust.sh -y
source $HOME/.cargo/env
rustup default stable
cargo build --release

for steps in $timesteps; do
    for seed in $seeds; do
        for alg in $algs; do

            uid=$env.$alg.$steps.$seed.infstart
            echo "Processing model $uid"

            dest=run_cmd_$uid.sh
            conf=toybox/toybox/intervention/defaults/amidar_quick_general_config.json
            model=$uid.`date -I`.model
            model_path=$work1/$model
   
            if ! ls $work1/$uid*.model 1> /dev/null 2>&1; then
                logdir=$logs/$uid
                mkdir -p $logdir

                echo "Running on $partition. Command saved to $dest."

                cmd="#!/bin/bash
#
#SBATCH --job-name=$uid
#SBATCH --output=$uid.out
#SBATCH -e $uid.err
#SBATCH --mem=16g

OPENAI_LOGDIR=$logdir OPENAI_FORMAT=csv ./start_python_gypsum -m baselines.run --alg=$alg --env=$env --seed=$seed --num_timesteps=$steps --save_path=$model_path --partial_config=$conf --rogue"
                echo "$cmd"
                echo "$cmd" > $dest
                sbatch -p $partition --gres=gpu:1 $dest
            fi 
        done;
    done;
done
