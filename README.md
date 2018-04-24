# rust-dag 
BlockDAG algorithms Rust language simulation.

BlockChain (for example Bitcoin, Etherum, etc.) is just a 'k=0' special subtype of BlockDAG, that's why they have the highly restrictive throughput. DAG is the future!

---

# How to build

Run the simulation for the example Fig.3, the 'k' is 3 in the example.

```bash
$ cargo test test_fig3 -- --nocapture
```

Run the simulation for the example Fig.4, the 'k' is 3 in the example.

```bash
$ cargo test test_fig4 -- --nocapture
```

Run the simulation for the example Fig.X1 or Fig.X2, the 'k' is set to 0 in these 2 examples.

```bash
$ cargo test test_fig_x1 -- --nocapture
$ cargo test test_fig_x2 -- --nocapture
```

Run the simulation for the example of generating 1000 random blocks, and execute the blue selection in a real-time calculation.

```bash
$ cargo test test_add_block -- --nocapture
```

The following is 3 figures of example Fig.3, Fig.4 and Fig.X1.

![Fig.3](https://github.com/garyyu/rust-dag/pics/Fig.3.png)

![Fig.4](https://github.com/garyyu/rust-dag/pics/Fig.4.jpg)

![Fig.X1](https://github.com/garyyu/rust-dag/pics/Fig.X1.jpg)



