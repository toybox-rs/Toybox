algs="deepq acer acktr a2c ppo2"
work1=/mnt/nfs/work1/jensen/etosch/issta
logs=$work1/logs

mkdir -p $logs

partition="titanx-long"


envs="AmidarToyboxNoFrameskip-v4 SpaceInvadersToyboxNoFrameskip-v4"
timesteps="1e7 5e7"
weights=0
seeds=`cat training_seeds`


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
    for seed in $seeds; do 
        for alg in $algs; do 
	    for steps in $timesteps; do
	        for weight in $weights; do
		    if [ "$weights" -eq "0" ]; then 
		        uid=$env.$alg.$steps.$seed
			wflg=""
		    else 
		        wname=`python3 -c "print('$weight'.replace('[','').replace(']','').replace(',','').replace('.',''))"`		    
		        uid=$env.$alg.$steps.$seed.$wname
			wflg="--weights=$weight"
		    fi
                    echo "Processing model $uid"
		    model=$work1/$uid.`date -I`.model
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

OPENAI_LOGDIR=$logdir OPENAI_FORMAT=csv ./start_python -m baselines.run --alg=$alg --seed=$seed --env=$env --num_timesteps=$steps --save_path=$model $wflg" 
		    echo "$cmd"
		    echo "$cmd" > $dest
		    sbatch -p $partition --gres=gpu:1 $dest
	        done;
	    done;
        done;
    done;
done
