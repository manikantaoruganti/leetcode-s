/**
 * @return {Generator<number>}
 */
var fibGenerator = function*() {
    const arr = [0, 0, 1];

  while (true) {

    yield arr[1];

    arr[0] = arr[1];

    arr[1] = arr[2];

    arr[2] = arr[0] + arr[1];

  }
};

/**
 * const gen = fibGenerator();
 * gen.next().value; // 0
 * gen.next().value; // 1
 */