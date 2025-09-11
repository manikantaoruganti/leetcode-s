/**
 * @param {Function} fn
 * @param {number} t milliseconds
 * @return {Function}
 */
var debounce = function(fn, t) {
    let timer = null;

    return function(...args) {
        if (timer) {
            clearTimeout(timer); // cancel the previous scheduled call
        }

        timer = setTimeout(() => {
            fn(...args); // call the function after delay
        }, t);
    };
};

/**
 * const log = debounce(console.log, 100);
 * log('Hello'); // cancelled
 * log('Hello'); // cancelled
 * log('Hello'); // Logged at t=100ms
 */