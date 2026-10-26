buffer_size: 1024 * 1024

file_name: "measurements.txt"

count: 10_000_000

cities: ["Jaipur","Lucknow","Hisar","Amritsar","Chennai","Ahemdabad","Shillong","Gwalior","Raipur","Raygada"]

## Perf Boost 1 (328 ms)

Increased the buffer size & removing write calls ()

## Perf Boost 2 (273 ms)

added ';' at the end of all city names to reduce the write_all calls & using a lightweight random number generator

## Perf Boost 3 (312 ms) 

compiler flags and reduced buffer size to fit into L2 cache

## Perf Boost 4 (220 ms)

Pre-formatted temperatures into a lookup table, 
batched buffer writes via raw pointers, 
and split a single u64 RNG call per row.
