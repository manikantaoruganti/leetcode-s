class FrontMiddleBackQueue {
public:
    void pushFront(int val) {
        left.push_front(val);
        balance();
    }

    void pushMiddle(int val) {
        if (left.size() == right.size()) {
            right.push_front(val);
        } else {
            left.push_back(val);
        }
    }

    void pushBack(int val) {
        right.push_back(val);
        balance();
    }

    int popFront() {
        if (!left.empty()) {
            int val = left.front();
            left.pop_front();
            balance();
            return val;
        } else if (!right.empty()) {
            int val = right.front();
            right.pop_front();
            balance();
            return val;
        }
        return -1;
    }

    int popMiddle() {
        if (left.empty() && right.empty()) return -1;

        int val;
        if (left.size() == right.size()) {
            val = left.back();
            left.pop_back();
        } else {
            val = right.front();
            right.pop_front();
        }
        return val;
    }

    int popBack() {
        if (right.empty()) return -1;

        int val = right.back();
        right.pop_back();
        balance();
        return val;
    }

private:
    deque<int> left, right;

    void balance() {
        // Maintain: size(left) == size(right) or size(left) == size(right) - 1
        while (left.size() > right.size()) {
            right.push_front(left.back());
            left.pop_back();
        }
        while (right.size() > left.size() + 1) {
            left.push_back(right.front());
            right.pop_front();
        }
    }
};


/**
 * Your FrontMiddleBackQueue object will be instantiated and called as such:
 * FrontMiddleBackQueue* obj = new FrontMiddleBackQueue();
 * obj->pushFront(val);
 * obj->pushMiddle(val);
 * obj->pushBack(val);
 * int param_4 = obj->popFront();
 * int param_5 = obj->popMiddle();
 * int param_6 = obj->popBack();
 */