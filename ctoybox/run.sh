envs="toybox-amidar-v0 AmidarNoFrameskip-v0 toybox-breakout-v0 BreakoutNoFrameskip-v0"
algs="acer acktr a2c ppo2 deepq"
timesteps="1e7 3e7 5e7"
work1=/mnt/nfs/work1/jensen/etosch/issta
logs=$work1/logs

mkdir -p $logs

partition="titanx-long"


envs="BreakoutToyboxNoFrameskip-v4"
algs="ppo2"
timesteps="1e7"
weights="[1.0,0.0] [0.5,0.5] [0.0,1.0]"


# make sure we have all the pip dependencies we want installed
pip3 install gym[atari] --user
pip3 install 'tensorboard<1.8.0,>=1.7.0' --user
curl https://sh.rustup.rs -sSf > install_rust.sh
chmod +x install_rust.sh
./install_rust.sh -y
source $HOME/.cargo/env
rustup default stable
cargo build --release 

for env in $envs; do
    for alg in $algs; do 
	for steps in $timesteps; do
	    for weight in $weights; do
		model=$work1/$env.$alg.$steps.`date -I`.$weight.model
		uid=$env.$alg.$steps.$weight
		dest=run_cmd_$uid.sh
		logdir=$logs/$uid

		mkdir -p $logdir
		
		echo "Running on $partition. Command saved to $dest."
		
		cmd="#!/bin/bash
#
#SBATCH --job-name=$uid
#SBATCH --output=$uid.out
#SBATCH -e $uid.err
#SBATCH --mem=16g

OPENAI_LOGDIR=$logdir OPENAI_LOG_FORMAT=csv ./start_python -m baselines.run --alg=$alg --env=$env --num_timesteps=$steps --save_path=$model --weights=$weight" 
	    echo "$cmd"
	    echo "$cmd" > $dest
	    sbatch -p $partition --gres=gpu:1 $dest
	    done;
	done;
    done;
done
