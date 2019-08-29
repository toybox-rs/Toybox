#!/bin/bash

set -eu

rm -rf dist/
pip install --upgrade twine
python3 setup.py sdist bdist_wheel
python3 -m twine upload dist/*
