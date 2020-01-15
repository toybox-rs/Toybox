work1=/mnt/nfs/work1/jensen/kclary/ijcai20
logs=$work1/logs

mkdir -p $logs

partition="titanx-long"

algs="ppo2"
env="amidartoyboxnoframeskip-v4"
timesteps="1e7 5e7"
configs=`cat block_configs.txt`
seeds="2202933152"

# make sure we have all the pip dependencies we want installed
pip3 install 'gym==0.10.5' --user
pip3 install 'tensorboard<1.12.0,>=1.11.0' --user
pip3 uninstall atari-py --user
pip3 install 'atari-py>=0.1.1,<0.1.2' --user
pip3 install tensorflow-gpu --user
curl https://sh.rustup.rs -ssf > install_rust.sh
chmod +x install_rust.sh
./install_rust.sh -y
source $home/.cargo/env
rustup default stable
cargo build --release

for steps in $timesteps; do
    for seed in $seeds; do
        for alg in $algs; do
            for fconfig in $configs; do

                uid=$env.$alg.$steps.$seed.config
                echo "processing model $uid"

                dest=run_cmd_$uid.sh
                conf=toybox/toybox/interventions/defaults/$fconfig
                echo "$conf"
                model=$uid.`date -i`.model
                model_path=$work1/$model
       
                if ! ls $work1/$uid*.model 1> /dev/null 2>&1; then
                    logdir=$logs/$uid
                    mkdir -p $logdir

                    echo "running on $partition. command saved to $dest."

                    cmd="#!/bin/bash
#
#sbatch --job-name=$uid
#sbatch --output=$uid.out
#sbatch -e $uid.err
#sbatch --mem=16g

openai_logdir=$logdir openai_format=csv ./start_python_gypsum -m baselines.run --alg=$alg --env=$env --seed=$seed --num_timesteps=$steps --save_path=$model_path --partial_config=$conf --rogue"
                    echo "$cmd"
                    echo "$cmd" > $dest
                    sbatch -p $partition --gres=gpu:1 $dest
                fi
            done; 
        done;
    done;
done
