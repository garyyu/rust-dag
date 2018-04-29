# rust-dag 
BlockDAG algorithms Rust language simulation.

BlockChain (for example Bitcoin, Etherum, etc.) is just a 'k=0' special subtype of BlockDAG, that's why they suffer from the highly restrictive throughput. DAG is the future!

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

To add a new example DAG to see the DAG blue selection behaviour, it's quite easy. For example, to test a DAG in this figure 'Fig.4', just add a piece of codes like this:
![Fig.4](https://github.com/garyyu/rust-dag/blob/master/pics/Fig.4.jpg)

```rust
    #[test]
    fn test_your_example() {

        let k: i32 = 3;

        let _ = env_logger::try_init();

        let node = Node::init("YourExampleDag");

        let mut node_w = node.write().unwrap();

        macro_rules! dag_add {
            ( block=$a:expr, references=$b:expr ) => (node_add_block($a, $b, &mut node_w, k, true));
        }
        dag_add!(block="Genesis", references=&Vec::new());

        dag_add!(block="B", references=&vec!["Genesis"]);
        dag_add!(block="C", references=&vec!["Genesis"]);
        dag_add!(block="D", references=&vec!["Genesis"]);
        dag_add!(block="E", references=&vec!["Genesis"]);

        dag_add!(block="F", references=&vec!["B","C"]);
        dag_add!(block="H", references=&vec!["E"]);
        dag_add!(block="I", references=&vec!["C","D"]);

        dag_add!(block="J", references=&vec!["F","D"]);
        dag_add!(block="K", references=&vec!["J","I","E"]);
        dag_add!(block="L", references=&vec!["F"]);
        dag_add!(block="N", references=&vec!["D","H"]);

        dag_add!(block="M", references=&vec!["L","K"]);
        dag_add!(block="O", references=&vec!["K"]);
        dag_add!(block="P", references=&vec!["K"]);
        dag_add!(block="Q", references=&vec!["N"]);

        dag_add!(block="R", references=&vec!["O","P","N"]);

        dag_add!(block="S", references=&vec!["Q"]);
        dag_add!(block="T", references=&vec!["S"]);
        dag_add!(block="U", references=&vec!["T"]);

        println!("{}", &node_w);

        dag_print(&node_w.dag);

        let blue_selection = dag_blue_print(&node_w.dag);
        println!("k={}, {}", k, &blue_selection);

        assert_eq!(2 + 2, 4);
    }
```

then run it by ```cargo test test_your_example -- --nocapture```
The output will be like this:

```console
running 1 test
node=fig4,height=7,size_of_dag=20,dag={Genesis,B,C,D,E,F,H,I,J,L,N,K,Q,M,O,P,S,R,T,U},tips={R,M,U}
dag={
 {name=Genesis,block=name=Genesis,height=0,size_of_past_set=0,size_of_past_blue=0,blue=1,prev={}}
 {name=B,block=name=B,height=1,size_of_past_set=1,size_of_past_blue=1,blue=1,prev={Genesis}}
 {name=C,block=name=C,height=1,size_of_past_set=1,size_of_past_blue=1,blue=1,prev={Genesis}}
 {name=D,block=name=D,height=1,size_of_past_set=1,size_of_past_blue=1,blue=1,prev={Genesis}}
 {name=E,block=name=E,height=1,size_of_past_set=1,size_of_past_blue=1,blue=0,prev={Genesis}}
 {name=F,block=name=F,height=2,size_of_past_set=3,size_of_past_blue=3,blue=1,prev={B,C}}
 {name=H,block=name=H,height=2,size_of_past_set=2,size_of_past_blue=1,blue=0,prev={E}}
 {name=I,block=name=I,height=2,size_of_past_set=3,size_of_past_blue=3,blue=1,prev={C,D}}
 {name=J,block=name=J,height=3,size_of_past_set=5,size_of_past_blue=5,blue=1,prev={F,D}}
 {name=L,block=name=L,height=3,size_of_past_set=4,size_of_past_blue=4,blue=0,prev={F}}
 {name=N,block=name=N,height=3,size_of_past_set=4,size_of_past_blue=2,blue=0,prev={H,D}}
 {name=K,block=name=K,height=4,size_of_past_set=8,size_of_past_blue=7,blue=1,prev={I,J,E}}
 {name=Q,block=name=Q,height=4,size_of_past_set=5,size_of_past_blue=2,blue=0,prev={N}}
 {name=M,block=name=M,height=5,size_of_past_set=10,size_of_past_blue=8,blue=1,prev={L,K}}
 {name=O,block=name=O,height=5,size_of_past_set=9,size_of_past_blue=8,blue=1,prev={K}}
 {name=P,block=name=P,height=5,size_of_past_set=9,size_of_past_blue=8,blue=1,prev={K}}
 {name=S,block=name=S,height=5,size_of_past_set=6,size_of_past_blue=2,blue=0,prev={Q}}
 {name=R,block=name=R,height=6,size_of_past_set=13,size_of_past_blue=10,blue=1,prev={O,N,P}}
 {name=T,block=name=T,height=6,size_of_past_set=7,size_of_past_blue=2,blue=0,prev={S}}
 {name=U,block=name=U,height=7,size_of_past_set=8,size_of_past_blue=2,blue=0,prev={T}}
}
k=3, blues={Genesis,B,C,D,F,I,J,K,M,O,P,R,} total=12/20
test tests::test_your_example ... ok
```

The following picture is another examples: Fig.3.

![Fig.3](https://github.com/garyyu/rust-dag/blob/master/pics/Fig.3.png)

Please join us the BlockDAG discussion on [https://godag.github.io](https://godag.github.io).





