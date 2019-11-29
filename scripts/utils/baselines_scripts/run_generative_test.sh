unset PYTHONPATH
module load cudnn/7.3-cuda_9.0

work1=/mnt/nfs/work1/jensen/kclary/NeurIPS_2019
e_work1=/mnt/nfs/work1/jensen/etosch/neurips/
logs=$work1/ijcai20/logs

mkdir -p $logs

partition="titanx-long"

models=`cat neurips_models_amidar`
xs=$(seq 0 33);
ys=$(seq 0 33);


# make sure we have all the pip dependencies we want installed
pip3 install 'gym==0.10.5' --user
pip3 install 'tensorboard<1.8.0,>=1.7.0' --user
pip3 uninstall atari-py --user
pip3 install 'atari-py>=0.1.1,<0.1.2' --user
curl https://sh.rustup.rs -sSf > install_rust.sh
chmod +x install_rust.sh
./install_rust.sh -y
source $HOME/.cargo/env
rustup default stable
cargo build --release

for model in $models; do
	minfo=$(echo $model | tr "." "\n")
	minfo=($minfo)
	env=${minfo[0]}
	alg=${minfo[1]}
	nsteps=${minfo[2]}
	seed=${minfo[3]}
	dt=${minfo[4]}

    for x in $xs; do 
    	for y in $ys; do 
    		uid=$env.$alg.$steps.$seed.$x.$y
            echo "Processing model $uid"

            dest=run_cmd_$uid.sh
            conf=trial_config_x_${x}_y_${y}.json

            logdir=$logs/$uid
            mkdir -p $logdir

            echo "Running on $partition. Command saved to $dest."

            trial_config="{
	'player_start': {
        'tx': 31,
        'ty': 15
    },
    'start_jumps': 0,
    'default_board_bugs': false,
    'no_chase': true,
    'randomize': {
        'player_start': {
            'comb_list': {
                'xrange': [$x],
                'yrange': [$y]
            }
        }

    },
    '_comment': 'Amidar quick config for removing nonrandom noise from training. Modifications: unpaint initial segment to avoid implementation headches; no chase mode; no jump mode; set starting player position from set'}"
                        
            echo "$trial_config"
            echo "$trial_config" > $conf


            cmd="#!/bin/bash
#
#SBATCH --job-name=$uid
#SBATCH --output=$uid.out
#SBATCH -e $uid.err
#SBATCH --mem=16g

OPENAI_LOGDIR=$logdir OPENAI_FORMAT=csv ./start_python -m baselines.run --alg=$alg --seed=$seed --env=$env --num_timesteps=0 --load_path=$e_work1/$model"
		    echo "$cmd"
		    #echo "$cmd" > $dest
		    #sbatch -p $partition --gres=gpu:1 $dest
        done;
    done;
done