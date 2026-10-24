buffer_size: 1024 * 1024

file_name: "measurements.txt"

count: 10_000_000

cities: ["Jaipur","Lucknow","Hisar","Amritsar","Chennai","Ahemdabad","Shillong","Gwalior","Raipur","Raygada"]

## Perf Boost 1 (328 ms)

Increased the buffer size & removing write calls ()

## Perf Boost 2 (273 ms)

added ';' at the end of all city names to reduce the write_all calls & using a lightweight random number generator
