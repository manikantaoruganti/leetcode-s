class PeekingIterator:public Iterator{
int nxt;
bool peeked=0;
public:
PeekingIterator(const vector<int>&nums):Iterator(nums){}

int peek(){
if(!peeked){
nxt=Iterator::next();
peeked=1;
}
return nxt;
}

int next(){
if(peeked){
peeked=0;
return nxt;
}
return Iterator::next();
}

bool hasNext()const{
return peeked||Iterator::hasNext();
}
};
