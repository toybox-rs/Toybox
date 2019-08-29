from setuptools import setup, find_packages

toybox_py_version = '0.1.0'
toybox_cffi_version = '0.1.0'

dependencies = ["toybox_cffi=={0}".format(toybox_cffi_version)]
with open('requirements.txt', 'r') as fp:
    dependencies = [line.strip() for line in fp.readlines()]

setup(
    name='toybox_api',
    version=toybox_py_version,
    author="John Foley",
    author_email="jjfoley@smith.edu",
    classifiers=[
        "Programming Language :: Python :: 3.5",
        "Operating System :: OS Independent",
    ],
    description="Core API for interacting with Toybox RL environments.",
    url="https://github.com/KDL-umass/Toybox",
    packages=find_packages(),
    install_requires=dependencies,
    platforms='any',
)
