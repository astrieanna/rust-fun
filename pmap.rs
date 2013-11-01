extern mod extra;
extern mod std;
use std::rand;

fn sqr(x:int) -> int {
	return x*x;
}


fn pmap(fun: extern fn(~str) -> uint, myvect:~[~str]) {

        let ports_chans = do std::vec::from_fn(3) |init_val| {
          let (pport, cchan) = stream();
          let (cport, pchan) = stream();
          do spawn {
		loop {
                        match cport.try_recv() {
			    Some(s) => cchan.send(fun(s)),
                            None => break
                        }
		}
          }
          (pport,pchan)
        };
       
        do spawn {
	for iptr in myvect.iter() {
		let s = (*iptr).to_owned();
		let t = s.clone();
                let i = rand::random::<uint>();
	
                match ports_chans[i % 2] {
                  (_,chan) => chan.send(s)
                }	
	}
        }

        for &(port,_) in ports_chans.iter() {

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

