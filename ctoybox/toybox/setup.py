# From https://github.com/getsentry/milksnake
from setuptools import setup

def build_native(spec):
    # build an example rust library
    build = spec.add_external_build(
        cmd=['cargo', 'build', '--release'],
        path='.'
    )

    spec.add_cffi_module(
        module_path='toybox._native',
        dylib=lambda: build.find_dylib('ctoybox', in_path='../../target/release'),
        header_filename=lambda: build.find_header('ctoybox.h', in_path='../../target'),
        rtld_flags=['NOW', 'NODELETE']
    )

setup(
    name='toybox',
    version='0.0.1',
    packages=['toybox', 'toybox.envs', 'toybox.interventions', 'toybox.sample_tests'],
    zip_safe=False,
    platforms='any',
    setup_requires=['milksnake'],
    install_requires=['milksnake'],
    milksnake_tasks=[
        build_native
    ]
)
