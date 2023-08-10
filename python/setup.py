import pathlib
from setuptools import setup

# The directory containing this file
HERE = pathlib.Path(__file__).parent

# This call to setup() does all the work
setup(
    name="leet",
    version="0.0.1",
    description="Leet solutions for python",
    long_description="Leet solutions for python",
    long_description_content_type="text/markdown",
    url="https://github.com/shaunryan/python_leet",
    project_urls={
        "GitHub": "https://github.com/shaunryan/python_leet",
        "Documentation": "https://github.com/shaunryan/python_leet",
    },
    author="Shaun Ryan",
    author_email="shaun_chiburi@hotmail.com",
    license="MIT",
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.11",
    ],
    packages=[
        "leet",
        "leet.easy"

    ],
    install_requires=[],
    zip_safe=False
)
