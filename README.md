# What is Archon?

Archon is an experimental Rust application focused on the concepts of Rust's security, async, multithreading, networking etc.

This application doesn't have a fixed development schedule or planning whatsoever. It serves the following purposes:

1. Learning about the above mentioned concepts of advanced Rust: security, async, multithreading, networking;
2. Getting used to using Rust quickly and efficiently;
3. Learning about both offensive and defensive security in a fun way that's practical and engaging;
4. In the future, this may evolve into a focused application that has a specific purpose and does something useful, but in the initial stages this is just an educational experiment.

# Current priorities:

1. [DONE] Implement a basic HTTP request client that gets the HTTP get request result: response's status and body.
2. [WE ARE HERE] Implement an ability to HTTP GET several URLs (i.e. from a file) instead of just one. The group of URL can be fetched e.g. from a file.
3. Parallelize: each URL should be requested in a separate thread. Functional approach preferred. Use threadpool or similar.
4. Instead of just HTTP GET, scan which ports are open on the target URL. Also in parallel.
5. Figure out how to make the port scanner async.

# Licensing

This is published under The Unlicense. This means it's public domain and you can do whatever you want with it.