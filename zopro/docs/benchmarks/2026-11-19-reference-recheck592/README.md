# Repeating the warm 12×2 reference comparison

Each row measures the reference request in the workload fixed before the run. The mixed workload changes request on every call, using the same requests and ordering for the flat and early settings. The ordinary SDK and streaming repeated arms use the same original request throughout. All 742 exports verified independently and all 980 measured calls returned a verified proof.

| Setting | Median process mean ms | Seven process means, ms | Processes at 28.4 ms | Individual calls at 28.4 ms | M5 CPU / Metal |
|---|---:|---|---:|---:|---:|
| sdk-repeat | 28.313 | 29.013 / 28.300 / 28.313 / 28.256 / 28.323 / 28.262 / 28.582 | 5/7 | 137/245 | 10.03× / 3.57× |
| stream-repeat | 28.339 | 28.283 / 29.018 / 28.330 / 28.471 / 28.413 / 28.339 / 28.069 | 4/7 | 141/245 | 10.02× / 3.56× |
| stream-mixed | 28.277 | 27.779 / 28.011 / 28.964 / 28.277 / 28.092 / 28.452 / 28.310 | 5/7 | 32/49 | 10.04× / 3.57× |
| stream-flat-mixed | 28.388 | 28.575 / 29.859 / 28.388 / 28.317 / 28.684 / 28.336 / 28.131 | 4/7 | 33/49 | 10.00× / 3.56× |

[Protocol](protocol.md) · [Samples](artifacts/samples.json.gz) · [Verification](artifacts/verification.json)
