
/**
 * Your OrderedStream object will be instantiated and called as such:
 * OrderedStream* obj = new OrderedStream(n);
 * vector<string> param_1 = obj->insert(idKey,value);
 */class OrderedStream {
    vector<string> stream;
    int ptr;
public:
    OrderedStream(int n) {
        stream = vector<string>(n);
        ptr = 0;
    }

    vector<string> insert(int idKey, string value) {
        stream[idKey - 1] = value;
        vector<string> result;
        while (ptr < stream.size() && !stream[ptr].empty()) {
            result.push_back(stream[ptr]);
            ptr++;
        }
        return result;
    }
};
