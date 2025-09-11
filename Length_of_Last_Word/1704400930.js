/**
 * @param {string} s
 * @return {number}
 */
var lengthOfLastWord = function(s) {
    s = s.trim(); // Remove leading/trailing spaces
    let lastSpaceIndex = s.lastIndexOf(' ');
    return s.length - lastSpaceIndex - 1;
};
