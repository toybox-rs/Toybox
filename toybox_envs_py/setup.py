from setuptools import setup, find_packages

toybox_api_version = '0.1.0'
toybox_envs_version = '0.1.0'

dependencies = ["toybox_api=={0}".format(toybox_api_version)]
with open('requirements.txt', 'r') as fp:
    dependencies = [line.strip() for line in fp.readlines()]

setup(
    name='toybox-envs',
    version=toybox_envs_version,
    author="John Foley",
    author_email="jjfoley@smith.edu",
    classifiers=[
        "Programming Language :: Python :: 3.5",
        "Operating System :: OS Independent",
    ],
    description="OpenAI Gym API for interacting with Toybox RL environments.",
    url="https://github.com/KDL-umass/Toybox",
    packages=find_packages(),
    install_requires=dependencies,
    platforms='any',
)
