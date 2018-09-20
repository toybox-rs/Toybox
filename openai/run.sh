envs="toybox-amidar-v0 AmidarNoFrameskip-v0 toybox-breakout-v0 BreakoutNoFrameskip-v0"
algs="ppo2 acer a2c trpo_mpi deepq"
timesteps="3e6 1e7"
work1=/mnt/nfs/work1/jensen/etosch

# make sure we have all the pip dependencies we want installed
pip3 install gym[atari] --user
pip3 install 'tensorboard<1.8.0,>=1.7.0' --user

# Run for 3e6 on titanx-short
# Run for 1e7 on titanx-long

for steps in $timesteps; do
    for alg in $algs; do 
	for env in $envs; do
	    if [[ "`echo $env | cut -d'-' -f1`" = "toybox" ]]; then
		runner="toybox_baselines.py"
	    else
		runner="ale_baselines.py"
	    fi

	    model=$work1/$env.$alg.$steps.model
	    
	    if [[ "$steps" = "3e6" ]]; then 
		partition="titanx-short"
	    else
		partition="titanx-long"
	    fi 
	    uid=$env.$alg.$steps
	    dest=run_cmd_$uid.sbatch

	    echo "Running on $partition. Command saved to $dest."

	    cmd="#!/bin/bash
#
#SBATCH --job-name=$uid
#SBATCH --output=$uid.out
#SBATCH -e $uid.err
#SBATCH --mem=16g


LD_LIBRARY_PATH=$LD_LIBRARY_PATH:~/toybox/openai/target/release ./start_python $runner --alg=$alg --env=$env --num_timesteps=$steps --save_path=$model" 
	    echo "$cmd"
	    echo "$cmd" > $dest
	    sbatch -p $partition --gres=gpu:1 $dest
	done;
    done;
done
