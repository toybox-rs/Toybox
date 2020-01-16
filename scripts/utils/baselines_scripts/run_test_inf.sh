work1=/mnt/nfs/work1/jensen/kclary/ijcai20
logs=$work1/logs

mkdir -p $logs

partition="titanx-long"

models=`cat inf_models.txt`
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

for model in $models; do
	minfo=$(echo $model | tr "." "\n")
	minfo=($minfo)
	env=${minfo[0]}
	alg=${minfo[1]}
	steps=${minfo[2]}
	seed=${minfo[3]}
	key=${minfo[4]}
    dt=${minfo[5]}

    uid=$env.$alg.$steps.$seed.$key

    echo "Processing model $uid"
	dest=run_cmd_$uid.sh
    echo "Running on $partition. Command saved to $dest."

    logdir=$logs/$uid
    mkdir -p $logdir

    cmd="#!/bin/bash
#
#SBATCH --job-name=$uid
#SBATCH --output=$uid.out
#SBATCH -e $uid.err
#SBATCH --mem=16g"

        for xy_pair in $xys; do
            xy=$(echo $xy_pair | tr "," " ")
            xy=($xy)
            x=${xy[0]}
            y=${xy[1]} 
            
            conf=trial_config_json/trial_config_x_${x}_y_${y}.json
        
        cmd="$cmd
OPENAI_LOGDIR=$logdir OPENAI_FORMAT=csv ./start_python_gypsum -m baselines.run --alg=$alg --env=$env --num_timesteps=0 --num_env=1 --load_path=$work1/$model --partial_config=$conf --play"
            done;
	    echo "$cmd"
        echo "$cmd" > $dest
        sbatch -p $partition --gres=gpu:1 $dest
done
