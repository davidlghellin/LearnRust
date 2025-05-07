from pyspark.sql import SparkSession

spark = SparkSession.builder.remote("sc://sail-server:50051").getOrCreate()
print("✅ Conectado a Spark:", spark.version)

df = spark.range(20)
df.show()
df.explain(True)
df.explain(mode="formatted")

df = spark.read.option("header", True).csv("file:///stage/in.csv")

df.show()

df2 = spark.read.option("header", True).csv("/stage/in.csv")
df2.show()

