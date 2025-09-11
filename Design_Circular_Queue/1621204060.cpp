
class MyCircularQueue {
public:
    /** Initialize your data structure here. Set the size of the queue to be k. */
    MyCircularQueue(int k) : capacity(k), queue(k), frontIndex(0), backIndex(k - 1), currentSize(0) {}

    /** Insert an element into the circular queue. Return true if the operation is successful. */
    bool enQueue(int value) {
        if (isFull()) {
            return false;
        }

        backIndex = (backIndex + 1) % capacity;
        queue[backIndex] = value;
        ++currentSize;
        return true;
    }

    /** Delete an element from the circular queue. Return true if the operation is successful. */
    bool deQueue() {
        if (isEmpty()) {
            return false;
        }

        frontIndex = (frontIndex + 1) % capacity;
        --currentSize;
        return true;
    }

    /** Get the front item from the queue. */
    int Front() {
        return isEmpty() ? -1 : queue[frontIndex];
    }

    /** Get the last item from the queue. */
    int Rear() {
        return isEmpty() ? -1 : queue[backIndex];
    }

    /** Checks whether the circular queue is empty or not. */
    bool isEmpty() {
        return currentSize == 0;
    }

    /** Checks whether the circular queue is full or not. */
    bool isFull() {
        return currentSize == capacity;
    }

private:
    const int capacity;
    vector<int> queue;
    int frontIndex, backIndex, currentSize;
};

/**
 * Your MyCircularQueue object will be instantiated and called as such:
 * MyCircularQueue* obj = new MyCircularQueue(k);
 * bool param_1 = obj->enQueue(value);
 * bool param_2 = obj->deQueue();
 * int param_3 = obj->Front();
 * int param_4 = obj->Rear();
 * bool param_5 = obj->isEmpty();
 * bool param_6 = obj->isFull();
 */