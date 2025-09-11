class NestedIterator{
stack<vector<NestedInteger>::const_iterator>beg,end;
public:
NestedIterator(vector<NestedInteger>&nestedList){
beg.push(nestedList.begin());
end.push(nestedList.end());
}
int next(){
return(beg.top()++)->getInteger();
}
bool hasNext(){
while(!beg.empty()){
if(beg.top()==end.top()){
beg.pop();
end.pop();
}else{
auto cur=beg.top();
if(cur->isInteger())return true;
beg.top()++;
beg.push(cur->getList().begin());
end.push(cur->getList().end());
}
}
return false;
}
};
