#include <functional>
#include <mutex>
#include <condition_variable>

class ZeroEvenOdd {
private:
    int n;
    int state = 0; // 0: zero, 1: odd, 2: even
    int curr = 1;

    std::mutex mtx;
    std::condition_variable cv;

public:
    ZeroEvenOdd(int n) : n(n) {}

    void zero(std::function<void(int)> printNumber) {
        for (int i = 1; i <= n; ++i) {
            std::unique_lock<std::mutex> lock(mtx);
            cv.wait(lock, [this] { return state == 0; });

            printNumber(0);

            if (curr % 2 == 1)
                state = 1; // allow odd
            else
                state = 2; // allow even

            cv.notify_all();
        }
    }

    void even(std::function<void(int)> printNumber) {
        while (true) {
            std::unique_lock<std::mutex> lock(mtx);
            cv.wait(lock, [this] { return state == 2 || curr > n; });

            if (curr > n) return;

            printNumber(curr);
            ++curr;
            state = 0; // allow zero again
            cv.notify_all();
        }
    }

    void odd(std::function<void(int)> printNumber) {
        while (true) {
            std::unique_lock<std::mutex> lock(mtx);
            cv.wait(lock, [this] { return state == 1 || curr > n; });

            if (curr > n) return;

            printNumber(curr);
            ++curr;
            state = 0; // allow zero again
            cv.notify_all();
        }
    }
};
