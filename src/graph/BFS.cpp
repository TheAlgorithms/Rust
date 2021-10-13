#include<bits/stdc++.h> 
#include<ext/pb_ds/assoc_container.hpp>
#include<ext/pb_ds/tree_policy.hpp>
#pragma GCC optimize("Ofast")
#pragma GCC target("sse,sse2,sse3,ssse3,sse4,popcnt,abm,mmx,avx,avx2,fma")
#pragma GCC optimize("unroll-loops")
const unsigned int M = 1000000007;
using namespace std;
// Check
using namespace __gnu_pbds;
typedef tree<int,null_type,less<int>,rb_tree_tag,tree_order_statistics_node_update> T_set; // PBDS_set
typedef tree<int,null_type,less_equal<int>,rb_tree_tag,tree_order_statistics_node_update> T_multiset; // PBDS_multiset

void solve()
{
    int n ,m,u,v;
    cin>>n>>m;
    vector<list<int>> adj(n+1);
    vector<bool> vis(n+1,false);
    for(int i = 0; i < n ; i++ ){
        cin>>u>>v;
        adj[u].push_back(v);
        adj[v].push_back(u);
    }
    queue<int> temp;
    temp.push(1);
    vis[1] = true;
    while(!temp.empty()){
        int curr = temp.front();
        cout<<curr<<" ";
        temp.pop();
        for(int elem : adj[curr]){
            if(!vis[elem]){
                temp.push(elem);
                vis[elem] = true;
             }
        }
    }

}
int main()
{
ios_base::sync_with_stdio(false);
cout.tie(NULL);
cin.tie(NULL);
solve();
return 0;
}
