extern mod extra;
extern mod std;
use std::rand;

fn sqr(x:int) -> int {
	return x*x;
}


fn pmap(fun: extern fn(~str) -> uint, myvect:~[~str]) {
        let ports = do myvect.move_iter().map |s| { 
          let (pport, cchan) = stream();
          do spawn {
	    cchan.send(fun(s))
          }
          pport
        }.to_owned_vec();
       
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

