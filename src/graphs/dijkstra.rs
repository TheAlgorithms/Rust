use std::io;

// #[allow(unused_macros)]
// macro_rules! read {
//     ($out:ident as $type:ty) => {
//         let mut inner = String::new();
//         io::stdin().read_line(&mut inner).expect("A String");
//         let $out = inner.trim().parse::<$type>().expect("Parseble");
//     };
// }

// #[allow(unused_macros)]
// macro_rules! read_str {
//     ($out:ident) => {
//         let mut inner = String::new();
//         io::stdin().read_line(&mut inner).expect("A String");
//         let $out = inner.trim();
//     };
// }

// #[allow(unused_macros)]
// macro_rules! read_vec {
//     ($out:ident as $type:ty) => {
//         let mut inner = String::new();
//         io::stdin().read_line(&mut inner).unwrap();
//         let $out = inner
//             .trim()
//             .split_whitespace()
//             .map(|s| s.parse::<$type>().unwrap())
//             .collect::<Vec<$type>>();
//     };
// }

const MAX_N: usize = 1000;
const INF: i32 = 100000000;

/*
n is the numbers of the node in graph
start is the start node
end is the end node
cost is 2-d vector. It is a adjacency matrix which store cost from i to j.assert_eq!

fn dijkstra output the min cost from start to end
*/
pub fn dijkstra(n:i32,start:i32,end:i32,cost:Vec<Vec<i32>>) -> i32{
 
    let mut vis:[i32;MAX_N] = [0;MAX_N];
    let mut pre:[i32;MAX_N] = [-1;MAX_N];
    let mut dis:[i32;MAX_N] = [INF;MAX_N];        
    
    
    // let mut cost = vec![];
    // let mut i:i32 = 0;
    // while i < x{
    //     read_vec!(v as i32);
    //     cost.push(v);
    //     i+=1;
    // }
    
    //println!("{:?}", cost); 

    dis[start as usize] = 0;
    let mut i:i32 = 0;

    while i < n{
        let mut j:i32 = 0;
        let mut k:i32 = -1;
        let mut min:i32 = INF;        
        while j < n{
            if vis[j as usize] == 0 && min > dis[j as usize]{
                min = dis[j as usize];
                k = j;
            }
            j+=1;
        }
        println!("MIN: {}",min);
        if k == -1{
            break;
        }
        vis[k as usize] = 1;
        
        j = 0;
        while j < n{
            if vis[j as usize] == 0 && dis[k as usize] + cost[k as usize][j as usize] < dis[j as usize] {
                dis[j as usize] = dis[k as usize] + cost[k as usize][j as usize];
                pre[j as usize] = k;               
            }
            j+=1;
        }
        i+=1;
    }
    dis[end as usize] 
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1(){
        assert_eq!(1,dijkstra(2, 0, 1, vec![vec![0, 1],
                                            vec![1, 0]]));
    }
    #[test]
    fn test2(){
        assert_eq!(3,dijkstra(3, 0, 2, vec![vec![0, 1, 100],   vec![1, 0, 2],   vec![100, 2, 0]]));
    }

}