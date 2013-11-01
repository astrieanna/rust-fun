extern mod extra;
extern mod std;
use std::rand;

fn sqr(x:int) -> int {
	return x*x;
}


fn pmap(fun: extern fn(~str) -> uint, myvect:~[~str]) {
        let ports = for initval in myvect.iter() {
          let (pport, cchan) = stream();
          do spawn {
	    cchan.send(fun(*initval))
          }
          pport
        };
       
        for port in ports.iter() {
          println("working on parent1");
          loop {
            match port.try_recv() {
             Some(v) => println(format!("{:u}",v)),
             None => break
            }
          }

        }
}

fn get_length(x:~str) -> uint {
	return x.len();
}

fn main() {
	let myvect: ~[~str] = ~[~"Alex", ~"Julia", ~"Yaron"];
	pmap(get_length, myvect);
}

