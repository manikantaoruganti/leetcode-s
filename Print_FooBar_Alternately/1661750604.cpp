#include <semaphore.h>
#include <functional>

class FooBar {
public:
    FooBar(int n) : n(n) {
        sem_init(&fooSem, 0, 1); // foo goes first
        sem_init(&barSem, 0, 0); // bar waits
    }

    ~FooBar() {
        sem_destroy(&fooSem);
        sem_destroy(&barSem);
    }

    void foo(std::function<void()> printFoo) {
        for (int i = 0; i < n; ++i) {
            sem_wait(&fooSem);    // wait for foo turn
            printFoo();           // print "foo"
            sem_post(&barSem);    // allow bar to run
        }
    }

    void bar(std::function<void()> printBar) {
        for (int i = 0; i < n; ++i) {
            sem_wait(&barSem);    // wait for bar turn
            printBar();           // print "bar"
            sem_post(&fooSem);    // allow foo to run again
        }
    }

private:
    int n;
    sem_t fooSem, barSem;
};
