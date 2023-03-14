maturin build -r -o dist
pip3 install dist/*.whl --force-reinstall 
rm -rf assets
cp -r ../assets assets
