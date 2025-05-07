FROM python:3.11-slim

WORKDIR /app

COPY client.py .

RUN pip install --no-cache-dir pyspark pandas pyarrow grpcio grpcio-status protobuf

