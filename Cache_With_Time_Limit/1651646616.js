var TimeLimitedCache = function() {
    this.cache = new Map(); // key -> { value, expireAt }
};

/**
 * @param {number} key
 * @param {number} value
 * @param {number} duration
 * @return {boolean}
 */
TimeLimitedCache.prototype.set = function(key, value, duration) {
    const now = Date.now();
    const isExisting = this.cache.has(key) && this.cache.get(key).expireAt > now;

    // Overwrite value and set new expiration
    this.cache.set(key, {
        value: value,
        expireAt: now + duration
    });

    return isExisting;
};

/**
 * @param {number} key
 * @return {number}
 */
TimeLimitedCache.prototype.get = function(key) {
    const now = Date.now();
    const item = this.cache.get(key);

    if (!item || item.expireAt <= now) {
        return -1;
    }

    return item.value;
};

/**
 * @return {number}
 */
TimeLimitedCache.prototype.count = function() {
    const now = Date.now();
    let count = 0;

    for (const [key, { expireAt }] of this.cache.entries()) {
        if (expireAt > now) count++;
    }

    return count;
};
