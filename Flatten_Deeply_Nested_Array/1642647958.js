/**
 * @param {Array} arr
 * @param {number} depth
 * @return {Array}
 */
var flat = function (arr, n) {
  let queue = arr.slice(); // clone input
  let level = 0;

  while (level < n) {
    let temp = [];
    let modified = false;

    for (let i = 0; i < queue.length; ++i) {
      if (Array.isArray(queue[i])) {
        temp.push(...queue[i]);
        modified = true;
      } else {
        temp.push(queue[i]);
      }
    }

    queue = temp;
    if (!modified) break;
    level++;
  }

  return queue;
};
