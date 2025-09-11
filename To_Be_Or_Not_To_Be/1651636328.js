/**
 * @param {any} val
 * @return {Object}
 */
var expect = function(val) {
    return {
        toBe: function(expected) {
            if (val === expected) return true;
            throw new Error("Not Equal");
        },
        notToBe: function(expected) {
            if (val !== expected) return true;
            throw new Error("Equal");
        }
    };
};
/*
 * Usage examples:
 * expect(5).toBe(5);           // returns true
 * expect(5).notToBe(10);       // returns true
 * expect(5).toBe(null);        // throws "Not Equ*/
 