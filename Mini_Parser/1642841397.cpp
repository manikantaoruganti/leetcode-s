
class Solution{
public:
NestedInteger deserialize(string s){
if(s.empty())return NestedInteger();
if(s[0]!='[')return NestedInteger(stoi(s));
stack<NestedInteger>st;
int num=0;bool neg=false,numStarted=false;
for(int i=0;i<(int)s.size();i++){
char c=s[i];
if(c=='[')st.push(NestedInteger());
else if(c==']'){
if(numStarted){
if(neg)num=-num;
st.top().add(NestedInteger(num));
num=0;neg=false;numStarted=false;
}
if(st.size()>1){
auto ni=st.top();st.pop();
st.top().add(ni);
}
}
else if(c==','){
if(numStarted){
if(neg)num=-num;
st.top().add(NestedInteger(num));
num=0;neg=false;numStarted=false;
}
}
else if(c=='-')neg=true;
else{
num=num*10+(c-'0');
numStarted=true;
}
}
return st.empty()?NestedInteger():st.top();
}
};
