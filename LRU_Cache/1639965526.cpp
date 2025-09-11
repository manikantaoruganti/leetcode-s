#include <unordered_map>

using namespace std;

class LRUCache {

private:

    struct Node {

        int key, value;

        Node* prev;

        Node* next;

        Node(int k, int v) : key(k), value(v), prev(nullptr), next(nullptr) {}

    };

    

    int cap;

    unordered_map<int, Node*> cache;

    Node *head, *tail;

    

    void remove(Node* node) {

        node->prev->next = node->next;

        node->next->prev = node->prev;

    }

    

    void insert(Node* node) {

        node->next = head->next;

        node->prev = head;

        head->next->prev = node;

        head->next = node;

    }

public:

    LRUCache(int capacity) {

        cap = capacity;

        head = new Node(0, 0); // dummy head

        tail = new Node(0, 0); // dummy tail

        head->next = tail;

        tail->prev = head;

    }

    

    int get(int key) {

        if (cache.find(key) != cache.end()) {

            Node* node = cache[key];

            remove(node);

            insert(node);

            return node->value;

        }

        return -1;

    }

    

    void put(int key, int value) {

        if (cache.find(key) != cache.end()) {

            Node* node = cache[key];

            node->value = value;

            remove(node);

            insert(node);

        } else {

            if (cache.size() == cap) {

                Node* lru = tail->prev;

                remove(lru);

                cache.erase(lru->key);

                delete lru;

            }

            Node* node = new Node(key, value);

            insert(node);

            cache[key] = node;

        }

    }

};

/**
 * Your LRUCache object will be instantiated and called as such:
 * LRUCache* obj = new LRUCache(capacity);
 * int param_1 = obj->get(key);
 * obj->put(key,value);
 */