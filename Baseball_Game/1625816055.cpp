class Solution {
public:
    int calPoints(vector<string>& operations) {
        stack<int> record;

        for (const string& op : operations) {
            if (op == "C") {
                record.pop(); // Remove previous score
            } else if (op == "D") {
                record.push(2 * record.top()); // Double previous score
            } else if (op == "+") {
                int top1 = record.top(); record.pop();
                int top2 = record.top();
                record.push(top1); // push top1 back
                record.push(top1 + top2); // sum of last two
            } else {
                record.push(stoi(op)); // Convert string to int and push
            }
        }

        // Sum all values in the stack
        int sum = 0;
        while (!record.empty()) {
            sum += record.top();
            record.pop();
        }

        return sum;
    }
};
