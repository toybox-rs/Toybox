from setuptools import setup, find_packages

toybox_api_version = '0.2.0-dev'
toybox_envs_version = '0.2.0-dev'

packages = """
gym
atari_py
""".strip()

dependencies = ["toybox_api=={0}".format(toybox_api_version)]
dependencies.extend([line.strip() for line in packages.split("\n")])

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
