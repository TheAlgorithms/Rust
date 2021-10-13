//Objective is to find the number of subsets that can have the sum as target.
/*
  Lets take an example to understand it much better.
  Consider array given as arr[]={2,3,6,7};
  And target as 7.
  So possible combinations are: 
  1. [2,2,3]
  2. [7]
  So only 2 subsets are possible.
  
  NOTE -> We can choose a number any number of times i.e. Repetition is allowed.
  
  To solve such problems recursion is the best approach to go for.Now for every index we have two options 
  1. Choose the index and decrease the sum, and next time a recusrive call is made start from the same indx.
  2. Move ahead with the next index and try out the possible combinations by not including the curr element.
  
  We will need a data structure that can store the sum at every recursive call. 
  So i have made use of a vector. You can choose whatever feels right to you!!.

  So lets move on to code :)
*/
#include<bits/stdc++.h>
using namespace std;

void helper(vector<int>&arr,vector<vector<int>>&ans,vector<int>temp,int target,int start_index,int curr_sum){

        if(target == curr_sum){ans.push_back(temp);return;}  // If we find current sum to be equal to target then we can push that subset into our 'ans' array.
        if(start_index>=arr.size()){return ;}      //Base case1 - if current postion exceeds the array size then reutrn.
        if(curr_sum>target){return ;}    // Base case2- if at ith postion we see that the value of curr_sum exceeds then target then their is no meaning in moving ahead.
        temp.push_back(arr[start_index]);
        helper(arr,ans,temp,target,start_index,curr_sum+arr[start_index]); // Recursive call with current element included
        temp.pop_back();  // popping out the element
        helper(arr,ans,temp,target,start_index+1,curr_sum); //Backtrack.
}

int main(){

  vector<int>arr{2,3,6,7};
  int n=arr.size(),target=7,curr_sum=0,start_index=0,i,j;
  vector<vector<int>>ans; // To store all of the subsets.
  vector<int>temp;        // To store possible numbers that can form a subset.
  helper(arr,ans,temp,target,start_index,curr_sum);   // A helper function for recursive calls.

  cout<<"Subsets with sum as "<<target<<" are:"<<endl;
  for(i=0;i<ans.size();i++){
    for(j=0;j<ans[i].size();j++){
      cout<<ans[i][j]<<" ";
    }
    cout<<endl;
  }

}
