# Rust + Python

Vamos a hacer un word count usando la tupla Rust y Python

Parece que falla algo, para ello haremos al crear el contaier:
```sh
sudo apt update
sudo apt-get install python3-venv


python3 -m venv .env
source .env/bin/activate
pip install maturin
maturin init #pio3
# hacemos los ficheros
maturin develop
python3 examples/example.py
```
