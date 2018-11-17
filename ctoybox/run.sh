envs="toybox-amidar-v0 AmidarNoFrameskip-v0 toybox-breakout-v0 BreakoutNoFrameskip-v0"
algs="acer acktr a2c ppo2 deepq"
timesteps="1e7 3e7 5e7"
work1=/mnt/nfs/work1/jensen/etosch

envs="BreakoutToyboxNoFrameskip-v0"
partition="titanx-long"

# make sure we have all the pip dependencies we want installed
pip3 install gym[atari] --user
pip3 install 'tensorboard<1.8.0,>=1.7.0' --user


for env in $envs; do
    for alg in $algs; do 
	for steps in $timesteps; do
	    model=$work1/$env.$alg.$steps.model
	    uid=$env.$alg.$steps
	    dest=run_cmd_$uid.sh

	    echo "Running on $partition. Command saved to $dest."

	    cmd="#!/bin/bash
#
#SBATCH --job-name=$uid
#SBATCH --output=$uid.out
#SBATCH -e $uid.err
#SBATCH --mem=16g


LD_LIBRARY_PATH=$LD_LIBRARY_PATH:~/toybox/ctoybox/target/release ./start_python -m baselines.run --alg=$alg --env=$env --num_timesteps=$steps --save_path=$model" 
	    echo "$cmd"
	    echo "$cmd" > $dest
	    sbatch -p $partition --gres=gpu:1 $dest
	done;
    done;
done
