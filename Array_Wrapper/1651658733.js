/**
 * @param {number[]} nums
 * @return {void}
 */
var ArrayWrapper = function(nums) {
    this.arr = nums;
};

/**
 * When used in addition (+), JavaScript uses valueOf()
 * @return {number}
 */
ArrayWrapper.prototype.valueOf = function() {
    return this.arr.reduce((sum, num) => sum + num, 0);
};

/**
 * When used with String(), JavaScript uses toString()
 * @return {string}
 */
ArrayWrapper.prototype.toString = function() {
    return `[${this.arr.join(',')}]`;
};
