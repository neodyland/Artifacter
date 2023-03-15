source ./.venv/bin/activate
maturin build -r -o dist
pip3 install dist/*.whl --force-reinstall 