work1=/mnt/nfs/work1/jensen/kclary/ijcai20
logs=$work1/logs

mkdir -p $logs

partition="titanx-long"

#algs="acktr a2c ppo2"
algs="ppo2"
env="AmidarToyboxNoFrameskip-v4"
#timesteps="1e7 5e7"
timesteps="5e7"
#seeds=`cat training_seeds`
seeds="2202933152"
xys=`cat ../tb_amidar/src/resources/tileset`

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

job_ct=0
for steps in $timesteps; do
    for seed in $seeds; do
        for alg in $algs; do
            for xy_pair in $xys; do
                xy=$(echo $xy_pair | tr "," " ")
                xy=($xy)
                x=${xy[0]}
                y=${xy[1]}

                uid=$env.$alg.$steps.$seed.$x.$y
                echo "Processing model $uid"

                dest=run_cmd_$uid.sh
                conf=trial_config_x_${x}_y_${y}.json
                model=$uid.`date -I`.model
                model_path=$work1/$model
       
                if ! ls $work1/$uid*.model 1> /dev/null 2>&1; then
                    job_ct=$(($job_ct +  1)) 
                    
                    if [ $job_ct -ge 0 ] && [ $job_ct -lt 113 ]; then 
                        logdir=$logs/$uid
                        mkdir -p $logdir

                        echo "Running on $partition. Command saved to $dest."

                        trial_config="{
        \"player_start\": {
        \"tx\": 31,
        \"ty\": 15
    },
    \"start_jumps\": 0,
    \"default_board_bugs\": false,
    \"no_chase\": true,
    \"randomize\": {
        \"player_start\": {
            \"comb_list\": {
                \"xrange\": ["$x"],
                \"yrange\": ["$y"]
            }
        }

    },
    \"_comment\": \"Amidar quick config for removing nonrandom noise from training. Modifications: unpaint initial segment to avoid implementation headches; no chase mode; no jump mode; set starting player position from set\"}"

                        echo "$trial_config"
                        echo "$trial_config" > $conf


                        cmd="#!/bin/bash
#
#SBATCH --job-name=$uid
#SBATCH --output=$uid.out
#SBATCH -e $uid.err
#SBATCH --mem=16g

OPENAI_LOGDIR=$logdir OPENAI_FORMAT=csv ./start_python_gypsum -m baselines.run --alg=$alg --env=$env --seed=$seed --num_timesteps=$steps --save_path=$model_path --partial_config=$conf "
                        echo "$cmd"
                        echo "$cmd" > $dest
                        sbatch -p $partition --gres=gpu:1 $dest
                    fi
                fi 
            done;
        done;
    done;
done
