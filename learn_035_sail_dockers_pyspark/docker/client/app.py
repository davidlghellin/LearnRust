from pyspark.sql import SparkSession
from pyspark.sql.functions import col, expr, lit
import random

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

# Crear DataFrame grande con 20.000 filas y 20 columnas
num_rows = 20000
num_cols = 20

# Generar filas como lista de tuplas
data = [
    tuple([i] + [random.randint(0, 100) for _ in range(num_cols - 1)])
    for i in range(num_rows)
]

# Crear nombres de columnas: col0 (clave), col1, col2, ..., col19
columns = [f"col{i}" for i in range(num_cols)]

df1 = spark.createDataFrame(data, schema=columns)
print("📦 df1 schema:")
df1.printSchema()

# Crear segunda tabla para hacer el join (clave col0)
# Por ejemplo: solo algunos ids coinciden
join_data = [(i, f"name_{i}") for i in range(0, num_rows, 1000)]
df2 = spark.createDataFrame(join_data, schema=["col0", "name"])

print("🔗 Haciendo join...")
joined = df1.join(df2, on="col0", how="left")

joined.sort(col("col9")).show(8)

spark.stop()
