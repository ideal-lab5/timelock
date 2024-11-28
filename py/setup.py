from setuptools import setup, find_packages

setup(
    name="timelock",
    version="0.1.0",
    packages=find_packages(),
    include_package_data=True,
    package_data={"": ["wasm/*.wasm"]},
    install_requires=["wasmer"],
)
