module purge
module add cuda90 cudnn/7.3-cuda_9.0 slurm python3/3.6.6-1810 openmpi/gcc hdf5
module initadd cuda90 cudnn/7.3-cuda_9.0 slurm python3/3.6.6-1810 openmpi/gcc hdf5
virtualenv -p python3 venv
source venv/bin/activate
pip install tensorflow-gpu==1.11.0 tensorboard pygame cffi --only-binary=numpy gym==0.10.5 atari_py pillow joblib mpi4py six scipy opencv-python tqdm dill progressbar2 cloudpickle click
