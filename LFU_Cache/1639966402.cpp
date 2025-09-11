

/**
 * Your LFUCache object will be instantiated and called as such:
 * LFUCache* obj = new LFUCache(capacity);
 * int param_1 = obj->get(key);
 * obj->put(key,value);
 */
#include <unordered_map>
#include <list>
using namespace std;

class LFUCache {
private:
    int capacity, minFreq;
    unordered_map<int, pair<int, int>> keyToValFreq;       // key -> {value, freq}
    unordered_map<int, list<int>> freqToKeys;              // freq -> list of keys
    unordered_map<int, list<int>::iterator> keyToIter;     // key -> iterator in freq list

    void updateFreq(int key) {
        int freq = keyToValFreq[key].second;
        freqToKeys[freq].erase(keyToIter[key]);
        if (freqToKeys[freq].empty()) {
            freqToKeys.erase(freq);
            if (minFreq == freq) minFreq++;
        }
        freq++;
        keyToValFreq[key].second = freq;
        freqToKeys[freq].push_front(key);
        keyToIter[key] = freqToKeys[freq].begin();
    }

public:
    LFUCache(int capacity) {
        this->capacity = capacity;
        minFreq = 0;
    }

    int get(int key) {
        if (keyToValFreq.find(key) == keyToValFreq.end()) return -1;
        updateFreq(key);
        return keyToValFreq[key].first;
    }

    void put(int key, int value) {
        if (capacity == 0) return;

        if (keyToValFreq.find(key) != keyToValFreq.end()) {
            keyToValFreq[key].first = value;
            updateFreq(key);
            return;
        }

        if (keyToValFreq.size() == capacity) {
            int evictKey = freqToKeys[minFreq].back();
            freqToKeys[minFreq].pop_back();
            if (freqToKeys[minFreq].empty()) freqToKeys.erase(minFreq);
            keyToValFreq.erase(evictKey);
            keyToIter.erase(evictKey);
        }

        keyToValFreq[key] = {value, 1};
        freqToKeys[1].push_front(key);
        keyToIter[key] = freqToKeys[1].begin();
        minFreq = 1;
    }
};
