#include <functional>
#include <mutex>
#include <condition_variable>

class Foo {
public:
    Foo() : state(0) {}

    void first(std::function<void()> printFirst) {
        std::unique_lock<std::mutex> lock(mtx);
        printFirst();        // print "first"
        state = 1;
        cv.notify_all();     // wake up waiting threads
    }

    void second(std::function<void()> printSecond) {
        std::unique_lock<std::mutex> lock(mtx);
        cv.wait(lock, [this] { return state == 1; }); // wait until first is done
        printSecond();       // print "second"
        state = 2;
        cv.notify_all();     // wake up waiting threads
    }

    void third(std::function<void()> printThird) {
        std::unique_lock<std::mutex> lock(mtx);
        cv.wait(lock, [this] { return state == 2; }); // wait until second is done
        printThird();        // print "third"
    }

private:
    int state;
    std::mutex mtx;
    std::condition_variable cv;
};
