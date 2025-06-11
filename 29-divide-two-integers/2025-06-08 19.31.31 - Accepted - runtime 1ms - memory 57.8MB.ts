function divide(dividend: number, divisor: number): number {
    const INT_MAX = 2 ** 31 - 1;
    const INT_MIN = -(2 ** 31);

    // Special overflow case
    if (dividend === INT_MIN && divisor === -1) return INT_MAX;

    // Basic integer division with truncation toward zero
    return Math.trunc(dividend / divisor);
}
