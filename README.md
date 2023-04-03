# Multithreading in Rust
Crates: crossterm, tokio, num_cpus, rand, reqwest

Description: multithread application sample implemented using tokio (parallel job execution). Every task accesses concurrently to a shared queue, makes a pop and simulates a job processing. 


# Diagram
![multithread](multithread.png)


# Screenshot

![Screenshot from 2023-04-03 23-42-45](https://user-images.githubusercontent.com/6343630/229634195-96923f58-79b3-4814-8ee2-fbb73b45da89.png)

# htop

![Screenshot from 2023-04-03 23-40-49](https://user-images.githubusercontent.com/6343630/229634234-85164d2e-4bd9-4ea8-836b-0997f373319e.png)

