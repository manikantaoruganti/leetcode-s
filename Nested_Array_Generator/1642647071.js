/**
 * @param {Array} arr
 * @return {Generator}
 */

var inorderTraversal = function* (arr) {
  const stack = [{ index: 0, value: arr }];

  while (stack.length) {
    const top = stack[stack.length - 1];

    if (top.index >= top.value.length) {
      stack.pop();
      continue;
    }

    const current = top.value[top.index++];
    
    if (typeof current === "number") {
      yield current;
    } else {
      stack.push({ index: 0, value: current });
    }
  }
};


/**
 * const gen = inorderTraversal([1, [2, 3]]);
 * gen.next().value; // 1
 * gen.next().value; // 2
 * gen.next().value; // 3
 */